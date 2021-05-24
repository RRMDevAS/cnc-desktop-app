
use raylib::prelude::*;
use raylib::core;
use raylib::rgui;
use std::{convert, io::prelude::*, net::Shutdown, str::FromStr, str};
use std::io;
use std::net::{TcpStream, SocketAddr, IpAddr};
use std::sync::mpsc;
use serde::{Deserialize, Serialize};

pub enum ECncCtrlMessage {
    eTargetPosition(CncCoordinates),
    eQuit,
}

impl ECncCtrlMessage {
    pub fn get_type_id(&self) -> u8 {
        match self {
            ECncCtrlMessage::eTargetPosition(coords) => 0,
            ECncCtrlMessage::eQuit => 1,
        }
    }
}

pub enum ECncStatusMessage {
    eCurrentPosition(CncCoordinates),
    eStatus,
    eDisconnected,
}

impl ECncStatusMessage {
    pub fn get_type_id(&self) -> u8 {
        match self {
            ECncStatusMessage::eCurrentPosition(coords) => 0,
            ECncStatusMessage::eStatus => 1,
            ECncStatusMessage::eDisconnected => 2,
        }
    }
}

enum ECncCtrlState {
    eOffline,
    eConnected,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CncCoordinates{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl CncCoordinates {
    pub fn new() -> CncCoordinates {
        CncCoordinates{
            x: 0.0f32,
            y: 0f32,
            z: 0f32,
        }
    }
}

// impl convert::Into<[u8;256]> for CncCoordinates {
//     fn into(self) -> [u8;256] {
//         let mut res;



//         res
//     }
// }

pub struct CncCtrl<'a>
{
    e_cnc_ctrl_state: ECncCtrlState,
    target_coords   : CncCoordinates,
    current_coords  : CncCoordinates,
    o_tx            : Option<mpsc::Sender<ECncCtrlMessage>>,
    o_rx            : Option<mpsc::Receiver<ECncStatusMessage>>,
    fn_connect      : Option< Box< dyn FnMut(String) + 'a > >,
    rect_target_bg  : Rectangle,
    rect_target_x   : Rectangle,
    rect_target_y   : Rectangle,
    rect_target_z   : Rectangle,
    rect_current_bg : Rectangle,
    rect_current_x  : Rectangle,
    rect_current_y  : Rectangle,
    rect_current_z  : Rectangle,
    rect_btn_send   : Rectangle,
}

impl<'a> CncCtrl<'a> {
    pub fn new() -> CncCtrl<'a> {
        CncCtrl{
            e_cnc_ctrl_state: ECncCtrlState::eOffline,
            target_coords   : CncCoordinates::new(),
            current_coords  : CncCoordinates::new(),
            o_tx            : None,
            o_rx            : None,
            fn_connect      : None,
            rect_target_bg  : Rectangle::new(100f32, 200f32, 400f32, 550f32),
            rect_target_x   : Rectangle::new(150f32, 300f32, 300f32, 50f32),
            rect_target_y   : Rectangle::new(150f32, 360f32, 300f32, 50f32),
            rect_target_z   : Rectangle::new(150f32, 420f32, 300f32, 50f32),
            rect_current_bg : Rectangle::new(600f32, 200f32, 400f32, 550f32),
            rect_current_x  : Rectangle::new(650f32, 300f32, 300f32, 50f32),
            rect_current_y  : Rectangle::new(650f32, 360f32, 300f32, 50f32),
            rect_current_z  : Rectangle::new(650f32, 420f32, 300f32, 50f32),
            rect_btn_send   : Rectangle::new(150f32, 650f32, 300f32, 60f32),
        }
    }

    // pub fn get_ip_address(&self) -> String {
    //     self.m_str_ip.clone()
    // }

    pub fn draw_ui(&mut self, d: &mut RaylibDrawHandle, font: &Font) {

        // if d.gui_text_box(self.rect_ip, &mut self.ab_ip_buffer, true) {
        //     let str_slice = match str::from_utf8(&mut self.ab_ip_buffer) {
        //         Ok(res) => res,
        //         Err(e) => panic!("invalid utf sequence: {}", e),
        //     };
        //     self.m_str_ip = String::from(str_slice);
        // }

        d.draw_rectangle_rec(&self.rect_target_bg, Color::LIGHTGRAY);
        d.draw_text_ex(&font,"TARGET POSITION", Vector2::new(150f32, 220f32), 30f32, 0f32, Color::BLACK);

        d.draw_rectangle_rec(&self.rect_current_bg, Color::LIGHTGRAY);
        d.draw_text_ex(&font,"CURRENT POSITION", Vector2::new(650f32, 220f32), 30f32, 0f32, Color::BLACK);

        // if d.gui_button(self.button_rect, Some(rstr!("CONNECT"))) {
        //     let str_ip = self.get_ip_address().clone();
        //     if let Some(ref mut ptr_call) = self.fn_connect {
        //         (*ptr_call)(str_ip);
        //         // ptr_call.call_mut( (self.get_ip_address().clone(),) );
        //         // (*call)(self.get_ip_address().clone());
        //         println!("Clicked connect button - running callback");
        //     }
        // }

        self.target_coords.x = d.gui_slider(self.rect_target_x, 
            Some(rstr!("X")), None,
            self.target_coords.x, 0f32, 500f32);
        self.target_coords.y = d.gui_slider(self.rect_target_y, 
            Some(rstr!("Y")), None,
            self.target_coords.y, 0f32, 500f32);
        self.target_coords.z = d.gui_slider(self.rect_target_z, 
            Some(rstr!("Z")), None,
            self.target_coords.z, 0f32, 150f32);

        self.current_coords.x = d.gui_slider(self.rect_current_x, 
            Some(rstr!("X")), None,
            self.current_coords.x, 0f32, 500f32);
        self.current_coords.y = d.gui_slider(self.rect_current_y, 
            Some(rstr!("Y")), None,
            self.current_coords.y, 0f32, 500f32);
        self.current_coords.z = d.gui_slider(self.rect_current_z, 
            Some(rstr!("Z")), None,
            self.current_coords.z, 0f32, 150f32);
        
        d.draw_text_ex(&font,format!("X:{:3.3}", self.target_coords.x).as_str(), 
        Vector2::new(150f32, 500f32), 30f32, 0f32, Color::RED);
        d.draw_text_ex(&font,format!("Y:{:3.3}", self.target_coords.y).as_str(), 
        Vector2::new(150f32, 550f32), 30f32, 0f32, Color::DARKGREEN);
        d.draw_text_ex(&font,format!("Z:{:3.3}", self.target_coords.z).as_str(), 
        Vector2::new(150f32, 600f32), 30f32, 0f32, Color::DARKBLUE);
        
        d.draw_text_ex(&font,format!("X:{:3.3}", self.current_coords.x).as_str(), 
        Vector2::new(650f32, 500f32), 30f32, 0f32, Color::RED);
        d.draw_text_ex(&font,format!("Y:{:3.3}", self.current_coords.y).as_str(), 
        Vector2::new(650f32, 550f32), 30f32, 0f32, Color::DARKGREEN);
        d.draw_text_ex(&font,format!("Z:{:3.3}", self.current_coords.z).as_str(), 
        Vector2::new(650f32, 600f32), 30f32, 0f32, Color::DARKBLUE);

        if d.gui_button(self.rect_btn_send, Some(rstr!("SEND"))) {
            if let Some(ref tx) = self.o_tx {
                match tx.send( ECncCtrlMessage::eTargetPosition(self.target_coords.clone()) ) {
                    Ok( () ) => {
                        println!("Message sent");
                    }, 
                    Err(e) => {
                        println!("Failed to send a message: {:?}", e);
                    }
                }
            }
        }

        if let Some(ref rx) = self.o_rx {
            match rx.try_recv() {
                Ok( res ) => {
                    match res {
                        ECncStatusMessage::eCurrentPosition( current ) => {
                            println!("Received coordinates: {:?}", current.clone());
                            // self.current_coords = current;
                            self.set_current_coords(current.x, current.y, current.z);
                        },
                        ECncStatusMessage::eDisconnected => {

                        },
                        ECncStatusMessage::eStatus => {

                        },
                    }
                },
                Err(e) => {
                    if e==mpsc::TryRecvError::Disconnected {
                        println!("Failed to receive: {:?}", e);
                    }
                },
            }
        }
    }

    pub fn set_current_coords(&mut self, x: f32, y: f32, z: f32) {
        self.current_coords.x = x;
        self.current_coords.y = y;
        self.current_coords.z = z;
    }

    pub fn set_channels(&mut self, tx: mpsc::Sender<ECncCtrlMessage>, rx: mpsc::Receiver<ECncStatusMessage>) {
        self.o_tx = Some(tx);
        self.o_rx = Some(rx);
        self.e_cnc_ctrl_state = ECncCtrlState::eConnected;
    }

    pub fn on_connect_clicked<F: FnMut(String) + 'a>(&mut self, connect_callback: F) {
        self.fn_connect = Some( Box::new(connect_callback) );
    }
}