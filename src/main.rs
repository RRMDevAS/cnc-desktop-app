
use std::{io::prelude::*, net::Shutdown, time::Duration};
use std::io;
use std::net::TcpStream;
use std::sync::mpsc;
use cnc_ctrl::{CncCtrl, ECncCtrlMessage, ECncStatusMessage};
use thread_pool::ThreadPool;
use raylib::prelude::*;
use cnc_ui::{cnc_connection_ui::{GuiIpAddress, configure_ip}, cnc_ctrl_ui::CncCtrlUi};

mod thread_pool;
mod cnc_ctrl;
mod cnc_ui;

enum EAppState {
    eConfigureIpAddress,
    eCncControl,
}


fn main() {

    let (mut rl, thread) = raylib::init()
        .size(1920/3*2, 1080/3*2)
        .title("CNC Control")
        .resizable()
        .build();
    
    rl.set_target_fps(60);
    
    let font_char_set = FontLoadEx::Default(127) ;
    let font = rl.load_font_ex(&thread, "./data/fonts/iosevka-fixed-regular.ttf",
            64, font_char_set).expect("Failed to load the font");
    let font_char_set = FontLoadEx::Default(127) ;
    let font_24 = rl.load_font_ex(&thread, "./data/fonts/iosevka-fixed-regular.ttf",
    24, font_char_set).expect("Failed to load the font");
    rl.gui_set_font(&font_24);

    let pool = ThreadPool::new(2);

    let mut cnc_ctrl: CncCtrl = CncCtrl::new();
    let mut cnc_ctrl_ui: CncCtrlUi = CncCtrlUi::new();

    let mut e_app_state = EAppState::eConfigureIpAddress;
    // let mut e_app_state = EAppState::eCncControl;

    // let mut tcp_stream: TcpStream;

    let mut gui_ip = GuiIpAddress::new();
    
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        d.draw_text_ex(&font,
            "CNC Control", 
            Vector2::new(12.0f32, 12.0f32), 50.0f32, 0f32, Color::BLACK);

        match e_app_state {
            EAppState::eConfigureIpAddress => {
                if let Some(stream) = configure_ip(&mut d, &font, &mut gui_ip) {
                    let (tx_ctrl, rx_ctrl) = mpsc::channel();
                    let (tx_status, rx_status) = mpsc::channel();
                    cnc_ctrl.set_channels(tx_ctrl, rx_status);
                    pool.execute( || {
                        handle_connection(stream, tx_status, rx_ctrl);
                    });
                    e_app_state = EAppState::eCncControl;
                }
            },
            EAppState::eCncControl => {
                cnc_ctrl.update_status();
                cnc_ctrl_ui.draw(&mut d, &font, &mut cnc_ctrl);
            },
        }

    }
}


fn handle_connection(mut stream: TcpStream, tx: mpsc::Sender<ECncStatusMessage>, rx: mpsc::Receiver<ECncCtrlMessage>) {
    // let mut str_input = String::new();
    let mut x_connected = true;
    stream.set_read_timeout( Some(Duration::from_millis(100)) ).unwrap();
    let mut ab_recv_buffer: [u8; 512] = [0; 512];
    while x_connected {
        // println!("enter message to send or 'quit' to quit");
        // str_input.clear();
        match rx.try_recv() {
            Ok(msg) => {
                let u_type_id = msg.get_type_id();
                match msg {
                    ECncCtrlMessage::eTargetPosition(coords) => {
                        match bincode::serialize(&coords) {
                            Ok(mut vec) => {
                                let payload = {
                                    let mut temp_vec = Vec::from( [u_type_id; 1] );
                                    temp_vec.append(&mut vec);
                                    temp_vec
                                };
                                match stream.write(payload.as_slice()) {
                                    Ok(bytes_written) => {
                                        if bytes_written!=payload.len() {
                                            println!("Failed to send all bytes: sent {} of {}", bytes_written, vec.len());
                                        } else {
                                            println!("Sent {:?}", payload);
                                        }
                                        stream.flush().unwrap();
                                    },
                                    Err(e) => {
                                        println!("Error sending: {:?}", e);
                                    }
                                }
                            },
                            Err(e) => {
                                println!("Failed to serialize the coords: {:?}", e);
                            }
                        }
                    },
                    ECncCtrlMessage::eQuit => {
                        stream.shutdown(Shutdown::Both).unwrap();
                        x_connected = false;
                    },
                }
            },
            Err(e) => {
                if e==mpsc::TryRecvError::Disconnected {
                    stream.shutdown(Shutdown::Both).unwrap();
                    x_connected = false;
                    println!("Stream thread receive error: {:?}", e);
                }
            },
        }


        match stream.read( &mut ab_recv_buffer ) {
            Ok( res ) => {
                if res>0 {
                    // println!("Received {} bytes", res);
                    let status_type = ab_recv_buffer[0];
                    if status_type==0 {
                        match bincode::deserialize(&ab_recv_buffer[1..]) {
                            Ok(res) => {
                                match tx.send(ECncStatusMessage::eCurrentPosition(res)) {
                                    Ok( () ) => {
    
                                    },
                                    Err(e) => {
                                        println!("Error sending a received message {:?}", e);
                                    },
                                }
                            },
                            Err(e) => {
                                println!("Error deserializeing {:?}", *e);
                            },
                        }
                    } else {
                        println!("Unknown status received: {}", status_type);
                    }
                }
            },
            Err(e) => {
                if e.kind()!=io::ErrorKind::WouldBlock {
                    println!("Error receiving {:?}", e);
                }
            },
        }
    }
}