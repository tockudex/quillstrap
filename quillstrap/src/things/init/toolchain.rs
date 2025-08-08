use std::{env, fs::File, io::Write, str::FromStr};

use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct Toolchain;

impl Toolchain {
    pub fn add_to_path(&self, options: &crate::Options) {
        // Add him to PATH
        let path = env::var("PATH").unwrap();
        let absolute_path_here = format!(
            "{}{}{}/aarch64-linux-musl-cross/bin",
            options.path_of_repo,
            self.path(),
            self.name()
        );
        info!("Absolute path to toolchain bins: {}", absolute_path_here);
        unsafe { env::set_var("PATH", format!("{}:{}", path, absolute_path_here)) };
    }
}

impl SetupThing for Toolchain {
    fn name(&self) -> &'static str {
        "toolchain"
    }

    fn path(&self) -> &'static str {
        "init/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec![]
    }

    fn git(&self) -> &'static str {
        todo!()
    }

    fn get(&self, options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        mkdir_p(self.name());
        dir_change(self.name());
        run_command("wget https://github.com/PorQ-Pine/aarch64-linux-musl-cross/releases/download/1/aarch64-linux-musl-cross.tgz", options.config.command_output).unwrap();
        run_command(
            "tar -xf aarch64-linux-musl-cross.tgz",
            options.config.command_output,
        )
        .unwrap();
        remove_file("aarch64-linux-musl-cross.tgz".to_string()).unwrap();
        dir_change("../");
        Ok(())
    }

    fn clean(&self) -> color_eyre::eyre::Result<(), String> {
        Ok(())
    }

    fn build(&self, options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        Ok(())
    }

    fn deploy(&self, options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        Ok(())
    }

    fn run(&self) -> color_eyre::eyre::Result<(), String> {
        let str = "#include <stdio.h>\nint main() { return printf(\"Hello world\"), 0; }";
        let mut file = File::create("hello.c").unwrap();
        file.write_all(String::from_str(str).unwrap().as_bytes())
            .unwrap();

        run_command("aarch64-linux-musl-gcc -static hello.c", false).unwrap();
        let output = run_command_get_output("./a.out");
        if output.contains("Hello world") {
            info!("Binfmt and toolchain works");
            return Ok(());
        } else {
            let err = "Binfmt and toolchain doesn't work, you need to enable binfmt on host system";
            error!("{}", err);
            return Err(String::from_str(err).unwrap());
        }
    }
}
