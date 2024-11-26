// main.rs

use nalgebra_glm::{Vec3, normalize};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;
use rayon::prelude::*;

mod framebuffer;
mod ray_intersect;
mod color;
mod camera;
mod material;
mod light;
mod texture;
mod cube;
mod diorama;

use framebuffer::Framebuffer;
use color::Color;
use ray_intersect::{Intersect, RayIntersect};
use camera::Camera;
use light::Light;
//use material::Material;
// use texture::Texture;
use cube::Cube;
use diorama::generate_diorama;

const ORIGIN_BIAS: f32 = 1e-4;
// el skybox debe tener un color azul oscuro
const SKYBOX_COLOR: Color = Color::new(40, 24, 128);

fn offset_origin(intersect: &Intersect, direction: &Vec3) -> Vec3 {
    let offset = intersect.normal * ORIGIN_BIAS;
    if direction.dot(&intersect.normal) < 0.0 {
        intersect.point - offset
    } else {
        intersect.point + offset
    }
}

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

fn refract(incident: &Vec3, normal: &Vec3, eta_t: f32) -> Vec3 {
    let cosi = -incident.dot(normal).max(-1.0).min(1.0);
    
    let (n_cosi, eta, n_normal);

    if cosi < 0.0 {
        // Ray is entering the object
        n_cosi = -cosi;
        eta = 1.0 / eta_t;
        n_normal = -normal;
    } else {
        // Ray is leaving the object
        n_cosi = cosi;
        eta = eta_t;  // Assuming it's going back into air with index 1.0
        n_normal = *normal;
    }
    
    let k = 1.0 - eta * eta * (1.0 - n_cosi * n_cosi);
    
    if k < 0.0 {
        // Total internal reflection
        reflect(incident, &n_normal)
    } else {
        eta * incident + (eta * n_cosi - k.sqrt()) * n_normal
    }
}

fn cast_shadow(
    intersect: &Intersect,
    light: &Light,
    objects: &[Cube],
) -> f32 {
    let light_dir = (light.position - intersect.point).normalize();
    //let light_distance = (light.position - intersect.point).magnitude();

    let shadow_ray_origin = offset_origin(intersect, &light_dir);
    //let mut shadow_intensity = 0.0;

    /* 
    for object in objects {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
        if shadow_intersect.is_intersecting && shadow_intersect.distance < light_distance {
            let distance_ratio = shadow_intersect.distance / light_distance;
            shadow_intensity = 1.0 - distance_ratio.powf(2.0).min(1.0);
            break;
        }
    }*/

    if objects.iter().any(|object| {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
        shadow_intersect.is_intersecting
    }) {
        return 1.0; 
    } else {
        return 0.0; 
    }
     

    //shadow_intensity
}

pub fn cast_ray(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    objects: &[Cube],
    light: &Light,
    depth: u32,
) -> Color {
    if depth > 1 {
        return SKYBOX_COLOR;
    }

    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    let closest = objects
    .par_iter()
    .filter_map(|object| {
        let i = object.ray_intersect(ray_origin, ray_direction);
        if i.is_intersecting {
            Some(i)
        } else {
            None
        }
    })
    .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    if let Some(i) = closest {
        intersect = i;
    }

    if !intersect.is_intersecting {
        return SKYBOX_COLOR;
    }

    // Add emission directly if the material is emissive
    let mut result_color = intersect.material.emission;

    let light_dir = (light.position - intersect.point).normalize();
    let view_dir = (ray_origin - intersect.point).normalize();
    let reflect_dir = reflect(&-light_dir, &intersect.normal).normalize();

    let shadow_intensity = cast_shadow(&intersect, light, objects);
    let light_intensity = light.intensity * (1.0 - shadow_intensity);

    let diffuse_intensity = intersect.normal.dot(&light_dir).max(0.0).min(1.0);
    let diffuse_color = intersect.material.get_diffuse_color(intersect.u, intersect.v);
    let diffuse = diffuse_color * intersect.material.albedo[0] * diffuse_intensity * light_intensity;

    let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersect.material.specular);
    let specular = light.color * intersect.material.albedo[1] * specular_intensity * light_intensity;

    let mut reflect_color = Color::black();
    let reflectivity = intersect.material.albedo[2];
    if reflectivity > 0.0 {
        let reflect_dir = reflect(&ray_direction, &intersect.normal).normalize();
        let reflect_origin = offset_origin(&intersect, &reflect_dir);
        reflect_color = cast_ray(&reflect_origin, &reflect_dir, objects, light, depth + 1);
    }

    let mut refract_color = Color::black();
    let transparency = intersect.material.albedo[3];
    if transparency > 0.0 {
        let refract_dir = refract(&ray_direction, &intersect.normal, intersect.material.refractive_index);
        let refract_origin = offset_origin(&intersect, &refract_dir);
        refract_color = cast_ray(&refract_origin, &refract_dir, objects, light, depth + 1);
    }

    // Combine emissive color with other effects
    result_color += (diffuse + specular) * (1.0 - reflectivity - transparency)
        + (reflect_color * reflectivity)
        + (refract_color * transparency);

    result_color
}


