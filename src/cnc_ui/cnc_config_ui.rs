use std::{str::FromStr, fmt::Debug};

use raylib::prelude::*;

use crate::{cnc_ctrl::CncCtrl, cnc_msg::PIDParams};

pub struct ValueInput<T: Default + ToString + FromStr + Copy + Debug > 
    where T: FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug
{
    pub rect        : Rectangle,
    pub value       : T,
    buffer          : Vec<u8>,
    pub edit_mode   : bool,
}

impl<T: Default + ToString + FromStr + Copy + Debug> ValueInput<T> 
    where T: FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug
{
    pub fn new(x: f32, y: f32, w: f32, h: f32, initial_value: T) -> Self {
        let text = initial_value.to_string();
        let mut buffer = text.as_bytes().to_vec();
        buffer.append(&mut Vec::from(['\0' as u8]));
        ValueInput{
            rect        : Rectangle::new(x, y, w, h),
            value       : initial_value,
            buffer      : buffer,
            edit_mode   : false,
        }
    }
    pub fn update(&mut self, d: &mut RaylibDrawHandle) {
        
        let mut buffer = self.buffer.clone();
        buffer.append(&mut Vec::from(['\0' as u8]));
        if d.gui_text_box(self.rect, buffer.as_mut_slice(), self.edit_mode) {
            if let Ok(text_input) = String::from_utf8( buffer.clone().into() ) {
                println!("Converting |{:?}|", text_input.trim_matches('\0').trim());
                match T::from_str(text_input.trim().trim_matches('\0').trim()) {
                    Ok(value) => {
                        self.value = value;
                        println!("Value assigned {:?}", value);
                    },
                    Err(e) => {
                        println!("Error converting {:?}", e);
                        println!("Value reset {:?}", self.value);
                    }
                }
                self.buffer = self.value.to_string().as_bytes().to_vec();
                println!("Text assigned {}", text_input);
            }
        } else {
            self.buffer = buffer;
        }
        
        // for some reason this needs to come after gui_text_box
        if d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            let mouse_pos = d.get_mouse_position();
            if self.rect.check_collision_point_rec(mouse_pos) {
                self.edit_mode = true;
            } else {
                self.edit_mode = false;
                // self.text = self.value.to_string();
                self.buffer = self.value.to_string().as_bytes().to_vec();
            }
        }
    }
}

pub struct  CncAxisConfigUi {
    pub rect_bg: Rectangle,
    pub axis: u8,
    pub current_params: PIDParams,
    pub new_params: PIDParams,
    pub rect_rows : [Rectangle; 4],
    pub rect_columns : [Rectangle; 2],
    pub inputs : [ValueInput<f32>; 3],
}

impl CncAxisConfigUi {
    pub fn new(axis: u8, x: f32, y: f32, w: f32, h: f32) -> Self {
        let row_height = h * 0.2f32;
        let col_width = w * 0.5f32;
        let row_spacing = 0f32;
        let col_spacing = 0f32;
        
        let title_row = Rectangle::new(x, y + row_height * 1f32 + row_spacing * 1f32, col_width * 2f32 + col_spacing * 1f32, row_height);
        let p_row = Rectangle::new(x, y + row_height * 2f32 + row_spacing * 2f32, col_width * 2f32 + col_spacing * 1f32, row_height);
        let i_row = Rectangle::new(x, y + row_height * 3f32 + row_spacing * 3f32, col_width * 2f32 + col_spacing * 1f32, row_height);
        let d_row = Rectangle::new(x, y + row_height * 4f32 + row_spacing * 4f32, col_width * 2f32 + col_spacing * 1f32, row_height);
        
        let curr_col = Rectangle::new(x + col_width * 0f32 + col_spacing * 0f32, y+ row_height * 1f32 + row_spacing * 1f32, col_width, row_height * 4f32 + row_spacing * 3f32);
        let new_col = Rectangle::new(x + col_width * 1f32 + col_spacing * 1f32, y+ row_height * 1f32 + row_spacing * 1f32, col_width, row_height * 4f32 + row_spacing * 3f32);
        
        let p_input = ValueInput::new(x + col_width * 1f32 + col_spacing * 1f32, y + row_height * 2f32 + row_spacing * 2f32, col_width, row_height, 0.0f32);
        let i_input = ValueInput::new(x + col_width * 1f32 + col_spacing * 1f32, y + row_height * 3f32 + row_spacing * 3f32, col_width, row_height, 0.0f32);
        let d_input = ValueInput::new(x + col_width * 1f32 + col_spacing * 1f32, y + row_height * 4f32 + row_spacing * 4f32, col_width, row_height, 0.0f32);
        
        CncAxisConfigUi {
            rect_bg: Rectangle::new(x, y, w, h),
            axis,
            current_params: PIDParams::new(),
            new_params: PIDParams::new(),
            rect_rows: [title_row, p_row, i_row, d_row],
            rect_columns: [curr_col, new_col],
            inputs: [p_input, i_input, d_input],
        }
    }
    
