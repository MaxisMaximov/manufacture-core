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

    // -- Resources --
    world.register_res::<resources::CMDInput>();
    #[cfg(feature = "cmd_render_test")]
    {
        world.register_res::<resources::CMDSpriteRegistry>();
        world.register_res::<resources::CMDRendererQueue>();
    }

    // -- Systems --
    disp_build.add::<systems::CMDInputGetter>();

    #[cfg(feature = "cmd_render_test")]
    {
        disp_build.add::<systems::CMDRenderer>();
        disp_build.add::<systems::CMDDebugRenders>();
    }
    
    // -- Misc --
    #[cfg(feature = "cmd_render_test")]
    world.fetch_res_mut::<resources::CMDSpriteRegistry>().register(
        "CMD_RENDER_TEST".to_owned(),
        types::ASCIIImage{
            size_x: 6,
            size_y: 3,
            data: vec![
                ('%', (255, 255, 255), (255, 0, 0)),
                (' ', (255, 255, 255), (255, 0, 0)),
                (' ', (255, 255, 255), (255, 0, 0)),
                (' ', (255, 255, 255), (255, 0, 0)),
                (' ', (255, 255, 255), (255, 0, 0)),
                ('%', (255, 255, 255), (255, 0, 0)),
                
                ('#', (255, 255, 255), (0, 255, 0)),
                (' ', (255, 255, 255), (0, 255, 0)),
                (' ', (255, 255, 255), (0, 255, 0)),
                (' ', (255, 255, 255), (0, 255, 0)),
                (' ', (255, 255, 255), (0, 255, 0)),
                ('#', (255, 255, 255), (0, 255, 0)),

                ('&', (255, 255, 255), (0, 0, 255)),
                (' ', (255, 255, 255), (0, 0, 255)),
                (' ', (255, 255, 255), (0, 0, 255)),
                (' ', (255, 255, 255), (0, 0, 255)),
                (' ', (255, 255, 255), (0, 0, 255)),
                ('&', (255, 255, 255), (0, 0, 255)),
            ].into_boxed_slice(),
        }
    );
}