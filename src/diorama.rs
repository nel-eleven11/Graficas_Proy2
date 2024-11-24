// diorama.rs

use nalgebra_glm::Vec3;
use crate::cube::Cube;
use crate::material::Material;

pub fn generate_diorama() -> Vec<Cube> {
    let mut objects = Vec::new();

    // Crear un cubo con textura
    let textured_cube = Cube::new(
		Vec3::new(-0.5, -0.5, -0.5), 
		Vec3::new(0.5, 0.5, 0.5),    
		Material::dirt()             
	);
    objects.push(textured_cube);

    objects
}
