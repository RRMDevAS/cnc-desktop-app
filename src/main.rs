

use cnc_connection::{CncConnection, CncConnectionManager};
use cnc_ctrl::{CncCtrl};
use raylib::prelude::*;
use cnc_ui::{cnc_connection_ui::{GuiIpAddress, configure_ip}, cnc_ctrl_ui::CncCtrlUi};

mod thread_pool;
mod cnc_ctrl;
mod cnc_ui;
mod cnc_connection;
mod cnc_msg;

enum EAppState {
    EConfigureIpAddress,
    ECncControl,
}


fn main() {

    let (mut rl, thread) = raylib::init()
        .size(1920/6*5, 1080/9*8)
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

    // let pool = ThreadPool::new(2);

    let mut cnc_ctrl: CncCtrl = CncCtrl::new();
    let mut cnc_ctrl_ui: CncCtrlUi = CncCtrlUi::new();

    let mut connection_manager = CncConnectionManager::new();

    // let mut e_app_state = EAppState::EConfigureIpAddress;
    let mut e_app_state = EAppState::ECncControl;

    // let mut tcp_stream: TcpStream;

    let mut gui_ip = GuiIpAddress::new();
    
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        d.draw_text_ex(&font,
            "CNC Control", 
            Vector2::new(12.0f32, 12.0f32), 50.0f32, 0f32, Color::BLACK);

        match e_app_state {
            EAppState::EConfigureIpAddress => {
                if let Some(stream) = configure_ip(&mut d, &font, &mut gui_ip) {

                    // let (cnc_ctrl_connection, manager_connection) = CncConnection::new_connected_pair();
                    // cnc_ctrl.set_connection(cnc_ctrl_connection);
                    // let mut connection_manager = CncConnectionManager::new(stream, manager_connection);
                    // pool.execute( move || connection_manager.run() );
                    
                    let connection = connection_manager.run(stream);
                    cnc_ctrl.set_connection(connection);

                    e_app_state = EAppState::ECncControl;
                }
            },
            EAppState::ECncControl => {
                cnc_ctrl.update_status();
                cnc_ctrl_ui.draw(&mut d, &font, &mut cnc_ctrl);
            },
        }

    }
    cnc_ctrl.quit();
}
