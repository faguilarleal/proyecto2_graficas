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

use framebuffer::Framebuffer;
use castray::cast_ray;
use color::Color;
use camera::Camera;
use light::Light;
use material::Material;
use cube::Cube;
use texture::Texture;

// texturas
static DIRT_TEXTURE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/dirt.jpg")));
static WATER_TEXTURE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/water.png")));
static GLASS_TEXTURE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/glass2.png")));
static LAVA_TEXTURE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/lava.jpg")));
static MADERA_TEXTURE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/madera.jpg")));
static LIBRO_TEXTURE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/libro.jpg")));
static HORNO_TEXTURE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/horno.jpg")));
static TREE_TEXTURE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/tree.jpg")));
static HOJAS_TEXTURE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/hojas.jpg")));


use rayon::prelude::*;

pub fn render(framebuffer: &mut Framebuffer, objects: &[Cube], camera: &Camera, lights: &[Light]) {
    let width = framebuffer.width;
    let height = framebuffer.height;
    let aspect_ratio = width as f32 / height as f32;
    let fov = PI / 3.0;
    let perspective_scale = (fov * 0.5).tan();

    framebuffer
        .buffer
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, pixel)| {
            let x = index % width;
            let y = index / width;

            let screen_x = (2.0 * x as f32) / width as f32 - 1.0;
            let screen_y = -(2.0 * y as f32) / height as f32 + 1.0;

            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));
            let rotated_direction = camera.basis_change(&ray_direction);
            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, lights, 0);

            *pixel = pixel_color.to_hex();
        });
}



