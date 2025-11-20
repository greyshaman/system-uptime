use std::{error::Error, time::Duration};

/// Returns OS uptime in milliseconds
///
/// # Example
///
/// ```
/// use system_uptime::get_os_uptime;
///
/// match get_os_uptime() {
///     Ok(uptime_ms) => println!("OS uptime {} ms", uptime_ms),
///     Err(e) => eprintln!("Error is {}", e),
/// }
/// ```
#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn get_os_uptime() -> Result<u64, Box<dyn Error>> {
    use std::fs;

    let uptime_content = fs::read_to_string("/proc/uptime")?;
    let parts = uptime_content.split_whitespace().collect::<Vec<_>>();

    if parts.is_empty() {
        return Err("Invalid /proc/uptime format".into());
    }

    let uptime_seconds: f64 = parts[0].parse()?;
    Ok((uptime_seconds * 1000.0) as u64)
}
#[cfg(target_os = "windows")]
pub fn get_os_uptime() -> Result<u64, Box<dyn Error>> {
    use winapi::um::sysinfoapi::{GetTickCount64, GetLastError};

    unsafe {
        let uptime_ms = GetTickCount64();
        if uptime_ms == 0 {
            let error_code = GetLastError();
            if error_code != 0 {
                return Err(format!("Windows API error: {}", error_code).into());
            }
        }
        Ok(uptime_ms)
    }
}
#[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd"))]
pub fn get_os_uptime() -> Result<u64, Box<dyn Error>> {
    use libc::{sysctl, timeval};
    use std::mem;
    use std::io;

    let mut mib = [libc::CTL_KERN, libc::KERN_BOOTTIME];
    let mut boot_time = timeval { tv_sec: 0, tv_usec: 0 };
    let mut size = mem::size_of_val(&boot_time);

    unsafe {
        if sysctl(
            mib.as_mut_ptr(),
            2,
            &mut boot_time as *mut _ as *mut _,
            &mut size,
            std::ptr::null_mut(),
            0
        ) != 0
        {
            return Err(io::Error::last_os_error().into());
        }

        let now = libc::time(std::ptr::null_mut());
        let uptime_seconds = now - boot_time.tv_sec;
        Ok(uptime_seconds as u64 * 1000)
    }
}
#[cfg(not(any(
    target_os = "windows",
    target_os = "linux",
    target_os = "android",
    target_os = "macos",
    target_os = "ios",
    target_os = "freebsd"
)))]
pub fn get_os_uptime() -> Result<u64, Box<dyn Error>> {
    Err("Unsupported operating system".into())
}

/// Returns OS uptime in useful Duration format
pub fn get_os_uptime_duration() -> Result<Duration, Box<dyn Error>> {
    let ms = get_os_uptime()?;
    Ok(Duration::from_millis(ms))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_os_uptime() {
        let uptime = get_os_uptime();
        assert!(uptime.is_ok(), "Failed to get os uptime: {:?}", uptime.err());

        let uptime_ms = uptime.unwrap();
        assert!(uptime_ms > 0, "Uptime should be greater than 0");

        let duration = get_os_uptime_duration().unwrap();
        assert_eq!(duration.as_millis() as u64, uptime_ms);
    }
}
