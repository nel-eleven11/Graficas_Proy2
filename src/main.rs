// main.rs

use nalgebra_glm::{Vec3, normalize};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;

mod framebuffer;
mod ray_intersect;
mod sphere; 
mod color;

use framebuffer::Framebuffer;
use sphere::Sphere;
use color::Color;
use ray_intersect::{Intersect, RayIntersect, Material};

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Sphere]) -> Color {
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
    
    let diffuse = intersect.material.diffuse;

    diffuse
}

pub fn render(framebuffer: &mut Framebuffer, objects: &[Sphere]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI/3.0;
    let perspective_scale = (fov * 0.5).tan();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            // Map the pixel coordinate to screen space [-1, 1]
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            // Adjust for aspect ratio and perspective 
            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            // Calculate the direction of the ray for this pixel
            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));

            // Cast the ray and get the pixel color
            let pixel_color = cast_ray(&Vec3::new(0.0, 0.0, 0.0), &ray_direction, objects);

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
        "Rust Graphics - Oso Renderizado",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    // move the window around
    window.set_position(500, 500);
    window.update();

    // Materiales para las partes del oso
    let brown = Material {
        diffuse: Color::new(139, 69, 19), // Marrón para la cabeza y orejas
    };

    let white = Material {
        diffuse: Color::new(255, 255, 255), // Blanco para el hocico y parte interna de las orejas
    };

    let black = Material {
        diffuse: Color::new(0, 0, 0), // Negro para los ojos y la nariz
    };

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

    while window.is_open() {
        // Escuchar entradas
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Dibujar los objetos del oso
        render(&mut framebuffer, &objects);

        // Actualizar la ventana con los contenidos del framebuffer
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
