use std::{fs::remove_dir_all, path, str::FromStr};

use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct Kernel;

impl Kernel {
    pub fn get_kernel_commit(options: &crate::Options) -> String {
        let cur_dir = dir_current();
        let thing = get_thing_by_name("kernel", &options.things);
        let path_thing = get_path_of_thing(&thing, options);
        if !path_exists(&path_thing) {
            let path_thing_up = path_thing.clone().replace("kernel/", "");
            dir_change(&path_thing_up);
            git_get_manage(&thing, options);
        }
        dir_change(&path_thing);
        let str = run_command_get_output("git rev-parse --short HEAD");
        dir_change(&cur_dir);
        info!("Got kernel commit: {}", str);
        str
    }
}

impl SetupThing for Kernel {
    fn name(&self) -> &'static str {
        "kernel"
    }

    fn path(&self) -> &'static str {
        "init/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec!["initrd", "quill-init"]
    }

    fn git(&self) -> &'static str {
        "kernel"
    }

    fn get(&self, options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        git_get_manage(self, &options);
        Ok(())
    }

    fn clean(&self) -> color_eyre::eyre::Result<(), String> {
        run_command("make distclean", true).unwrap();
        Ok(())
    }

    fn build(&self, options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        remove_dir_all("../initrd/initrd_base/lib/").ok();

        copy_file(
            "../quill-init/out/qinit",
            "../initrd/initrd_base/etc/init.d/qinit",
        )
        .unwrap();
        copy_file("../quill-init/out/init", "../initrd/initrd_base/sbin/init").unwrap();

        run_shell_command(
            "make ARCH=arm64 CROSS_COMPILE=aarch64-linux-gnu- pinenote_defconfig",
            options.config.command_output,
        )
        .unwrap();

        run_shell_command(
            &format!(
                "make -j{} ARCH=arm64 CROSS_COMPILE=aarch64-linux-gnu-",
                get_cores()
            ),
            options.config.command_output,
        )
        .unwrap();

        let path_of_kernel_absolute =
            get_path_of_thing(&get_thing_by_name(self.name(), &options.things), options);
        run_shell_command(
            &format!("make -j{} ARCH=arm64 CROSS_COMPILE=aarch64-linux-gnu- modules_install INSTALL_MOD_PATH=\"{}../initrd/initrd_base/\"", get_cores(), path_of_kernel_absolute),
            options.config.command_output,
        )
        .unwrap();

        run_shell_command(
            &format!(
                "make -j{} ARCH=arm64 CROSS_COMPILE=aarch64-linux-gnu-",
                get_cores()
            ),
            options.config.command_output,
        )
        .unwrap();

        if !path_exists("arch/arm64/boot/Image.gz") {
            let err = "No Image.gz detected";
            error!("{}", err);
            return Err(err.to_string());
        }

        if !path_exists("arch/arm64/boot/dts/rockchip/rk3566-pinenote-v1.2.dtb") {
            let err = "No rk3566-pinenote-v1.2.dtb detected";
            error!("{}", err);
            return Err(err.to_string());
        }

        mkdir_p("out");
        copy_file("arch/arm64/boot/Image.gz", "out/Image.gz").unwrap();
        copy_file(
            "arch/arm64/boot/dts/rockchip/rk3566-pinenote-v1.2.dtb",
            "out/DTB",
        )
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
