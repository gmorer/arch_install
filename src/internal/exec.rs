use std::process::{Command, Stdio};

macro_rules! exe {
	($e:expr, $($argv:tt)*) => {{
		let res = crate::internal::exec::exec($e, vec![$($argv)*]);
		res
	}};
	($e:tt) => {{
		let res = crate::internal::exec::exec($e, std::vec::Vec::<&str>::new());
		res
	}};
}
macro_rules! exe_io {
	($e:expr, $($argv:tt)*) => {{
		let res = crate::internal::exec::execio($e, vec![$($argv)*]);
		res
	}};
	($e:tt) => {{
		let res = crate::internal::exec::execio($e, std::vec::Vec::<&str>::new());
		res
	}};
}

macro_rules! exe_chroot {
	($e:expr, $($argv:tt)*) => {{
		let res = crate::internal::exec::exec_chroot($e, vec![$($argv)*]);
		res
	}};
	($e:tt) => {{
		let res = crate::internal::exec::exec_chroot($e, std::vec::Vec::<String>::new());
		res
	}};
}
pub(crate) use exe;
pub(crate) use exe_chroot;
pub(crate) use exe_io;

pub fn exec<T>(command: &str, args: Vec<T>) -> Result<std::process::ExitStatus, std::io::Error>
where
    T: AsRef<std::ffi::OsStr>,
{
    Command::new(command).args(args).status()
}

pub fn execio<T>(command: &str, args: Vec<T>) -> Result<std::process::ExitStatus, std::io::Error>
where
    T: AsRef<std::ffi::OsStr>,
{
    let returncode = Command::new(command)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .args(args)
        .status();
    returncode
}

pub fn exec_chroot<T>(
    command: &str,
    args: Vec<T>,
) -> Result<std::process::ExitStatus, std::io::Error>
where
    T: std::string::ToString,
{
    let args = args
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>();
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
