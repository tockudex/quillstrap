use crate::{common::commands::{run_command, run_shell_command}, prelude::*};

#[derive(Clone, Copy, Default)]
pub struct Uboot {}

impl Uboot {
    pub fn new() -> Self {
        Uboot {}
    }
}

// https://github.com/PorQ-Pine/u-boot-pinenote
impl SetupThing for Uboot {
    fn name(&self) -> &'static str {
        "uboot"
    }

    fn path(&self) -> &'static str {
        "low/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec!["rkbin"]
    }

    fn git(&self) -> &'static str {
        "u-boot-pinenote"
    }

    fn get(&self, options: &Options) -> std::result::Result<(), String> {
        git_get_manage(self, &options);
        Ok(())
    }

    fn clean(&self) -> std::result::Result<(), String> {
        todo!()
    }

    fn build(&self, options: &Options) -> std::result::Result<(), String> {
        set_var("CROSS_COMPILE", "/usr/bin/aarch64-linux-gnu-");

        info!("Building uboot defconfig");
        run_command("make rk3566-pinenote_defconfig", options.config.command_output)
            .expect("Failed to make rk3566-pinenote_defconfig");

        info!("Building uboot rk3566-pinenote");
        run_command("./make.sh rk3566-pinenote", options.config.command_output).expect("Failed to make.sh rk3566-pinenote");
        info!("Running uboot trust");
        run_command("./make.sh trust", options.config.command_output).expect("Failed to make.sh trust");

        info!("Building uboot spl");
        run_command("./make.sh spl", options.config.command_output).expect("Failed to make.sh spl");
        info!("Running uboot trust, again");
        run_command("./make.sh trust", options.config.command_output).expect("Failed to make.sh trust (second time)");
        
        info!("Running build of logotool_mod");
        let cur_dir = dir_current();
        mkdir_p("pinenote_ui/logotool_mod/build");
        dir_change("pinenote_ui/logotool_mod/build");
        run_command("cmake ..", options.config.command_output).expect("logotool_mod cmake failed");
        run_command("cmake --build .", options.config.command_output).expect("logotool_mod cmake failed");
        dir_change(cur_dir.to_str().unwrap());

        info!("Running build of boot menu");
        run_shell_command("pushd pinenote_ui/; ./build_all.sh; popd", options.config.command_output).expect("Failed to build uboot boot menu");

        // Clean the var
        set_var("CROSS_COMPILE", "");
        if !path_exists("uboot.img") {
            let err =
                "Failed to build uboot, everything seems fine but output file uboot.img is missing";
            error!("{}", err);
            return Err(err.to_string());
        } else {
            info!("Builded uboot succesfully!");
            return Ok(());
        }
    }

    fn deploy(&self) -> std::result::Result<(), String> {
        todo!()
    }

    fn run(&self) -> std::result::Result<(), String> {
        todo!()
    }
}
