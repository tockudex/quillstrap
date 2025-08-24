use std::{fs::remove_dir_all, io::Write};

use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct InitRD;

impl InitRD {}

impl SetupThing for InitRD {
    fn name(&self) -> &'static str {
        "initrd"
    }

    fn path(&self) -> &'static str {
        "init/"
    }

    fn deps(&self) -> Vec<&'static str> {
        // Also kernel repo for .commit
        vec!["alpine-chroot-install"]
    }

    fn git(&self) -> &'static str {
        "initrd"
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        git_get_manage(self, _options);
        Ok(())
    }

    fn clean(&self) -> color_eyre::eyre::Result<(), String> {
        remove_dir_all("initrd_alpine").ok();
        Ok(())
    }

    fn build(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        let cur_dir = dir_current();
        // TODO: someone make this list smaller, not all is needed
        let package_vec: Vec<&str> = vec![
            "busybox",
            "busybox-extras",
            "libxkbcommon",
            "eudev",
            "udev-init-scripts",
            "libinput",
            "libgcc",
            "musl",
            "mtdev",
            "libevdev",
            "openssl",
            "dropbear",
            "dropbear-ssh",
            "dropbear-scp",
            "openssh-sftp-server",
            "fontconfig",
            "openrc",
            "fuse-overlayfs",
            "xz",
            "iwd",
            "openresolv",
            "gocryptfs",
            "tzdata",
            "gptfdisk",
        ];

        if !path_exists("initrd_alpine") {
            AlpineChrootInstall::setup_alpine_chroot(
                _options,
                &format!("initrd_alpine"),
                package_vec,
                "aarch64",
            );
        }

        // AlpineChrootInstall::turn_on_chroot(_options, &format!("{}/", self.name()));
        dir_change("initrd_alpine");

        run_command(
            &format!("chmod 555 bin/bbsuid"),
            _options.config.command_output,
        )
        .unwrap();

        remove_file(format!("env.sh")).ok();
        remove_file(format!("destroy")).ok();
        remove_file(format!("enter-chroot")).ok();
        remove_file(format!("etc/motd")).ok();
        remove_file(format!("etc/resolv.conf")).ok();

        remove_dir_all("var/cache").ok();

        mkdir_p("opt/key");
        generate_public_key("opt/key/public.pem", _options);

        // initrd_base is in kernel!
        // copy_file("../quill-init/out/qinit", "etc/init.d/qinit").unwrap();
        // copy_file("../quill-init/out/init", "sbin/init").unwrap();

        let commit = Kernel::get_kernel_commit(_options);
        let mut file = File::create(".commit").unwrap();
        file.write_all(commit.as_bytes()).unwrap();

        dir_change(&cur_dir);
        Ok(())
    }

    fn deploy(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        todo!();
    }

    fn run(&self) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }
}
