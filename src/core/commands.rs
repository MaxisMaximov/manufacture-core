use super::*;

use events::{EntitySpawned, EntityDespawned};

pub struct Spawn;
impl Command for Spawn{
    fn execute(&mut self, World: &mut World) {
        let token = World.spawn().get_token();
        World.get_event_writer::<EntitySpawned>().send(EntitySpawned(token));
    }
}

pub struct DespawnID(pub usize);
impl Command for DespawnID{
    fn execute(&mut self, World: &mut World) {
        World.despawn(self.0);
        World.get_event_writer::<EntityDespawned>().send(EntityDespawned(self.0));
    }
}

pub struct DespawnToken(pub Token);
impl Command for DespawnToken{
    fn execute(&mut self, World: &mut World) {
        if World.despawn_with_token(self.0){
            World.get_event_writer::<EntityDespawned>().send(EntityDespawned(self.0.id()));
        }
    }
}