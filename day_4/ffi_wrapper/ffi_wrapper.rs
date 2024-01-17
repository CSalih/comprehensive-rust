// TODO: remove this when you're done with your implementation.
#![allow(unused_imports, unused_variables, dead_code)]

mod ffi {
    use std::os::raw::{c_char, c_int};
    #[cfg(not(target_os = "macos"))]
    use std::os::raw::{c_long, c_uchar, c_ulong, c_ushort};

    // Opaque type. See https://doc.rust-lang.org/nomicon/ffi.html.
    #[repr(C)]
    pub struct DIR {
        _data: [u8; 0],
        _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
    }

    // Layout according to the Linux man page for readdir(3), where ino_t and
    // off_t are resolved according to the definitions in
    // /usr/include/x86_64-linux-gnu/{sys/types.h, bits/typesizes.h}.
    #[cfg(not(target_os = "macos"))]
    #[repr(C)]
    pub struct dirent {
        pub d_ino: c_ulong,
        pub d_off: c_long,
        pub d_reclen: c_ushort,
        pub d_type: c_uchar,
        pub d_name: [c_char; 256],
    }

    // Layout according to the macOS man page for dir(5).
    #[cfg(all(target_os = "macos"))]
    #[repr(C)]
    pub struct dirent {
        pub d_fileno: u64,
        pub d_seekoff: u64,
        pub d_reclen: u16,
        pub d_namlen: u16,
        pub d_type: u8,
        pub d_name: [c_char; 1024],
    }

    extern "C" {
        pub fn opendir(s: *const c_char) -> *mut DIR;

        #[cfg(not(all(target_os = "macos", target_arch = "x86_64")))]
        pub fn readdir(s: *mut DIR) -> *const dirent;

        // See https://github.com/rust-lang/libc/issues/414 and the section on
        // _DARWIN_FEATURE_64_BIT_INODE in the macOS man page for stat(2).
        //
        // "Platforms that existed before these updates were available" refers
        // to macOS (as opposed to iOS / wearOS / etc.) on Intel and PowerPC.
        #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
        #[link_name = "readdir$INODE64"]
        pub fn readdir(s: *mut DIR) -> *const dirent;

        pub fn closedir(s: *mut DIR) -> c_int;
    }
}

mod err {
    use std::ffi::CStr;
    use std::fs;
    use std::os::raw::{c_char, c_int};
    use std::str;

    const TMPBUF_SZ: usize = 128;

    // from https://github.com/rust-lang/rust/blob/1.26.2/src/libstd/sys/unix/os.rs#L87-L107
    pub fn error_string(errno: i32) -> String {
        extern "C" {
            #[cfg_attr(
                any(target_os = "linux", target_env = "newlib"),
                link_name = "__xpg_strerror_r"
            )]
            fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: libc::size_t) -> c_int;
        }

        let mut buf = [0 as c_char; TMPBUF_SZ];

        let p = buf.as_mut_ptr();
        unsafe {
            if strerror_r(errno as c_int, p, buf.len()) < 0 {
                panic!("strerror_r failure");
            }

            let p = p as *const _;
            str::from_utf8(CStr::from_ptr(p).to_bytes())
                .unwrap()
                .to_owned()
        }
    }
}

use crate::ffi::{closedir, opendir, readdir};
use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

#[derive(Debug)]
struct DirectoryIterator {
    path: CString,
    dir: *mut ffi::DIR,
}

impl DirectoryIterator {
    fn new(path: &str) -> Result<DirectoryIterator, String> {
        let c_path = CString::new(path).expect("CString::new failed");

        // Call opendir and return a Ok value if that worked,
        // otherwise return Err with a message.
        //
        // # Safety
        // opendir is unsafe because it is an extern function.
        // The input c_path and pointer returned by opendir is checked for null,
        // so it is safe to use.
        let dir = unsafe {
            let dir = opendir(c_path.as_ptr());
            if dir.is_null() {
                return Err(match std::io::Error::last_os_error().raw_os_error() {
                    Some(err) => format!("{}", err::error_string(err)),
                    None => String::from("opendir failed. No error code found."),
                });
            }
            dir
        };

        Ok(Self {
            path: CString::new(path).unwrap(),
            dir,
        })
    }
}

impl Iterator for DirectoryIterator {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        debug_assert!(
            !self.dir.is_null(),
            "You should never see this error but you see this because dir is NULL! \n\
                Make sure you call DirectoryIterator::new to create a new DirectoryIterator."
        );

        // Keep calling readdir until we get a NULL pointer back.
        //
        // # Safety
        // readdir is unsafe because it is a extern function.
        // The input dir cannot be NULL and pointer returned by readdir is checked for null,
        // so it is safe to use.
        let c_path = unsafe {
            let dirent = readdir(self.dir);
            let Some(dirent) = dirent.as_ref() else {
                return None;
            };
            CStr::from_ptr(dirent.d_name.as_ptr())
        };

        let path = String::from_utf8_lossy(c_path.to_bytes()).to_string();

        Some(OsString::from(path))
    }
}

impl Drop for DirectoryIterator {
    fn drop(&mut self) {
        debug_assert!(
            !self.dir.is_null(),
            "You should never see this error but you see this because dir is NULL! \n\
                Make sure you call DirectoryIterator::new to create a new DirectoryIterator."
        );

        // # Safety
        // readdir is unsafe because it is a extern function.
        // The input dir cannot be NULL, so it is safe to use.
        let result = unsafe { closedir(self.dir) };
        if result != 0 {
            match std::io::Error::last_os_error().raw_os_error() {
                Some(err) => panic!("closedir failed: {}", err::error_string(err)),
                None => panic!("closedir failed. No error code found."),
            }
        }
    }
}

fn main() -> Result<(), String> {
    let iter = DirectoryIterator::new(".")?;
    println!("files: {:#?}", iter.collect::<Vec<_>>());
    Ok(())
}
