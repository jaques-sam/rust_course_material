//! Crate for handling the domotics in the house

/// Modules which contains all rooms in the house
pub mod house  // --> move to house/mod.rs
{
    pub mod kitchen  // --> move to house/kitchen.rs
    {
        pub fn turn_on_lights() {
        }
    }

    pub mod livingroom  // --> move to house/livingroom.rs
    {
        pub fn turn_on_lights() {
        }
    }

    /// Turn on all lights in the house
    pub fn turn_on_lights()
    {
        kitchen::turn_on_lights();
        livingroom::turn_on_lights();
    }
}

/// Turn on all lights
fn turn_on_all_lights()
{
    house::turn_on_lights();
}
