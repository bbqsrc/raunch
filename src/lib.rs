#![cfg(target_os = "macos")]

use std::ffi::CString;
use std::os::unix::io::RawFd;

#[no_mangle]
extern "C" {
    fn launch_activate_socket(
        name: *const libc::c_char,
        fds: *mut *mut libc::c_int,
        cnt: *mut libc::size_t,
    ) -> libc::c_int;
}

#[derive(Debug, Clone)]
/// See `man launch` for error cases of `launch_activate_socket`.
pub enum Error {
    Null,
    NotInPlist,
    NotManaged,
    AlreadyActivated,
    Unknown(libc::c_int),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Null => f.write_str("Null found in name string."),
            Error::NotInPlist => f.write_str("The socket name specified does not exist in the caller's launchd.plist."),
            Error::NotManaged => f.write_str("The calling process is not managed by launchd."),
            Error::AlreadyActivated => f.write_str("The specified socket has already been activated."),
            Error::Unknown(x) => f.write_str(&format!("An unknown error occurred with code {}.", x))
        }
    }
}

/// Pass the name of a socket listed in a launchd.plist, receive `RawFd`s.
///
/// See `man launch` for usage of `launch_activate_socket`.
pub fn activate_socket(name: &str) -> Result<Vec<RawFd>, Error> {
    let c_name = CString::new(name).map_err(|_| Error::Null)?;
    let mut fds: *mut libc::c_int = std::ptr::null_mut();
    let mut cnt: libc::size_t = 0;

    let error = unsafe { launch_activate_socket(c_name.as_ptr(), &mut fds, &mut cnt) };
    match error {
        0 => { /* :+1: */ }
        libc::ENOENT => return Err(Error::NotInPlist),
        libc::ESRCH => return Err(Error::NotManaged),
        libc::EALREADY => return Err(Error::AlreadyActivated),
        unknown => return Err(Error::Unknown(unknown)),
    };

    let out = unsafe {
        let slice = std::slice::from_raw_parts(fds, cnt);
        slice.iter().copied().collect::<Vec<RawFd>>()
    };

    unsafe {
        libc::free(fds as *mut _);
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        println!("{:?}", activate_socket("noidea"));
    }
}
