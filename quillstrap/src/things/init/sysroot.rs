use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct Sysroot;

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
        let packages = "busybox busybox-extras libxkbcommon eudev udev-init-scripts libinput libgcc musl mtdev libevdev openssl fontconfig openrc pkgconf openssl-dev";
        AlpineChrootInstall::setup_alpine_chroot(options, &format!("{}", self.name()), packages, "aarch64");
        dir_change(cur_dir.to_str().unwrap());
        Ok(())
    }

    fn deploy(&self, options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        todo!();
    }

    fn run(&self) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }
}
