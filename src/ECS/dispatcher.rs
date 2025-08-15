use std::collections::{HashMap, HashSet};
use std::time::{Instant};

use super::system::*;
use super::world::World;

type Stage = Vec<Box<dyn SystemWrapper>>;

pub struct Dispatcher{
    registry: HashMap<&'static str, SystemInfo>,
    preproc: Vec<Stage>,
    logic: Vec<Stage>,
    postproc: Vec<Stage>
}
impl Dispatcher{
    pub fn dispatch(&mut self, World: &mut World){
        let mut previous_tick = Instant::now();
        loop{
            // -- PREPROCESSORS --
            for stage in self.preproc.iter_mut(){
                for system in stage.iter_mut(){
                    system.execute(World);
                }
            }
            // -- LOGIC LOOP --
            if previous_tick.elapsed().as_millis() < 50{
                for stage in self.logic.iter_mut(){
                    for system in stage.iter_mut(){
                        system.execute(World);
                    }
                }
            }
            // -- POSTPROCESSORS --
            for stage in self.postproc.iter_mut(){
                for system in stage.iter_mut(){
                    system.execute(World);
                }
            }
            previous_tick = Instant::now();
        }
    }
}

#[must_use]
pub struct DispatcherBuilder{
    systems: HashMap<&'static str, SystemInfo>,
    preproc: StagesBuilder,
    logic: StagesBuilder,
    postproc: StagesBuilder,
}
impl DispatcherBuilder{
    pub fn new() -> Self{
        Self{
            systems: HashMap::new(),
            preproc: StagesBuilder::new(),
            logic: StagesBuilder::new(),
            postproc: StagesBuilder::new(),            
        }
    }
    pub fn add<S: System>(&mut self){

        if self.systems.contains_key(S::ID){
            panic!("ERROR: System {} already exists", S::ID)
        }

        self.systems.insert(S::ID, SystemInfo::new::<S>());

        match S::TYPE{
            SystemType::Preprocessor => self.preproc.add::<S>(),
            SystemType::Logic => self.logic.add::<S>(),
            SystemType::Postprocessor => self.postproc.add::<S>(),
        }
    }
    // Verify dependencies of each system
    fn verify_deps(&self){
        for system in self.systems.values(){
            for dep in system.depends.iter(){
                if !self.systems.contains_key(dep){
                    panic!("ERROR: System {}'s dependency system {} does not exist", system.id, dep)
                }
            }
        }
    }
    pub fn build(self) -> Dispatcher{

        self.verify_deps();

        Dispatcher{
            registry: self.systems,
            preproc: self.preproc.build(),
            logic: self.logic.build(),
            postproc: self.postproc.build(),
        }
    }
}

struct SystemInfo{
    id: &'static str,
    depends: &'static [&'static str],
    run_ord: &'static [RunOrder],
    sys_type: SystemType
}
impl SystemInfo{
    fn new<S: System>() -> Self{
        Self{
            id: S::ID,
            depends: S::DEPENDS,
            run_ord: S::RUNORD,
            sys_type: S::TYPE,
        }
    }
}

struct StagesBuilder{
    systems: HashMap<&'static str, Box<dyn SystemWrapper>>,
    graph: RunOrderGraph
}
impl StagesBuilder{
    fn new() -> Self{
        Self{
            systems: HashMap::new(),
            graph: RunOrderGraph::new(),
        }
    }
    fn add<S: System>(&mut self){
        self.systems.insert(S::ID, Box::new(S::new()));
        self.graph.add::<S>();
    }
    fn build(mut self) -> Vec<Vec<Box<dyn SystemWrapper>>>{

        let mut stages = Vec::new();

        // We don't need to use `.iter()` as the final graph will not be used for anything else, we also own it
        for layer in self.graph.build(){
            stages.push(Vec::new());
            for system_id in layer{
                // Don't like that I have to use so many unwraps
                stages.last_mut()
                    .unwrap()
                    .push(
                        self.systems.remove(system_id)
                        .unwrap()
                    );

                if stages.last().unwrap().len() == 5{
                    stages.push(Vec:: new());
                }
            }
        }

        stages
    }
}

struct RunOrderGraph{
    graph: Vec<HashMap<&'static str, &'static [RunOrder]>>
}
impl RunOrderGraph{
    fn new() -> Self{
        Self{
            graph: Vec::from([HashMap::new()]),
        }
    }
    fn add<S: System>(&mut self){
        self.graph[0].insert(S::ID, S::RUNORD);
    }
    fn build(mut self) -> Vec<Vec<&'static str>>{
        // Welcome to indentation hell
        // Population: Graph Building

        // Here to prevent unnecessary reallocation
        let mut shifts = HashSet::new();

        // Essentially iterate until everything is resolved
        for layer_id in 0..{

            // Unwrap: We only push new layers when there were shifts on previous layers
            // If there were none, we would break out of the loop
            let layer = self.graph.get(layer_id).unwrap();

            // Iterate over layer's systems to see which we should shift
            for (system_id, order_deps) in layer.iter(){
                
                for order_dep in order_deps.iter(){

                    match order_dep{
                        // If we need this system to run before, we shift the other system to later
                        RunOrder::Before(id) => {
                            if layer.contains_key(id){
                                shifts.insert(*id);
                            }
                        },
                        // Equivalent of the other system having `Before(this_system)`
                        // So we shift *this* one to later instead
                        RunOrder::After(id) => {
                            if layer.contains_key(id){
                                shifts.insert(*system_id);
                            }
                        },
                    }
                }
            }

            // No shifts happened, we're done with our graph
            if shifts.is_empty() {
                break;
            }

            // This should not happen unless there's a circular dependency between the systems
            if shifts.len() == layer.len(){
                panic!("ERROR: There are circular run orders between {} systems:\n{:#?}\nPlease resolve them", layer.len(), layer.keys())
            }

            // This is here to ensure the layer reference gets dropped
            // The compiler doesn't complain that we're pushing to the graph while having
            // a part of it borrowed in the later step, no idea why, usually it yells at me for that
            drop(layer);

            // Push a new layer and move all the shifted systems from current layer to next layer
            self.graph.push(HashMap::new());
            
            for system_id in shifts.drain(){ // Clear the shifts while we're at it
            let orders = self.graph[layer_id].remove(system_id).unwrap();
            self.graph[layer_id + 1].insert(
                system_id,
                orders
                );
            }
        };

        // Now convert it into a graph without the extra data
        let mut final_graph = Vec::new();

        for mut layer in self.graph.drain(0..){
            final_graph.push(Vec::new());
            for (id, _) in layer.drain(){
                final_graph.last_mut().unwrap().push(id);
            }
        }

        final_graph
    }
}

pub enum RunOrder{
    Before(&'static str),
    After(&'static str),
}
impl RunOrder{
    pub fn value(&self) -> &'static str{
        match *self{
            RunOrder::Before(val) => val,
            RunOrder::After(val) => val,
        }
    }
}

pub enum SystemType{
    Preprocessor,
    Logic,
    Postprocessor
}