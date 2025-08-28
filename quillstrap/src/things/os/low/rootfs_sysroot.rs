use crate::prelude::*;

const ESSENTIAL_PACKAGES: &[&str] = &["zsh", "vim", "nano", "busybox", "libinput", "libinput-devel", "libinput-utils"];

#[derive(Clone, Copy, Default)]
pub struct RootfsSysroot;

// https://github.com/PorQ-Pine/rkbin
impl SetupThing for RootfsSysroot {
    fn name(&self) -> &'static str {
        "rootfs_sysroot"
    }

    fn path(&self) -> &'static str {
        "os/low/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec!["rootfs"]
    }

    fn git(&self) -> &'static str {
        todo!();
    }

    fn get(&self, _options: &Options) -> std::result::Result<(), String> {
        mkdir_p(self.name());
        dir_change(self.name());

        if path_exists("sysroot") {
            warn!(
                "Sysroot dir already present. We won't reextract it, use clean if you want to force this"
            );
            dir_change("../");
            return Ok(());
        }

        mkdir_p("sysroot");
        run_command(
            &format!("tar -xJf ../rootfs/rootfs.tar.xz -C sysroot"),
            _options.config.command_output,
        )
        .unwrap();

        dir_change("../");
        Ok(())
    }

    fn clean(&self) -> std::result::Result<(), String> {
        Ok(())
    }

    fn build(&self, _options: &Options) -> std::result::Result<(), String> {
        const RD: &str = "sysroot/";
        Rootfs::turn_on_chroot(RD);

        // Packages
        Rootfs::execute(RD, "dnf --assumeyes update", true);
        Rootfs::execute(
            RD,
            &format!("dnf --assumeyes install {}", ESSENTIAL_PACKAGES.join(" ")),
            true,
        );

        Ok(())
    }

    fn deploy(&self, _options: &Options) -> std::result::Result<(), String> {
        Ok(())
    }

    fn run(&self) -> std::result::Result<(), String> {
        Ok(())
    }
}
