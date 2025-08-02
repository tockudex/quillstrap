use crate::{prelude::*, things::low::uboot::Uboot};

pub mod low;

pub fn get_things() -> Vec<Box<dyn SetupThing>> {
    vec![Box::new(Uboot::new())]
}

pub fn get_thing_by_name(name: &str, things: Vec<Box<dyn SetupThing>>) -> impl SetupThing {
    for thing in things.iter() {
        if thing.name() == name {
            return thing;
        }
    }
}