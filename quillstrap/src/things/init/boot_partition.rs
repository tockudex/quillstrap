use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct BootPartition;

impl SetupThing for BootPartition {
    fn name(&self) -> &'static str {
        "boot_partition"
    }

    fn path(&self) -> &'static str {
        "init/"
    }

    fn deps(&self) -> Vec<&'static str> {
        // Initrd, firmware
        vec!["expose_mmc", "partition_setup"]
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
        Ok(())
    }

    fn deploy(&self, options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        warn!("We assume because of expose_mmc deploy, the mmc is exposed as a block device");

        let _disk = choose_disk();

        let partition = get_partition("quill_boot");
        mkdir_p("/mnt/quill_boot");
        run_command(&format!("mount {} /mnt/quill_boot", partition), options.config.command_output).unwrap();
        run_command("sync", false).unwrap();
        copy_file("../kernel/out/Image.gz", "/mnt/quill_boot/Image.gz").unwrap();
        copy_file("../kernel/out/DTB", "/mnt/quill_boot/DTB").unwrap();

        copy_file("../firmware/wifi_bt/firmware.squashfs", "/mnt/quill_boot/firmware.squashfs").unwrap();
        sign("/mnt/quill_boot/firmware.squashfs", "/mnt/quill_boot/firmware.squashfs.dgst", options);

        run_command("sync", false).unwrap();
        run_command(&format!("umount {}", partition), options.config.command_output).unwrap();

        info!("Done, in theory, reboot the device now manually via the power button");
        Ok(())
    }

    fn run(&self) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }
}
