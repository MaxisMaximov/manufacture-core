use std::collections::HashSet;

use super::ECS;

use ECS::comp::Component;
use crate::core::storage::*;

pub struct Vector2{
    pub x: f32,
    pub y: f32,
    pub z_index: u8,
}
impl Component for Vector2{
    type STORAGE = BTreeMapStorage<Self>;

    const ID: &'static str = "Vector2";
}

pub struct Vector3{
    pub x: f32,
    pub y: f32,
    pub z: f32
}
impl Component for Vector3{
    type STORAGE = BTreeMapStorage<Self>;

    const ID: &'static str = "Vector3";
}

pub struct Tags{
    pub inner: HashSet<&'static str>
}
impl Component for Tags{
    type STORAGE = HashMapStorage<Self>;

    const ID: &'static str = "Tags";
}

pub struct CMDSprite{
    pub size_x: u8,
    pub size_y: u8,
    pub data: Vec<(char, (u8, u8, u8), (u8, u8, u8))> // Symbol, Foreground RGB, Background RGB
}
impl Component for CMDSprite{
    type STORAGE = HashMapStorage<Self>;

    const ID: &'static str = "CMDSprite";
}

pub struct PlayerController{
    pub pid: u16,
    pub active: bool,
}
impl Component for PlayerController{
    type STORAGE = VecStorage<Self>;

    const ID: &'static str = "PlayerController";
}