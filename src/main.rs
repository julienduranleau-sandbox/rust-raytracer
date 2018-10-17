use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use vec2::Vec2;
use vec3::Vec3;

mod vec2;
mod vec3;

struct Screen { width: u32, height: u32 }

#[derive(Debug)]
struct Color { r: f64, g: f64, b: f64 }

struct Camera {
    origin: Vec3,
    look_at: Vec3,
    up_guide: Vec3,
    fov: f64,
    aspect_ratio: f64,
}

struct Ray {
    origin: Vec3,
    direction: Vec3,
}

fn main() {
    let screen = Screen { width: 800, height: 600 };

    let camera = Camera {
        origin: Vec3::new(0.0, 2.0, 1.0),
        look_at: Vec3::new(0.0, -0.2, -8.0),
        up_guide: Vec3::new(0.0, 1.0, 0.0),
        fov: 45.0,
        aspect_ratio: screen.width as f64 / screen.height as f64,
    };

    let mut pixels_str = String::new();

    for y in 0..screen.height {
        for x in 0..screen.width {
            // -1.0 to 1.0
            let normalized_pixel_location = Vec2::new(
                x as f64 / screen.width as f64 * 2.0 - 1.0,
                y as f64 / screen.height as f64 * 2.0 - 1.0
            );

            let color = render_pixel(normalized_pixel_location, &camera);

            let color_str = format!(" {} {} {}", (color.r * 255.0) as u8, (color.g * 255.0) as u8, (color.b * 255.0) as u8);
            pixels_str.push_str(&color_str);
        }
    }

    let v1 = Vec2::new(-1.0, 3.0);
    let v1b = Vec2::new(-1.0, 3.0);
    let v2 = Vec2::new(2.0, 1.0);
    println!("{:?}", &v1 + &v2);
    println!("{:?}", v1 + &v2);
    println!("{:?}", v1b + v2);
    
    write_to_file("out.ppm", pixels_str, &screen);
}

fn write_to_file(path_str: &str, pixels_str: String, screen: &Screen) {
    let ppm_path = Path::new(path_str);
    let output_str  = format!("P3 {} {} 255\n{}\n\n", screen.width, screen.height, pixels_str);

    let mut ppmfile = match File::create(&ppm_path) {
        Err(_why) => panic!("Couldn't create ppm file"),
        Ok(file) => file,
    };

    match ppmfile.write_all(output_str.as_bytes()) {
        Err(_why) => panic!("Couldn't write to file"),
        Ok(_) => println!("Success"),
    }
}

fn render_pixel(normailized_pixel: Vec2, camera: &Camera) -> Color {
    let origin = Vec3::new(camera.origin.x, camera.origin.y, camera.origin.z);
    let direction = Vec3::new(normailized_pixel.x, normailized_pixel.y, 0.0);

    trace_ray(Ray { origin, direction })
}

fn trace_ray(ray: Ray) -> Color {
    let mut c = Color { r: 0.0, g: 0.0, b: 0.0 };
    c.r = (ray.direction.x + 1.0) / 2.0;
    c.g = 0.0;
    c.b = 0.0;
    c
}

