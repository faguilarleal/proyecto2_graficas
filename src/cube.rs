
use nalgebra_glm::{Vec3, dot, cross};
use crate::ray_intersect::{RayIntersect, Intersect};
use crate::material::Material;



pub struct Cube {
    pub center: Vec3,       // Centro del cubo
    pub size: f32,          // Longitud de la arista del cubo
    pub material: Material,
}