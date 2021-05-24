
use std::{str, time::Duration};
use std::net::{TcpStream, SocketAddr};
use raylib::prelude::*;


pub struct GuiIpAddress {
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

pub fn configure_ip(d: &mut RaylibDrawHandle, font: &Font, gui: &mut GuiIpAddress) -> Option<TcpStream> {
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