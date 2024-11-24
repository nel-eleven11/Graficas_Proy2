// diorama.rs

use nalgebra_glm::Vec3;
use crate::cube::Cube;
use crate::material::Material;

pub fn generate_diorama() -> Vec<Cube> {
    let mut objects = Vec::new();

    let cube_size = 0.5;

    // Crear un cubo con textura
    let textured_cube = Cube::new(
		Vec3::new(-0.5, -0.5, -0.5), 
		Vec3::new( cube_size, cube_size, cube_size),    
		Material::dirt()             
	);

    let glass = Cube::new(
        Vec3::new(-1.0, -1.0, -1.0),
        Vec3::new( cube_size -0.5, cube_size -0.5, cube_size-0.5),
        Material::glass()
    );




    objects.push(glass);
    objects.push(textured_cube);

    objects
}
