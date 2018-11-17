use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use vec2::Vec2;
use vec3::Vec3;

mod vec2;
mod vec3;

const RAY_MIN_LENGTH: f64 = 0.000000001;
const INFINITY: f64 = 100000000000.0;

const IOR_AIR: f64 = 1.00;
// const IOR_WATER: f64 = 1.3333;
// const IOR_ICE: f64 = 1.31;
const IOR_GLASS: f64 = 1.52;
// const IOR_DIAMOND: f64 = 2.42;

#[derive(Debug)]
struct Screen { width: u32, height: u32 }

#[derive(Debug)]
struct Color { r: f64, g: f64, b: f64 }

#[derive(Debug)]
struct CameraSize {
    width: f64,
    height: f64,
}

#[derive(Debug)]
struct Camera {
    origin: Vec3,
    target: Vec3,
    forward: Vec3,
    right: Vec3,
    up: Vec3,
    size: CameraSize,
}

#[derive(Debug, Clone)]
struct Material {
    color: Vec3,
    damping: f64,
    reflectivity: f64,
    refractivity: f64,
    ior: f64,
}

#[derive(Debug)]
struct Light {
    position: Vec3,
    color: Vec3,
    force: f64,
}

#[derive(Debug)]
struct Scene {
    lights: Vec<Light>,
    spheres: Vec<Sphere>,
    planes: Vec<Plane>,
}

#[derive(Debug)]
struct Ray {
    origin: Vec3,
    direction: Vec3,
}

#[derive(Debug)]
struct RayIntersection {
    t: f64,
    normal: Vec3,
    material: Material,
}

#[derive(Debug)]
struct Sphere {
    position: Vec3,
    radius: f64,
    material: Material,
}

#[derive(Debug)]
struct Plane {
    position: Vec3,
    normal: Vec3,
    material: Material,
}

