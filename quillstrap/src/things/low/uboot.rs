use crate::{prelude::*, thetrait::SetupThing};

#[derive(Clone, Copy, Default)]
pub struct Uboot {
    
}

impl Uboot {
    pub fn new() -> Self {
        Uboot {}
    }
}

impl SetupThing for Uboot {
    fn name(&self) -> &'static str {
        "uboot"
    }

    fn path(&self) -> &'static str {
        "low/"
    }

    fn deps(&self) -> Vec< &'static str> {
        todo!()
    }

    fn git(&self) -> &'static str {
        "u-boot-pinenote"
    }

    fn get(&self) -> std::result::Result<(), String> {
        todo!()
    }

    fn clean(&self) -> std::result::Result<(), String> {
        todo!()
    }

    fn build(&self) -> std::result::Result<(), String> {
        todo!()
    }

    fn deploy(&self) -> std::result::Result<(), String> {
        todo!()
    }

    fn run(&self) -> std::result::Result<(), String> {
        todo!()
    }
}