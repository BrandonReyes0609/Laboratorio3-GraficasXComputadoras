// color.rs



#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    // Constructor para color negro
    pub fn black() -> Self {
        Color { r: 0.0, g: 0.0, b: 0.0 }
    }

    // Constructor para color rojo
    pub fn red() -> Self {
        Color { r: 1.0, g: 0.0, b: 0.0 }
    }

    // Convierte el color a formato hexadecimal
    pub fn to_hex(&self) -> u32 {
        let r = (self.r * 255.0) as u32;
        let g = (self.g * 255.0) as u32;
        let b = (self.b * 255.0) as u32;
        (r << 16) | (g << 8) | b
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, intensity: f32) -> Self::Output {
        Color {
            r: self.r * intensity,
            g: self.g * intensity,
            b: self.b * intensity,
        }
    }
}