use once_cell::sync::Lazy;
use std::sync::Arc;
use nalgebra_glm::Vec3;

use crate::color::Color;
use crate::texture::Texture;


#[derive(Debug, Clone)]
pub struct Material {
  pub diffuse: Color,
  pub specular: f32,
  pub albedo: [f32; 4],
  pub refractive_index: f32,
  pub has_texture: bool,
  pub has_normal_map: bool,
  pub texture: Option<Arc<Texture>>,
  pub emission: Color,            // Color de la emisión
  pub emission_strength: f32,     // Intensidad de la emisión
  pub has_emission: bool, // Si el material usa una textura para la emisión
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
      emission: Color::new(0, 0, 0),
      emission_strength: 0.0,
      has_emission: false,
    }
  }

  pub fn new_with_emission_texture(
    specular: f32,
    albedo: [f32; 4],
    refractive_index: f32,
    texture: Arc<Texture>,
    emission: Color,
    emission_strength: f32,
) -> Self {
    Material {
        diffuse: Color::new(255, 255, 255), // Color difuso por defecto
        specular,
        albedo,
        refractive_index,
        has_texture: true,
        has_normal_map: false,
        texture: Some(texture),
        emission,
        emission_strength,
        has_emission: true,
    }
}

pub fn get_emission_color(&self, u: f32, v: f32) -> Color {
  if self.has_emission {
      // Si el material tiene una textura de emisión, tomar el color de la textura
      let texture = self.texture.as_ref().unwrap();
      let x = (u * (texture.width as f32 - 1.0)) as usize;
      let y = ((1.0 - v) * (texture.height as f32 - 1.0)) as usize;
      let tex_color = texture.get_color(x, y);
      Color::new(
          (tex_color.r as f32 * self.emission_strength) as u8,
          (tex_color.g as f32 * self.emission_strength) as u8,
          (tex_color.b as f32 * self.emission_strength) as u8,
      )
  } else {
      self.emission
  }
}

  pub fn new_with_texture(
    specular: f32,
    albedo: [f32; 4],
    refractive_index: f32,
    texture: Arc<Texture>,

  ) -> Self {
    Material {
      diffuse: Color::new(255, 255, 255), // Color difuso por defecto
      specular,
      albedo,
      refractive_index,
      has_texture: true,
      has_normal_map: false,
      texture: Some(texture),
      emission: Color::new(0, 0, 0),
      emission_strength: 0.0,
      has_emission: false,
    }
  }

  pub fn get_diffuse_color(&mut self, u: f32, v: f32) -> Color {
    if self.has_texture {
      let texture = self.texture.as_ref().unwrap();
      let x = (u * (texture.width as f32 - 1.0)) as usize;
      let y = ((1.0 - v) * (texture.height as f32 - 1.0)) as usize;
      texture.get_color(x, y)
      // Color::new(255, 0, 0)
    }
    else {
      self.diffuse
    }
  }

  pub fn get_normal_from_map(&self, u: f32, v: f32) -> Vec3 {
    if self.has_normal_map {
      let texture = self.texture.as_ref().unwrap();
      let x = (u * (texture.width as f32 - 1.0)) as usize;
      let y = ((1.0 - v) * (texture.height as f32 - 1.0)) as usize;
      let texture = self.texture.as_ref().unwrap();
      let color = texture.get_color(x, y);
    
      // Correctly decode the normal map
      let nx = (color.r as f32 / 255.0) * 2.0 - 1.0;
      let ny = (color.g as f32 / 255.0) * 2.0 - 1.0;
      let nz = color.b as f32 / 255.0; // Note: only 0 to 1 range for Z

      Vec3::new(nx, ny, nz).normalize()
    } else {
      Vec3::new(0.0, 0.0, 1.0) // Default normal if no normal map is present
    }
  }

  pub fn black() -> Self {
    Material {
      diffuse: Color::new(0, 0, 0),
      specular: 0.0,
      albedo: [0.0, 0.0, 0.0, 0.0],
      refractive_index: 0.0,
      has_texture: false,
      texture: None,
      has_normal_map: false,
      emission: Color::new(0, 0, 0),
      emission_strength: 0.0,
      has_emission: false,
    }
  }
}