fn main() {
    let screen = Screen { width: 800, height: 600 };

    let origin = Vec3::new(0.0, -0.6, 2.0);
    let target = Vec3::new(0.0, 0.0, 0.0);
    let up_guide = Vec3::new(0.0, 1.0, 0.0);
    let fov = 45.0;
    let aspect_ratio = screen.width as f64 / screen.height as f64;

    let camera = create_camera(origin, target, fov, aspect_ratio, up_guide);
    let scene = create_scene();

    let mut pixels_str = String::new();

    for y in 0..screen.height {
        for x in 0..screen.width {
            // -1.0 to 1.0
            let normalized_pixel_location = Vec2::new(
                x as f64 / screen.width as f64 * 2.0 - 1.0,
                y as f64 / screen.height as f64 * 2.0 - 1.0
            );

            let color = render_pixel(normalized_pixel_location, &camera, &scene);

            let color_str = format!(" {} {} {}", (color.r * 255.0) as u8, (color.g * 255.0) as u8, (color.b * 255.0) as u8);
            pixels_str.push_str(&color_str);
        }
    }

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

fn render_pixel(normailized_pixel: Vec2, camera: &Camera, scene: &Scene) -> Color {
    let camera_ray = create_ray_from_camera(&camera, &normailized_pixel);
    trace_ray(camera_ray, &scene)
}

fn create_scene() -> Scene {
    let sphere1 = Sphere {
        position: Vec3::new(0.0, 1.0, -3.0),
        radius: 0.9,
        material: Material {
            color: Vec3::new(1.0, 0.2, 0.2),
            damping: 1.0,
            reflectivity: 0.0,
            refractivity: 0.0,
            ior: IOR_GLASS,
        },
    };
    let sphere2 = Sphere {
        position: Vec3::new(-1.0, 1.0, -2.0),
        radius: 0.4,
        material: Material {
            color: Vec3::new(0.0, 0.7, 0.0),
            damping: 1.0,
            reflectivity: 0.5,
            refractivity: 0.0,
            ior: IOR_GLASS,
        },
    };
    let sphere3 = Sphere {
        position: Vec3::new(1.0, 1.2, -2.0),
        radius: 0.6,
        material: Material {
            color: Vec3::new(0.5, 0.5, 0.5),
            damping: 1.0,
            reflectivity: 1.0,
            refractivity: 1.0,
            ior: IOR_GLASS,
        },
    };
    let plane_floor = Plane {
        position: Vec3::new(0.0, 1.9, 0.0),
        normal: Vec3::new(0.0, -1.0, 0.0),
        material: Material {
            color: Vec3::new(0.1, 0.1, 0.1),
            damping: 1.0,
            reflectivity: 0.4,
            refractivity: 1.0,
            ior: IOR_GLASS,
        },
    };
    let plane_ceiling = Plane {
        position: Vec3::new(0.0, -6.5, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        material: Material {
            color: Vec3::new(0.2, 0.2, 0.2),
            damping: 0.0,
            reflectivity: 0.0,
            refractivity: 0.0,
            ior: 0.0,
        },
    };
    let plane_left = Plane {
        position: Vec3::new(-2.3, 0.0, 0.0),
        normal: Vec3::new(-1.0, 0.0, 0.0),
        material: Material {
            color: Vec3::new(0.2, 0.2, 0.2),
            damping: 0.0,
            reflectivity: 0.0,
            refractivity: 0.0,
            ior: 0.0,
        },
    };
    let plane_right = Plane {
        position: Vec3::new(2.3, 0.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
        material: Material {
            color: Vec3::new(0.2, 0.2, 0.2),
            damping: 0.0,
            reflectivity: 0.0,
            refractivity: 0.0,
            ior: 0.0,
        },
    };
    let plane_backwall = Plane {
        position: Vec3::new(0.0, 0.0, -3.5),
        normal: Vec3::new(0.0, 0.0, 1.0),
        material: Material {
            color: Vec3::new(0.2, 0.2, 0.2),
            damping: 0.0,
            reflectivity: 0.0,
            refractivity: 0.0,
            ior: 0.0,
        },
    };
    let plane_wall = Plane {
        position: Vec3::new(0.0, 0.0, 4.0),
        normal: Vec3::new(0.0, 0.0, -1.0),
        material: Material {
            color: Vec3::new(0.2, 0.2, 0.2),
            damping: 0.0,
            reflectivity: 0.0,
            refractivity: 0.0,
            ior: 0.0,
        },
    };
    let light1 = Light {
        position: Vec3::new(2.2, -6.0, 0.0),
        color: Vec3::new(1.0, 1.0, 1.0),
        force: 50.0
    };
    let light2 = Light {
        position: Vec3::new(-2.2, -6.0, 2.0),
        color: Vec3::new(1.0, 1.0, 1.0),
        force: 39.0
    };

    let lights = vec![light1, light2];
    let spheres = vec![sphere1, sphere2, sphere3];
    let planes = vec![plane_ceiling, plane_floor, plane_wall, plane_backwall, plane_left, plane_right];

    Scene {
        lights,
        spheres,
        planes,
    }
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
    let x_offset = camera.right.clone() * px.x * camera.size.width;
    let y_offset = camera.up.clone() * px.y * camera.size.height;
    let ray_direction = camera.forward.clone() + x_offset + y_offset;

    Ray {
        origin: camera.origin.clone(),
        direction: ray_direction,
    }
}

fn trace_ray(initial_ray: Ray, scene: &Scene) -> Color {
    let mut final_color = Vec3::new(0.10, 0.10, 0.11);
    let mut frac = 1.0;
    let mut ray = initial_ray;

    for _raybounce in 0..6 {
        let mut next_ray: Option<Ray> = None;
        let mut next_frac = 1.0;
        let mut break_bounce_loop = false;
        let intersection = trace(&ray, &scene);
        
        if intersection.t >= INFINITY {
            break_bounce_loop = true;
        } else {
            let ray_hit = &ray.origin + &(&ray.direction * intersection.t);
            let mut diffuse = Vec3::new(0.0, 0.0, 0.0);
            let mut specular = Vec3::new(0.0, 0.0, 0.0);
            let refraction = Vec3::new(0.0, 0.0, 0.0);

            for light in &scene.lights {
                let ray_hit_to_light = &light.position - &ray_hit;
                let light_direction = ray_hit_to_light.unit();
                let light_dst_sq = ray_hit_to_light.dot(&ray_hit_to_light);
                let distance_fade = 1.0 / light_dst_sq;

                // === Diffuse
                let mut brightness = 0.2 * distance_fade;
                
                let light_ray = Ray {
                    origin: &ray_hit + &(&intersection.normal * 0.001),
                    direction: light_direction.unit(),
                };
                let light_intersection = trace(&light_ray, &scene);

                if light_intersection.t * light_intersection.t > light_dst_sq {
                    let mut light_on_surface = intersection.normal.dot(&light_direction);
                    if light_on_surface < 0.0 {
                        light_on_surface = 0.0;
                    }
                    brightness += 0.8 * light_on_surface * (light.force * distance_fade);
                }

                diffuse = diffuse + &intersection.material.color * brightness;
                
                // === Specular
                let reflected_light_direction = light_direction.reflect(&intersection.normal);
                let mut specular_factor = reflected_light_direction.dot(&ray.direction);
                if specular_factor < 0.0 {
                    specular_factor = 0.0;
                }
                let damped_specular = specular_factor.powf(intersection.material.damping);
                specular = specular + damped_specular * distance_fade;

                // === Reflection
                if intersection.material.reflectivity > 0.0 {
                    next_ray = Some(Ray {
                        origin: ray_hit.clone(),
                        direction: ray.direction.reflect(&intersection.normal),
                    });
                    next_frac *= intersection.material.reflectivity;
                } else if intersection.material.refractivity > 0.0 {
                    next_ray = Some(Ray {
                        origin: &ray_hit + &(&ray.direction * 0.0001),
                        direction: ray.direction.refract(&intersection.normal, IOR_AIR / intersection.material.ior),
                    });
                    next_frac *= intersection.material.refractivity;
                } else {
                    break_bounce_loop = true;
                }
            }

            let diff_spec_refrac = (diffuse + specular).mix(&refraction, intersection.material.refractivity);

            final_color = final_color + diff_spec_refrac * frac;
        }

        if break_bounce_loop || frac < 0.01 {
            break;
        } else  {
            if let Some(next_ray_safe) = next_ray {
                frac = next_frac;
                ray = next_ray_safe;
            } else {
                break;
            }
        }
    }

    if final_color.x > 1.0 { final_color.x = 1.0; }
    if final_color.y > 1.0 { final_color.y = 1.0; }
    if final_color.z > 1.0 { final_color.z = 1.0; }

    Color { r: final_color.x, g: final_color.y, b: final_color.z }
}

fn trace(ray: &Ray, scene: &Scene) -> RayIntersection {
    let mut closest_intersection = RayIntersection {
        t: INFINITY,
        normal: Vec3::new(0.0, 0.0, 0.0),
        material: Material{
            color: Vec3::new(0.0, 0.0, 0.0),
            damping: 0.0,
            reflectivity: 0.0,
            refractivity: 0.0,
            ior: 0.0,
        }
    };

    for sphere in &scene.spheres {
        match intersect_sphere(ray, sphere) {
            Ok(intersection) => {
                if intersection.t < closest_intersection.t {
                    closest_intersection = intersection;
                }
            },
            _ => {}
        }
    }

    for plane in &scene.planes {
        match intersect_plane(ray, plane) {
            Ok(intersection) => {
                if intersection.t < closest_intersection.t {
                    closest_intersection = intersection;
                }
            },
            _ => {}
        }
    }

    closest_intersection
}

fn intersect_plane(ray: &Ray, plane: &Plane) -> Result<RayIntersection, RayError> {
    // First, check if we intersect
    let d_dot_n = ray.direction.dot(&plane.normal);

    if d_dot_n == 0.0 {
        // We just assume the ray is not embedded in the plane
        return Err(RayError::NoIntersections)
    }

    // Find point of intersection
    let t = (&plane.position - &ray.origin).dot(&plane.normal) / d_dot_n;

    if t <= RAY_MIN_LENGTH || t >= INFINITY {
        // Outside relevant range
        return Err(RayError::NoIntersections)
    }

    Ok(RayIntersection {
        t: t,
        normal: plane.normal.clone(),
        material: plane.material.clone(),
    })
}

enum RayError {
    NoIntersections
}

fn intersect_sphere(ray: &Ray, sphere: &Sphere) -> Result<RayIntersection, RayError> {
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

    if discriminant < 0.0 {
        return Err(RayError::NoIntersections)
    }

    // Find two points of intersection, t1 close and t2 far
    let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
    // let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

    // First check if close intersection is valid
    if t1 > RAY_MIN_LENGTH {
        Ok(RayIntersection {
            t: t1,
            normal: (&ray.origin + &(&ray.direction * t1) - &sphere.position).unit(),
            material: sphere.material.clone(),
        })
    } else {
        // Neither are valid
        return Err(RayError::NoIntersections)
    }
}
