use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;
use std::env;
use regex::Regex;
use happyQR::{QR, Pixel};


fn main() {
    let mut args: Vec<String> = env::args().collect();
    let qr_text = String::from(args.pop().unwrap());
    let re = Regex::new("--(.*)").unwrap();
    // let re2 = Regex::new("[=|:](\\D*):(\\d+),(\\d+),(\\d+)").unwrap();
    // let re3 = Regex::new("[=|:](\\D*),(\\d*):(\\d+),(\\d+),(\\d+)").unwrap();
    let mut dark_colors: Vec<(Pixel, f32)> = vec![];
    let mut light_colors: Vec<(Pixel, f32)> = vec![];
    let mut mode: u32 = 0;
    let mut typ: u32 = 0;
    let mut pix: Pixel = Pixel::new(0,0,0,"".to_string(),"".to_string(), 0).unwrap();
    for arg in args {
        if &arg[0..1] == "/" {
            pix = Pixel::new(0,0,0,"".to_string(),"".to_string(), 0).unwrap();
            typ = 0;
            // let (_, [op, str]) = re.captures(arg.as_str()).unwrap().extract();
            match arg.as_str() {
                "/dark"=> mode = 1,
                "/light"=> mode = 2,
                _ => println!("{} is not a valid argument", arg)
            }
        }
        else if typ == 0 {
            match arg.as_str() {
                "col" => typ = 10,
                "img" => typ = 20,
                "pat" => typ = 30,
                _ => println!("Must specify one of following for color, image, or jgraph pattern string:\n 
                /[dark|light] col [0-100] [0-100] [0-100]\n
                /[dark|light] img [pathToFile]\n
                /[dark|light] pat [jgraphPatternString]")
            }
        }
        else if mode > 0 && typ > 0 {
            match typ {
                10 => {
                    typ = 11;
                },
                20 => {
                    typ = 21;
                },
                21 => {
                    todo!()
                }
                30 => {
                    typ = 31;
                },
                31 => {
                    todo!()
                }
                _ => todo!(),
            }
        }
        else {
            todo!()
        }
        
    }
    

    if let Ok(lines) = read_lines("./hosts.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            println!("{}", line);
        }
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}