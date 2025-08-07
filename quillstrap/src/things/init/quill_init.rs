use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct QuillInit;

impl SetupThing for QuillInit {
    fn name(&self) -> &'static str {
        "quill-init"
    }

    fn path(&self) -> &'static str {
        "init/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec![]
    }

    fn git(&self) -> &'static str {
        "quill-init"
    }

    fn get(&self, options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        git_get_manage(self, &options);
        Ok(())
    }

    fn clean(&self) -> color_eyre::eyre::Result<(), String> {
        run_command("cargo clean", false).expect("Failed to clean quill init");
        Ok(())
    }

    fn build(&self, options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        run_command("cargo build --release", options.config.command_output)
            .expect("Failed to build quill init");
        Ok(())
    }

    fn deploy(&self, options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
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
