


use std::io::{Error};
pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
}
impl Color {
    pub fn new(red:u32, green:u32, blue:u32) -> Result<Self, Error> {
        
        let col = Color {
            red: (red as f32)/255.0,
            green: (green as f32)/255.0,
            blue: (blue as f32)/255.0,
        };
        Ok(col)
    }
}

pub struct Pixel {
    id: u32,
    color: Color,
    pic: String,
    pattern: String
}

impl Pixel {
    pub fn new(red:u32, green:u32, blue:u32, pic:String, pattern:String, id: u32) -> Result<Self, Error>{
        let pix = Pixel {
            id,
            color: Color::new(red, green, blue).unwrap(),
            pic,
            pattern
        };
        Ok(pix)
    }
}
pub struct QR {
    dark_colors: Vec<(Pixel, f32)>,
    light_colors: Vec<(Pixel, f32)>,
    pattern: String,
    dimensions: (u32, u32)
}

impl QR {
    pub fn new() -> Result<Self, Error> {
        let pattern = "no".to_string();
        let qr = QR {
            dark_colors: Vec::new(),
            light_colors: Vec::new(),
            pattern,
            dimensions: (0,0)
        };

        Ok(qr)
    }

    pub fn add_color_row (row: Vec<(Color, f32)>){

    }
}