use crate::{prelude::*};

#[derive(Clone, Copy, Default)]
pub struct Kernel;

impl SetupThing for Kernel {
    fn name(&self) -> &'static str {
        "kernel"
    }

    fn path(&self) -> &'static str {
        "low/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec!["uboot"]
    }

    fn git(&self) -> &'static str {
        "kernel"
    }

    fn get(&self, options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        git_get_manage(self, &options);
        Ok(())
    }

    fn clean(&self) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }

    fn build(&self, options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }

    fn deploy(&self, options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }

    fn run(&self) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }
}