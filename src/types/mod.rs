use std::marker::PhantomData;

use super::*;
use manufacture_engine::ECS::fetch::Fetch;
use manufacture_engine::ECS::entity::EntityBuilder;

mod vector;
pub use vector::*;

/// # Query Filter: With
/// Only allows Entities that have the specified Component to pass through
/// 
/// There is no need to include it in the filters if you fetch the Component,  
/// Query automatically checks whether the requested Components exist for an Entity
pub struct With<C: Component>(PhantomData<C>);
impl<C: Component> QueryFilter for With<C>{
    type Item<'b> = Fetch<'b, C>;

    fn fetch<'a>(world: &'a World) -> Self::Item<'a> {
        world.fetch::<C>()
    }

    fn filter<'qref, 'query: 'qref>(fetched: &'qref Self::Item<'query>, index: &usize) -> bool {
        fetched.get(index).is_some()
    }
}

/// # Query Filter: Without
/// Only allows Entities without the specified Component to pass through
pub struct Without<C: Component>(PhantomData<C>);
impl<C: Component> QueryFilter for Without<C>{
    type Item<'b> = Fetch<'b, C>;

    fn fetch<'a>(world: &'a World) -> Self::Item<'a> {
        world.fetch::<C>()
    }

    fn filter<'qref, 'query: 'qref>(fetched: &'qref Self::Item<'query>, index: &usize) -> bool {
        fetched.get(index).is_none()
    }
}

/// # Entity Prefab trait
/// A tiny rudimentary trait to make spawning Entities with Components easier
/// 
/// `spawn` method takes `&self` so that you can send custom data for Prefab to have right away
pub trait EntityPrefab{
    const ID: &'static str = "idkfa";
    fn spawn(&self, builder: EntityBuilder<'_>);
}

/// Screenspace coords
pub type SSCoords = (isize, isize);
/// Normalized Device Coordinates
pub type NDCoords = (f32, f32);
/// (R, G, B)
pub type CMDColor = (u8, u8, u8);

pub enum CMDRenderCommand{
    DrawLine{
        a: SSCoords,
        b: SSCoords,
        chr: char,
        fg: CMDColor,
        bg: CMDColor},
    WriteText{
        pos: SSCoords,
        text: String,
        fg: CMDColor,
        bg: CMDColor
    },
    DrawSequence{
        pos: SSCoords,
        sequence: Vec<(char, CMDColor, CMDColor)>
    },
    DrawRect{
        a: SSCoords,
        b: SSCoords,
        chr: char,
        fg: CMDColor,
        bg: CMDColor
    },
    DrawBox{
        a: SSCoords,
        b: SSCoords,
        chr: char,
        fg: CMDColor,
        bg: CMDColor
    },
    DrawSprite{
        pos: SSCoords,
        sprite_id: String
    }
}

pub struct ASCIIImage{
    pub size_x: u8,
    pub size_y: u8,
    pub data: Vec<(char, (u8, u8, u8), (u8, u8, u8))> // Symbol, Foreground RGB, Background RGB
}