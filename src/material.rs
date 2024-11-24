// material.rs

use once_cell::sync::Lazy;
use std::sync::Arc;
use nalgebra_glm::Vec3;
use crate::color::Color;
use crate::texture::Texture;


static DIRT_TEXTURE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/dirt.png")));

#[derive(Debug, Clone)]
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    pub albedo: [f32; 4],
    pub refractive_index: f32,
    pub has_texture: bool,
    pub has_normal_map: bool,
	pub texture: Option<Arc<Texture>>,       // Texture
    pub normal_map: Option<Arc<Texture>>,    // Normal map
}

impl Material {
    pub fn new(
		diffuse: Color,
		specular: f32,
		albedo: [f32; 4],
		refractive_index: f32,
  ) -> Self {
    Material {
		diffuse,
		specular,
		albedo,
		refractive_index,
		has_texture: false,
		has_normal_map: false,
		texture: None,
		normal_map: None,
    }
  }

    // Generic Material
    pub fn material() -> Self {
        Material {
            diffuse: Color::new(255, 255, 255),
            specular: 50.0,
            albedo: [0.6, 0.3, 0.0, 0.0],
            refractive_index: 1.0,
            has_texture: false,
            has_normal_map: false,
			texture: None,
			normal_map: None,
        }
    }

    
    pub fn dirt() -> Self {
        Material {
            diffuse: Color::black(), // Ignorado cuando hay textura
            specular: 10.0,
            albedo: [0.9, 0.05, 0.0, 0.0],
            refractive_index: 0.0,
            has_texture: true,
            has_normal_map: false,
			texture: Some(DIRT_TEXTURE.clone()),
			normal_map: None,
        }
    }

	pub fn black() -> Self {
		Material {
			diffuse: Color::new(0, 0, 0),
			specular: 0.0,
			albedo: [0.0, 0.0, 0.0, 0.0],
			refractive_index: 0.0,
			has_texture: false,
			has_normal_map: false,
			texture: None,
			normal_map: None,
		}
	}

	pub fn get_diffuse_color(&self, u: f32, v: f32) -> Color {
		if self.has_texture {
			if let Some(texture) = &self.texture {
				let x = (u * (texture.width as f32 - 1.0)) as usize;
				let y = ((1.0 - v) * (texture.height as f32 - 1.0)) as usize;
				return texture.get_color(x, y);
			}
		}
		self.diffuse
	}

    pub fn get_normal_from_map(&self, u: f32, v: f32) -> Vec3 {
        if self.has_normal_map {
            if let Some(texture) = &self.normal_map {
                let x = (u * (texture.width as f32 - 1.0)) as usize;
                let y = ((1.0 - v) * (texture.height as f32 - 1.0)) as usize;
                let color = texture.get_color(x, y);

                let nx = (color.r as f32 / 255.0) * 2.0 - 1.0;
                let ny = (color.g as f32 / 255.0) * 2.0 - 1.0;
                let nz = color.b as f32 / 255.0;

                return Vec3::new(nx, ny, nz).normalize();
            }
        }
        Vec3::new(0.0, 0.0, 1.0)
    }
}
