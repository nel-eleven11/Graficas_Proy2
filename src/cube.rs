// cube.rs

use nalgebra_glm::Vec3;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::material::Material;

pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub material: Material,
}

impl Cube {
    pub fn new(min: Vec3, max: Vec3, material: Material) -> Self {
        Cube { min, max, material }
    }

    fn get_normal(&self, hit_point: &Vec3) -> Vec3 {
        let mut normal = Vec3::zeros();

        if (hit_point.x - self.min.x).abs() < 1e-4 {
            normal = Vec3::new(-1.0, 0.0, 0.0);
        } else if (hit_point.x - self.max.x).abs() < 1e-4 {
            normal = Vec3::new(1.0, 0.0, 0.0);
        } else if (hit_point.y - self.min.y).abs() < 1e-4 {
            normal = Vec3::new(0.0, -1.0, 0.0);
        } else if (hit_point.y - self.max.y).abs() < 1e-4 {
            normal = Vec3::new(0.0, 1.0, 0.0);
        } else if (hit_point.z - self.min.z).abs() < 1e-4 {
            normal = Vec3::new(0.0, 0.0, -1.0);
        } else if (hit_point.z - self.max.z).abs() < 1e-4 {
            normal = Vec3::new(0.0, 0.0, 1.0);
        }

        normal
    }

	// Calculate UV coordinates for each face
    fn get_uv(&self, point: &Vec3) -> (f32, f32) {
        let size = self.max - self.min; // Size of the cube
        let local_point = (point - self.min).component_div(&size); // Normalize point to [0, 1]

        if (point.x - self.min.x).abs() < 1e-4 { // Left face
            (local_point.z, local_point.y)
        } else if (point.x - self.max.x).abs() < 1e-4 { // Right face
            (local_point.z, local_point.y)
        } else if (point.y - self.min.y).abs() < 1e-4 { // Bottom face
            (local_point.x, local_point.z)
        } else if (point.y - self.max.y).abs() < 1e-4 { // Top face
            (local_point.x, local_point.z)
        } else if (point.z - self.min.z).abs() < 1e-4 { // Back face
            (local_point.x, local_point.y)
        } else { // Front face
            (local_point.x, local_point.y)
        }
    }
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let mut t_min = (self.min.x - ray_origin.x) / ray_direction.x;
        let mut t_max = (self.max.x - ray_origin.x) / ray_direction.x;

        if t_min > t_max {
            std::mem::swap(&mut t_min, &mut t_max);
        }

        let mut t_ymin = (self.min.y - ray_origin.y) / ray_direction.y;
        let mut t_ymax = (self.max.y - ray_origin.y) / ray_direction.y;

        if t_ymin > t_ymax {
            std::mem::swap(&mut t_ymin, &mut t_ymax);
        }

        if (t_min > t_ymax) || (t_ymin > t_max) {
            return Intersect::empty();
        }

        if t_ymin > t_min {
            t_min = t_ymin;
        }
        if t_ymax < t_max {
            t_max = t_ymax;
        }

        let mut t_zmin = (self.min.z - ray_origin.z) / ray_direction.z;
        let mut t_zmax = (self.max.z - ray_origin.z) / ray_direction.z;

        if t_zmin > t_zmax {
            std::mem::swap(&mut t_zmin, &mut t_zmax);
        }

        if (t_min > t_zmax) || (t_zmin > t_max) {
            return Intersect::empty();
        }

        if t_zmin > t_min {
            t_min = t_zmin;
        }

        if t_min < 0.0 {
            return Intersect::empty();
        }

        let hit_point = ray_origin + ray_direction * t_min;
        let geometric_normal = self.get_normal(&hit_point); // Normal at the hit point

        // Use get_uv to calculate texture coordinates
        let (u, v) = self.get_uv(&hit_point);

        Intersect::new(
            hit_point,
            geometric_normal,
            t_min,
            self.material.clone(),
            u, // Pass u
            v, // Pass v
        )
    }
}