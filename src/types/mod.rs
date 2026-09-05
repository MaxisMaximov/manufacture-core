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
/// `isize` instead of `usize` to prevent Underflow warnings without much additional code
pub type SSCoords = (isize, isize);
/// Normalized Device Coordinates
pub type NDCoords = (f32, f32);
/// (R, G, B)
pub type CMDColor = (u8, u8, u8);

/// # CMD Renderer Command
/// A command that tells CMDRenderer what to draw to the Terminal screen
/// 
/// Push new Commands into `CMDRenderQueue` to schedule them
pub enum CMDRenderCommand{
    /// Draw a line between `a` and `b`
    DrawLine{
        a: NDCoords,
        b: NDCoords,
        z: f32,
        chr: char,
        fg: CMDColor,
        bg: CMDColor},
    /// Write text starting from `pos`
    /// 
    /// Supports newline breaks
    WriteText{
        pos: NDCoords,
        z: f32,
        text: String,
        fg: CMDColor,
        bg: CMDColor
    },
    /// Draw a rectangle from `a` to `b`
    /// 
    /// Not to be confused with `draw_box` which draws a hollow rectangle
    DrawRect{
        a: NDCoords,
        b: NDCoords,
        z: f32,
        chr: char,
        fg: CMDColor,
        bg: CMDColor
    },
    /// Draw a box from `a` to `b`
    /// 
    /// Not to be confused with `draw_rectangle` which draws a filled in box
    DrawBox{
        a: NDCoords,
        b: NDCoords,
        z: f32,
        chr: char,
        fg: CMDColor,
        bg: CMDColor
    },
    /// Draw a specified sprite from Sprite Registry at `pos`
    /// 
    /// Note: Sprites' anchor is currently on top-left corner
    /// 
    /// TODO: Add varying origin point
    DrawSprite{
        pos: NDCoords,
        z: f32,
        sprite_id: String
    }
}

/// # ASCII Image
/// Represents an ASCII art image
/// 
/// Individual *"pixels"* are `(ch, fg, bg)` tuples: `ch`aracter, `f`ore`g`round color and `b`ack`g`round color.  
/// FG and BG colors are `(R, G, B)` tuples that use `u8` as values
pub struct ASCIIImage{
    pub size_x: u8,
    pub size_y: u8,
    pub data: Box<[(char, (u8, u8, u8), (u8, u8, u8))]> // Symbol, Foreground RGB, Background RGB
}