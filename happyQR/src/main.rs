use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::process::exit;
use std::process::Command;
use happyQR::{Color, PixelTemplate, QR};


fn main() {
    let mut qr = QR::new().unwrap();
    let mut id: u32 = 0;
    let mut args: VecDeque<String> = env::args().collect();
    if args.len() < 2 {println!("Program usage: ./happyQR [options] TextToEncode");}
    
    let qr_text = args.pop_back().unwrap();
    args.pop_front(); //Removes executable argument
    qr.make_qr(qr_text.as_str());
    Command::new("convert")
        .arg("temp.png")
        .arg("-compress")
        .arg("none")
        .arg("temp.ppm")
        .output()
        .expect("failed to execute process");
    
    let mut dark_colors: Vec<(PixelTemplate, i32)> = vec![];
    dark_colors.push((PixelTemplate::new(0, 0, 0, String::new(), String::new(), 0, 0).unwrap(), 100));
    let mut light_colors: Vec<(PixelTemplate, i32)> = vec![];
    light_colors.push((PixelTemplate::new(255, 255, 255, String::new(), String::new(), 1, 0).unwrap(), 100));
    let mut mode: u32 = 0;
    let mut typ: u32 = 0;
    let (mut red, mut green, mut blue)= (0, 0, 0);
    let mut perc:i32 = 0;
    let mut output = String::from("testing.jgr");

    
    for arg in args {
        // println!("Arg is {}", arg);
        if &arg[0..1] == "-" {
            // pair = (Pixel::new(0,0,0,"".to_string(),"".to_string(), 0).unwrap(), 0.0);
            typ = 0;
            // let (_, [op, str]) = re.captures(arg.as_str()).unwrap().extract();
            match arg.as_str() {
                "--dark"=> mode = 1,
                "--light"=> mode = 2,
                "--output"=> {mode = 3; typ = 3;},
                _ => println!("{} is not a valid argument", arg)
            }
        }
        else if typ == 0 && typ < 3{
            match arg.as_str() {
                "col" => typ = 10,
                "img" => typ = 20,
                "pat" => typ = 30,
                _ => {println!("Must specify one of following for color, image, or jgraph pattern string:\n 
                --[dark|light] col [percent] [0-255] [0-255] [0-255]\n
                --[dark|light] img [percent] [pathToFile]\n
                --[dark|light] pat [percent] [jgraphPatternString]\n
                For pat the part you supply is what's in brackets in this example pattern , the start and pts at the end are provided:\n
                newline poly \"[linethickness 5 color 1 1 0 pcfill 1 0 1 ppattern stripe 60]\" pts  4 4  4 6  6 6  6 4");
                exit(0);}
            }
        }
        else if mode > 0 && typ > 0 {
            match typ {
                3 => {
                    output = arg.to_string();
                }
                10 | 20 | 30 => {
                    perc = arg.parse::<i32>().unwrap();
                    typ += 1;
                },
                11 => {
                    red = arg.parse::<u32>().unwrap();
                    typ += 1;
                },
                12 => {
                    green = arg.parse::<u32>().unwrap();
                    typ += 1;
                },
                13 => {
                    blue = arg.parse::<u32>().unwrap();
                    if mode == 1 {
                        id = dark_colors.len() as u32 * 2;
                        dark_colors.push((PixelTemplate::new(red, green, blue, String::new(), String::new(), id, 0).unwrap(), perc));
                        dark_colors[0].1 -= perc;
                    }
                    else {
                        id = (light_colors.len() as u32 * 2) + 1;
                        light_colors.push((PixelTemplate::new(red, green, blue, String::new(), String::new(), id, 0).unwrap(), perc));
                        light_colors[0].1 -= perc;
                    }
                    typ += 1;
                    // id += 1;
                },
                21 => {
                    Command::new("convert")
                    .arg(arg.clone())
                    .arg("-compress")
                    .arg("None")
                    .arg("-resize")
                    .arg("10%")
                    .arg("eps2:".to_owned() + arg.clone().as_str() + ".eps")
                    .output().expect("failed to execute process");

                    if mode == 1 {
                        id = dark_colors.len() as u32 * 2;
                        dark_colors.push((PixelTemplate::new(0, 0, 0, arg.to_string(), String::new(), id, 1).unwrap(), perc));
                        dark_colors[0].1 -= perc;
                    }
                    else {
                        id = (light_colors.len() as u32 * 2) + 1;
                        light_colors.push((PixelTemplate::new(0, 0, 0, arg.to_string(), String::new(), id, 1).unwrap(), perc));
                        light_colors[0].1 -= perc;
                    }
                    // id +=1;
                },
                31 => {
                    if mode == 1 {
                        id = dark_colors.len() as u32 * 2;
                        dark_colors.push((PixelTemplate::new(0, 0, 0, String::new(), arg.to_string(), id, 2).unwrap(), perc));
                        dark_colors[0].1 -= perc;
                    }
                    else {
                        id = (light_colors.len() as u32 * 2) + 1;
                        light_colors.push((PixelTemplate::new(0, 0, 0, String::new(), arg.to_string(), id, 2).unwrap(), perc));
                        light_colors[0].1 -= perc;
                    }
                    // id +=1;
                }
                _ => todo!(),
            }
        }
        else {
            todo!()
        }
        
    }
    if light_colors[0].1 < 0 || dark_colors[0].1 < 0 {
        println!("Percentages specified over 100 percent");
        exit(0);
    }
    // println!("percentage at end is {}", light_colors[0].1);
    // println!("percentage at end is {}", dark_colors[0].1);
    qr.dark_colors = dark_colors;
    qr.light_colors = light_colors;
    

    let mut first:u32 = 0;
    let mut pixels:Vec<(Color, u32)> = Vec::new();
    let mut nums:Vec<u32> = Vec::new();
    let mut size:usize = 0;
    
    // let mut count = 0;
    if let Ok(lines) = read_lines("./temp.ppm") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            // println!("Count is {}", count);
            // count += 1;
            // println!("{}", line);
            if first == 0 || first == 2{first += 1; continue;}
            nums = line.split_whitespace()
            .map(|n| {
                n.parse::<u32>().unwrap()
            }).collect::<Vec<u32>>();
            if first == 1 {first += 1; size = nums[0] as usize; qr.dimensions = size as u32; continue;}
            let mut i = 0;
            // println!("Len of nums is {}", nums.len());
            while i < nums.len() {
                pixels.push((Color::new(nums[i], nums[i+1], nums[i+2]).unwrap(), nums[i]/255));
                i+=3;
                if pixels.len() == size {
                    qr.add_pixels(&pixels);
                    pixels = Vec::new();
                }
            }
            // println!("Size is {}", pixels.len());
        }
    }
    qr.gen_colors();
    qr.draw_jgraph(output);
    // println!("{:?}", pixels);
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}