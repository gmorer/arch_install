use crate::internal::exec::*;
use crate::internal::*;

pub fn set_timezone() {
    //TODO: -> curl -s http://ip-api.com/line?fields=timezone
    let timezone = "Europe/Paris";
    os_eval(
        exe_chroot!(
            "ln",
            "-sf",
            &format!("/usr/share/zoneinfo/{}", timezone),
            "/etc/localtime",
        ),
        "Set timezone",
    );
    exec_eval(exe_chroot!("hwclock", "--systohc"), "Set system clock");
}

pub fn set_locale() {
    // TODO
    let locale = "en_US.UTF-8 UTF-8";
    files_eval(
        files::append_file("/mnt/etc/locale.gen", "en_US.UTF-8 UTF-8"),
        "add en_US.UTF-8 UTF-8 to locale.gen",
    );
    files::create_file("/mnt/etc/locale.conf");
    files_eval(
        files::append_file("/mnt/etc/locale.conf", "LANG=en_US.UTF-8"),
        "edit locale.conf",
    );
    for i in (0..locale.split(' ').count()).step_by(2) {
        files_eval(
            files::append_file(
                "/mnt/etc/locale.gen",
                &format!(
                    "{} {}\n",
                    locale.split(' ').collect::<Vec<&str>>()[i],
                    locale.split(' ').collect::<Vec<&str>>()[i + 1]
                ),
            ),
            "add locales to locale.gen",
        );
        if locale.split(' ').collect::<Vec<&str>>()[i] != "en_US.UTF-8" {
            files_eval(
                files::sed_file(
                    "/mnt/etc/locale.conf",
                    "en_US.UTF-8",
                    locale.split(' ').collect::<Vec<&str>>()[i],
                ),
                format!(
                    "Set locale {} in /etc/locale.conf",
                    locale.split(' ').collect::<Vec<&str>>()[i]
                )
                .as_str(),
            );
        }
    }
    exec_eval(exe_chroot!("locale-gen"), "generate locales");
}

pub fn set_keyboard() {
    files::create_file("/mnt/etc/vconsole.conf");
    files_eval(
        files::append_file("/mnt/etc/vconsole.conf", "KEYMAP=us"),
        "set keyboard layout to us",
    );
}
