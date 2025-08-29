use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct Qoms;

impl SetupThing for Qoms {
    fn name(&self) -> &'static str {
        "qoms"
    }

    fn path(&self) -> &'static str {
        "os/low/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec![]
    }

    fn git(&self) -> &'static str {
        "qoms"
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        git_get_manage(self, _options);
        Ok(())
    }

    fn clean(&self) -> color_eyre::eyre::Result<(), String> {
        Ok(())
    }

    fn build(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        run_command("./build.sh", _options.config.command_output).unwrap();
        mkdir_p("out");
        copy_file("qoms/target/aarch64-unknown-linux-gnu/release/qoms", "out/qoms").unwrap();
        Ok(())
    }

    fn deploy(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        todo!();
    }

    fn run(&self) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }
}