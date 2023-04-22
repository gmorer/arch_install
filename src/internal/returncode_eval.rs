use crate::internal::*;

pub fn exec_eval(return_code: Result<std::process::ExitStatus, std::io::Error>, logmsg: &str) {
    match &return_code {
        Ok(_) => {
            log::info!("{}", logmsg);
        }
        Err(e) => {
            crash(
                format!("{}  ERROR: {}", logmsg, e),
                e.raw_os_error().unwrap(),
            );
        }
    }
}

pub fn files_eval(return_code: Result<(), std::io::Error>, logmsg: &str) {
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

pub fn os_eval<T>(return_code: Result<T, std::io::Error>, logmsg: &str) {
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
