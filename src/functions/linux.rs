use crate::internal::*;

// Should run an hardened version of linux with
// some extra secure conf

// mkinitcpio.conf
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
    files::create_file("/mnt/etc/systemd/zram-generator.conf");
    files_eval(
        files::append_file("/mnt/etc/systemd/zram-generator.conf", "[zram0]"),
        "Write zram-generator config",
    );
}

pub fn create_kernel_cmdline() {
    // Something like rd.luks.name=<UUID>=root root=/dev/mapper/root rootflags=subvol=@ rw zswap.enabled=0
}
pub fn create_preset() {
    //
}
