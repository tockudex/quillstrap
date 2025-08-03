use crate::{common::commands::run_command, prelude::*};

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

    fn build(&self) -> std::result::Result<(), String> {
        set_var("CROSS_COMPILE", "/usr/bin/aarch64-linux-gnu-");

        info!("Building uboot defconfig");
        run_command("make rk3566-pinenote_defconfig", true)
            .expect("Failed to make rk3566-pinenote_defconfig");

        info!("Building uboot rk3566-pinenote");
        run_command("./make.sh rk3566-pinenote", true).expect("Failed to make.sh rk3566-pinenote");
        info!("Running uboot trust");
        run_command("./make.sh trust", true).expect("Failed to make.sh trust");

        info!("Building uboot spl");
        run_command("./make.sh spl", true).expect("Failed to make.sh spl");
        info!("Running uboot trust, again");
        run_command("./make.sh trust", true).expect("Failed to make.sh trust (second time)");
        
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
