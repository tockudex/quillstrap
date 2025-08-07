use std::{env::current_dir, fs::remove_dir_all};

use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct AlpineChrootInstall;

impl AlpineChrootInstall {
    pub fn execute(options: &crate::Options, arguments: &str) {
        let thing = get_thing_by_name("alpine-chroot-install", &options.things);
        let path = format!(
            "{}{}/{}{}/alpine-chroot-install",
            options.path_of_repo,
            MAIN_BUILD_DIR,
            thing.path(),
            thing.name()
        );
        run_command(
            &format!("{} {}", path, arguments),
            options.config.command_output,
        )
        .expect("Failed to run alpine-chroot-install");
    }

    // Chroot dir without / at the end
    pub fn setup_alpine_chroot(
        options: &crate::Options,
        chroot_dir: &str,
        alpine_packages: &str,
        arch: &str,
    ) {
        umount_recursive(chroot_dir);

        remove_dir_all(chroot_dir).unwrap();
        AlpineChrootInstall::execute(
            options,
            &format!("-d {} -p {} -a {}", chroot_dir, alpine_packages, arch),
        );
        umount_recursive(chroot_dir);

        // TODO
        run_shell_command("mv sysroot/sysroot/* sysroot/", true).unwrap();
        remove_dir_all("sysroot/sysroot").unwrap();

        run_command(
            &format!("chmod 555 {}/bin/bbsuid", chroot_dir),
            options.config.command_output,
        )
        .unwrap();

        remove_file(format!("{}/env.sh", chroot_dir)).ok();
        remove_file(format!("{}/destroy", chroot_dir)).ok();
        remove_file(format!("{}/enter-chroot", chroot_dir)).ok();
        remove_file(format!("{}/etc/motd", chroot_dir)).ok();
        remove_file(format!("{}/etc/resolv.conf", chroot_dir)).ok();
    }
}

impl SetupThing for AlpineChrootInstall {
    fn name(&self) -> &'static str {
        "alpine-chroot-install"
    }

    fn path(&self) -> &'static str {
        "init/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec![]
    }

    fn git(&self) -> &'static str {
        "alpine-chroot-install"
    }

    fn get(&self, options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        git_get_manage(self, &options);
        Ok(())
    }

    fn clean(&self) -> color_eyre::eyre::Result<(), String> {
        todo!()
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
