use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct SlintGallery;

impl SetupThing for SlintGallery {
    fn name(&self) -> &'static str {
        "slint_gallery"
    }

    fn path(&self) -> &'static str {
        "os/gui/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec![]
    }

    fn git(&self) -> &'static str {
        "slint-gallery"
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        git_get_manage(self, _options);
        Ok(())
    }

    fn clean(&self) -> color_eyre::eyre::Result<(), String> {
        Ok(())
    }

    fn build(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        mkdir_p("out");
        // Building for host
        run_command("cargo build --release --features=host", _options.config.command_output).unwrap();
        copy_file(
            "target/release/gallery",
            "out/slint_gallery_host",
        )
        .unwrap();

        // Building for rootfs
        set_var("PKG_CONFIG_ALLOW_CROSS", "1");
        set_var("PKG_CONFIG_SYSROOT_DIR", "/workspaces/quillstrap/build_all/os/low/rootfs_sysroot/sysroot");
        set_var("RUSTFLAGS", "-L /workspaces/quillstrap/build_all/os/low/rootfs_sysroot/sysroot/usr/lib64");
        // set_var("CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER", "aarch64-linux-gnu-gcc");

        // Important! glibc version specified by zig
        run_command(
            "cargo zigbuild --release --features=pinenote --target aarch64-unknown-linux-gnu.2.41",
            _options.config.command_output,
        )
        .unwrap();

        set_var("PKG_CONFIG_ALLOW_CROSS", "");
        set_var("PKG_CONFIG_SYSROOT_DIR", "");
        set_var("PKG_CONFIG_PATH", "");
        set_var("RUSTFLAGS", "");

        copy_file(
            "target/aarch64-unknown-linux-gnu/release/gallery",
            "out/slint_gallery_rootfs",
        )
        .unwrap();

        // Building for initrd
        Ok(())
    }

    fn deploy(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        todo!();
    }

    fn run(&self) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }
}
