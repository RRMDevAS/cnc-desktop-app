use std::str::FromStr;

use raylib::{ffi::IsMouseButtonDown, math::Rectangle};
use raylib::prelude::*;

use crate::cnc_ctrl::{CncCtrl};
use crate::cnc_msg::{CncCoordinates};

struct CoordIndicator {
    background: Rectangle,
    label_background: Rectangle,
    label: String,
    coords: f32,
    color: Color,
}

impl CoordIndicator {
    pub fn new(label: &str, color: Color) -> Self {
        CoordIndicator{
            background: Rectangle::new(0.0f32, 0.0f32, 100.0f32, 100.0f32),
            label_background: Rectangle::new(0.0f32, 0.0f32, 100.0f32, 100.0f32),
            label: String::from_str(label).unwrap(),
            coords: 0.0f32,
            color: color,
        }
    }
    pub fn new_with_color_and_position(label: &str, pos: Vector2, size: Vector2, color: Color) -> Self {
        CoordIndicator{
            background: Rectangle::new(pos.x, pos.y, size.x, size.y),
            label_background: Rectangle::new(pos.x, pos.y, size.y, size.y),
            label: String::from_str(label).unwrap(),
            coords: 0.0f32,
            color: color,
        }
    }
    pub fn set_pos(&mut self, pos: Vector2) {
        self.background.x = pos.x;
        self.background.y = pos.y;
        self.label_background.x = pos.x;
        self.label_background.y = pos.y;
    }
    pub fn set_size(&mut self, w: f32, h: f32) {
        self.background.width = w;
        self.background.height = h;
        self.label_background.width = h;
        self.label_background.height = h;
    }
    pub fn set_coords(&mut self, coords: f32) {
        self.coords = coords;
    }
    pub fn draw(&self, d: &mut RaylibDrawHandle, font: &Font) {
        let font_size = self.background.height * 0.75f32;

        d.draw_rectangle_rec(&self.background, Color::WHITE);
        d.draw_rectangle_lines_ex(self.background, (font_size * 0.075f32) as i32, self.color);

        d.draw_rectangle_rec(&self.label_background, self.color);
        
        let text_label = self.label.clone();
        let text_label_size = measure_text_ex(font, text_label.as_str(), font_size, 0f32);
        let position = Vector2::new( self.label_background.x + self.label_background.width * 0.5f32 - text_label_size.x * 0.5f32,
            self.label_background.y + self.label_background.height * 0.5f32 - text_label_size.y * 0.5f32);
        d.draw_text_ex(&font,text_label.as_str(), 
            position, font_size, 0f32, Color::WHITE);
        
        let text_coords = format!("{:3.3}", self.coords);
        let text_coords_size = measure_text_ex(font, text_coords.as_str(), font_size, 0f32);
        let position = Vector2::new( self.background.x + self.background.width - font_size * 0.2f32 - text_coords_size.x,
            self.background.y + self.background.height * 0.5f32 - text_coords_size.y * 0.5f32);
        d.draw_text_ex(&font,text_coords.as_str(), 
            position, font_size, 0f32, Color::BLACK);
        
        
    }
}

struct CncCoordsDisplay{
    background: Rectangle,
    title: String,
    indicators: [CoordIndicator; 3],
    coords: CncCoordinates,
}

