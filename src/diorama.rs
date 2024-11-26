// diorama.rs

use nalgebra_glm::Vec3;
use crate::cube::Cube;
use crate::material::Material;

pub fn generate_diorama() -> Vec<Cube> {
    let mut objects = Vec::new();
    let cube_size = 1.0; // Uniform size for all cubes

    // Material maps for layers

    let layer_0 = [
        [2, 2, 2, 2, 2, 2, 2, 2], // Layer 0 map
        [2, 2, 2, 2, 2, 2, 2, 2],
        [2, 2, 2, 2, 2, 2, 2, 2],
        [2, 2, 2, 2, 2, 2, 2, 2],
        [2, 2, 2, 2, 2, 2, 2, 2],
        [2, 2, 2, 2, 2, 2, 2, 2],
        [2, 2, 2, 2, 2, 2, 2, 2],
        [2, 2, 2, 2, 2, 2, 2, 2],
    ];


    let layer_1 = [
        [1, 1, 1, 1, 1, 1, 4, 4], // Layer 1 map
        [1, 1, 1, 9, 1, 9, 4, 4], 
        [1, 1, 1, 9, 1, 9, 4, 4],
        [1, 9, 9, 9, 9, 9, 4, 4],
        [1, 1, 1, 9, 9, 9, 4, 4],
        [1, 1, 9, 9, 9, 9, 4, 4],
        [4, 4, 4, 4, 4, 4, 4, 4],
        [4, 4, 4, 4, 4, 4, 4, 4],
    ];

    let layer_2 = [
        [1, 1, 1, 2, 2, 2, 0, 0],
        [1, 1, 2, 2, 5, 3, 0, 0],
        [1, 2, 5, 5, 5, 8, 0, 0],
        [2, 2, 5, 5, 5, 0, 0, 0],
        [2, 5, 5, 5, 5, 0, 0, 0],
        [2, 3, 8, 0, 0, 2, 0, 0],
    ];

    let layer_3 = [
        [2, 2, 2, 3, 3, 3, 0, 0], // Layer 3 map
        [2, 2, 0, 0, 0, 0, 0, 0],
        [2, 0, 0, 0, 0, 0, 0, 0],
        [3, 0, 0, 0, 0, 0, 0, 0],
        [3, 0, 0, 7, 0, 0, 0, 0],
        [3, 0, 0, 0, 0, 0, 0, 0],
    ];

    let layer_4 = [
        [9, 9, 9, 3, 3, 3, 0, 0], // Layer 4 map
        [9, 0, 0, 0, 0, 0, 0, 0],
        [9, 0, 0, 0, 0, 0, 0, 0],
        [3, 0, 0, 0, 0, 0, 0, 0],
        [3, 0, 0, 0, 0, 0, 0, 0],
        [3, 0, 0, 0, 0, 0, 0, 0],
    ];

    let layer_5 = [
        [0, 0, 0, 3, 3, 3, 0, 0], // Layer 5 map
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [3, 0, 0, 0, 0, 0, 0, 0],
        [3, 0, 0, 0, 0, 0, 0, 0],
        [3, 0, 0, 0, 0, 0, 0, 0],
    ];

    let layer_6 = [
        [0, 0, 0, 3, 3, 3, 0, 0], // Layer 6 map
        [0, 0, 0, 0, 0, 0, 0, 0],
        [3, 0, 0, 0, 0, 0, 0, 0],
        [3, 0, 0, 0, 0, 0, 0, 0],
        [3, 0, 0, 0, 0, 0, 0, 0],
        [3, 0, 0, 0, 0, 0, 0, 0],
    ];

    let layer_7 = [
        [6, 6, 6, 3, 3, 3, 0, 0], // Layer 7 map
        [6, 6, 0, 0, 0, 3, 0, 0],
        [6, 0, 0, 0, 0, 0, 0, 0],
        [3, 0, 0, 0, 0, 0, 0, 0],
        [3, 0, 0, 0, 0, 0, 0, 0],
        [3, 0, 0, 0, 0, 0, 0, 0],
    ];

    let layer_8 = [
        [3, 3, 3, 10, 10, 10, 0, 0], // Layer 8 map
        [3, 0, 0, 0, 0, 0, 0, 0],
        [3, 0, 0, 0, 0, 0, 0, 0],
        [10, 0, 0, 0, 0, 0, 0, 0],
        [10, 0, 0, 0, 0, 0, 0, 0],
        [10, 0, 0, 0, 0, 0, 0, 0],
    ];

    let layer_9 = [
        [3, 3, 3, 3, 3, 3, 0, 0], // Layer 9 map
        [3, 3, 0, 0, 0, 0, 0, 0],
        [3, 0, 0, 0, 0, 0, 0, 0],
        [3, 0, 0, 0, 0, 0, 0, 0],
        [3, 0, 0, 0, 0, 0, 0, 0],
        [3, 0, 0, 0, 0, 0, 0, 0],
    ];

    let layer_10 = [
        [11, 0, 0, 0, 0, 11, 0, 0], // Layer 10 map
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [11, 0, 0, 0, 0, 0, 0, 0],
    ];

    // Material key based on indices
    let materials = vec![
        Material::dirt(),           // 1
        Material::obsidian(),       // 2
        Material::crying_osidian(), // 3
        Material::glass(),          // 4
        Material::diamond_ore(),    // 5
        Material::beacon(),         // 6
        Material::crafting_table(), // 7
        Material::tnt(),            // 8
        Material::cobblestone(),    // 9
        Material::bookshelf(),      // 10
        Material::redstone_lamp(),  // 11
    ];

    // Function to process a single layer
    let mut process_layer = |layer: &[[u8; 8]], y_layer: f32| {
        for (z, row) in layer.iter().enumerate() {
            for (x, &material_index) in row.iter().enumerate() {
                if material_index > 0 {
                    // Calculate cube coordinates
                    let x_pos = x as f32 * cube_size;
                    let z_pos = z as f32 * cube_size;

                    // Convert `material_index` from u8 to usize
                    let material = materials[(material_index - 1) as usize].clone();

                    // Create a cube with the corresponding material
                    let cube = Cube::new(
                        Vec3::new(x_pos, y_layer, z_pos),                      // Minimum corner
                        Vec3::new(x_pos + cube_size, y_layer + cube_size, z_pos + cube_size), // Maximum corner
                        material,
                    );

                    objects.push(cube);
                }
            }
        }
    };

    // Process each layer with its respective height
    process_layer(&layer_0, 0.0); // Layer 0 at y = 0
    process_layer(&layer_1, 1.0); // Layer 1 at y = 1
    process_layer(&layer_2, 2.0); // Layer 2 at y = 2
    process_layer(&layer_3, 3.0); // Layer 3 at y = 3
    process_layer(&layer_4, 4.0); // Layer 4 at y = 4
    process_layer(&layer_5, 5.0); // Layer 5 at y = 5
    process_layer(&layer_6, 6.0); // Layer 6 at y = 6
    process_layer(&layer_7, 7.0); // Layer 7 at y = 7
    process_layer(&layer_8, 8.0); // Layer 8 at y = 8
    process_layer(&layer_9, 9.0); // Layer 9 at y = 9
    process_layer(&layer_10, 10.0); // Layer 10 at y = 10

    objects
}


