use super::*;

/// Announces what Entity has just spawned
/// 
/// Provides a Token alongside for Systems to copy
pub struct EntitySpawned(pub Token);
impl Event for EntitySpawned{
    const ID: &'static str = "EntitySpawned";
}

/// Announced what Entity has just despawned
pub struct EntityDespawned(pub usize);
impl Event for EntityDespawned{
    const ID: &'static str = "EntityDespawned";
}

/// **System Level Event**
/// 
/// Announces to the Dispatcher that the app is to be shut down
/// 
/// TODO: Error Codes themselves
pub struct ExitApp(pub i32);
impl Event for ExitApp{
    const ID: &'static str = "_APP_EXIT";
}