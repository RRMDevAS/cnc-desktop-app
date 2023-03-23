use raylib::{prelude::*, text::{Font, FontLoadEx}, RaylibHandle};

use crate::{cnc_ctrl::CncCtrl, cnc_connection::CncConnectionManager};

use super::{cnc_ctrl_ui::CncCtrlUi, cnc_config_ui::CncConfigUi, cnc_connection_ui::{configure_ip, GuiIpAddress}};


pub enum EAppState {
    EConfigureIpAddress,
    ECncControl,
    ECncConfig,
}


pub struct CncUi {
    pub title: String,
    pub font: Font,
    pub app_state: EAppState,
    pub connection: bool,
    pub btn_tabs: [Rectangle; 3],
    pub ip_address: GuiIpAddress,
    pub ctrl_ui: CncCtrlUi,
    pub config_ui: CncConfigUi,
}

impl CncUi {
    pub fn new(rl: &mut RaylibHandle, thread: &raylib::RaylibThread) -> Self {
        let font_char_set = FontLoadEx::Default(127) ;
        let font_24 = rl.load_font_ex(&thread, "./data/fonts/iosevka-fixed-regular.ttf",
        24, font_char_set).expect("Failed to load the font");
        rl.gui_set_font(&font_24);
        
        let x_pos = 600f32;
        let y_pos = 30f32;
        let btn_w = 150f32;
        let btn_h = 50f32;
        let btn_connection = Rectangle::new(x_pos, y_pos, btn_w, btn_h);
        let btn_ctrl = Rectangle::new(x_pos + btn_w * 1f32, y_pos, btn_w, btn_h);
        let btn_config = Rectangle::new(x_pos + btn_w * 2f32, y_pos, btn_w, btn_h);
        
        
        CncUi{
            title: String::from("CNC"),
            font : font_24,
            app_state: EAppState::EConfigureIpAddress,
            connection: false,
            btn_tabs: [btn_connection, btn_ctrl, btn_config],
            ip_address: GuiIpAddress::new(),
            ctrl_ui: CncCtrlUi::new(),
            config_ui: CncConfigUi::new(),
        }
    }
    
    pub fn update(&mut self, d: &mut raylib::prelude::RaylibDrawHandle, cnc: &mut CncCtrl, connection_manager: &mut CncConnectionManager) {

        d.draw_text_ex(&self.font,
            &self.title, 
            Vector2::new(12.0f32, 12.0f32), 50.0f32, 0f32, Color::BLACK);
            
        if d.gui_button(self.btn_tabs[0], Some(rstr!("Connection"))) {
            self.set_state(EAppState::EConfigureIpAddress);
        }
        if d.gui_button(self.btn_tabs[1], Some(rstr!("Control"))) {
            self.set_state(EAppState::ECncControl);
        }
        if d.gui_button(self.btn_tabs[2], Some(rstr!("Configuration"))) {
            self.set_state(EAppState::ECncConfig);
        }
            
        let accent_height = self.btn_tabs[0].height as i32 / 10;
        match self.app_state {
            EAppState::EConfigureIpAddress => {
                d.draw_rectangle( self.btn_tabs[0].x as i32  , (self.btn_tabs[0].y + self.btn_tabs[0].height) as i32 - accent_height, self.btn_tabs[0].width as i32 , accent_height, Color::DARKGRAY);
                if let Some(stream) = configure_ip(d, &self.font, &mut self.ip_address) {
                    
                    let connection = connection_manager.run(stream);
                    cnc.set_connection(connection);

                    // self.app_state = EAppState::ECncControl;
                    self.set_state(EAppState::EConfigureIpAddress);
                }
                
            },
            EAppState::ECncControl => {
                d.draw_rectangle( self.btn_tabs[1].x as i32 , (self.btn_tabs[1].y + self.btn_tabs[0].height)as i32  - accent_height, self.btn_tabs[1].width as i32 , accent_height, Color::DARKGRAY);
                cnc.update_status();
                self.ctrl_ui.draw(d, &self.font, cnc);
            },
            EAppState::ECncConfig => {
                d.draw_rectangle( self.btn_tabs[2].x as i32 , (self.btn_tabs[2].y  + self.btn_tabs[0].height)  as i32 - accent_height, self.btn_tabs[2].width as i32 , accent_height, Color::DARKGRAY);
                cnc.update_status();
                self.config_ui.draw(d, &self.font, cnc);
            },
        }
    }
    
    pub fn set_state(&mut self, state: EAppState) {
        self.app_state = state;
        
        self.title = match self.app_state {
            EAppState::EConfigureIpAddress => {
                String::from("Configure IP Address")
            },
            EAppState::ECncControl => {
                String::from("CNC Control")
            },
            EAppState::ECncConfig => {
                String::from("Config Parameters")
            }
        }
    }
}