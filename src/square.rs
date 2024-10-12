use nalgebra_glm::{Vec3, dot, cross};
use crate::ray_intersect::RayIntersect;

pub struct Square {
    pub center: Vec3,       // Centro del cuadrado
    pub normal: Vec3,       // Vector normal del plano del cuadrado
    pub side_length: f32,   // Longitud del lado del cuadrado
}

impl RayIntersect for Square {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> bool {
        // Paso 1: Calcula la intersección del rayo con el plano del cuadrado.
        // El plano está definido por el punto `center` y el vector normal `normal`.
        let denom = dot(&self.normal, ray_direction);

        // Si el denominador es cero, el rayo es paralelo al plano, no hay intersección
        if denom.abs() < f32::EPSILON {
            return false;
        }

        // Calcula la distancia desde el origen del rayo al plano
        let t = dot(&(self.center - ray_origin), &self.normal) / denom;

        // Si t es negativo, el cuadrado está detrás del origen del rayo
        if t < 0.0 {
            return false;
        }

        // Paso 2: Calcula el punto de intersección en el plano
        let intersection_point = ray_origin + ray_direction * t;

        // Paso 3: Verifica si el punto de intersección está dentro del cuadrado
        // El cuadrado está en un plano, por lo que necesitamos verificar si el punto está dentro de los límites.
        // Vamos a proyectar el punto de intersección sobre las direcciones de los ejes del cuadrado.

        // Asumamos que el cuadrado está alineado con dos vectores ortogonales en el plano.
        // Calculamos los vectores de los bordes del cuadrado a partir del normal (esto puede variar si el cuadrado está rotado).
        let edge1 = cross(&self.normal, &Vec3::new(1.0, 0.0, 0.0)).normalize(); // Primer eje del cuadrado
        let edge2 = cross(&self.normal, &edge1).normalize(); // Segundo eje ortogonal del cuadrado

        // Calculamos las distancias desde el centro del cuadrado hasta el punto de intersección en los ejes locales del cuadrado.
        let local_point = intersection_point - self.center;

        let dist_edge1 = dot(&local_point, &edge1).abs();
        let dist_edge2 = dot(&local_point, &edge2).abs();

        // El punto está dentro del cuadrado si está dentro de los límites en ambos ejes.
        let half_side = self.side_length / 2.0;
        dist_edge1 <= half_side && dist_edge2 <= half_side
    }
}
