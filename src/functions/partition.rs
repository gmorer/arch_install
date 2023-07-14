use std::path::PathBuf;
use std::str;
use regex::Regex;

use files::create_directory as mkdir;
use crate::internal::exec::{exe, exe_io, exec};
use crate::internal::*;

// Disk:
// GPT: [ efi: fat32 1GiB ] [ LUKS2 1Gib..100% ]
// Luks2: [ Btrfs ]
// Btrfs: [ [ @ ] [ @home ][ @snapshots ][ @var_log ][ @var_pkgs ][ @swap ] [@boot] ]
//
// TODO: automount via systemctl

fn set_default_btrfs() {
    let default_vol_regex = Regex::new("^ID .* path @$").unwrap();

    let output = exec_eval(exe!("btrfs", "subvolume", "list", "/mnt"), "Getting the list of btrfs subvolume");
    let output = match str::from_utf8(&output.stdout) {
        Ok(out) => out,
        Err(_) => crash("Fail to parse output of the btrfs command", 1)
    };

    /*
     * Example output:
     * ID 256 gen 85155 top level 5 path @
     * ID 257 gen 85155 top level 5 path @home
     * ID 258 gen 85155 top level 5 path @var_lo
     */

    for line in output.lines() {
        if default_vol_regex.is_match(line) {
            match line.split(" ").nth(1) {
                Some(id) => {
                    os_eval(
                        exe!("btrfs", "subvolume", "set-default", id, "/"),
                        "Setting default btrfs subvolume"
                    );
                },
                None => crash("Fail to get btrfs default subvolum id", 1),
            }
        }
    }

}

pub fn partition(device: String) {
    if !PathBuf::from(&device).exists() {
        crash(format!("The device {device:?} doesn't exist"), 1);
    }
    log::debug!("automatically partitioning {device:?}");
    let (esp, cryptroot) = create_table(device);
    let root = create_luks(&cryptroot);
    part(&esp, root);
}

fn create_table(device: String) -> (String /* esp */, String /* cryptroot */) {
    let esp_label = "ESP";
    let cryptroot_label = "CRYPTROOT";
    os_eval(
        exe!(
            "parted",
            "-s",
            &device,
            /* Create GPT */
            "mklabel",
            "gpt",
            /* Create efi partition */
            "mkpart",
            esp_label,
            "fat32",
            "0",
            "1GiB",
            /* set esp=on flag on partition 1 */
            "set",
            "1",
            "esp",
            "on",
            /* create luks partition */
            "mkpart",
            cryptroot_label,
            "1GiB",
            "100%"
        ),
        format!("Creating the GPT table"),
    );
    os_eval(
        exe!("partprobe", device),
        "Informing the kernel about disk changes",
    );
    (
        format!("/dev/disk/by-partlabel/{}", esp_label),
        format!("/dev/disk/by-partlabel/{}", cryptroot_label),
    )
}

fn create_luks(cryptroot: &str) -> String {
    let container_name = "root";
    println!("Enter password to create the encrypted container:");
    os_eval(
        // Using default options, should be ok
        exe_io!("cryptsetup", "luksFormat", "--type", "luks2", "--integrity", "hmac-sha256", cryptroot),
        "Creating the luks2 container for root",
    );
    println!("Enter password to enter the encrypted container (same as before):");
    os_eval(
        exe_io!("cryptsetup", "open", cryptroot, container_name),
        "Opening the luks2 container for root",
    );
    format!("/dev/mapper/{}", container_name)
}

fn mount_btrfs_su(subvol: &str, device: &str, path: &str) {
    let btrfs_mount_opt = "rw,noatime,compress-force=zstd:3";
    let options = format!("{},subvol={}", btrfs_mount_opt, subvol);
    exec_eval(
        exe!("btrfs", "-t", "btrfs", "-o", &options, device, path),
        format!(
            "Create btrfs subvolume {} from {} to {}",
            subvol, device, path
        ),
    );
}

fn create_btrfs_su(mountpoint: &str) {
    os_eval(
        exe!("btrfs", "subvolume", "create", mountpoint),
        format!("Create btrfs subvolume {}", mountpoint),
    );
}

fn part(esp: &str, btrfs: String) {
    // Efi partition
    os_eval(
        exe!("mkfs.vfat", "-F32", esp),
        format!("format {} as fat32", esp),
    );
    // /
    os_eval(
        exe!("mkfs.btrfs", "-f", &btrfs),
        format!("format {} as btrfs", btrfs),
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
        os_eval(mkdir(&mounted_path), format!("create {}", mounted_path));
        mount_btrfs_su(label, &btrfs, &mounted_path);
    }

    set_default_btrfs();

    let swapfile = "/mnt/swap/swapfile";
    os_eval(
        exe!(
            "btrfs",
            "filesystem",
            "mkswapfile",
            "--size",
            "8G",
            swapfile
        ),
        format!("Creating the swap file {}", swapfile),
    );
    os_eval(exe!("swapon", swapfile), format!("swapon {}", swapfile));

    os_eval(mkdir("/mnt/boot"), "create /mnt/boot");
    os_eval(mkdir("/mnt/efi"), "create /mnt/efi");
    mount(esp, "/mnt/efi", None);
}

pub fn mount(partition: &str, mountpoint: &str, options: Option<&str>) {
    let args = match options {
        Some(options) => vec![partition, mountpoint, "-o", options],
        None => vec![partition, mountpoint],
    };
    os_eval(
        exec("mount", args),
        format!("mount {} with no options at {}", partition, mountpoint),
    );
}

pub fn umount(mountpoint: &str) {
    os_eval(
        exe!("umount", "-R", mountpoint),
        format!("unmount {}", mountpoint),
    );
}
