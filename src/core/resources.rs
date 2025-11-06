use super::ECS;

use ECS::resource::Resource;

/// # Delta Time Resources
/// Tracks milliseconds elapsed since last frame and last Logic run. Use provided `delta_frame` and `delta_logic` methods to get the time
/// 
/// Only the Dispatcher can modify the inner values, fetching this mutably does not do anything
pub struct DeltaT{
    delta_frame: u128,
    delta_logic: u128
}
impl DeltaT{
    /// Get milliseconds elapsed since last frame
    pub fn delta_frame(&self) -> u128{
        self.delta_frame
    }
    /// Get milliseconds elapsed since last time Logic ran
    pub fn delta_logic(&self) -> u128{
        self.delta_logic
    }
}
impl Resource for DeltaT{
    const ID: &'static str = "DeltaT";

    fn new() -> Self {
        Self{
            delta_frame: 0,
            delta_logic: 0
        }
    }
}