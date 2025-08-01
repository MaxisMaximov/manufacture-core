use std::collections::{HashMap, HashSet};

use super::system::*;
use super::world::World;

type Stage = Vec<Box<dyn SystemWrapper>>;

pub struct Dispatcher{
    registry: HashSet<&'static str>,
    systems: Vec<Stage>
}
impl Dispatcher{
    pub fn dispatch(&mut self, World: &mut World){
        for stage in self.systems.iter_mut(){
            for system in stage.iter_mut(){
                system.execute(World);
            }
        }
    }
}

#[must_use]
pub struct DispatcherBuilder{
    registry: HashMap<&'static str, Box<dyn SystemWrapper>>,
    dep_graph: Vec<HashSet<&'static str>>
}
impl DispatcherBuilder{
    pub fn new() -> Self{
        Self{
            registry: HashMap::new(),
            dep_graph: Vec::new()
        }
    }
    pub fn add<S: System>(&mut self){
        if self.registry.contains_key(S::ID){
            panic!("ERROR: System {} already exists", S::ID)
        }
        self.registry.insert(S::ID, Box::new(S::new()));
        self.dep_graph[0].insert(S::ID);
    }
    // Verify dependencies of each system
    fn verify_deps(&mut self){
        for system_id in self.dep_graph[0].iter(){
            for dep in self.registry.get(system_id).unwrap().depends(){
                if !self.registry.contains_key(dep){
                    panic!("ERROR: System {}'s dependency system {} does not exist", system_id, dep)
                }
            }
        }
    }
    // Build dependency 'graph' and resolve system order
    fn build_dep_graph(&mut self){
        // Welcome to indentation hell
        // Population: Graph Building
        let mut shifts = HashSet::new();
        for layer_id in 0..{
            let layer = self.dep_graph.get(layer_id).unwrap();
            // Iterate over layer's systems to see which we should shift
            for system_id in layer.iter(){
                for order_dep in self.registry.get(system_id).unwrap().run_order(){
                    match order_dep{
                        // If we need this system to run before, we shift the other system to later
                        RunOrder::Before(id) => {
                            if layer.contains(id){
                                shifts.insert(id.clone());
                            }
                        },
                        // Equivalent of the other system running before this one
                        // So we simply shift this one down
                        RunOrder::After(id) => {
                            if layer.contains(id){
                                shifts.insert(system_id);
                            }
                        },
                    }
                }
            }
            // No shifts happened, we're done with our graph
            if shifts.is_empty() {
                break;
            }
            // Push a new layer for the shifts..
            self.dep_graph.push(HashSet::new());
            // ..and now move all the systems from current layer to next layer
            // Also clears the shifts set for next layer
            for system_id in shifts.drain(){
                self.dep_graph[layer_id + 1].insert(system_id);
            }
        }
    }
    // Convert layers to stages & split them accordingly
    fn finalize_build(&mut self) -> Dispatcher{
        let mut registry = HashSet::new();
        let mut stages: Vec<Stage> = Vec::new();
        for mut layer in self.dep_graph.drain(0..){
            let mut stage = Vec::new();
            for system_id in layer.drain(){
                let system = self.registry.remove(system_id).unwrap();

                registry.insert(system_id);
                stage.push(system);
                // If stage is full already, push it to Stages and put a new one in it's place
                if stage.len() == 5{
                    stages.push(stage);
                    stage = Vec::new()
                }
            }
            // Push the incomplete stage just in case
            if !stage.is_empty(){
                stages.push(stage);
            }
        };
        
        Dispatcher{
            registry,
            systems: stages,
        }
    }
    pub fn build(mut self) -> Dispatcher{

        // First time splitting something into sepparate functions
        // But it's for the sake of readibility here
        self.verify_deps();
        self.build_dep_graph();
        self.finalize_build()
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
    Normal,
    Postprocessor
}