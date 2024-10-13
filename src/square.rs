use nalgebra_glm::{Vec3, dot, cross};
use crate::ray_intersect::{RayIntersect, Intersect};
use crate::material::Material;

pub struct Square {
    pub center: Vec3,       // Centro del cuadrado
    pub normal: Vec3,       // Vector normal del plano del cuadrado
    pub size: f32,   // Longitud del lado del cuadrado
    pub material: Material,
}

impl Square {
    pub fn ray_intersect(&self, origin: &Vec3, direction: &Vec3) -> Intersect {
        let denom = self.normal.dot(direction);
        
        // Si el rayo es paralelo al plano del cuadrado, no hay intersección
        if denom.abs() < 1e-6 {
            return Intersect::empty();
        }

        // Encuentra la distancia desde el origen del rayo al plano del cuadrado
        let d = (self.center - *origin).dot(&self.normal) / denom;
        if d < 0.0 {
            return Intersect::empty();  // El cuadrado está detrás del origen del rayo
        }

        // Calcula el punto de intersección en el plano del cuadrado
        let hit_point = origin + direction * d;

        // Chequea si el punto de intersección está dentro de los límites del cuadrado
        let local_hit_point = hit_point - self.center;
        let half_size = self.size / 2.0;

        if local_hit_point.x.abs() <= half_size && local_hit_point.y.abs() <= half_size {
            Intersect {
                point: hit_point,
                normal: self.normal,
                distance: d,
                is_intersecting: true,
                material: self.material.clone(),
                u: 0.0,  // coordenadas UV para texturizado
                v: 0.0,
            }
        } else {
            Intersect::empty()  // No está dentro de los límites del cuadrado
        }
    }
}
