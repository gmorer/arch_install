use std::fs;
use std::process::Command;
use std::io::{ self, BufReader, BufRead };
use regex::Regex;

use crate::internal::exec::*;
use crate::internal::files::append_file;
use crate::internal::*;

enum CpuVendor {
    Amd,
    Intel,
}

impl CpuVendor {
    pub fn get() -> io::Result<Option<Self>>{
        let amd_regex = Regex::new(r"^vendor_id\s*: AuthenticAMD$").unwrap();
        let intel_regex = Regex::new(r"^vendor_id\s*: GenuineIntel$").unwrap();
        let file = fs::File::open("/proc/cpuinfo")?;
        let reader = BufReader::new(file).lines();
        for line in reader {
            if let Ok(line) = line {
                if amd_regex.is_match(&line) {
                    return Ok(Some(Self::Amd))
                } else if intel_regex.is_match(&line) {
                    return Ok(Some(Self::Intel))
                }
            }
        }
        Ok(None)
    }
    pub fn get_microde(&self) -> &'static str {
        match self {
            Self::Amd => "amd_ucode",
            Self::Intel => "intel_ucode",
        }
    }
}

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
    fs::create_dir_all("/mnt/etc").unwrap();
    let kernel_to_install = "linux-hardened";
    let headers = format!("{}-headers", kernel_to_install);
    let mut to_install = vec![
        // Base Arch
        "base",
        "base-devel",
        "git",
        kernel_to_install,
        &headers,
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
    ];
    if let Some(microcode) = CpuVendor::get().ok().flatten() {
        to_install.push(microcode.get_microde());
    }
    install(to_install);
}

pub fn install_bootloader() {
    // It take /efi has default
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
