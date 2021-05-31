
use std::{str, time::Duration};
use std::net::{TcpStream, SocketAddr};
use raylib::prelude::*;

pub struct ValueEdit {
    pub rect        : Rectangle,
    pub value       : i32,
    pub min_value   : i32,
    pub max_value   : i32,
    pub text        : String,
    pub edit_mode   : bool,
}

impl ValueEdit {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        ValueEdit{
            rect: Rectangle::new(x, y, w, h),
            value       : 0,
            min_value         : 0,
            max_value         : 10,
            text: String::new(),
            edit_mode: false,
        }
    }
    pub fn update(&mut self, d: &mut RaylibDrawHandle) {
        if d.gui_value_box(self.rect, None, &mut self.value, self.min_value, self.max_value, self.edit_mode) {
            self.text = self.value.to_string();
        }
        // for some reason this needs to come after gui_text_box
        if d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            let mouse_pos = d.get_mouse_position();
            if self.rect.check_collision_point_rec(mouse_pos) {
                self.edit_mode = true;
            } else {
                self.edit_mode = false;
                self.text = self.value.to_string();
            }
        }
    }
}

pub struct GuiIpAddress {
    pub a_ip            : [ValueEdit; 4],
    pub port            : ValueEdit, 
    pub button_rect     : Rectangle,
    rect_ip             : Rectangle,
    rect_port           : Rectangle,
    connecting          : bool,
}

impl GuiIpAddress {
    pub fn new() -> GuiIpAddress {
        let base_x = 100f32;
        let base_y = 200f32;
        let ip_rect_width = 80f32;
        let ip_rect_height = 50f32;
        let margin = ip_rect_height * 0.1f32;
        let a_ip = {
            let mut ip_0 = ValueEdit::new(base_x, base_y, ip_rect_width, ip_rect_height);
            let mut ip_1 = ValueEdit::new(base_x + (ip_rect_width + margin) * 1f32, base_y, ip_rect_width, ip_rect_height);
            let mut ip_2 = ValueEdit::new(base_x + (ip_rect_width + margin) * 2f32, base_y, ip_rect_width, ip_rect_height);
            let mut ip_3 = ValueEdit::new(base_x + (ip_rect_width + margin) * 3f32, base_y, ip_rect_width, ip_rect_height);
            ip_0.value = 127;
            ip_0.min_value = 0;
            ip_0.max_value = 255;
            ip_1.value = 0;
            ip_1.min_value = 0;
            ip_1.max_value = 255;
            ip_2.value = 0;
            ip_2.min_value = 0;
            ip_2.max_value = 255;
            ip_3.value = 1;
            ip_3.min_value = 0;
            ip_3.max_value = 255;
            [ip_0, ip_1, ip_2, ip_3]
        };
        let port = {
            let mut port_mut =ValueEdit::new(base_x + (ip_rect_width + margin * 2f32) * 4f32, base_y, ip_rect_width * 1.5f32, ip_rect_height);
            port_mut.value = 5555;
            port_mut.min_value = 0;
            port_mut.max_value = 255*255;
            port_mut
        };
        let port_rect = port.rect.clone();
        GuiIpAddress{
            a_ip            : a_ip,
            port            : port,
            button_rect     : Rectangle::new(base_x, base_y + ip_rect_height + margin * 4f32, 150.0f32, 30f32),
            rect_ip         : Rectangle::new(base_x - margin, base_y - margin, ip_rect_width * 4f32 + margin * 5f32, ip_rect_height + margin * 2f32),
            rect_port       : Rectangle::new(port_rect.x - margin, port_rect.y - margin, port_rect.width + margin * 2.0f32, port_rect.height + margin * 2.0f32),
            connecting      : false,
        }
    }
}

pub fn configure_ip(d: &mut RaylibDrawHandle, font: &Font, gui: &mut GuiIpAddress) -> Option<TcpStream> {
    if gui.connecting {
        d.gui_set_state(GuiControlState::GUI_STATE_DISABLED);
    } else {
        d.gui_set_state(GuiControlState::GUI_STATE_NORMAL);
    }
    d.draw_rectangle_lines(gui.rect_ip.x as i32, gui.rect_ip.y as i32, gui.rect_ip.width as i32, gui.rect_ip.height as i32, Color::BLACK);
    d.draw_rectangle_lines(gui.rect_port.x as i32, gui.rect_port.y as i32, gui.rect_port.width as i32, gui.rect_port.height as i32, Color::BLACK);
    d.draw_text_ex(&font, 
        "IP Address", 
        Vector2::new(gui.rect_ip.x, gui.rect_ip.y - gui.rect_ip.height / 4f32 * 2f32), 
        gui.rect_ip.height / 2f32, 0.0f32,Color::BLACK);
    d.draw_text_ex(&font, 
        "Port", 
        Vector2::new(gui.rect_port.x, gui.rect_port.y - gui.rect_port.height / 4f32 * 2f32), 
        gui.rect_port.height / 2f32, 
        0.0f32,
        Color::BLACK);
    for i in 0..4 {
        gui.a_ip[i].update(d);
    }
    gui.port.update(d);

    let button_text = {
        if gui.connecting {
            Some(rstr!("CONNECTING"))
        } else {
            Some(rstr!("CONNECT"))
        }
    };

    if d.gui_button(gui.button_rect, button_text) {
        let str_input: String = format!("{}.{}.{}.{}:{}",
            gui.a_ip[0].text.clone(),
            gui.a_ip[1].text.clone(),
            gui.a_ip[2].text.clone(),
            gui.a_ip[3].text.clone(), 
            gui.port.text.clone());
        
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
            gui.connecting = true;
            // let mut tcp_stream: TcpStream;
            println!("Connecting...");
            match TcpStream::connect_timeout(&address, Duration::from_secs(5)) {
            // match TcpStream::connect("192.168.43.184:5555") {
                Ok(stream) => {
                    gui.connecting = false;
                    return Some(stream);
                },
                Err(error) => {
                    gui.connecting = false;
                    println!("Failed to connect to {:?} with error {:?}", saddr, error);
                }
            }
        }
    }

    None
}