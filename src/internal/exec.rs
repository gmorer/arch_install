use std::process::Command;

pub fn exec<T>(command: &str, args: Vec<T>) -> Result<std::process::ExitStatus, std::io::Error>
where
    T: AsRef<std::ffi::OsStr>,
{
    let returncode = Command::new(command).args(args).status();
    returncode
}

pub fn exec_chroot(
    command: &str,
    args: Vec<String>,
) -> Result<std::process::ExitStatus, std::io::Error> {
    let returncode = Command::new("bash")
        .args([
            "-c",
            format!("arch-chroot /mnt {} {}", command, args.join(" ")).as_str(),
        ])
        .status();
    returncode
}

pub fn exec_workdir<T>(
    command: &str,
    workdir: &str,
    args: Vec<T>,
) -> Result<std::process::ExitStatus, std::io::Error>
where
    T: AsRef<std::ffi::OsStr>,
{
    let returncode = Command::new(command)
        .args(args)
        .current_dir(workdir)
        .status();
    returncode
}
