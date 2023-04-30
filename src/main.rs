mod args;
mod functions;
mod internal;
mod logging;

use crate::functions::partition;
use crate::functions::base;
use crate::functions::locale;
use crate::functions::network;
use crate::functions::linux;

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
    base::copy_pacman_conf();
    base::install_aur();
    // fstab
    base::genfstab();
    base::install_bootloader();
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
    // root passwd
    // pacman settings
}
