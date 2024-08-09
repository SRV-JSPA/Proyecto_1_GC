mod framebuffer;
mod maze;
mod player;
mod caster;

use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::Vec2;
use std::f32::consts::PI;
use std::time::Duration;
use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::{Player, eventos_jugador};
use crate::caster::cast_ray;

fn dibujar_celdas(framebuffer: &mut Framebuffer, xo: usize, yo: usize, tamaño_block: usize, celda: char) {
    if celda == ' ' {
        return;
    }

    framebuffer.set_current_color(0xFFDDDD);

    for x in xo..xo + tamaño_block {
        for y in yo..yo + tamaño_block {
            framebuffer.point(x, y);
        }
    }
}

fn render(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("./maze.txt");
    let tamaño_block = 100;


    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            dibujar_celdas(framebuffer, col * tamaño_block, row * tamaño_block, tamaño_block, maze[row][col])
        }
    }


    framebuffer.set_current_color(0xFFDDD);
    framebuffer.point(player.pos.x as usize, player.pos.y as usize);


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

        framebuffer.set_current_color(0xFFFFFF);

        for y in stake_t..stake_b {
            framebuffer.point(i, y);
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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.clear();


        eventos_jugador(&window, &mut player);

        render3d(&mut framebuffer, &player);

        window
            .update_with_buffer(&framebuffer.buffer, ancho_framebuffer, altura_framebuffer)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}