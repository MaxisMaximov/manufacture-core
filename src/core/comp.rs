use std::collections::HashSet;

use super::ECS;

use ECS::comp::Component;
use crate::core::storage::*;

pub struct Vector2{
    pub x: f32,
    pub y: f32,
    pub z_index: u8,
}

pub struct Vector3{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub struct Tags{
    pub inner: HashSet<&'static str>
}

pub struct CMDSprite{
    pub size_x: u8,
    pub size_y: u8,
    pub data: Vec<(char, (u8, u8, u8), (u8, u8, u8))> // Symbol, Foreground RGB, Background RGB
}

pub struct PlayerController{
    pub pid: u16,
    pub active: bool,
}