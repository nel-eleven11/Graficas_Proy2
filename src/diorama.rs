// diorama.rs

use nalgebra_glm::Vec3;
use crate::cube::Cube;
use crate::material::Material;

pub fn generate_diorama() -> Vec<Cube> {
    let mut objects = Vec::new();
    let cube_size = 0.5; // Tamaño del cubo

    
    for x in (0..(1.0 / cube_size) as usize).map(|i| i as f32 * cube_size) {
        for z in (0..(1.0 / cube_size) as usize).map(|i| i as f32 * cube_size) {
            objects.push(Cube {
                min: Vec3::new(x, 0.0, z),  
                max: Vec3::new(x + cube_size, cube_size, z + cube_size), 
                material: Material::diamond_ore(),  
            });
        }
    }

    
    let mut x = 1.0;
    while x < 4.0 {
        let mut z = 0.0;
        while z < 4.0 {
            objects.push(Cube {
                min: Vec3::new(x, 0.0, z), 
                max: Vec3::new(x + cube_size, cube_size, z + cube_size),  
                material: Material::obsidian(),  
            });
            z += cube_size;  
        }
        x += cube_size;  
    }
    
    
    for x in (0..(1.0 / cube_size) as usize).map(|i| i as f32 * cube_size) {
        for z in (0..(4.0 / cube_size) as usize).map(|i| i as f32 * cube_size) {
            objects.push(Cube {
                min: Vec3::new(x, 0.0, z),  
                max: Vec3::new(x + cube_size, cube_size, z + cube_size),  
                material: Material::obsidian(),  
            });
        }
    }

    //Wall 1
    for y in (0..(4.0 / cube_size) as usize).map(|i| i as f32 * cube_size) {
        for z in (0..(4.0 / cube_size) as usize).map(|i| i as f32 * cube_size) {
            objects.push(Cube {
                min: Vec3::new(4.0, y, z),  
                max: Vec3::new(4.0 + cube_size, y + cube_size, z + cube_size),  
                material: Material::obsidian(),  
            });
        }
    }

    // Wall 2
    for y in (0..(4.0 / cube_size) as usize).map(|i| i as f32 * cube_size) {
        for x in (0..(5.0 / cube_size) as usize).map(|i| i as f32 * cube_size) {
            objects.push(Cube {
                min: Vec3::new(x, y, 4.0),  
                max: Vec3::new(cube_size, y + cube_size, 4.0 + cube_size),  
                material: Material::obsidian(),  
            });
        }
    }

    
    


    objects
}

