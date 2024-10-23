mod house {
    // extract to file with rust-mod-generator
    pub mod kitchen {
        pub fn turn_on_light() { println!("light turned on virtually"); }
        pub fn fridge_open() -> bool { true } // output of a sensor
    }
    pub use kitchen::fridge_open;
}

// belongs to this crate
pub fn turn_on_all_lights() {
    crate::house::kitchen::turn_on_light();
    // ...
}

fn main() {
    turn_on_all_lights();
    crate::turn_on_all_lights(); // same as previous

    let _ = house::fridge_open();
}
