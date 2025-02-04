use core::ffi::CStr;
use rustix::fd::BorrowedFd;
use rustix::fs::{cwd, Mode};

use libc::{c_char, c_int, mode_t};

use crate::convert_res;

#[no_mangle]
unsafe extern "C" fn chmod(pathname: *const c_char, mode: mode_t) -> c_int {
    libc!(libc::chmod(pathname, mode));

    fchmodat(libc::AT_FDCWD, pathname, mode, 0)
}

#[no_mangle]
unsafe extern "C" fn fchmod(fd: c_int, mode: mode_t) -> c_int {
    libc!(libc::fchmod(fd, mode));

    let mode = Mode::from_bits((mode & !libc::S_IFMT) as _).unwrap();
    match convert_res(rustix::fs::fchmod(BorrowedFd::borrow_raw(fd), mode)) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn fchmodat(
    fd: c_int,
    pathname: *const c_char,
    mode: mode_t,
    flags: c_int,
) -> c_int {
    libc!(libc::fchmodat(fd, pathname, mode, flags));

    if flags != 0 {
        todo!("flags support in fchmodat");
    }

    let mode = Mode::from_bits((mode & !libc::S_IFMT) as _).unwrap();
    match convert_res(rustix::fs::chmodat(
        cwd(),
        CStr::from_ptr(pathname.cast()),
        mode,
    )) {
        Some(()) => 0,
        None => -1,
    }
}
