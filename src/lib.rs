use manufacture_engine::ECS::dispatcher::DispatcherBuilder;
use manufacture_engine::prelude::*;

pub mod commands;
pub mod comp;
pub mod events;
pub mod resources;
pub mod storage;
pub mod types;
pub mod prelude;
pub mod systems;

/// Initiate Core library
/// # THIS IS NOT OPTIONAL
/// The engine relies on some things from the Core library and will not function without them, plugins rely on it too
pub fn init(world: &mut World, disp_build: &mut DispatcherBuilder){
    // -- Components --
    world.register_comp::<comp::CMDSprite>();
    world.register_comp::<comp::PlayerController>();
    world.register_comp::<comp::Tags>();
    world.register_comp::<comp::Transform2D>();
    world.register_comp::<comp::Transform3D>();

    // -- Events --
    world.register_event::<events::EntitySpawned>();
    world.register_event::<events::EntityDespawned>();
    world.register_event::<ExitApp>();

    // -- Resources --
    world.register_res::<resources::CMDInput>();
    world.register_res::<DeltaT>();

    // -- Systems --
    disp_build.add::<systems::CMDInputHandler>();
}