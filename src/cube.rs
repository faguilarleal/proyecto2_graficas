
use nalgebra_glm::{Vec3, dot, cross};
use crate::ray_intersect::{RayIntersect, Intersect};
use crate::material::Material;



pub struct Cube {
    pub center: Vec3,       // Centro del cubo
    pub size: f32,          // Longitud de la arista del cubo
    pub material: Material,
}

impl Cube {
    pub fn ray_intersect(&self, origin: &Vec3, direction: &Vec3) -> Intersect {
        // Las 6 caras del cubo, con sus respectivas normales
        let normals = [
            Vec3::new(1.0, 0.0, 0.0),  // Cara derecha
            Vec3::new(-1.0, 0.0, 0.0), // Cara izquierda
            Vec3::new(0.0, 1.0, 0.0),  // Cara superior
            Vec3::new(0.0, -1.0, 0.0), // Cara inferior
            Vec3::new(0.0, 0.0, 1.0),  // Cara frontal
            Vec3::new(0.0, 0.0, -1.0), // Cara trasera
        ];

        let mut closest_intersect = Intersect::empty();
        let half_size = self.size / 2.0;
        let bounds = (-half_size, half_size);

        for normal in normals.iter() {
            // Calcula la distancia desde el origen del rayo hasta el plano de la cara
            let denom = normal.dot(direction);
            if denom.abs() < 1e-6 {
                continue;  // Si es paralelo a la cara, no hay intersección
            }

            // Encuentra la posición de la cara en el cubo (sumar o restar half_size al centro)
            let face_center = self.center + normal * half_size;

            // Calcula la distancia al plano de la cara
            let d = (face_center - *origin).dot(normal) / denom;
            if d < 0.0 {
                continue;  // La cara está detrás del rayo
            }

            // Calcula el punto de intersección en el plano de la cara
            let hit_point = origin + direction * d;
            let local_hit_point = hit_point - self.center;

            // Verifica si el punto de intersección está dentro de los límites de la cara
            let (min_bound, max_bound) = bounds;
            if normal.x.abs() == 1.0 {
                if local_hit_point.y >= min_bound && local_hit_point.y <= max_bound &&
                   local_hit_point.z >= min_bound && local_hit_point.z <= max_bound {
                    // Cara derecha o izquierda
                    if d < closest_intersect.distance {
                        closest_intersect = Intersect {
                            point: hit_point,
                            normal: *normal,
                            distance: d,
                            is_intersecting: true,
                            material: self.material.clone(),
                            u: 0.0,  // Puedes agregar texturizado si lo necesitas
                            v: 0.0,
                        };
                    }
                }
            } else if normal.y.abs() == 1.0 {
                if local_hit_point.x >= min_bound && local_hit_point.x <= max_bound &&
                   local_hit_point.z >= min_bound && local_hit_point.z <= max_bound {
                    // Cara superior o inferior
                    if d < closest_intersect.distance {
                        closest_intersect = Intersect {
                            point: hit_point,
                            normal: *normal,
                            distance: d,
                            is_intersecting: true,
                            material: self.material.clone(),
                            u: 0.0,
                            v: 0.0,
                        };
                    }
                }
            } else if normal.z.abs() == 1.0 {
                if local_hit_point.x >= min_bound && local_hit_point.x <= max_bound &&
                   local_hit_point.y >= min_bound && local_hit_point.y <= max_bound {
                    // Cara frontal o trasera
                    if d < closest_intersect.distance {
                        closest_intersect = Intersect {
                            point: hit_point,
                            normal: *normal,
                            distance: d,
                            is_intersecting: true,
                            material: self.material.clone(),
                            u: 0.0,
                            v: 0.0,
                        };
                    }
                }
            }
        }

        closest_intersect  // Devolver el punto más cercano de intersección
    }
}
