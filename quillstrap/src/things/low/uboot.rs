use crate::{
    common::commands::{run_command, run_shell_command},
    prelude::*,
};

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

        info!("Building U-boot defconfig");
        run_command(
            "make rk3566-pinenote_defconfig",
            options.config.command_output,
        )
        .expect("Failed to make rk3566-pinenote_defconfig");

        info!("Building U-boot rk3566-pinenote");
        run_command("./make.sh rk3566-pinenote", options.config.command_output)
            .expect("Failed to make.sh rk3566-pinenote");
        info!("Running U-boot trust");
        run_command("./make.sh trust", options.config.command_output)
            .expect("Failed to make.sh trust");

        info!("Building U-boot spl");
        run_command("./make.sh spl", options.config.command_output).expect("Failed to make.sh spl");
        info!("Running U-boot trust, again");
        run_command("./make.sh trust", options.config.command_output)
            .expect("Failed to make.sh trust (second time)");

        info!("Running build of logotool_mod");
        let cur_dir = dir_current();
        mkdir_p("pinenote_ui/logotool_mod/build");
        dir_change("pinenote_ui/logotool_mod/build");
        run_command("cmake ..", options.config.command_output).expect("logotool_mod cmake failed");
        run_command("cmake --build .", options.config.command_output)
            .expect("logotool_mod cmake failed");
        dir_change(cur_dir.to_str().unwrap());

        info!("Running build of boot menu");
        run_shell_command(
            "pushd pinenote_ui/; ./build_all.sh; popd",
            options.config.command_output,
        )
        .expect("Failed to build U-boot boot menu");

        // Clean the var
        set_var("CROSS_COMPILE", "");
        let uboot_img_file = "U-boot.img";
        if !path_exists(&uboot_img_file) {
            let err = format!(
                "Failed to build U-boot, everything seems fine but output file {} is missing",
                &uboot_img_file
            );
            error!("{}", err);
            return Err(err.to_string());
        } else {
            info!("Built U-Boot succesfully!");
            return Ok(());
        }
    }

    fn deploy(&self, options: &Options) -> std::result::Result<(), String> {
        let port = choose_serial_port();
        info!("Serial port choosed: {}", port);
        let message = format!(
            "Make sure U-boot serial cli is running. First reboot, then click the next button in the boot menu, then use a command like \"tio -b 1500000 {}\" to enter the uboot cli.",
            port
        );
        show_wait_toast(&message);

        // Hehe
        let _ = run_command("killall -9 tio", false);
        for _ in 0..5 {
            send_serial_ascii(port.clone(), 0x03);
            sleep_millis(50);
        }

        for _ in 0..5 {
            send_serial_message(port.clone(), "\n\r");
            sleep_millis(50);
        }
        sleep_millis(100);
        send_serial_message(port.clone(), "rockusb 0 mmc 0\n\r");
        sleep_millis(1000);
        let mut serial_buf: Vec<u8> = vec![0; 200];
        read_serial(port.clone(), &mut serial_buf);
        sleep_millis(500);
        read_serial(port.clone(), &mut serial_buf);

        let ascii_chars: Vec<char> = serial_buf.iter().map(|&b| b as char).collect();
        debug!("Serial buf received: {:?}", ascii_chars);

        if ascii_chars.contains(&'/') && ascii_chars.contains(&'\\') && ascii_chars.contains(&'|') {
            info!("Detected that rockusb is on!");
        } else {
            show_wait_toast(
                "Failed to detect rockusb mode. Try to trigger it manually by typing \"rockusb 0 mmc 0\" in the serial monitor",
            );
        }
        show_wait_toast("Ok, now disconnect the usb dongle and connect directly to the pinenote");
        info!("Ok, Putting pinenote into download mode");
        run_command(
            "rkdeveloptool reboot-maskrom",
            options.config.command_output,
        )
        .expect("Failed to run: rkdeveloptool reboot-maskrom");

        /*
        info!("Running rkdeveloptool boot rk356x_spl_loader_v1.20.114.bin");
        run_command(
            "rkdeveloptool boot rk356x_spl_loader_v1.20.114.bin",
            options.config.command_output,
        )
        .expect("Failed to write uboot");
        */

        info!("Running rkdeveloptool write-partition uboot uboot.img");
        run_command(
            "rkdeveloptool write-partition uboot uboot.img",
            options.config.command_output,
        )
        .expect("Failed to write uboot");

        info!("Running rkdeveloptool write-partition logo pinenote_ui/logo_new.img");
        run_command(
            "rkdeveloptool write-partition logo pinenote_ui/logo_new.img",
            options.config.command_output,
        )
        .expect("Failed to write logo partition");

        Ok(())
    }

    fn run(&self) -> std::result::Result<(), String> {
        info!("Rebooting...");

        info!("Running rkdeveloptool reboot");
        run_command("rkdeveloptool reboot", true).expect("Failed to write logo partition");

        Ok(())
    }
}
