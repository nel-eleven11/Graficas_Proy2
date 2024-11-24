// diorama.rs

use nalgebra_glm::Vec3;
use crate::cube::Cube;
use crate::material::Material;

pub fn generate_diorama() -> Vec<Cube> {
    let mut objects = Vec::new();
    let cube_size = 1.0; // Tamaño del cubo

    

    let cube = Cube::new(
        Vec3::new(0.0, 0.0, 0.0),                           
        Vec3::new(cube_size, cube_size, cube_size),    
        Material::bookshelf()                            
    );

    objects.push(cube);

    objects
}
