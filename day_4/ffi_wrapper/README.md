---
minutes: 30
---

# Safe FFI Wrapper

Rust has great support for calling functions through a _foreign function
interface_ (FFI). We will use this to build a safe wrapper for the `libc`
functions you would use from C to read the names of files in a directory.

You will want to consult the manual pages:

- [`opendir(3)`](https://man7.org/linux/man-pages/man3/opendir.3.html)
- [`readdir(3)`](https://man7.org/linux/man-pages/man3/readdir.3.html)
- [`closedir(3)`](https://man7.org/linux/man-pages/man3/closedir.3.html)

You will also want to browse the [`std::ffi`] module. There you find a number of
string types which you need for the exercise:

| Types                      | Encoding       | Use                            |
| -------------------------- | -------------- | ------------------------------ |
| [`str`] and [`String`]     | UTF-8          | Text processing in Rust        |
| [`CStr`] and [`CString`]   | NUL-terminated | Communicating with C functions |
| [`OsStr`] and [`OsString`] | OS-specific    | Communicating with the OS      |

You will convert between all these types:

- `&str` to `CString`: you need to allocate space for a trailing `\0` character,
- `CString` to `*const i8`: you need a pointer to call C functions,
- `*const i8` to `&CStr`: you need something which can find the trailing `\0`
  character,
- `&CStr` to `&[u8]`: a slice of bytes is the universal interface for "some
  unknown data",
- `&[u8]` to `&OsStr`: `&OsStr` is a step towards `OsString`, use
  [`OsStrExt`](https://doc.rust-lang.org/std/os/unix/ffi/trait.OsStrExt.html) to
  create it,
- `&OsStr` to `OsString`: you need to clone the data in `&OsStr` to be able to
  return it and call `readdir` again.

The [Nomicon] also has a very useful chapter about FFI.

[`std::ffi`]: https://doc.rust-lang.org/std/ffi/
[`str`]: https://doc.rust-lang.org/std/primitive.str.html
[`String`]: https://doc.rust-lang.org/std/string/struct.String.html
[`CStr`]: https://doc.rust-lang.org/std/ffi/struct.CStr.html
[`CString`]: https://doc.rust-lang.org/std/ffi/struct.CString.html
[`OsStr`]: https://doc.rust-lang.org/std/ffi/struct.OsStr.html
[`OsString`]: https://doc.rust-lang.org/std/ffi/struct.OsString.html
[Nomicon]: https://doc.rust-lang.org/nomicon/ffi.html

Copy the code below to <https://play.rust-lang.org/> and fill in the missing
functions and methods:

```rust
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

use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

#[derive(Debug)]
struct DirectoryIterator {
    path: CString,
    dir: *mut ffi::DIR,
}

impl DirectoryIterator {
    fn new(path: &str) -> Result<DirectoryIterator, String> {
        // Call opendir and return a Ok value if that worked,
        // otherwise return Err with a message.
        unimplemented!()
    }
}

impl Iterator for DirectoryIterator {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        // Keep calling readdir until we get a NULL pointer back.
        unimplemented!()
    }
}

impl Drop for DirectoryIterator {
    fn drop(&mut self) {
        // Call closedir as needed.
        unimplemented!()
    }
}

fn main() -> Result<(), String> {
    let iter = DirectoryIterator::new(".")?;
    println!("files: {:#?}", iter.collect::<Vec<_>>());
    Ok(())
}
```
