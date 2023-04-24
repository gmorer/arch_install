use crate::internal::exec::*;
use crate::internal::files::append_file;
use crate::internal::*;

pub fn config_pacman() {
    // TODO: paru
    os_eval(
        append_file("/etc/pacman.conf", "ParallelDownloads = 5"),
        "Setting pacman parralel download",
    );
    // Ranking the mirrors
    os_eval(
        std::fs::copy(
            "/etc/pacman.d/mirrorlist",
            "/etc/pacman.d/mirrorlist.backup",
        ),
        "Creating temporary mirrorlist",
    );
    os_eval(
        exe!(
            "sed",
            "-i",
            "'s/^#Server/Server/'",
            "/etc/pacman.d/mirrorlist.backup"
        ),
        "Uncommenting every pacman mirrors",
    );
    os_eval(
        exe!(
            "bash",
            "-c",
            "rankmirrors -n 6 /etc/pacman.d/mirrorlist.backup > /etc/pacman.d/mirrorlist"
        ),
        "Ranking the mirrors",
    );
}

pub fn install_base_packages() {
    // TODO: microcode
    std::fs::create_dir_all("/mnt/etc").unwrap();
    let kernel_to_install = "linux-hardened";
    install::install(vec![
        // Base Arch
        "base",
        kernel_to_install,
        format!("{kernel_to_install}-headers").as_str(),
        "archlinux-keyring",
        "linux-firmware",
        "systemd-sysvcompat",
        "bootctl",
        "networkmanager",
        "man-db",
        "man-pages",
        "texinfo",
        "nano",
        "vim",
        "sudo",
        "curl",
        "btrfs-progs",
        "which",
        "base-devel",
        // Superior
        "zsh",
        "zsh-completions",
        "zsh-autosuggestions",
        "zsh-syntax-highlighting",
        // Common packages for all desktops
        "flatpak",
    ]);
    files::copy_file("/etc/pacman.conf", "/mnt/etc/pacman.conf");
}

pub fn genfstab() {
    os_eval(
        exe!("bash", "-c", "genfstab -U /mnt >> /mnt/etc/fstab",),
        "Generate fstab",
    );
}

pub fn install_bootloader() {
    exe_chroot!("bootctl", "install");
}

// TODO: move to linux.rs
pub fn install_zram() {
    // TODO: disable swap -> zswap.enabled=0 kernel param
    install(vec!["zram-generator"]);
    files::create_file("/mnt/etc/systemd/zram-generator.conf");
    files_eval(
        files::append_file("/mnt/etc/systemd/zram-generator.conf", "[zram0]"),
        "Write zram-generator config",
    );
}