    pub fn draw(&mut self, d: &mut RaylibDrawHandle, font: &Font, cnc: &mut CncCtrl) {
        let (bg_color, axis_name) = match self.axis {
            0 => (Color::RED,   "X Axis"),
            1 => (Color::DARKGREEN, "Y Axis"),
            2 => (Color::BLUE,  "Z Axis"),
            _ => (Color::GRAY,  "Unknown Axis"),
        };
        d.draw_rectangle_rec(self.rect_bg, bg_color);
        d.draw_text_rec(font, axis_name, self.rect_bg, self.rect_bg.height as f32 * 0.2f32, 0f32, false, Color::WHITE);
        
        d.draw_rectangle_rec(self.rect_rows[0], Color::LIGHTGRAY);
        d.draw_rectangle_rec(self.rect_rows[1], Color::RAYWHITE);
        d.draw_rectangle_rec(self.rect_rows[2], Color::LIGHTGRAY);
        d.draw_rectangle_rec(self.rect_rows[3], Color::RAYWHITE);
        
        d.draw_rectangle_lines_ex(self.rect_columns[0], 2, Color::DARKGRAY);
        d.draw_rectangle_lines_ex(self.rect_columns[1], 2, Color::DARKGRAY);
        
        self.inputs[0].update(d);
        self.inputs[1].update(d);
        self.inputs[2].update(d);
        
        self.new_params.prop = self.inputs[0].value;
        self.new_params.inte = self.inputs[1].value;
        self.new_params.deri = self.inputs[2].value;
        
        d.draw_text_rec(font, "CURRENT", self.rect_rows[0], self.rect_rows[0].height as f32 * 0.5f32, 0f32, false, Color::DARKGRAY);
        d.draw_text_rec(font, "NEW CONFIG", self.rect_columns[1], self.rect_rows[0].height as f32 * 0.5f32, 0f32, false, Color::DARKGRAY);
        
        d.draw_text_rec(font, self.current_params.prop.to_string().as_str(), self.rect_rows[1], self.rect_rows[0].height as f32 * 0.5f32, 0f32, false, Color::DARKGRAY);
        d.draw_text_rec(font, self.current_params.deri.to_string().as_str(), self.rect_rows[2], self.rect_rows[0].height as f32 * 0.5f32, 0f32, false, Color::DARKGRAY);
        d.draw_text_rec(font, self.current_params.inte.to_string().as_str(), self.rect_rows[3], self.rect_rows[0].height as f32 * 0.5f32, 0f32, false, Color::DARKGRAY);
    }
}

pub struct CncConfigUi {
    pub axis_params: [CncAxisConfigUi; 3],
    pub rect_button_set_params : Rectangle,
}

impl CncConfigUi {
    pub fn new() -> Self {
        let row_height = 70f32;
        let col_width = 300f32;
        let row_spacing = 10f32;
        let col_spacing = 10f32;
        
        let x_axis = CncAxisConfigUi::new(0, 100f32 + col_width * 0f32 + col_spacing * 0f32, 100f32, col_width, row_height * 3f32 + row_spacing * 2f32);
        let y_axis = CncAxisConfigUi::new(1, 100f32 + col_width * 1f32 + col_spacing * 1f32, 100f32, col_width, row_height * 3f32 + row_spacing * 2f32);
        let z_axis = CncAxisConfigUi::new(2, 100f32 + col_width * 2f32 + col_spacing * 2f32, 100f32, col_width, row_height * 3f32 + row_spacing * 2f32);
        
        let button_rect = Rectangle::new(100f32 + col_width * 2f32 + col_spacing * 2f32, row_height * 6f32 + row_spacing * 4f32, col_width, row_height );
        
        CncConfigUi {
            axis_params: [x_axis, y_axis, z_axis],
            rect_button_set_params: button_rect,
        }
    }
    
    pub fn draw(&mut self, d: &mut RaylibDrawHandle, font: &Font, cnc: &mut CncCtrl) {
        self.axis_params[0].current_params = cnc.pid_params[0].clone();
        self.axis_params[1].current_params = cnc.pid_params[1].clone();
        self.axis_params[2].current_params = cnc.pid_params[2].clone();
        
        self.axis_params[0].draw(d, font, cnc);
        self.axis_params[1].draw(d, font, cnc);
        self.axis_params[2].draw(d, font, cnc);
        
        if d.gui_button(self.rect_button_set_params, Some(rstr!("SET PARAMS"))) {
            println!("Setting params...");
            cnc.set_pid_params( &self.axis_params[0].new_params, &self.axis_params[1].new_params, &self.axis_params[2].new_params);
        }
    }
}