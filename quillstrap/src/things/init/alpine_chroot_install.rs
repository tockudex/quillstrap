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
        let str = &format!("{} {}", path, arguments);
        info!("Running command: {}", str);
        run_command(str, options.config.command_output)
            .expect("Failed to run alpine-chroot-install");
    }

    // Chroot dir without / at the end
    pub fn setup_alpine_chroot(
        options: &crate::Options,
        chroot_dir: &str,
        alpine_packages: Vec<&str>,
        arch: &str,
    ) {
        umount_recursive(chroot_dir);

        let chroot_dir_absolute = format!("{}/{}", dir_current(), chroot_dir);
        warn!("Chroot dir absolute is: {}", chroot_dir_absolute);

        remove_dir_all(chroot_dir).unwrap();
        AlpineChrootInstall::execute(
            options,
            &format!(
                "-d {} -p {} -a {}",
                chroot_dir_absolute,
                alpine_packages.join(" -p "),
                arch
            ),
        );
        umount_recursive(chroot_dir);

        /*
        // Path changed, this is wrong
        run_command(
            &format!("chmod 555 {}/{}/bin/bbsuid", chroot_dir),
            options.config.command_output,
        )
        .unwrap();

        remove_file(format!("{}/{}/env.sh", chroot_dir)).ok();
        remove_file(format!("{}/{}/destroy", chroot_dir)).ok();
        remove_file(format!("{}/{}/enter-chroot", chroot_dir)).ok();
        remove_file(format!("{}/{}/etc/motd", chroot_dir)).ok();
        remove_file(format!("{}/{}/etc/resolv.conf", chroot_dir)).ok();
        */
    }

    // Dir, with / at the end. It's the path to the actual sysroot, not sysroot and usr next to it
    pub fn turn_on_chroot(options: &crate::Options, dir: &str) {
        // let thing = get_thing_by_name("alpine-chroot-install", &options.things);
        mount_point(&format!("{}proc", dir), "proc");
        mount_point(&format!("{}sys", dir), "sysfs");
        mount_point(&format!("{}tmp", dir), "tmpfs");
        mount_point(&format!("{}run", dir), "tmpfs");
        mount_point(&format!("{}dev", dir), "devtmpfs");
        let dev_pts_path = &format!("{}dev/pts", dir);
        mkdir_p(dev_pts_path);
        mount_point(dev_pts_path, "devpts");
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
