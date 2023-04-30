use std::fs;
use crate::internal::*;

// Should run an hardened version of linux with
// some extra secure conf

// mkinitcpio.conf
// remove base and udev, because systemd add vconsole and /etc/vconsole.conf
// HOOKS=(base systemd udev autodetect keyboard modconf block sd-encrypt filesystems keyboard fsck)
//
// kernel.unprivileged_bpf_disabled=1
//
//
// add btrfs to mkinitcpio binaries
//
// fix btrfs issue with async discard: /etc/tmpfiles.d/btrfs-discard.conf
// w /sys/fs/btrfs/uuid/discard/iops_limit - - - - 1000

//
pub fn install_zram() {
    // TODO: disable swap -> zswap.enabled=0 kernel param
	// default priority of zram-generator is 100 while default prio of swapon is -1(aka max)
	let conf = include_str!("../../resources/etc_systemd_zram-generator.conf");
    files_eval(
		fs::write(conf, "/mnt/etc/systemd/zram-generator.conf"),
        "Write zram-generator config",
    );
}

pub fn config_kernel_install() {
	// man kernel_isntall(8)

}

pub fn config_mkinitcpio() {
	// TODO: mkinitcpio HOOKS conf
	// TODO: /etc/kernel/cmdline
	// TODO: /etc/mnkinitcpio.d/linux.preset
}

pub fn config_secure_boot() {
	// sbctl(8)
	// sbctl create-keys
	// sbctl enroll-keys
	
	// sign bootlaoder
}
