use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct Backup {}

impl SetupThing for Backup {
    fn name(&self) -> &'static str {
        "backup"
    }

    fn path(&self) -> &'static str {
        "low/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec!["uboot"]
    }

    fn git(&self) -> &'static str {
        warn!("No git for backup, obviously");
        ""
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        mkdir_p("backup");
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
        warn!("We assume because of uboot deploy, we are in rkdeveloptool state");

        run_command("rkdeveloptool read-partition uboot uboot.bin", true)
            .expect("Failed to read partition");
        run_command("rkdeveloptool read-partition waveform waveform.bin", true)
            .expect("Failed to read partition");
        run_command("rkdeveloptool read-partition uboot_env uboot_env.bin", true)
            .expect("Failed to read partition");
        run_command("rkdeveloptool read-partition logo logo.bin", true)
            .expect("Failed to read partition");

        let partitions = vec!["uboot", "waveform", "uboot_env", "logo"];
        let mut wrong = false;
        for part in partitions {
            run_command(&format!("rkdeveloptool read-partition {} {}.bin", part, part), true)
                .expect(&format!("Failed to read partition: {}", part));

            if !path_exists(&format!("{}.bin", part)) {
                error!("File doesn't exist: {}.bin - we have a problem probably, the backup is bad!", part);
                wrong = true;
            }
        }

        if wrong {
            return Err(String::from("Failed to take backup!"));
        } else {
            info!("Backup taken succesfully!");
        }

        Ok(())
    }
}