pub fn render(framebuffer: &mut Framebuffer, objects: &[Cube], camera: &Camera, light: &Light) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI/3.0;
    let perspective_scale = (fov * 0.5).tan();

    // Parallel iteration over rows (y-axis)
    framebuffer
        .buffer
        .par_chunks_mut(framebuffer.width) // Divide the buffer into rows
        .enumerate() // Keep track of row index (y)
        .for_each(|(y, row)| {
            for (x, pixel) in row.iter_mut().enumerate() {
                // Map the pixel coordinate to screen space [-1, 1]
                let screen_x = (2.0 * x as f32) / width - 1.0;
                let screen_y = -(2.0 * y as f32) / height + 1.0;

                // Adjust for aspect ratio and perspective
                let screen_x = screen_x * aspect_ratio * perspective_scale;
                let screen_y = screen_y * perspective_scale;

                // Calculate the direction of the ray for this pixel
                let ray_direction = nalgebra_glm::normalize(&Vec3::new(screen_x, screen_y, -1.0));

                // Apply camera rotation to the ray direction
                let rotated_direction = camera.basis_change(&ray_direction);

                // Cast the ray and get the pixel color
                let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, light, 0);

                // Set the pixel color in the framebuffer
                *pixel = pixel_color.to_hex();
            }
        });
}

fn main() {
    let window_width = 500;
    let window_height = 350;
    let framebuffer_width = 500;
    let framebuffer_height = 350;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Rust Graphics - Proyect 2 Raytracer",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    // move the window around
    window.set_position(500, 500);
    window.update();

    
    // Generate scene
    let objects = generate_diorama();


    // Initialize camera
    let mut camera = Camera::new(
        Vec3::new(10.0, 15.0, 15.0),  // eye: Initial camera position
        Vec3::new(2.5, 3.0, 2.5),  // center: Point the camera is looking at (origin)
        Vec3::new(0.0, 1.0, 0.0)   // up: World up vector
    );

    let rotation_speed = PI/10.0;
    let zoom_speed = 0.5;

    // Initialize light
    let mut light = Light::new(
        Vec3::new(5.0, 10.0, 5.0),
        Color::new(255, 255, 200),
        2.5
    );

    // Frame counter
    let mut frame = 0; // Initialize the frame counter

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

        // camera zoom controls
        if window.is_key_down(Key::Q) {
            camera.zoom(zoom_speed);
        }
        if window.is_key_down(Key::E) {
            camera.zoom(-zoom_speed);
        }

        // Change light color
        if window.is_key_down(Key::Key1) {  
            light.color = Color::new(255, 223, 128); // Warm light
            //println!("Current light color: {:?}", light.color);
        }
        if window.is_key_down(Key::Key2) {  
            light.color = Color::new(128, 128, 255); // Cool light
            //println!("Current light color: {:?}", light.color);
        }
        if window.is_key_down(Key::Key3) {  
            light.color = Color::new(128, 255, 128); // Greenish light
            //println!("Current light color: {:?}", light.color);
        }
        if window.is_key_down(Key::Key4) {  
            light.color = Color::new(255, 255, 255); // White light
            //println!("Current light color: {:?}", light.color);
        }

        // Only render every N frames to improve responsiveness
        if frame % 5 == 0 {
            framebuffer.clear();
            render(&mut framebuffer, &objects, &camera, &light);
        }
        frame += 1;

        // Update the window with the rendered frame
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
