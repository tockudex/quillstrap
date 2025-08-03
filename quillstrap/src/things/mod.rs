use crate::{prelude::*, things::low::{rkbin::Rkbin, uboot::Uboot}};

pub mod low;

#[derive(Clone, Copy)]
pub enum TraitWrapper {
    TWUboot(Uboot),
    TWRkbin(Rkbin),
}

// This is weird but I won't kill you with lifetimes at least
macro_rules! forward {
    ($self:ident.$method:ident $( ( $($arg:expr),* ) )? ) => {
        match $self {
            TraitWrapper::TWUboot(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TWRkbin(inner) => inner.$method($($($arg),*)?),
        }
    };
}

impl SetupThing for TraitWrapper {
    fn name(&self) -> &'static str {
        forward!(self.name())
    }

    fn path(&self) -> &'static str {
        forward!(self.path())
    }

    fn deps(&self) -> Vec<&'static str> {
        forward!(self.deps())
    }

    fn git(&self) -> &'static str {
        forward!(self.git())
    }

    fn get(&self, options: &Options) -> Result<(), String> {
        forward!(self.get(options))
    }

    fn clean(&self) -> Result<(), String> {
        forward!(self.clean())
    }

    fn build(&self, options: &Options) -> Result<(), String> {
        forward!(self.build(options))
    }

    fn deploy(&self, options: &Options) -> Result<(), String> {
        forward!(self.deploy(options))
    }

    fn run(&self) -> Result<(), String> {
        forward!(self.run())
    }
}

pub fn get_things() -> Vec<TraitWrapper> {
    vec![TWUboot(Uboot::default()), TWRkbin(Rkbin::default())]
}

pub fn get_thing_by_name(name: &str, things: &Vec<TraitWrapper>) -> TraitWrapper {
    for thing in things {
        if name == thing.name() {
            return *thing;
        }
    }
    panic!("You probably mistyped this: {}", name);
}