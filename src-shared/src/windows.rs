use std::io::Error;
use std::{ffi::OsStr, os::windows::prelude::OsStrExt, ptr};

use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::winnt::{TokenElevation, HANDLE, TOKEN_ELEVATION, TOKEN_QUERY};
use windows_sys::Win32::UI::Shell::ShellExecuteW;

pub fn is_elevated() -> bool {
    _is_app_elevated().unwrap_or(false)
}

pub fn relaunch_with_elevation(main_port: u16, main_pid: u32, force_exit: bool) {
    // Get executable path
    let exe_path = std::env::current_exe().unwrap();
    let path = exe_path.as_os_str();
    let mut path_result: Vec<_> = path.encode_wide().collect();
    path_result.push(0);
    let path = path_result;
    // Get port parameter
    let mut port_result: Vec<_> = OsStr::new(format!("{main_port} {main_pid}").as_str()).encode_wide().collect();
    port_result.push(0);
    let port = port_result;
    // Run as administrator
    let operation: Vec<u16> = OsStr::new("runas\0").encode_wide().collect();
    let r = unsafe {
        ShellExecuteW(
            0,
            operation.as_ptr(),
            path.as_ptr(),
            port.as_ptr(),
            ptr::null(),
            0,
        )
    };
    // Quit non-admin process if successful (self)
    if r > 32 || force_exit {
        std::process::exit(0);
    }
}

/// On success returns a bool indicating if the current process has admin rights.
/// Otherwise returns an OS error.
///
/// This is unlikely to fail but if it does it's even more unlikely that you have admin permissions anyway.
/// Therefore the public function above simply eats the error and returns a bool.
fn _is_app_elevated() -> Result<bool, Error> {
    let token = QueryAccessToken::from_current_process()?;
    token.is_elevated()
}

/// A safe wrapper around querying Windows access tokens.
pub struct QueryAccessToken(HANDLE);
impl QueryAccessToken {
    pub fn from_current_process() -> Result<Self, Error> {
        unsafe {
            let mut handle: HANDLE = ptr::null_mut();
            if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut handle) != 0 {
                Ok(Self(handle))
            } else {
                Err(Error::last_os_error())
            }
        }
    }

    /// On success returns a bool indicating if the access token has elevated privilidges.
    /// Otherwise returns an OS error.
    pub fn is_elevated(&self) -> Result<bool, Error> {
        unsafe {
            let mut elevation = TOKEN_ELEVATION::default();
            let size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;
            let mut ret_size = size;
            // The weird looking repetition of `as *mut _` is casting the reference to a c_void pointer.
            if GetTokenInformation(
                self.0,
                TokenElevation,
                &mut elevation as *mut _ as *mut _,
                size,
                &mut ret_size,
            ) != 0
            {
                Ok(elevation.TokenIsElevated != 0)
            } else {
                Err(Error::last_os_error())
            }
        }
    }
}
impl Drop for QueryAccessToken {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { CloseHandle(self.0) };
        }
    }
}