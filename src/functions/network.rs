use crate::internal::*;
use crate::exec::exe_chroot;

pub fn set_hostname() {
    let hostname = "laptop";
    os_eval(
        exe_chroot!("hostnamectl", "set-hostname" , hostname),
        "Setting hostname"
    );
}

pub fn create_hosts() {
    files::create_file("/mnt/etc/hosts");
    files_eval(
        files::append_file("/mnt/etc/hosts", "127.0.0.1     localhost"),
        "create /etc/hosts",
    );
}

pub fn enable_ipv6() {
    files_eval(
        files::append_file("/mnt/etc/hosts", "::1 localhost"),
        "add ipv6 localhost",
    );
}
