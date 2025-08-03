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

        info!("Building U-Boot (defconfig)");
        run_command("make rk3566-pinenote_defconfig", true)
            .expect("Failed to make rk3566-pinenote_defconfig");

        info!("Building U-Boot (rk3566-pinenote)");
        run_command("./make.sh rk3566-pinenote", true).expect("Failed to make.sh rk3566-pinenote");
        info!("Building U-Boot (trust)");
        run_command("./make.sh trust", true).expect("Failed to make.sh trust");

        info!("Building U-Boot (spl)");
        run_command("./make.sh spl", true).expect("Failed to make.sh spl");
        info!("Building U-Boot (trust, again)");
        run_command("./make.sh trust", true).expect("Failed to make.sh trust (second time)");
        
        // Clean the var
        set_var("CROSS_COMPILE", "");
        let uboot_img_file = "uboot.img";
        if !path_exists(&uboot_img_file) {
            let err =
                format!("Failed to build U-boot, everything seems fine but output file {} is missing", &uboot_img_file);
            error!("{}", err);
            return Err(err.to_string());
        } else {
            info!("Built U-Boot succesfully!");
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
