// main.rs

use nalgebra_glm::{Vec3, normalize};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;

mod framebuffer;
mod ray_intersect;
mod sphere; 
mod color;
mod camera;
mod material;
mod light;

use framebuffer::Framebuffer;
use sphere::Sphere;
use color::Color;
use ray_intersect::{Intersect, RayIntersect};
use camera::Camera;
use light::Light;
use material::Material;

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Sphere], light: &Light) -> Color {
    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;  // what is the closest element this ray has hit? 

    for object in objects {
        let tmp = object.ray_intersect(ray_origin, ray_direction);
        if tmp.is_intersecting && 
            tmp.distance < zbuffer { // is this distance less than the previous?
            zbuffer = intersect.distance;  // this is the closest
            intersect = tmp;
        }
    }

    if !intersect.is_intersecting {
        // return default sky box color
        return Color::new(4, 12, 36);
    }
    
    let light_dir = (light.position - intersect.point).normalize();
    let view_dir = (ray_origin - intersect.point).normalize();
    let reflect_dir = reflect(&-light_dir, &intersect.normal);


    let diffuse_intensity = intersect.normal.dot(&light_dir).max(0.0).min(1.0);
    let diffuse = intersect.material.diffuse * intersect.material.albedo[0] * diffuse_intensity * light.intensity;

    let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersect.material.specular);
    let specular = light.color * intersect.material.albedo[1] * specular_intensity * light.intensity;

    diffuse + specular
}

pub fn render(framebuffer: &mut Framebuffer, objects: &[Sphere], camera: &Camera, light: &Light) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI/3.0;
    let perspective_scale = (fov * 0.5).tan();

    // random number generator
    // let mut rng = rand::thread_rng();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            // if rng.gen_range(0.0..1.0) < 0.3 {
            //     // we skip 30% of the points
            //     continue;
            // }

            // Map the pixel coordinate to screen space [-1, 1]
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            // Adjust for aspect ratio and perspective 
            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            // Calculate the direction of the ray for this pixel
            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));

            // Apply camera rotation to the ray direction
            let rotated_direction = camera.basis_change(&ray_direction);

            // Cast the ray and get the pixel color
            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, light);

            // Draw the pixel on screen with the returned color
            framebuffer.set_current_color(pixel_color.to_hex());
            framebuffer.point(x, y);
        }
    }
}


fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Rust Graphics - Raytracer",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    // move the window around
    window.set_position(500, 500);
    window.update();

    // Materiales para las partes del oso
    let brown = Material::new(Color::new(139, 69, 19), 1.0, [0.9, 10.0]);
    
    let white = Material::new(Color::new(255, 255, 255), 1.0, [0.9, 10.0]);

    let black = Material::new(Color::new(0, 0, 0), 1.0,[0.9, 10.0]);


    // Objetos del oso
    let objects = [
        
        // Nariz (negra pequeña)
        Sphere {
            center: Vec3::new(0.0, -0.5, -4.2),
            radius: 0.2,
            material: black,
        },
        // Ojo izquierdo (negro pequeño)
        Sphere {
            center: Vec3::new(-0.4, 0.2, -4.5),
            radius: 0.1,
            material: black,
        },
        // Ojo derecho (negro pequeño)
        Sphere {
            center: Vec3::new(0.4, 0.2, -4.5),
            radius: 0.1,
            material: black,
        },
        // Hocico (blanco grande)
        Sphere {
            center: Vec3::new(0.0, -0.6, -4.5),
            radius: 0.7,
            material: white,
        },
        // Parte interna de la oreja izquierda (blanca)
        Sphere {
            center: Vec3::new(-1.1, 1.1, -4.5),
            radius: 0.4,
            material: white,
        },
        // Parte interna de la oreja derecha (blanca)
        Sphere {
            center: Vec3::new(1.1, 1.1, -4.5),
            radius: 0.4,
            material: white,
        },
        // Oreja izquierda (marrón externa)
        Sphere {
            center: Vec3::new(-1.2, 1.2, -5.0),
            radius: 0.6,
            material: brown,
        },
        // Oreja derecha (marrón externa)
        Sphere {
            center: Vec3::new(1.2, 1.2, -5.0),
            radius: 0.6,
            material: brown,
        },
        // Cabeza principal
        Sphere {
            center: Vec3::new(0.0, 0.0, -5.0),
            radius: 1.5,
            material: brown,
        },
    ];

    // Initialize camera
    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 5.0),  // eye: Initial camera position
        Vec3::new(0.0, 0.0, 0.0),  // center: Point the camera is looking at (origin)
        Vec3::new(0.0, 1.0, 0.0)   // up: World up vector
    );
    let rotation_speed = PI/10.0;

    let light = Light::new(
        Vec3::new(5.0, 5.0, 5.0),
        Color::new(255, 255, 255),
        1.0
    );

    while window.is_open() {
        // listen to inputs
        if window.is_key_down(Key::Escape) {
            break;
        }

        //  camera orbit controls
        if window.is_key_down(Key::Left) {
            camera.orbit(rotation_speed, 0.0);
        }
        if window.is_key_down(Key::Right) {
            camera.orbit(-rotation_speed, 0.0);
        }
        if window.is_key_down(Key::Up) {
            camera.orbit(0.0, -rotation_speed);
        }
        if window.is_key_down(Key::Down) {
            camera.orbit(0.0, rotation_speed);
        }

        // Dibujar los objetos del oso
        render(&mut framebuffer, &objects, &camera, &light);

        // Actualizar la ventana con los contenidos del framebuffer
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