impl CncCoordsDisplay {
    pub fn new(title: &str) -> Self {
        CncCoordsDisplay{
            background: Rectangle::new(0f32, 0f32, 10f32, 10f32),
            title: String::from_str(title).unwrap(),
            indicators: [
                CoordIndicator::new("X", Color::RED), 
                CoordIndicator::new("Y", Color::DARKGREEN),
                CoordIndicator::new("Z", Color::BLUE)],
            coords: CncCoordinates::new(),
        }
    }
    pub fn set_coords(&mut self, coords: CncCoordinates) {
        self.coords = coords;
        self.indicators[0].set_coords(self.coords.x);
        self.indicators[1].set_coords(self.coords.y);
        self.indicators[2].set_coords(self.coords.z);

        self.calculate_indicator_positions();
    }
    fn calculate_indicator_positions(&mut self) {
        let position = Vector2::new( self.background.x + self.background.width * 0.333f32 * 0.5f32 - self.indicators[0].background.width * 0.5f32,
            self.background.y + self.background.height * 0.75f32 - self.indicators[0].background.height * 0.5f32);
        self.indicators[0].set_pos(position);

        let position = Vector2::new( self.background.x + self.background.width * 0.333f32 * 1.5f32 - self.indicators[0].background.width * 0.5f32,
            self.background.y + self.background.height * 0.75f32 - self.indicators[0].background.height * 0.5f32);
        self.indicators[1].set_pos(position);

        let position = Vector2::new( self.background.x + self.background.width * 0.333f32 * 2.5f32 - self.indicators[0].background.width * 0.5f32,
            self.background.y + self.background.height * 0.75f32 - self.indicators[0].background.height * 0.5f32);
        self.indicators[2].set_pos(position);
    }
    pub fn set_pos(&mut self, pos: Vector2) {
        self.background.x = pos.x;
        self.background.y = pos.y;

        self.calculate_indicator_positions();
    }
    pub fn set_size(&mut self, w: f32, h: f32) {
        self.background.width = w;
        self.background.height = h;

        for indicator in &mut self.indicators {
            (*indicator).set_size(self.background.width * 0.25f32, self.background.height * 0.45f32);
        }

        self.calculate_indicator_positions();
    }
    pub fn draw(&self, d: &mut RaylibDrawHandle, font: &Font) {
        let font_size = self.background.height * 0.4f32;

        let position = Vector2::new( self.background.x + font_size,
            self.background.y + self.background.height * 0.25f32 - font_size * 0.5f32);

        d.draw_rectangle_rec(&self.background, Color::LIGHTGRAY);
        d.draw_text_ex(&font,self.title.as_str(), position, font_size, 0f32, Color::BLACK);

        for indicator in &self.indicators {
            (*indicator).draw(d, font);
        }
    }
}

struct CncZCoordIndicator {
    pub pos: Vector2,
    size: f32,
    color: Color,
}