fn update_sun_position(time: f32) -> Vec3 {
    let radius = 15.0; // Radio de la órbita del sol
    let angle = time;  // Controlar el ángulo de la órbita con el tiempo
    let x = radius * angle.cos();
    let y = radius * angle.sin(); // La altura del sol variará con el seno del ángulo
    let z = 10.0; // El sol permanece en una altura fija en el eje Z
    Vec3::new(x, y, z)
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

   

// -----------------   texturas  ---------------

    let dirt = Material::new_with_texture(
        1.0,
        [0.9, 0.1, 0.0, 0.0],
        0.0, DIRT_TEXTURE.clone(),
    );

    let water= Material::new_with_texture(
        50.0,                           // Especularidad baja, ajustada a la naturaleza del agua
    [0.8, 0.2, 0.1, 0.3],           // Albedo: aumenta el azul en los reflejos, disminuye un poco la transparencia
    1.33,                           // Índice de refracción del agua
    WATER_TEXTURE.clone(), 
                );

    let madera = Material::new_with_texture(
    1.0,
    [0.9, 0.1, 0.0, 0.0],
    0.0, MADERA_TEXTURE.clone(),           
        );
                
    let glass = Material::new_with_texture(
        0.0,                           // Especularidad baja
        [0.8, 0.2, 0.1, 0.6],          // Albedo con alta transparencia (0.05 en el canal alfa)
        1.5,                            // Índice de refracción del vidrio
        GLASS_TEXTURE.clone(),           // Textura de vidrio
    );

    let lava= Material::new_with_texture(
        1.0,
        [0.9, 0.1, 0.0, 0.0],
        0.0,                // Índice de refracción (opcional)
    LAVA_TEXTURE.clone(),       // La textura del cubo
     );            

    let libro= Material::new_with_texture(
        1.0,
        [0.9, 0.1, 0.0, 0.0],
        0.0, 
        LIBRO_TEXTURE.clone(),           
            );        

    let horno= Material::new_with_texture(
        1.0,
        [0.9, 0.1, 0.0, 0.0],
        0.0, 
        HORNO_TEXTURE.clone(),           
            );  

    let tree= Material::new_with_texture(
        1.0,
        [0.9, 0.1, 0.0, 0.0],
        0.0, 
        TREE_TEXTURE.clone(),           
            );    
    let hojas= Material::new_with_texture(
        1.0,
        [0.9, 0.1, 0.0, 0.1],
        0.0, 
        HOJAS_TEXTURE.clone(),           
            );    

// ------------ LUCES--------------
// Definir dos luces con diferentes posiciones
let mut lights = vec![
    Light {
        position: Vec3::new(1.0, 4.0, 10.0),  // Luz desde arriba (cara superior)
        // position: Vec3::new(10.0, 10.0, 10.0),  // Luz desde arriba (cara superior)
        intensity: 1.0,
        color: Color::new(255, 255, 255),
    },
   
];
        
    // ------------------- objetos ------------

    let cube_size = 0.5; // Tamaño de cada cubo
    let mut objects = Vec::new(); // Vec donde almacenaremos los cubos

    // Bucle anidado para generar cuadrícula 8x8
    for row in -3..3{
        for col in -3..3{
            let min = Vec3::new(col as f32 * cube_size, 0.0, row as f32 * cube_size);
            let max = Vec3::new(
                (col as f32 + 1.0) * cube_size,
                cube_size,
                (row as f32 + 1.0) * cube_size
            );
            // TEXTURA DE LAVA 
            if row == 0 && col == 0 || row == 0 && col == 1 {
                objects.push(Cube {
                    min,
                    max,
                    material: lava.clone(), // Utiliza el mismo material para todos los cubos
                    has_shadow: true, 
                    is_transparent:false
                });
                //  La luz emana desde la parte superior del cubo de lava
                let light_position = Vec3::new(
                    (col as f32 + 0.5) * cube_size,  // Centro del cubo en X
                    cube_size + 0.5,                       // Parte superior del cubo en Y
                    (row as f32 + 0.5) * cube_size   // Centro del cubo en Z
                );

                lights.push(Light {
                    position: light_position,            // Posición en la parte superior del cubo
                    intensity: 0.6,
                    color: Color::new(238, 163, 79),    // Luz blanca
                });
            }

            
            // TEXTURA DE AGUA 
            else if  (row == -2 && col == -1) ||( row == -1 && col == -1) ||( row == -2 && col == 0) {
                objects.push(Cube {
                    min,
                    max,
                    material: water.clone(), // Utiliza el mismo material para todos los cubos
                    has_shadow: true, 
                    is_transparent:false
                });
            }

            // TEXTURA DE TIERRA
            else {
                objects.push(Cube {
                    min,
                    max,
                    material: dirt.clone(), // Utiliza el mismo material para todos los cubos
                    has_shadow: true, 
                    is_transparent:false
                
                });
            }
        }
    }    
// ARBOL
    for height in 1..5{
        for row in -4..-1{
            let min = Vec3::new(
                row as f32 * cube_size,
                height as f32 * cube_size,
                -1.5);
            let max = Vec3::new(
                (row as f32 + 1.0) * cube_size,
                (height as f32+ 1.0) * cube_size,
                cube_size- 1.5
            );


            if row == -4 && height == 3 || row == -2 && height == 3 ||height == 4 && row != -2{
                objects.push(Cube {
                    min,
                    max,
                    material: hojas.clone(), // Utiliza el mismo material para todos los cubos
                    has_shadow: true, 
                    is_transparent:false

                });
            }

            
            else if row == -3 {
                objects.push(Cube {
                    min,
                    max,
                    material: tree.clone(), // Utiliza el mismo material para todos los cubos
                    has_shadow: true, 
                    is_transparent:false

                });
            }
        }
    } 

    objects.push(Cube{
        min:Vec3::new(
            -3 as f32 * cube_size,
            3 as f32 * cube_size,
            -0.5), 
        max:Vec3::new(
            -2 as f32 * cube_size,
            4 as f32 * cube_size,
            -1.0), 
        material: hojas.clone(), 
        has_shadow:true, 
        is_transparent: false,
    });
    objects.push(Cube{
        min:Vec3::new(
            -3 as f32 * cube_size,
            3 as f32 * cube_size,
            -2.0), 
        max:Vec3::new(
            -2 as f32 * cube_size,
            4 as f32 * cube_size,
            -1.5), 
        material: hojas.clone(), 
        has_shadow:true, 
        is_transparent: false,
    });

    let wall_height = 5;  // Define la altura de la pared
  
    // Crea una pared en un solo lado (a lo largo de X o Z)
    for height in 1..wall_height {  // Loop para la altura de la pared (de 0 a wall_height)
        for row in -3..3 {  // Rango horizontal de la pared (columna única, es decir, solo una pared)
            let min = Vec3::new(
                row as f32 * cube_size,       // Posición en X (horizontal, solo en un lado)
                height as f32 * cube_size,    // Posición en Y (altura, que crece hacia arriba)
                1.0                           // Posición fija en Z (o cambia a X para variar)
            );
            let max = Vec3::new(
                (row as f32 + 1.0) * cube_size,
                (height as f32 + 1.0) * cube_size,
                cube_size + 1.0                    // Altura de la pared
            );
            
            // Ventanas en las orillas (fila más baja y más alta)
            if (row == 1 ) && height >= 2 && height <= 3 {
                objects.push(Cube {
                    min,
                    max,
                    material: glass.clone(),  // Material de la ventana
                    has_shadow: false, 
                    is_transparent:true

                });
            }
            // NADA
            // COL X ROW Y
            else if  (row == -3 && height >= 2) ||( row == -2 && height >= 3)||( row == -1 && height >= 4) {
            
            }
            // hORNO
            else if  row == -2 && height == 1 {
                objects.push(Cube {
                    min,
                    max,
                    material: horno.clone(),  // Material de la pared
                    has_shadow: true,
                    is_transparent:false

                });
                let light_position = Vec3::new(
                    (row as f32 + 0.5) * cube_size,  // Centro del cubo en X
                    cube_size + 0.2,                       // Parte superior del cubo en Y
                    (height as f32 + 0.5) * cube_size   // Centro del cubo en Z
                );

                lights.push(Light {
                    position: light_position,            // Posición en la parte superior del cubo
                    intensity: 0.5,
                    color: Color::new(234, 210, 75),    // Luz blanca
                });
            }
            // LIBRO
            else if row == -1 && height == 1 {
                objects.push(Cube {
                    min,
                    max,
                    material: libro.clone(),  // Material de la pared
                    has_shadow: true,
                    is_transparent:false

                });
            }
            // Pared (sin ventanas)
            else {
                objects.push(Cube {
                    min,
                    max,
                    material: madera.clone(),  // Material de la pared
                    has_shadow: true,
                    is_transparent:false

                });
            }
        }
    }
    
   
    


   
    // Initialize camera
    let mut camera = Camera::new(
        Vec3::new(5.0, 5.0, 5.0),  // eye: Nueva posición de la cámara en diagonal
        Vec3::new(0.0, 0.0, 0.0),  // center: El cubo está en el origen
        Vec3::new(0.0, 1.0, 0.0)   // up: El eje "arriba" sigue siendo el eje Y
    );
    let rotation_speed = PI/50.0;
    let zoom_speed = 0.5;


    let mut time = 0.0;
    

    while window.is_open() {
        // listen to inputs
        if window.is_key_down(Key::Escape) {
            break;
        }

        //  camera orbit controls
        if window.is_key_down(Key::A) {
            camera.orbit(rotation_speed, 0.0);
        }
        if window.is_key_down(Key::D) {
            camera.orbit(-rotation_speed, 0.0);
        }
        if window.is_key_down(Key::W) {
            camera.orbit(0.0, -rotation_speed);
        }
        if window.is_key_down(Key::S) {
            camera.orbit(0.0, rotation_speed);
        }

        // camera zoom controls
        if window.is_key_down(Key::Up) {
            camera.zoom(zoom_speed);
        }
        if window.is_key_down(Key::Down) {
            camera.zoom(-zoom_speed);
        }

        // if camera.is_changed() {
        //     // Render the scene
        //     render(&mut framebuffer, &objects, &camera, &lights[..]);
        // }
        let sun_position = update_sun_position(time);
        lights[0].position = sun_position; // Mover la luz principal

        // Incrementa el tiempo para simular el paso del día
        time += 0.05; // Ajusta la velocidad del tiempo si es necesario

        render(&mut framebuffer, &objects, &camera, &lights[..]);


        // update the window with the framebuffer contents
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
