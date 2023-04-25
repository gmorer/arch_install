mod args;
mod functions;
mod internal;
mod logging;

use crate::functions::*;
pub use crate::internal::*;

fn main() {
    human_panic::setup_panic!();
    logging::init(1);

    // checks
    // partition
    partition::partition();
    // pacstrap
    base::config_pacman();
    base::install_base_packages();
    // fstab
    base::genfstab();
    linux::install_zram();
    // locals
    locale::set_locale();
    locale::set_keyboard();
    locale::set_timezone();
    // network
    network::set_hostname();
    network::create_hosts();
    network::enable_ipv6();
    // mkinitcpio
    // snapper
    // systemd-boot
    // root passwd
    // pacman settings
}
