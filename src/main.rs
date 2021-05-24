
use std::{collections::vec_deque, env::temp_dir, io::prelude::*, net::Shutdown, str::FromStr, str, time::Duration};
use std::io;
use std::net::{TcpStream, SocketAddr, IpAddr};
use std::sync::{mpsc, Arc};
use std::ffi::CString;
use cnc_ctrl::{CncCtrl, ECncCtrlMessage, ECncStatusMessage};
use thread_pool::ThreadPool;
use raylib::prelude::*;
use raylib::core;
use raylib::rgui;

mod thread_pool;
mod cnc_ctrl;

enum EAppState {
    eConfigureIpAddress,
    eCncControl,
}

struct GuiIpAddress {
    pub m_str_ip        : String,
    pub ab_ip_buffer    : [u8; 30],
    pub rect_ip         : Rectangle,
    pub button_rect     : Rectangle,
}

impl GuiIpAddress {
    pub fn new() -> GuiIpAddress {
        GuiIpAddress{
            m_str_ip        : String::new(),
            ab_ip_buffer    : [0u8; 30],
            rect_ip         : Rectangle::new(100.0f32, 100.0f32, 250.0f32, 50f32),
            button_rect     : Rectangle::new(150.0f32, 150.0f32, 150.0f32, 30f32),
        }
    }
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
    // let font = match rl.load_font(&thread, "./data/fonts/Slabo27px-Regular.ttf") {
    //     Ok(v) => {
    //         println!("Font has been loaded and succesfully at that!!!!!!!!!!!!!");
    //         v
    //     },
    //     Err(e) => {
    //         println!("Font has NOT been loaded succesfully!!!!!!!!!!!!!");
    //         panic!("Failed to load the font: {}", e);
    //     }
    // };

    let pool = ThreadPool::new(2);

    let mut cnc_ctrl: CncCtrl = CncCtrl::new();

    // let mut e_app_state = EAppState::eConfigureIpAddress;
    let mut e_app_state = EAppState::eCncControl;

    let mut tcp_stream: TcpStream;

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
                cnc_ctrl.draw_ui(&mut d, &font);
            },
        }

    }
}

fn configure_ip(d: &mut RaylibDrawHandle, font: &Font, gui: &mut GuiIpAddress) -> Option<TcpStream> {
    if d.gui_text_box(gui.rect_ip, &mut gui.ab_ip_buffer, true) {
        let str_slice: &str = str::from_utf8( &gui.ab_ip_buffer ).unwrap().trim_end_matches('\0');
        gui.m_str_ip = String::from(str_slice);
    }

    if d.gui_button(gui.button_rect, Some(rstr!("CONNECT"))) {
        let str_input: String = gui.m_str_ip.clone();
        
        let mut saddr: Option<SocketAddr> = None;
        let str_trimmed: &str = str_input.trim();
        // assert_eq!(str_trimmed, "192.168.43.184:5555");
        match str_trimmed.parse() {
        // match SocketAddr::from_str("192.168.43.184:5555") {
            Ok(sock_addres) => {
                saddr = Some(sock_addres);
            },
            Err(parse_error) => {
                println!("error parsing the input [{}] with error '{:?}'", str_trimmed, parse_error);
            },
        }

        if let Some(address) = saddr {
            // let mut tcp_stream: TcpStream;
            println!("Connecting...");
            match TcpStream::connect_timeout(&address, Duration::from_secs(5)) {
            // match TcpStream::connect("192.168.43.184:5555") {
                Ok(stream) => {
                    return Some(stream);
                },
                Err(error) => {
                    println!("Failed to connect to {:?} with error {:?}", saddr, error);
                }
            }
        }
    }

    None
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
                    println!("Received {} bytes", res);
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
        // match io::stdin().read_line(&mut str_input) {
        //     Ok(lenght) => {
        //         // str_input.trim();
        //         match str_input.trim() {
        //             "quit" => {
        //                 stream.shutdown(Shutdown::Both).unwrap();
        //                 x_connected = false;
        //             },
        //             _ => {
        //                 stream.write(str_input.as_bytes()).unwrap();
        //                 stream.flush().unwrap();
        //             },
        //         }
        //     },
        //     Err(error) => {
        //         println!("error reading the input: {:?}", error);
        //     },
        // }
    }
}