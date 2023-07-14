use crate::internal::*;
use std::str;
use std::process::Output;

pub fn exec_eval<D>(return_code: Result<Output, std::io::Error>, logmsg: D) -> Output
where
    D: std::fmt::Display,
{
    match return_code {
        Ok(output) => {
            if output.status.success() {
                log::info!("{}", logmsg);
                output
            } else {
                match str::from_utf8(&output.stderr) {
                    Ok(e) => crash(format!("{}  ERROR: {}", logmsg, e), 1),
                    Err(_) => crash(format!("{}  ERROR: unreadable output", logmsg), 1)
                }
            }
        }
        Err(e) => {
            crash(
                format!("{}  ERROR: {}", logmsg, e),
                e.raw_os_error().unwrap(),
            );
        }
    }
}

pub fn files_eval<T>(return_code: Result<(), std::io::Error>, logmsg: T)
where
    T: std::fmt::Display,
{
    match &return_code {
        Ok(_) => {
            log::info!("{}", logmsg);
        }
        Err(e) => {
            crash(
                format!("{} ERROR: {}", logmsg, e),
                e.raw_os_error().unwrap(),
            );
        }
    }
}

pub fn os_eval<T, D>(return_code: Result<T, std::io::Error>, logmsg: D)
where
    D: std::fmt::Display,
{
    match &return_code {
        Ok(_) => {
            log::info!("{}", logmsg);
        }
        Err(e) => {
            crash(
                format!("{} ERROR: {}", logmsg, e),
                e.raw_os_error().unwrap(),
            );
        }
    }
}
