use nalgebra_glm::{Vec3, normalize};
use once_cell::sync::Lazy;
use std::sync::Arc;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;
// use rand::{Rng};

mod framebuffer;
mod ray_intersect;
mod color;
mod camera;
mod light;
mod material;
mod texture;
mod castray;
mod cube;
mod square;

use framebuffer::Framebuffer;
use castray::{cast_ray};
use color::Color;
use ray_intersect::{Intersect, RayIntersect};
use camera::Camera;
use light::Light;
use material::Material;
use cube::Cube;
use texture::Texture;
use square::Square;

static TEXTURE1: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/bricks.png")));

pub fn render(framebuffer: &mut Framebuffer, objects: &[Cube], camera: &Camera, lights: &[Light]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI/3.0;
    let perspective_scale = (fov * 0.5).tan();

    // random number generator
    // let mut rng = rand::thread_rng();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            // if rng.gen_range(0.0..1.0) < 0.9 {
            //      continue;
            // }

            // Map the pixel coordinate to screen space [-1, 1]
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            // Adjust for aspect ratio and perspective 
            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            // Calculate the direction of the ray for this pixel
            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));

            // Apply camera rotation to the ray direction
            let rotated_direction = camera.basis_change(&ray_direction);

            // Cast the ray and get the pixel color
            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, lights, 0);

            // Draw the pixel on screen with the returned color
            framebuffer.set_current_color(pixel_color.to_hex());
            framebuffer.point(x, y);
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Rust Graphics - Raytracer Example",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    // move the window around
    window.set_position(500, 500);
    window.update();

    // let rubber = Material::new(
    //     Color::new(255, 100, 80),
    //     1.0,
    //     [0.9, 0.1, 0.0, 0.0],
    //     0.0,
    // );

    let rubber = Material::new_with_texture(
        1.0,
        [0.9, 0.1, 0.0, 0.0],
        1.0, TEXTURE1.clone(),
    );

    let ivory = Material::new(
        Color::new(100, 100, 80),
        50.0,
        [0.6, 0.3, 0.6, 0.0],
        0.0,
    );

    let glass = Material::new(
        Color::new(255, 255, 255),
        1425.0,
        [0.0, 10.0, 0.5, 0.5],
        0.3,
    );

    // let objects = [
    //     Sphere { center: Vec3::new(0.0, 0.0, 0.0), radius: 1.0, material: rubber },
    //     Sphere { center: Vec3::new(-1.0, -1.0, 1.5), radius: 0.5, material: ivory },
    //     Sphere { center: Vec3::new(-0.3, 0.3, 1.5), radius: 0.3, material: glass },
    //     // Sphere { center: Vec3::new(-2.0, 2.0, -5.0), radius: 1.0, material: ivory },
    // ];

    // let objects = [
    //     Square{center: Vec3::new(0.0, 0.0, 0.0),
    //     size: 2.0,
    //     normal: Vec3::new(0.0, 0.0, 1.0),  // cuadrado orientado hacia el eje Z
    //     material: Material::new(
    //         Color::new(255, 100, 80),
    //         1.0,
    //         [0.9, 0.1, 0.0, 0.0],
    //         0.0,),
    //     },
    // ];     

    let objects=[
        Cube{min: Vec3::new(0.0, 0.0, 0.0),
            max: Vec3::new(1.0, 1.0, 1.0),
            material: rubber.clone(),},
        Cube{min: Vec3::new(1.0, 0.0, 0.0),  // Segundo cubo desplazado a la derecha en el eje X
            max: Vec3::new(2.0, 1.0, 1.0),
            material: glass.clone(),},
    ];

    // Initialize camera
    let mut camera = Camera::new(
        Vec3::new(5.0, 5.0, 5.0),  // eye: Nueva posición de la cámara en diagonal
        Vec3::new(0.0, 0.0, 0.0),  // center: El cubo está en el origen
        Vec3::new(0.0, 1.0, 0.0)   // up: El eje "arriba" sigue siendo el eje Y
    );
    let rotation_speed = PI/10.0;
    let zoom_speed = 1.0;

    // let light = Light::new(
    //     Vec3::new(-1.0, 10.0, 1.0),
    //     Color::new(255, 255, 255),
    //     1.0
    // );


    // Definir dos luces con diferentes posiciones
    let lights = vec![
        Light {
            position: Vec3::new(5.0, 10.0, 5.0),  // Luz desde arriba (cara superior)
            intensity: 1.0,
            color: Color::new(255, 255, 255),
        },
        // Light {
        //     position: Vec3::new(0.0, -10.0, 0.0),  // Luz desde abajo (cara inferior)
        //     intensity: 0.8,
        //     color: Color::new(255, 255, 255),
        // },
        // Light {
        //     position: Vec3::new(-10.0, 0.0, 0.0),  // Luz desde la izquierda (cara izquierda)
        //     intensity: 0.8,
        //     color: Color::new(255, 255, 255),
        // },
        // Light {
        //     position: Vec3::new(10.0, 0.0, 0.0),  // Luz desde la derecha (cara derecha)
        //     intensity: 0.8,
        //     color: Color::new(255, 255, 255),
        // },
        // Light {
        //     position: Vec3::new(0.0, 0.0, 10.0),  // Luz desde adelante (cara frontal)
        //     intensity: 0.8,
        //     color: Color::new(255, 255, 255),
        // },
        // Light {
        //     position: Vec3::new(0.0, 0.0, -10.0),  // Luz desde atrás (cara trasera)
        //     intensity: 0.8,
        //     color: Color::new(255, 255, 255),
        // },
    ];

    while window.is_open() {
        // listen to inputs
        if window.is_key_down(Key::Escape) {
            break;
        }

        //  camera orbit controls
        if window.is_key_down(Key::Left) {
            camera.orbit(rotation_speed, 0.0);
        }
        if window.is_key_down(Key::Right) {
            camera.orbit(-rotation_speed, 0.0);
        }
        if window.is_key_down(Key::Up) {
            camera.orbit(0.0, -rotation_speed);
        }
        if window.is_key_down(Key::Down) {
            camera.orbit(0.0, rotation_speed);
        }

        // camera zoom controls
        if window.is_key_down(Key::Q) {
            camera.zoom(zoom_speed);
        }
        if window.is_key_down(Key::E) {
            camera.zoom(-zoom_speed);
        }

        if camera.is_changed() {
            // Render the scene
            render(&mut framebuffer, &objects, &camera, &lights[..]);
        }

        // update the window with the framebuffer contents
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
