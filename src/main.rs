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

struct CameraSize {
    width: f64,
    height: f64,
}

struct Camera {
    origin: Vec3,
    target: Vec3,
    forward: Vec3,
    right: Vec3,
    up: Vec3,
    size: CameraSize,
}

struct Ray {
    origin: Vec3,
    direction: Vec3,
}

struct Sphere {
    position: Vec3,
    radius: f64,
}

fn main() {
    let screen = Screen { width: 800, height: 600 };

    let origin = Vec3::new(0.0, 2.0, 1.0);
    let target = Vec3::new(0.0, -0.2, -8.0);
    let up_guide = Vec3::new(0.0, 1.0, 0.0);
    let fov = 45.0;
    let aspect_ratio = screen.width as f64 / screen.height as f64;

    let camera = create_camera(origin, target, fov, aspect_ratio, up_guide);

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

    let v1 = Vec3::new(1.0, 2.0, 0.0);
    let v2 = Vec3::new(2.0, 3.0, 0.0);
    println!("{:?}", v1.dot(&v2));

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
    let camera_ray = create_ray_from_camera(&camera, &normailized_pixel);

    trace_ray(camera_ray)
}

fn create_camera(origin: Vec3, target: Vec3, fov: f64, aspect_ratio: f64, up_guide: Vec3) -> Camera {
    let height = (fov.to_radians() * 0.5).tan();
    let width = height * aspect_ratio;
    let forward = (&target - &origin).unit(); 
    let right = forward.cross(&up_guide).unit();
    let up = right.cross(&forward).unit();
    let size = CameraSize { width, height };

    Camera {
        origin,
        target,
        forward, 
        right,
        up,
        size,
    }
}

fn create_ray_from_camera(camera :&Camera, px: &Vec2) -> Ray {
    let xOffset = camera.right.clone() * px.x * camera.size.width;
    let yOffset = camera.up.clone() * px.y * camera.size.height;
    let rayDirection = camera.forward.clone() + xOffset + yOffset;

    Ray {
        origin: camera.origin.clone(),
        direction: rayDirection,
    }
}

fn trace_ray(ray: Ray) -> Color {
    let mut c = Color { r: 0.0, g: 0.0, b: 0.0 };
    let sphere = Sphere {
        position: Vec3::new(1.0, 0.0, -6.0),
        radius: 1.0,
    };

    if (intersect_sphere(&ray, &sphere)) {
        c.r = 1.0;
        c.g = 0.0;
        c.b = 0.0;
    }

    c
}

fn intersect_plane(ray: &Ray) -> bool {
    let plane_normal = Vec3::new(0.0, 1.0, 0.0);
    // First, check if we intersect
    let d_dot_n = ray.direction.dot(&plane_normal);

    d_dot_n != 0.0
}


fn intersect_sphere(ray: &Ray, sphere: &Sphere) -> bool {
    // Transform ray so we can consider origin-centred sphere
    let local_ray = Ray {
        origin: &ray.origin - &sphere.position,
        direction: ray.direction.clone(),
    };

    // Calculate quadratic coefficients
    let a = local_ray.direction.dot(&local_ray.direction);
    let b = 2.0 * local_ray.direction.dot(&local_ray.origin);
    let c = local_ray.origin.dot(&local_ray.origin) - sphere.radius * sphere.radius;

    // Check whether we intersect
    let discriminant = b * b - 4.0 * a * c;

    if (discriminant < 0.0) {
        return false;
    }

    // Find two points of intersection, t1 close and t2 far
    let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
    let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

    // First check if close intersection is valid
    if (t1 > 0.0000001 /* && t1 < intersection.t*/) {
        //intersection.t = t1;
        //intersection.mat = sphere.mat;
        //intersection.normal = normalize(ray.origin + ray.direction t1 - sphere.position);
        return true;
    } else {
        // Neither are valid
        return false;
    }
}
