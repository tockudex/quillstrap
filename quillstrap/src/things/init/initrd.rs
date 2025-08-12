use std::io::Write;

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
        vec!["quill-init"]
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
        ];
        AlpineChrootInstall::setup_alpine_chroot(
            options,
            &format!("{}", self.name()),
            package_vec,
            "aarch64",
        );
        // AlpineChrootInstall::turn_on_chroot(options, &format!("{}/", self.name()));
        dir_change(&cur_dir);

        run_command(
            &format!("chmod 555 bin/bbsuid"),
            options.config.command_output,
        )
        .unwrap();

        remove_file(format!("env.sh")).ok();
        remove_file(format!("destroy")).ok();
        remove_file(format!("enter-chroot")).ok();
        remove_file(format!("etc/motd")).ok();
        remove_file(format!("etc/resolv.conf")).ok();

        mkdir_p("opt/key");
        generate_public_key("opt/key/public.pem", options);

        copy_file("../quill-init/out/qinit", "etc/init.d/qinit").unwrap();
        copy_file("../quill-init/out/init", "sbin/init").unwrap();

        Ok(())
    }

    fn deploy(&self, options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        todo!();
    }

    fn run(&self) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }
}
