mod framebuffer;
mod maze;
mod player;
mod caster;

use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::Vec2;
use std::f32::consts::PI;
use std::time::{Duration, Instant};
use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::{Player, eventos_jugador};
use crate::caster::cast_ray;
use rodio::{Sink, OutputStream};
use std::fs::File;
use std::io::BufReader;
use gilrs::Gilrs;

const FUENTE_NUMEROS: [[u8; 5]; 10] = [
    [0b01110, 0b10001, 0b10001, 0b10001, 0b01110], 
    [0b00100, 0b01100, 0b00100, 0b00100, 0b01110], 
    [0b01110, 0b10001, 0b00110, 0b01000, 0b11111], 
    [0b01110, 0b10001, 0b00110, 0b10001, 0b01110], 
    [0b00100, 0b01100, 0b10100, 0b11111, 0b00100], 
    [0b11111, 0b10000, 0b11110, 0b00001, 0b11110], 
    [0b01110, 0b10000, 0b11110, 0b10001, 0b01110], 
    [0b11111, 0b00010, 0b00100, 0b01000, 0b10000], 
    [0b01110, 0b10001, 0b01110, 0b10001, 0b01110], 
    [0b01110, 0b10001, 0b01111, 0b00001, 0b01110], 
];

fn dibujar_digitos(framebuffer: &mut Framebuffer, x: usize, y: usize, digito: u8) {
    if digito > 9 {
        return;
    }
    for (row, bits) in FUENTE_NUMEROS[digito as usize].iter().enumerate() {
        for col in 0..5 {
            if bits & (1 << (4 - col)) != 0 {
                if x + col < framebuffer.width && y + row < framebuffer.height {
                    framebuffer.point(x + col, y + row);
                }
            }
        }
    }
}

fn dibujar_fps(framebuffer: &mut Framebuffer, fps: u32) {
    let mut fps_string = fps.to_string();
    let eje_x = 10;
    let eje_y = 10;
    let tamaño_digito = 6;

    framebuffer.set_current_color(0xFFFFFF);

    for (i, ch) in fps_string.chars().enumerate() {
        if let Some(digito) = ch.to_digit(10) {
            dibujar_digitos(framebuffer, eje_x + i * tamaño_digito, eje_y, digito as u8);
        }
    }
}


fn dibujar_celdas(framebuffer: &mut Framebuffer, xo: usize, yo: usize, tamaño_block: usize, celda: char) {
    match celda {
        '+' => framebuffer.set_current_color(0xFFFFFF), 
        '|' => framebuffer.set_current_color(0x66CCFF), 
        '-' => framebuffer.set_current_color(0x003366), 
        _ => return,
    }

    for x in xo..xo + tamaño_block {
        for y in yo..yo + tamaño_block {
            if x < framebuffer.width && y < framebuffer.height {
                framebuffer.point(x, y);
            }
        }
    }
}

fn render(framebuffer: &mut Framebuffer, player: &Player, x: usize, y: usize, escala: f32) {
    let maze = load_maze("./maze.txt");
    let tamaño_block = (100.0 * escala) as usize;


    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            dibujar_celdas(framebuffer, x + col * tamaño_block, y + row * tamaño_block, tamaño_block, maze[row][col])
        }
    }


    framebuffer.set_current_color(0xFFDDD);
    let jugador_x = x + (player.pos.x * escala) as usize;
    let jugador_y = y + (player.pos.y * escala) as usize;
    if jugador_x < framebuffer.width && jugador_y < framebuffer.height {
        framebuffer.point(jugador_x, jugador_y);
    }


    let num_rays = 5;
    for i in 0..num_rays {
        let ray_actual = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * ray_actual);

        cast_ray(framebuffer, &maze, &player, a, tamaño_block, false);
    }
}

fn render3d(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("./maze.txt");
    let tamaño_block = 100;
    let num_rays = framebuffer.width;

    let hw = framebuffer.width as f32 / 2.0;
    let hh = framebuffer.height as f32 / 2.0;

    framebuffer.set_current_color(0xFFFFFF);

    for i in 0..num_rays {
        let ray_actual = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * ray_actual);
        let interseccion = cast_ray(framebuffer, &maze, &player, a, tamaño_block, false);

        let distancia_a_pared = interseccion.distance;
        let distancia_al_plano = 80.0; 
        let altura_stake = (hh / distancia_a_pared) * distancia_al_plano;

        let stake_t = (hh - (altura_stake / 2.0)) as usize;
        let stake_b = (hh + (altura_stake / 2.0)) as usize;

        match interseccion.impact {
            '+' => framebuffer.set_current_color(0xFFFFFF), 
            '|' => framebuffer.set_current_color(0x66CCFF), 
            '-' => framebuffer.set_current_color(0x003366), 
            _ => framebuffer.set_current_color(0xFFFFFF),  
        }

        if stake_t < framebuffer.height && stake_b <= framebuffer.height {
            for y in stake_t..stake_b {
                framebuffer.point(i, y);
            }
        }
    }
}


fn main() {
    let ancho_ventana = 1300;
    let altura_ventana = 900;
    let ancho_framebuffer = 1300;
    let altura_framebuffer = 900;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(ancho_framebuffer, altura_framebuffer);

    let mut window = Window::new(
        "Maze Runner",
        ancho_ventana,
        altura_ventana,
        WindowOptions::default(),
    ).unwrap();

    framebuffer.set_background_color(0x333355);

    let mut player = Player {
        pos: Vec2::new(150.0, 150.0),
        a: PI / 3.0,
        fov: PI / 3.0
    };

    let maze = load_maze("./maze.txt");
    let tamaño_bloque = 100; 

    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open("assets/musica.wav").unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    sink.play();
    let mut gilrs = Gilrs::new().unwrap();
    let mut tiempo = Instant::now();
    let mut contador_frame = 0;
    let mut fps = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let tiempo_inicial = Instant::now();
        framebuffer.clear();


        eventos_jugador(&window, &mut player, &maze, tamaño_bloque, &mut gilrs);

        render3d(&mut framebuffer, &player);

        let escala_minimapa = 0.1; 
        let ancho_minimapa = (ancho_framebuffer as f32 * escala_minimapa) as usize;
        let ancho_minimapa = (altura_framebuffer as f32 * escala_minimapa) as usize;
        let minimapa_x = ancho_framebuffer - ancho_minimapa - 45;
        let minimapa_y = 10;

        
        render(&mut framebuffer, &player, minimapa_x, minimapa_y, escala_minimapa);

        let duracion = tiempo_inicial.elapsed();
        let tiempo_frame = duracion.as_secs_f32();
        fps = (1.0 / tiempo_frame) as u32;
        dibujar_fps(&mut framebuffer, fps);

        window
            .update_with_buffer(&framebuffer.buffer, ancho_framebuffer, altura_framebuffer)
            .unwrap();

        std::thread::sleep(frame_delay);

        contador_frame += 1;
        if contador_frame % 60 == 0 {
            println!("FPS: {:.2}", fps);
        }

        if tiempo.elapsed() >= Duration::from_secs(1) {
            tiempo = Instant::now();
            contador_frame = 0;
        }
    }
    
    sink.stop();
}