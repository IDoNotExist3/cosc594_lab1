use qrcode_generator::QrCodeEcc;

use std::{fs::{self, OpenOptions}, io::{Error, Write}};
#[derive(Debug, Clone)]
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

pub struct PixelTemplate {
    typ: u32,
    id: u32,
    color: Color,
    pic: String,
    pattern: String
}

impl PixelTemplate {
    pub fn new(red:u32, green:u32, blue:u32, pic:String, pattern:String, id: u32, typ: u32) -> Result<Self, Error>{
        let pix = PixelTemplate {
            typ,
            id,
            color: Color::new(red, green, blue).unwrap(),
            pic,
            pattern
        };
        Ok(pix)
    }
}
pub struct QR {
    pub dark_colors: Vec<(PixelTemplate, i32)>,
    pub light_colors: Vec<(PixelTemplate, i32)>,
    pattern: String,
    pub dimensions: u32,
    pixels: Vec<Vec<(Color, u32)>>
}

impl QR {
    pub fn new() -> Result<Self, Error> {
        let pattern = "no".to_string();
        let qr = QR {
            dark_colors: Vec::new(),
            light_colors: Vec::new(),
            pattern,
            dimensions: 0,
            pixels: Vec::new()
        };

        Ok(qr)
    }

    pub fn add_pixels (& mut self, row: &Vec<(Color, u32)>){
        println!("Size is {}", row.len());
        self.pixels.push(row.clone());
    }

    pub fn make_qr(&self, str: &str) {
        qrcode_generator::to_png_to_file(str, QrCodeEcc::Low, 23, "temp.png").unwrap();
    }

    pub fn draw_jgraph(&self) {
        let mut strings: Vec<String> = Vec::new();
        let mut min = 0; let mut count = 0;
        let mut x = self.dimensions*2;
        println!("x starts at {}", x);
        // let max = 0;
        let mut id:u32 = self.pixels[0][0].1;
        for row in &self.pixels {
            for pix in row {
                count+=2;
                if id != pix.1 {
                    if id % 2 == 0 {
                        strings.push(
                            format!("newline poly pcfill {} {} {} color {} {} {} pts {} {}  {} {}  {} {}  {} {}",
                            self.dark_colors[(id/2) as usize].0.color.red,
                            self.dark_colors[(id/2) as usize].0.color.green,
                            self.dark_colors[(id/2) as usize].0.color.blue,
                            self.dark_colors[(id/2) as usize].0.color.red,
                            self.dark_colors[(id/2) as usize].0.color.green,
                            self.dark_colors[(id/2) as usize].0.color.blue,
                            min, x, count, x, count, x-2, min, x-2,) 
                        );
                    }
                    else {
                        strings.push(
                            format!("newline poly pcfill {} {} {} color {} {} {} pts {} {}  {} {}  {} {}  {} {}",
                            self.light_colors[(id/2) as usize].0.color.red,
                            self.light_colors[(id/2) as usize].0.color.green,
                            self.light_colors[(id/2) as usize].0.color.blue,
                            self.light_colors[(id/2) as usize].0.color.red,
                            self.light_colors[(id/2) as usize].0.color.green,
                            self.light_colors[(id/2) as usize].0.color.blue,
                            min, x, count, x, count, x-2, min, x-2,) 
                        );
                    }
                    
                    id = pix.1;
                    min = count;
                }
                
            }
            if min != count {
                strings.push(
                    format!("newline poly pcfill {} {} {} color {} {} {} pts {} {}  {} {}  {} {}  {} {}",
                    self.dark_colors[id as usize].0.color.red,
                    self.dark_colors[id as usize].0.color.green,
                    self.dark_colors[id as usize].0.color.blue,
                    self.dark_colors[id as usize].0.color.red,
                    self.dark_colors[id as usize].0.color.green,
                    self.dark_colors[id as usize].0.color.blue,
                    min, x, count, x, count, x-2, min, x-2,) 
                );
            }
            min = 0;
            x -= 2; count = 0;
        }
        println!("strings length is {}", strings.len());
        let joined = strings.join("\n");
        // fs::write("./testing.jgr");
        fs::write("./testing.jgr", "newgraph\nxaxis min 0 max 46 nodraw\nyaxis min 0 max 46 nodraw\n").expect("Unable to write file");
        let mut file = OpenOptions::new().append(true).open("./testing.jgr").unwrap();
        file.write_all(joined.as_bytes()).unwrap();
        
        // println!("{}",joined);
    }
    
}