// Create a second diorama, more simple than the first one
pub fn generate_diorama2() -> Vec<Cube> {

    let mut objects = Vec::new();
    let cube_size = 1.0; // Uniform size for all cubes

    // Material maps for layers

    let layer_0 = [
        [0, 0, 0, 0, 0, 0, 0, 0], // Layer 0 map
        [0, 11, 2, 0, 0, 2, 11, 0],
        [0, 2, 2, 2, 2, 2, 2, 0],
        [0, 0, 2, 2, 2, 2, 0, 0],
        [0, 0, 2, 2, 2, 2, 0, 0],
        [0, 2, 2, 2, 2, 2, 2, 0],
        [0, 11, 2, 0, 0, 2, 11, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];

    let layer_1 = [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 4, 0, 0, 0, 0, 4, 0],
        [0, 0, 8, 0, 0, 8, 0, 0],
        [0, 0, 0, 6, 6, 0, 0, 0],
        [0, 0, 0, 6, 6, 0, 0, 0],
        [0, 0, 8, 0, 0, 8, 0, 0],
        [0, 4, 0, 0, 0, 0, 4, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];

    let layer_2 = [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 3, 3, 0, 0, 0],
        [0, 0, 0, 3, 3, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];


    // Material key based on indices
    let materials = vec![
        Material::dirt(),           // 1
        Material::obsidian(),       // 2
        Material::crying_osidian(), // 3
        Material::glass(),          // 4
        Material::diamond_ore(),    // 5
        Material::beacon(),         // 6
        Material::crafting_table(), // 7
        Material::tnt(),            // 8
        Material::cobblestone(),    // 9
        Material::bookshelf(),      // 10
        Material::redstone_lamp(),  // 11
    ];

    // Function to process a single layer
    let mut process_layer = |layer: &[[u8; 8]], y_layer: f32| {
        for (z, row) in layer.iter().enumerate() {
            for (x, &material_index) in row.iter().enumerate() {
                if material_index > 0 {
                    // Calculate cube coordinates
                    let x_pos = x as f32 * cube_size;
                    let z_pos = z as f32 * cube_size;

                    // Convert `material_index` from u8 to usize
                    let material = materials[(material_index - 1) as usize].clone();

                    // Create a cube with the corresponding material
                    let cube = Cube::new(
                        Vec3::new(x_pos, y_layer, z_pos),                      // Minimum corner
                        Vec3::new(x_pos + cube_size, y_layer + cube_size, z_pos + cube_size), // Maximum corner
                        material,
                    );

                    objects.push(cube);
                }
            }
        }
    };

    // Process each layer with its respective height
    process_layer(&layer_0, 0.0); // Layer 0 at y = 0
    process_layer(&layer_1, 1.0); // Layer 1 at y = 1
    process_layer(&layer_2, 2.0); // Layer 2 at y = 2

    objects

}
