
use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct QuillInit;

impl SetupThing for QuillInit {
    fn name(&self) -> &'static str {
        "quill_init"
    }

    fn path(&self) -> &'static str {
        "init/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec!["sysroot", "branding"]
    }

    fn git(&self) -> &'static str {
        "quill-init"
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        git_get_manage(self, &_options);
        Ok(())
    }

    fn clean(&self) -> color_eyre::eyre::Result<(), String> {
        run_command("cargo clean", false).expect("Failed to clean quill init");
        Ok(())
    }

    fn build(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        let cur_dir = dir_current();
        mkdir_p("out/");

        dir_change("qinit");

        let mut features_normal: Vec<&str> = vec![];
        let mut features_wrapper: Vec<&str> = vec!["init_wrapper"];

        if _options.config.unrestricted {
            features_normal.push("free_roam");
            features_wrapper.push("free_roam");
        }

        if _options.config.unsecure_debug {
            features_normal.push("debug");
            features_wrapper.push("debug");
        }

        // RUST_FLAGS=\"-C target-feature=-crt-static\" is applied in config.toml

        // TODO features from config
        Sysroot::execute_sysroot_command_dir(
            &format!("cargo build --release --features={}", features_normal.join(",")),
            &cur_dir,
            _options,
        );
        copy_file("../target/release/qinit", "../out/qinit").unwrap();
        Sysroot::execute_sysroot_command_dir(
            &format!("cargo build --release --features={}", features_wrapper.join(",")),
            &cur_dir,
            _options,
        );
        copy_file("../target/release/qinit", "../out/init").unwrap();

        Ok(())
    }

    fn deploy(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        /*
        cd "$(dirname ${0})"
        BINARY_NAME="qinit"
        BINARY_PATH="/tmp/qinit"
        ssh "root@${1}" -p "${2}" killall qinit
        scp -P "${2}" "target/release/${BINARY_NAME}" "root@${1}:${BINARY_PATH}"
        ssh "root@${1}" -t -p "${2}" 'env RUST_LOG=info SLINT_KMS_ROTATION=270 '"${BINARY_PATH}"''
        */
        todo!();
    }

    fn run(&self) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }
}
