use crate::ray_intersect::{RayIntersect, Intersect};
use crate::color::Color;
use crate::light::Light;
use crate::cube::Cube;
use nalgebra_glm::{Vec3};

const ORIGIN_BIAS: f32 = 1e-4;
const SKYBOX_COLOR: Color = Color::new(253, 255, 146);


pub fn offset_origin(intersect: &Intersect, direction: &Vec3) -> Vec3 {
    let offset = intersect.normal * ORIGIN_BIAS;
    if direction.dot(&intersect.normal) < 0.0 {
        intersect.point - offset
    } else {
        intersect.point + offset
    }
}

pub fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}


// maneja los rayos que entran y salen del ofecto dependiendo de el indica de refraccion del objecto 
pub fn refract(incident: &Vec3, normal: &Vec3, eta_t: f32) -> Vec3 {
    let cosi = -incident.dot(normal).max(-1.0).min(1.0);
    
    let (n_cosi, eta, n_normal);

    if cosi < 0.0 {
        // Ray is entering the object
        n_cosi = -cosi;
        eta = 1.0 / eta_t;
        n_normal = -normal;
    } else {
        // Ray is leaving the object
        n_cosi = cosi;
        eta = eta_t;  // Assuming it's going back into air with index 1.0
        n_normal = *normal;
    }
    
    let k = 1.0 - eta * eta * (1.0 - n_cosi * n_cosi);
    
    if k < 0.0 {
        // Total internal reflection
        reflect(incident, &n_normal)
    } else {
        eta * incident + (eta * n_cosi - k.sqrt()) * n_normal
    }
}

// determina si un punto de la escena tiene sombra
pub fn cast_shadow(
    intersect: &Intersect,
    light: &Light,
    objects: &[Cube],
) -> f32 {
    let light_dir = (light.position - intersect.point).normalize();
    let light_distance = (light.position - intersect.point).magnitude();

    let shadow_ray_origin = offset_origin(intersect, &light_dir);
    let mut shadow_intensity = 0.0;

    for object in objects {
        // Solo considera objetos que proyectan sombras
        if object.has_shadow {
            let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);

            // Si hay una intersección y está dentro del rango de la luz
            if shadow_intersect.is_intersecting && shadow_intersect.distance < light_distance {
                let distance_ratio = shadow_intersect.distance / light_distance;

                // Si el objeto es transparente, atenúa la sombra en función de la transparencia
                if object.material.albedo[3] > 0.0 {
                    // La luz pasa parcialmente según la transparencia del objeto
                    shadow_intensity += (1.0 - object.material.albedo[3]) * (1.0 - distance_ratio.powf(2.0).min(1.0));
                } else {
                    // Si no es transparente, bloquea completamente la luz
                    shadow_intensity = 1.0;
                    break; // Salimos del bucle ya que la sombra es total
                }
            }
        }
    }

    shadow_intensity.min(1.0) // Asegura que el valor de la sombra no sea mayor a 1.0
}


// solo se muestra si el rayo intersecta 
pub fn cast_ray(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    objects: &[Cube],
    lights: &[Light],  // Ahora acepta un arreglo de luces
    depth: u32,        // Recursión de reflexión/refracción
) -> Color {
    if depth > 3 {
        return SKYBOX_COLOR;
    }

    

    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    // Buscar la intersección más cercana
    for object in objects {
        let i = object.ray_intersect(ray_origin, ray_direction);
        if i.is_intersecting && i.distance < zbuffer {
            zbuffer = i.distance;
            intersect = i;
        }
    }

 

    if !intersect.is_intersecting {
        return SKYBOX_COLOR;  // Fondo de cielo por defecto si no hay intersección
    }

    let material = &intersect.material;
    let mut pixel_color = Color::new(0, 0, 0);

    if material.has_emission {
        let emission_color = material.get_emission_color(intersect.u, intersect.v);
        pixel_color += emission_color; // Sumar el color de emisión
    }


    // Inicializar los colores de difusión y especular
    let mut final_color = Color::black();

    // Iterar sobre todas las luces para acumular contribuciones
    for light in lights {
        let light_dir = (light.position - intersect.point).normalize();
        let view_dir = (ray_origin - intersect.point).normalize();
        let reflect_dir = reflect(&-light_dir, &intersect.normal).normalize();

        let shadow_intensity = cast_shadow(&intersect, light, objects);
        let light_intensity = light.intensity * (1.0 - shadow_intensity);

        // Componente difusa
        let diffuse_intensity = intersect.normal.dot(&light_dir).max(0.0).min(1.0);
        let diffuse_color = intersect.material.get_diffuse_color(intersect.u, intersect.v);
        let diffuse = diffuse_color * intersect.material.albedo[0] * diffuse_intensity * light_intensity;

        // Componente especular
        let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersect.material.specular);
        let specular = light.color * intersect.material.albedo[1] * specular_intensity * light_intensity;

        // Sumar la contribución de esta luz al color final
        final_color = final_color + diffuse + specular;
    }

    // Manejo de reflejos y refracciones
    let mut reflect_color = Color::black();
    let reflectivity = intersect.material.albedo[2];
    if reflectivity > 0.0 {
        let reflect_dir = reflect(&ray_direction, &intersect.normal).normalize();
        let reflect_origin = offset_origin(&intersect, &reflect_dir);
        reflect_color = cast_ray(&reflect_origin, &reflect_dir, objects, lights, depth + 1);
    }

    let mut refract_color = Color::black();
    let transparency = intersect.material.albedo[3];
    if transparency > 0.0 {
        let refract_dir = refract(&ray_direction, &intersect.normal, intersect.material.refractive_index);
        let refract_origin = offset_origin(&intersect, &refract_dir);
        refract_color = cast_ray(&refract_origin, &refract_dir, objects, lights, depth + 1);
    }

    final_color * (1.0 - reflectivity - transparency) + reflect_color * reflectivity + refract_color * transparency
}
