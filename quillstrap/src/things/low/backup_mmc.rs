use std::path;

use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct BackupMmc {}

impl SetupThing for BackupMmc {
    fn name(&self) -> &'static str {
        "backup_mmc"
    }

    fn path(&self) -> &'static str {
        "low/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec!["expose_mmc"]
    }

    fn git(&self) -> &'static str {
        warn!("No git for backup, obviously");
        ""
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        mkdir_p("backup_mmc");
        Ok(())
    }

    fn clean(&self) -> color_eyre::eyre::Result<(), String> {
        warn!("No clean for backup, obviously");
        Ok(())
    }

    fn build(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        warn!("No build for backup, obviously");
        Ok(())
    }

    fn deploy(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        warn!("No deploy for backup, obviously");
        Ok(())
    }

    fn run(&self) -> color_eyre::eyre::Result<(), String> {
        warn!("We assume because of expose_mmc deploy, the mmc is exposed as a block device");

        let disk = choose_disk();
        info!("Choosed disk: {}", disk);

        warn!("Removing old backup");
        remove_file("pinenote_disk.qcow2".to_string()).ok();

        info!("Taking the backup...");
        run_command(&format!("qemu-img convert -p -O qcow2 -c {} pinenote_disk.qcow2", disk), true).unwrap();

        if path_exists("pinenote_disk.qcow2") {
        info!("Done! Hopefully it worked");
                Ok(())

        } else {
            let str = "Backup wasn't taken, that's bad";
            error!("{}", str);
            Err(str.to_string())
        }
    }
}