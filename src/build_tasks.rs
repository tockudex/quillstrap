use std::fs;
use anyhow::{Context, Result};
use log::{info, warn, error};
use sys_mount::{unmount, Mount, MountFlags, UnmountFlags};

use crate::common;
use crate::signing;

use common::{run_command, download_file, bind_mount};

/* BEGIN U-BOOT */
pub fn u_boot(build_dir: &str) -> Result<()> {
    Ok(())
}
/* END U-BOOT */

/* BEGIN KERNEL */
pub fn kernel(build_dir: &str) -> Result<()> {
    Ok(())
}
/* END KERNEL */

/* BEGIN ROOTFS */
pub fn rootfs(build_dir: &str, private_key_path: &str) -> Result<()> {
    let rootfs_build_dir_path = format!("{}/{}", &build_dir, &common::ROOTFS_BUILD_DIR);
    let base_rootfs_archive_path = format!("{}/../{}/{}", &build_dir, &common::DOWNLOADS_DIR, &common::BASE_ROOTFS_FILE);
    let base_rootfs_path = format!("{}/{}/{}", &build_dir, &common::ROOTFS_BUILD_DIR, &common::ROOTFS_BASE_DIR);

    fs::create_dir_all(&rootfs_build_dir_path)?;
    fs::create_dir_all(&base_rootfs_path)?;

    // Download base
    if !fs::exists(&base_rootfs_archive_path)? {
        info!("Downloading root filesystem base");
        download_file(&common::BASE_ROOTFS_URL, &base_rootfs_archive_path)?;
    }
    // Extract it
    info!("Extracting root filesystem base");
    run_command("tar", &["-C", &base_rootfs_path, "-xvf", &base_rootfs_archive_path])?;
    // Set up the chroot
    rootfs_setup_chroot(&base_rootfs_path)?;
    // Installing packages/updating the chroot
    // Prevent crash in case this command errors out because xbps needs to be updated
    rootfs_manage_packages(&base_rootfs_path)?;
    rootfs_tear_down_chroot(&base_rootfs_path)?;
    rootfs_compress_and_sign(&rootfs_build_dir_path, &base_rootfs_path, &private_key_path)?;

    Ok(())
}

pub fn rootfs_setup_chroot(base_rootfs_path: &str) -> Result<()> {
    Mount::builder().fstype("proc").mount("proc", &format!("{}/proc", &base_rootfs_path)).with_context(|| "Failed to mount proc filesystem in chroot")?;
    Mount::builder().fstype("sysfs").mount("sysfs", &format!("{}/sys", &base_rootfs_path))?;
    Mount::builder().fstype("tmpfs").mount("tmpfs", &format!("{}/tmp", &base_rootfs_path))?;
    Mount::builder().fstype("tmpfs").mount("tmpfs", &format!("{}/run", &base_rootfs_path))?;
    Mount::builder().fstype("devtmpfs").mount("devtmpfs", &format!("{}/dev", &base_rootfs_path))?;
    let resolv_conf_path = format!("{}/etc/resolv.conf", &base_rootfs_path);
    info!("{}", &resolv_conf_path);
    fs::File::create(&resolv_conf_path)?;
    bind_mount("/etc/resolv.conf", &resolv_conf_path)?;

    Ok(())
}

pub fn rootfs_tear_down_chroot(base_rootfs_path: &str) -> Result<()> {
    unmount(format!("{}/proc", &base_rootfs_path), UnmountFlags::DETACH)?;
    unmount(format!("{}/sys", &base_rootfs_path), UnmountFlags::DETACH)?;
    unmount(format!("{}/tmp", &base_rootfs_path), UnmountFlags::DETACH)?;
    unmount(format!("{}/run", &base_rootfs_path), UnmountFlags::DETACH)?;
    unmount(format!("{}/dev", &base_rootfs_path), UnmountFlags::DETACH)?;
    unmount(format!("{}/etc/resolv.conf", &base_rootfs_path), UnmountFlags::DETACH)?;

    Ok(())
}

pub fn rootfs_run_chroot_command(base_rootfs_path: &str, command: &[&str]) -> Result<()> {
    info!("Running the following command in rootfs base chroot: '{}'", &command.join(" "));
    let mut args: Vec<&str> = Vec::with_capacity(1 + command.len());
    args.push(&base_rootfs_path);
    args.extend_from_slice(&command);

    run_command("chroot", &args)?;

    Ok(())
}

pub fn rootfs_manage_packages(base_rootfs_path: &str) -> Result<()> {
    let _ = rootfs_run_chroot_command(&base_rootfs_path, &["xbps-install", "-Syu"]);
    rootfs_run_chroot_command(&base_rootfs_path, &["xbps-install", "-uy", "xbps"])?;
    rootfs_run_chroot_command(&base_rootfs_path, &["xbps-install", "-Syu"])?;
    fs::remove_dir_all(&format!("{}/var/cache", &base_rootfs_path))?;

    Ok(())
}

pub fn rootfs_compress_and_sign(rootfs_build_dir_path: &str, base_rootfs_path: &str, private_key_path: &str) -> Result<()> {
    let archive_path = format!("{}/{}", &rootfs_build_dir_path, &common::ROOTFS_FILE);
    run_command("mksquashfs", &[&base_rootfs_path, &archive_path, "-b", "32768", "-comp", "zstd", "-Xcompression-level", "22", "-no-xattrs"])?;
    signing::sign_file(&private_key_path, &archive_path)?;

    Ok(())
}
/* END ROOTFS */
