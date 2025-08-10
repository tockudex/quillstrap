use std::io::Write;

use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct Sysroot;

impl Sysroot {
    pub fn execute_sysroot_command(command: &str, options: &crate::Options) {
        let cur_dir = dir_current();
        let thing = get_thing_by_name("sysroot", &options.things);
        let path = &format!(
            "{}{}/{}{}",
            options.path_of_repo,
            MAIN_BUILD_DIR,
            thing.path(),
            thing.name()
        );
        // info!("Sysroot path: {}", path);
        AlpineChrootInstall::turn_on_chroot(options, &format!("{}/", path));
        dir_change(path);
        if !path_exists("quillstrap") || !is_mount_point("quillstrap") {
            mkdir_p("quillstrap");
            run_command(&format!("mount --bind {} quillstrap/", options.path_of_repo), options.config.command_output).unwrap();
        }

        run_shell_command(
            &format!("./enter-chroot bash -c \"source ~/.bashrc; {}\"", command),
            options.config.command_output,
        )
        .unwrap();
        dir_change(&cur_dir);
    }
}

impl SetupThing for Sysroot {
    fn name(&self) -> &'static str {
        "sysroot"
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
        Ok(())
    }

    fn clean(&self) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }

    fn build(&self, options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        let cur_dir = dir_current();
        dir_change("../");
        // TODO: someone make this list smaller, not all is needed
        let package_vec: Vec<&str> = vec![
            "busybox",
            "busybox-extras",
            "libxkbcommon",
            "eudev",
            "libinput",
            "libgcc",
            "musl",
            "mtdev",
            "libevdev",
            "openssl",
            "fontconfig",
            "pkgconf",
            "openssl-dev",
            "eudev-dev",
            "libinput-dev",
            "libxkbcommon-dev",
            "curl",
            "gcc",
            "bash",
            "musl-dev",
            "clang",
            "clang19-libclang",
            "clang19-dev",
        ];
        AlpineChrootInstall::setup_alpine_chroot(
            options,
            &format!("{}", self.name()),
            package_vec,
            "aarch64",
        );
        AlpineChrootInstall::turn_on_chroot(options, &format!("{}/", self.name()));
        dir_change(&cur_dir);

        Sysroot::execute_sysroot_command(
            "curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y",
            options,
        );

        let mut file = File::create("root/.bashrc").unwrap();
        file.write_all(". \"$HOME/.cargo/env\"".to_string().as_bytes())
            .unwrap();

        Ok(())
    }

    fn deploy(&self, options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        todo!();
    }

    fn run(&self) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }
}