impl CncZCoordIndicator {
    pub fn new(size: f32, color: Color) -> Self {
        CncZCoordIndicator{
            pos : Vector2::zero(),
            size,
            color,
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let start = self.pos - Vector2::new(1.0f32 * self.size, 0f32);
        let end = self.pos + Vector2::new(0.15f32 * self.size, 0f32);
        d.draw_line_v(start, end, self.color);
        let start = self.pos - Vector2::new( 0f32, 0.15f32 * self.size);
        let end = self.pos + Vector2::new(0f32, 0.15f32 * self.size);
        d.draw_line_v(start, end, self.color);
        
        d.draw_circle_lines(self.pos.x as i32, self.pos.y as i32, self.size * 0.5f32, self.color);
    }
}

struct CncXyCoordsIndicator {
    pub pos: Vector2,
    size: f32,
    color: Color,
}

impl CncXyCoordsIndicator {
    pub fn new(size: f32, color: Color) -> Self {
        CncXyCoordsIndicator{
            pos : Vector2::zero(),
            size,
            color,
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let start = self.pos - Vector2::new(0.5f32 * self.size, 0f32);
        let end = self.pos + Vector2::new(0.5f32 * self.size, 0f32);
        d.draw_line_v(start, end, self.color);
        let start = self.pos - Vector2::new( 0f32, 0.5f32 * self.size);
        let end = self.pos + Vector2::new(0f32, 0.5f32 * self.size);
        d.draw_line_v(start, end, self.color);

        let size = Vector2::new( 0.4f32 * self.size, 0.4f32 * self.size);
        let start = self.pos - size * 0.5f32;
        d.draw_rectangle_lines(start.x as i32, start.y as i32, size.x as i32, size.y as i32, self.color);
    }
}

struct CncAreaRect{
    rect:   Rectangle,
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
}

impl CncAreaRect {
    pub fn new(rec_width: f32, x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> Self {
        let x_length = x_max - x_min;
        let y_length = y_max - y_min;
        let rect = Rectangle{
            x: 0f32,
            y: 0f32,
            width: rec_width,
            height: rec_width * y_length / x_length,
        };
        CncAreaRect {
            rect,
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.rect.x = x;
        self.rect.y = y;
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, font: &Font) {
        d.draw_rectangle_rec(&self.rect, Color::WHITE);
        d.draw_rectangle_lines_ex(self.rect, 2, Color::BLACK);
    }


    pub fn map_to_screen(&self, coords: &Vector2) -> Vector2 {
        let x_length = self.x_max - self.x_min;
        let y_length = self.y_max - self.y_min;
        let x_ratio = (coords.x - self.x_min) / x_length;
        let y_ratio = (coords.y - self.y_min) / y_length;

        Vector2::new(self.rect.x + self.rect.width * x_ratio, self.rect.y + self.rect.height * y_ratio)
    }

    pub fn map_to_machine(&self, coords: &Vector2) -> Option<Vector2> {
        if !self.rect.check_collision_point_rec(coords) {
            return None;
        }

        let x_length = self.x_max - self.x_min;
        let y_length = self.y_max - self.y_min;

        let x_ratio = (coords.x - self.rect.x) / self.rect.width;
        let y_ratio = (coords.y - self.rect.y) / self.rect.height;

        Some(Vector2::new(self.x_min + x_length * x_ratio, self.y_min + y_length * y_ratio))
    }
}

pub struct CncCtrlUi {
    target_coords           : CncCoordinates,
    current_coords          : CncCoordinates,
    cnc_target_coords       : CncCoordinates,
    cnc_area_xy             : CncAreaRect,
    cnc_area_z              : CncAreaRect,
    current_indicator       : CncXyCoordsIndicator,
    cnc_target_indicator    : CncXyCoordsIndicator,
    target_indicator        : CncXyCoordsIndicator,
    ind_z_current           : CncZCoordIndicator,
    ind_z_cnc_target        : CncZCoordIndicator,
    ind_z_target            : CncZCoordIndicator,
    current_pos_display     : CncCoordsDisplay,
    cnc_target_display      : CncCoordsDisplay,
    target_display          : CncCoordsDisplay,
    rect_btn_send           : Rectangle,
}

impl CncCtrlUi {
    pub fn new() -> CncCtrlUi {
        let area_size = 600.0f32;

        let xy_area = {
            let mut xy_area_mut = CncAreaRect::new(area_size, 0f32, 500f32, 0f32, 500f32);
            xy_area_mut.set_pos(50f32, 100f32);
            xy_area_mut
        };
        let z_area = {
            let mut area_mut = CncAreaRect::new(30f32, 0f32, 150f32/area_size*30f32, 0f32, 150f32);
            area_mut.set_pos( xy_area.rect.x + xy_area.rect.width + area_size * 0.05f32, xy_area.rect.y );
            area_mut
        };

        let left_align = z_area.rect.x + z_area.rect.width + area_size * 0.1f32;
        let top_align = z_area.rect.y;
        let rect_h = area_size * 0.15f32;
        let vert_spacing = rect_h + area_size * 0.033f32;
        let coords_display_w = 1200f32 - xy_area.rect.width;
        let current_pos = {
            let mut display_mut = CncCoordsDisplay::new("CURRENT POSITION");
            display_mut.set_pos( Vector2::new(left_align, top_align + vert_spacing * 0.0f32) );
            display_mut.set_size(coords_display_w, rect_h);
            display_mut
        };
        let cnc_target_display = {
            let mut display_mut = CncCoordsDisplay::new("ACTIVE TARGET");
            display_mut.set_pos( Vector2::new(left_align, top_align + vert_spacing * 1.0f32) );
            display_mut.set_size(coords_display_w, rect_h);
            display_mut
        };
        let target_display = {
            let mut display_mut = CncCoordsDisplay::new("NEW TARGET");
            display_mut.set_pos( Vector2::new(left_align, top_align + vert_spacing * 2.0f32) );
            display_mut.set_size(coords_display_w, rect_h);
            display_mut
        };

        CncCtrlUi{
            target_coords           : CncCoordinates::new(),
            current_coords          : CncCoordinates::new(),
            cnc_target_coords       : CncCoordinates::new(),
            cnc_area_xy             : xy_area,
            cnc_area_z              : z_area,
            current_indicator       : CncXyCoordsIndicator::new(20f32, Color::BLACK),
            cnc_target_indicator    : CncXyCoordsIndicator::new(30f32, Color::DARKGRAY),
            target_indicator        : CncXyCoordsIndicator::new(40f32, Color::GRAY),
            ind_z_current           : CncZCoordIndicator::new(20f32, Color::BLACK),
            ind_z_cnc_target        : CncZCoordIndicator::new(30f32, Color::DARKGRAY),
            ind_z_target            : CncZCoordIndicator::new(40f32, Color::GRAY),
            current_pos_display     : current_pos,
            cnc_target_display      : cnc_target_display,
            target_display          : target_display,
            rect_btn_send           : Rectangle::new(left_align, top_align + vert_spacing * 3.0f32, coords_display_w * 0.3f32, rect_h * 0.75f32),
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, font: &Font, cnc: &mut CncCtrl) {

        if d.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            let mouse_pos = d.get_mouse_position();
            if let Some(machine_coords) = self.cnc_area_xy.map_to_machine(&mouse_pos) {
                self.target_coords.x = machine_coords.x;
                self.target_coords.y = machine_coords.y;
            }
            if let Some(machine_coords) = self.cnc_area_z.map_to_machine(&mouse_pos) {
                self.target_coords.z = machine_coords.y;
            }
        }

        self.current_indicator.pos = self.cnc_area_xy.map_to_screen(&Vector2::new(self.current_coords.x, self.current_coords.y) );
        self.cnc_target_indicator.pos = self.cnc_area_xy.map_to_screen(&Vector2::new(self.cnc_target_coords.x, self.cnc_target_coords.y) );
        self.target_indicator.pos = self.cnc_area_xy.map_to_screen(&Vector2::new(self.target_coords.x, self.target_coords.y) );

        self.ind_z_current.pos = self.cnc_area_z.map_to_screen(&Vector2::new(self.cnc_area_z.x_max, self.current_coords.z) );
        self.ind_z_cnc_target.pos = self.cnc_area_z.map_to_screen(&Vector2::new(self.cnc_area_z.x_max, self.cnc_target_coords.z) );
        self.ind_z_target.pos = self.cnc_area_z.map_to_screen(&Vector2::new(self.cnc_area_z.x_max, self.target_coords.z) );


        if d.gui_button(self.rect_btn_send, Some(rstr!("SEND"))) {
            cnc.set_target_coords(self.target_coords.clone() );
        }

        self.set_current_coords(cnc.current_coords.x, cnc.current_coords.y, cnc.current_coords.z);

        self.cnc_target_coords = cnc.get_target_coords();
        
        self.current_pos_display.set_coords(self.current_coords.clone());
        self.cnc_target_display.set_coords(self.cnc_target_coords.clone());
        self.target_display.set_coords(self.target_coords.clone());


        self.cnc_area_xy.draw(d, font);
        self.cnc_area_z.draw(d, font);
        self.current_indicator.draw(d);
        self.cnc_target_indicator.draw(d);
        self.target_indicator.draw(d);
        self.ind_z_current.draw(d);
        self.ind_z_cnc_target.draw(d);
        self.ind_z_target.draw(d);

        self.current_pos_display.draw(d, font);
        self.cnc_target_display.draw(d, font);
        self.target_display.draw(d, font);
    }

    pub fn set_current_coords(&mut self, x: f32, y: f32, z: f32) {
        self.current_coords.x = x;
        self.current_coords.y = y;
        self.current_coords.z = z;
    }
}