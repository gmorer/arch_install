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
    install(vec![
        // Base Arch
        "base",
        kernel_to_install,
        &format!("{}-headers", kernel_to_install),
        "archlinux-keyring",
        "zram-generator",
        "linux-firmware",
        "systemd-sysvcompat",
        "bubblewrap-suid",
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
    // Custom hook to update bootloader
    exe_chroot!("bootctl", "--path=/boot", "install");
}

pub fn install_paru() {
    /*
        git clone https://aur.archlinux.org/packages/paru/
    cd paru
    makepkg -si PKGBUILD
    */
}
