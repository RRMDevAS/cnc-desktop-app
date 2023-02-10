

use cnc_connection::{CncConnection, CncConnectionManager};
use cnc_ctrl::{CncCtrl};
use raylib::prelude::*;
use cnc_ui::{cnc_connection_ui::{GuiIpAddress, configure_ip}, cnc_ctrl_ui::CncCtrlUi, cnc_config_ui::CncConfigUi, cnc_ui::CncUi};

mod thread_pool;
mod cnc_ctrl;
mod cnc_ui;
mod cnc_connection;
mod cnc_msg;

fn main() {

    let (mut rl, thread) = raylib::init()
        .size(1920/6*5, 1080/9*8)
        .title("CNC Control")
        .resizable()
        .build();
    
    rl.set_target_fps(60);

    let mut cnc_ctrl: CncCtrl = CncCtrl::new();

    let mut connection_manager = CncConnectionManager::new();

    let mut cnc_ui = CncUi::new(&mut rl, &thread);
    
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        cnc_ui.update(&mut d, &mut cnc_ctrl, &mut connection_manager);
    }
    cnc_ctrl.quit();
}
