use crate::args::PartitionMode;
use crate::internal::exec::*;
use crate::internal::*;
use std::path::{Path, PathBuf};

use files::create_directory as mkdir;

// TODO: encryption :D

pub fn partition(device: PathBuf, mode: PartitionMode) {
    println!("{:?}", mode);
    match mode {
        PartitionMode::Auto => {
            if !device.exists() {
                crash(format!("The device {device:?} doesn't exist"), 1);
            }
            log::debug!("automatically partitioning {device:?}");
            partition_with_efi(&device);
            part(&device);
        }
        PartitionMode::Manual => {
            log::debug!("Manual partitioning");
            println!("Just do it manually then...")
        }
    }
}

fn partition_with_efi(device: &Path) {
    let device = device.to_string_lossy().to_string();
    os_eval(
        exec("parted", vec!["-s", &device, "mklabel", "gpt"]),
        format!("create gpt label on {}", &device).as_str(),
    );
    // TODO: encryption
    // cryptsetup lukSformat
    // cryptsetup open
    os_eval(
        exec(
            "parted",
            vec!["-s", &device, "mkpart", "fat32", "0", "1GiB"],
        ),
        "create EFI partition",
    );
    os_eval(
        exec(
            "parted",
            vec!["-s", &device, "mkpart", "primary", "btrfs", "1GiB", "100%"],
        ),
        "create btrfs root partition",
    );
}

fn mount_btrfs_su(subvol: &str, device: &str, path: &str) {
    let btrfs_mount_opt = "rw,noatime,compress-force=zstd:3";
    let options = format!("{},subvol={}", btrfs_mount_opt, subvol);
    exec_eval(
        exec("btrfs", vec!["-t", "btrfs", "-o", &options, device, path]),
        &format!(
            "Create btrfs subvolume {} from {} to {}",
            subvol, device, path
        ),
    );
}

fn create_btrfs_su(mountpoint: &str) {
    os_eval(
        exec("btrfs", vec!["subvolume", "create", mountpoint]),
        &format!("Create btrfs subvolume {}", mountpoint),
    );
}

fn part(device: &Path) {
	let device = device.to_string_lossy().to_string();
    let boot = format!("{}p1", device);
    let btrfs = format!("{}p2", device);
    // Boot partition
    os_eval(
        exec("mkfs.vfat", vec!["-F32", &boot]),
        &format!("format {} as fat32", boot),
    );
    // /
    os_eval(
        exec("mkfs.btrfs", vec!["-f", &btrfs]),
        &format!("format {} as btrfs", btrfs),
    );

    let subvs = vec![
        ("@", "/"),
        ("@home", "/home"),
        ("@snapshots", "/.snapshots"),
        ("@var_log", "/var/log"),
        ("@var_pkgs", "/var/cache/pacman/pkgs"),
        ("@swap", "/swap"),
    ];

    mount(&btrfs, "/mnt", None);
    for (label, _) in &subvs {
        create_btrfs_su(&format!("/mnt/{}", label));
    }

    umount("/mnt");

    mount_btrfs_su("@", &btrfs, "/mnt");
    for (label, path) in subvs.iter().skip(1) {
        let mounted_path = format!("/mnt{}", path);
        os_eval(mkdir(&mounted_path), &format!("create {}", mounted_path));
        mount_btrfs_su(label, &btrfs, &mounted_path);
    }

    // Since 6.1 :D
    let swapfile = "/mnt/swap/swapfile";
    os_eval(
        exec(
            "btrfs",
            vec!["filesystem", "mkswapfile", "--size", "8G", swapfile],
        ),
        &format!("Creating the swap file {}", swapfile),
    );
    os_eval(
        exec("swapon", vec![swapfile]),
        &format!("swapon {}", swapfile),
    );

    os_eval(mkdir("/mnt/boot/efi"), "create /mnt/boot/efi");
    mount(&boot, "/mnt/boot/efi", None);
}

pub fn mount(partition: &str, mountpoint: &str, options: Option<&str>) {
    let args = match options {
        Some(options) => vec![partition, mountpoint, "-o", options],
        None => vec![partition, mountpoint],
    };
    os_eval(
        exec("mount", args),
        &format!("mount {} with no options at {}", partition, mountpoint),
    );
}

pub fn umount(mountpoint: &str) {
    os_eval(
        exec("umount", vec!["-R", mountpoint]),
        &format!("unmount {}", mountpoint),
    );
}
