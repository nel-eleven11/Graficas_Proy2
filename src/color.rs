// color.rs

use std::fmt;
use nalgebra_glm::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    // Constructor to initialize the color using r, g, b values
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    // Function to create a color from a hex value
    pub const fn from_hex(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        Color { r, g, b }
    }

    pub const fn black() -> Self {
        Color { r: 0, g: 0, b: 0 }
    }

    // Function to return the color as a hex value
    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    // Function to return the color as a hex value
    pub fn is_black(&self) -> bool {
        self.r == 0 && self.g == 0 && self.b == 0
    }

    /// Convert the color to an f32 representation
    pub fn to_f32(&self) -> Color {
        Color {
            r: (self.r as f32 / 255.0) as u8,
            g: (self.g as f32 / 255.0) as u8,
            b: (self.b as f32 / 255.0) as u8,
        }
    }

    // Convert to a Vec3 representation
    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
        )
    }

    // Create a Color from a Vec3
    pub fn from_vec3(vec: Vec3) -> Self {
        Color {
            r: (vec.x.clamp(0.0, 1.0) * 255.0) as u8,
            g: (vec.y.clamp(0.0, 1.0) * 255.0) as u8,
            b: (vec.z.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }   
}

// Implement addition for Color
use std::ops::Add;

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r.saturating_add(other.r),
            g: self.g.saturating_add(other.g),
            b: self.b.saturating_add(other.b),
        }
    }
}

// Implement multiplication by a constant for Color
use std::ops::Mul;

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, scalar: f32) -> Color {
        Color {
            r: (self.r as f32 * scalar).clamp(0.0, 255.0) as u8,
            g: (self.g as f32 * scalar).clamp(0.0, 255.0) as u8,
            b: (self.b as f32 * scalar).clamp(0.0, 255.0) as u8,
        }
    }
}

// Implement addition assignment (+=) for Color
use std::ops::AddAssign;

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        self.r = self.r.saturating_add(other.r);
        self.g = self.g.saturating_add(other.g);
        self.b = self.b.saturating_add(other.b);
    }
}

// Implement display formatting for Color
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color(r: {}, g: {}, b: {})", self.r, self.g, self.b)
    }
}