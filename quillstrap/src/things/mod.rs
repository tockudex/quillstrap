use crate::{prelude::*, things::low::uboot::Uboot};

pub mod low;

#[derive(Clone, Copy)]
pub enum TraitWrapper {
    TWUboot(Uboot),
}

pub fn get_things() -> Vec<TraitWrapper> {
    vec![TWUboot(Uboot::default())]
}

pub fn get_thing_by_name(name: &str, things: &Vec<TraitWrapper>) -> impl SetupThing {
    for thing in things {
        match thing {
            TWUboot(uboot) => return *uboot,
        }
    }
    panic!("No item found in things, this is really weird: {}", name);
}