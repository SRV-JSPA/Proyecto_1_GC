use nalgebra_glm::Vec2;
use std::f32::consts::PI;
use minifb::{Window, Key};
use crate::caster::tope_pared; 
use crate::framebuffer::Framebuffer;
use gilrs::{Gilrs, Button, Event, EventType, Axis};

pub struct Player {
    pub pos: Vec2,
    pub a: f32, 
    pub fov: f32, 
}

pub fn eventos_jugador(window: &Window, player: &mut Player, maze: &Vec<Vec<char>>, tamaño_bloque: usize, gilrs: &mut Gilrs) {
    const MOVE_SPEED: f32 = 5.0;
    const ROTATION_SPEED: f32 = PI / 35.0;
    const JOYSTICK_SENS: f32 = 0.1;

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

    if window.is_key_down(Key::A) {
        player.a -= ROTATION_SPEED;
    }
    if window.is_key_down(Key::D) {
        player.a += ROTATION_SPEED;
    }
    if window.is_key_down(Key::W) {
        let direction = Vec2::new(player.a.cos(), player.a.sin());
        let nueva_pos = player.pos + direction * MOVE_SPEED;
        if !tope_pared(maze, &nueva_pos, tamaño_bloque) {
            player.pos = nueva_pos;
        }
    }
    if window.is_key_down(Key::S) {
        let direction = Vec2::new(player.a.cos(), player.a.sin());
        let nueva_pos = player.pos - direction * MOVE_SPEED;
        if !tope_pared(maze, &nueva_pos, tamaño_bloque) {
            player.pos = nueva_pos;
        }
    }

    let mut joystick_x = 0.0;
    let mut joystick_y = 0.0;
    let mut joystick_rotacion = 0.0;

    while let Some(Event { id: _, event, time: _ }) = gilrs.next_event() {
        match event {
            EventType::ButtonPressed(Button::DPadLeft, _) => {
                player.a -= ROTATION_SPEED;
            }
            EventType::ButtonPressed(Button::DPadRight, _) => {
                player.a += ROTATION_SPEED;
            }
            EventType::ButtonPressed(Button::DPadUp, _) => {
                let direccion = Vec2::new(player.a.cos(), player.a.sin());
                let nueva_pos = player.pos + direccion * MOVE_SPEED;
                if !tope_pared(maze, &nueva_pos, tamaño_bloque) {
                    player.pos = nueva_pos;
                }
            }
            EventType::ButtonPressed(Button::DPadDown, _) => {
                let direccion = Vec2::new(player.a.cos(), player.a.sin());
                let nueva_pos = player.pos - direccion * MOVE_SPEED;
                if !tope_pared(maze, &nueva_pos, tamaño_bloque) {
                    player.pos = nueva_pos;
                }
            }
            EventType::AxisChanged(Axis::LeftStickX, value, _) => {
                joystick_x = value;
            }
            EventType::AxisChanged(Axis::LeftStickY, value, _) => {
                joystick_y = value;
            }
            EventType::AxisChanged(Axis::RightStickX, value, _) => {
                joystick_rotacion = value;
            }
            _ => {}
        }
    }

    if joystick_rotacion.abs() > JOYSTICK_SENS {
        player.a += joystick_rotacion * ROTATION_SPEED;
    }
    if joystick_x.abs() > JOYSTICK_SENS || joystick_y.abs() > JOYSTICK_SENS {
        let direccion = Vec2::new(player.a.cos(), player.a.sin());
        let zona_segura = Vec2::new(player.a.sin(), -player.a.cos());
        let nueva_pos = player.pos
            + direccion * joystick_y * MOVE_SPEED
            + zona_segura * joystick_x * MOVE_SPEED;
        if !tope_pared(maze, &nueva_pos, tamaño_bloque) {
            player.pos = nueva_pos;
        }
    }
}