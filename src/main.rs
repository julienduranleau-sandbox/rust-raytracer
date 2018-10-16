use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

struct Screen { width: u32, height: u32 }

struct Color { r: u8, g: u8, b: u8 }

fn main() {
    let screen = Screen { width: 2, height: 2 };
    let mut pixels_str = String::new()

    for y in 0..screen.height {
        for x in 0..screen.width {
            let color = render_pixel(x, y);

            let color_str = format!(" {} {} {}", color.r, color.g, color.b);
            pixels_str.push_str(&color_str);
        }
    }

    write_to_file("out.ppm", output_str);
}

fn render_pixel(x: u32, y: u32) -> Color {
    Color { r: 1, g: 2, b: 3 }
}

fn write_to_file(path_str: &str, pixels_str: String) {
    let img_path = Path::new(path_str);
    let ppm_base = format!("P3 {} {} 255", screen.width, screen.height);

    let mut output_str = String::from(ppm_base);
    let mut file = match File::create(&img_path) {
        Err(_why) => panic!("Couldn't create file"),
        Ok(file) => file,
    };

    match file.write_all(output_str.as_bytes()) {
        Err(_why) => panic!("Couldn't write to file"),
        Ok(_) => println!("Success")
    }
}
