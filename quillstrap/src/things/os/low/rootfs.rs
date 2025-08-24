use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct Rootfs;

impl Rootfs {}

impl SetupThing for Rootfs {
    fn name(&self) -> &'static str {
        "rootfs"
    }

    fn path(&self) -> &'static str {
        "os/low/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec![]
    }

    fn git(&self) -> &'static str {
        todo!()
    }

    fn get(&self, options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        mkdir_p(self.name());
        Ok(())
    }

    fn clean(&self) -> color_eyre::eyre::Result<(), String> {
        Ok(())
    }

    fn build(&self, options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        Ok(())
    }

    fn deploy(&self, options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        todo!();
    }

    fn run(&self) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }
}