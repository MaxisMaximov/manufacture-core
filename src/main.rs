use manufacture_engine::prelude::*;

fn main(){
    println!("RUNNING MANUFACTURE CORE AS A BINARY\nDO NOT DO THIS UNLESS IT'S FOR TESTING PURPOSES");
    eprintln!("RUNNING MANUFACTURE CORE AS A BINARY\nDO NOT DO THIS UNLESS IT'S FOR TESTING PURPOSES");

    let mut world = World::new();
    let mut disp_build = Dispatcher::new();

    manufacture_core::init(&mut world, &mut disp_build);

    let mut dispatcher = disp_build.build();

    dispatcher.dispatch(&mut world);
}