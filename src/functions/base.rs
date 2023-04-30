use std::fs;
use std::process::Command;
use crate::internal::exec::*;
use crate::internal::files::append_file;
use crate::internal::*;

fn install(pkgs: Vec<&str>) {
    os_eval(
        Command::new("pacstrap").arg("/mnt").arg("-K").args(&pkgs).status(),
        format!("Install packages {}", pkgs.join(", ")).as_str(),
    );
    // umount("/mnt/dev");
}

pub fn install_aur() {
	os_eval(
	exe_chroot!(
		"bash",
		"-c",
		"git clone https://aur.archlinux.org/paru.git /tmp/paru && cd /tmp/paru && makepkg -si --noconfirm",
	),
	"Installing paru to access aur",
	);
}

pub fn config_pacman() {
    os_eval(
        append_file("/etc/pacman.conf", "ParallelDownloads = 5"),
        "Setting pacman parralel download",
    );
    // Ranking the mirrors
    os_eval(
        fs::copy(
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

pub fn copy_pacman_conf() {
	os_eval(
		fs::copy(
			"/etc/pacman.conf",
			"/mnt/etc/pacman.conf"
		).and_then(|_| fs::copy(
				"/etc/pacman.d/mirrorlist",
				"/mnt/etc/pacman/mirrorlist"
		)),
		"Copying pacman conf"
	);

}

pub fn install_base_packages() {
    // TODO: microcode
    fs::create_dir_all("/mnt/etc").unwrap();
    let kernel_to_install = "linux-hardened";
    install(vec![
        // Base Arch
        "base",
        "base-devel",
        "git",
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
	os_eval(
		exe_chroot!("bootctl", "install"),
		"Installing the bootloader",
	);
	let system_d_hook = include_str!("../../resources/etc_pacman_d_hooks_95-systemd-boot.hook");
	let hook_path = "/mnt/etc/pacman.d/hooks/95-systemd-boot.hook";
	os_eval(
		fs::write(hook_path, system_d_hook),
		"Creating hook to update the bootloader"
	);
}
