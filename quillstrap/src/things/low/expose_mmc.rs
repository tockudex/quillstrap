use std::fs::remove_dir_all;

use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct ExposeMmc {}

impl SetupThing for ExposeMmc {
    fn name(&self) -> &'static str {
        "expose_mmc"
    }

    fn path(&self) -> &'static str {
        "low/"
    }

    fn deps(&self) -> Vec<&'static str> {
        Vec::new()
    }

    fn git(&self) -> &'static str {
        todo!()
    }

    fn get(&self, options: &Options) -> std::result::Result<(), String> {
        remove_dir_all(self.name()).ok();
        mkdir_p(self.name());
        dir_change(self.name());
        run_command(
            "wget https://github.com/PorQ-Pine/initrd/releases/download/1/Image.gz",
            options.config.command_output,
        )
        .unwrap();
        run_command(
            "wget https://github.com/PorQ-Pine/initrd/releases/download/1/dtb",
            options.config.command_output,
        )
        .unwrap();
        dir_change("../");
        Ok(())
    }

    fn clean(&self) -> std::result::Result<(), String> {
        Ok(())
    }

    fn build(&self, _options: &Options) -> std::result::Result<(), String> {
        Ok(())
    }

    fn deploy(&self, options: &Options) -> std::result::Result<(), String> {
        let (port, status) = enter_uboot_cli().unwrap();

        // Kernel
        let output = send_read_serial(port.clone(), "loady ${kernel_addr_r}");
        if output.contains("Ready for binary (ymodem) download") {
            info!("Loading kernel now!");
            run_shell_command(
                &format!("sz -vv -Y Image.gz --serial {}", port.clone()),
                options.config.command_output,
            )
            .unwrap();
        } else {
            error!("kernel loady failed, this is bad");
        }
        sleep_millis(1000);
        clear_uboot_cli(port.clone());

        // DTB
        let output = send_read_serial(port.clone(), "loady ${fdt_addr_r}");
        if output.contains("Ready for binary (ymodem) download") {
            info!("Loading dtb now!");
            run_shell_command(
                &format!("sz -vv -Y dtb --serial {}", port.clone()),
                options.config.command_output,
            )
            .unwrap();
        } else {
            error!("dtb loady failed, this is bad");
        }
        sleep_millis(1000);
        clear_uboot_cli(port.clone());

        // Unzip
        send_read_serial(port.clone(), "unzip ${kernel_addr_c} ${kernel_addr_r}");
        sleep_millis(1000);
        clear_uboot_cli(port.clone());

        // Boot
        let str = send_read_serial(port.clone(), "booti ${kernel_addr_r} - ${fdt_addr_r}");
        sleep_millis(1000);
        if str.contains("Bad Linux ARM64 Image magic!") {
            error!("Failed to boot expose eemc kernel, bad");
        }

        show_wait_toast(
            "Maybe it worked, maybe not, unplug serial, plug in normally, there should be another block device",
        );
        Ok(())
    }

    fn run(&self) -> std::result::Result<(), String> {
        Ok(())
    }
}
