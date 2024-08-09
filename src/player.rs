use nalgebra_glm::Vec2;
use std::f32::consts::PI;
use minifb::{Window, Key};
use crate::caster::tope_pared; 
use crate::framebuffer::Framebuffer;

pub struct Player {
    pub pos: Vec2,
    pub a: f32, 
    pub fov: f32, 
}

pub fn eventos_jugador(window: &Window, player: &mut Player, maze: &Vec<Vec<char>>, tamaño_bloque: usize) {
    const MOVE_SPEED: f32 = 5.0;
    const ROTATION_SPEED: f32 = PI / 35.0;

    if window.is_key_down(Key::Left) {
        player.a -= ROTATION_SPEED;
    }
    if window.is_key_down(Key::Right) {
        player.a += ROTATION_SPEED;
    }
    if window.is_key_down(Key::Up) {
        let direction = Vec2::new(player.a.cos(), player.a.sin());
        let nueva_pos = player.pos + direction * MOVE_SPEED;
        if !tope_pared(maze, &nueva_pos, tamaño_bloque) {
            player.pos = nueva_pos;
        }
    }
    if window.is_key_down(Key::Down) {
        let direction = Vec2::new(player.a.cos(), player.a.sin());
        let nueva_pos = player.pos - direction * MOVE_SPEED;
        if !tope_pared(maze, &nueva_pos, tamaño_bloque) {
            player.pos = nueva_pos;
        }
    }
}