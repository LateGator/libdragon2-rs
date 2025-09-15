use core::ffi::CStr;

use crate::sys::debug::*;

bitflags::bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
    pub struct Features: u8 {
        const LOG_USB = DEBUG_FEATURE_LOG_USB as _;
        const LOG_ISVIEWER = DEBUG_FEATURE_LOG_ISVIEWER as _;
        const LOG_SD = DEBUG_FEATURE_LOG_SD as _;
        const FILE_SD = DEBUG_FEATURE_FILE_SD as _;
    }
}

#[doc = "Initialize debugging features of libdragon.\n\nThis function should be called at the beginning of main to request activation of debugging features. Passing [`Features::all`] will try to activate all features.\n\n@param channels a bitmask of debugging features to activate.\n\n@return true if at least a feature was activated, false otherwise."]
pub fn init(features: Features) -> bool {
    if cfg!(feature = "debug") {
        let mut ok = false;
        if features.contains(Features::LOG_USB) {
            ok |= init_usblog();
        }
        if features.contains(Features::LOG_ISVIEWER) {
            ok |= init_isviewer();
        }
        if features.contains(Features::FILE_SD) {
            ok |= init_sdfs(c"sd:/", -1);
        }
        if features.contains(Features::LOG_SD) {
            ok = init_sdlog(c"sd:/libdragon.log", c"a");
        }
        ok
    } else {
        false
    }
}
#[doc = "Initialize USB logging."]
pub fn init_usblog() -> bool {
    if cfg!(feature = "debug") {
        unsafe { debug_init_usblog() }
    } else {
        false
    }
}
#[doc = "Initialize ISViewer logging."]
#[inline]
pub fn init_isviewer() -> bool {
    if cfg!(feature = "debug") {
        unsafe { debug_init_isviewer() }
    } else {
        false
    }
}
#[doc = "Initialize SD logging."]
#[inline]
pub fn init_sdlog(_fn: &CStr, _openfmt: &CStr) -> bool {
    if cfg!(feature = "debug") {
        unsafe { debug_init_sdlog(_fn.as_ptr(), _openfmt.as_ptr()) }
    } else {
        false
    }
}
#[doc = "Initialize SD filesystem"]
#[inline]
pub fn init_sdfs(_prefix: &CStr, _npart: i32) -> bool {
    if cfg!(feature = "debug") {
        unsafe { debug_init_sdfs(_prefix.as_ptr(), _npart) }
    } else {
        false
    }
}

#[doc = "Do a hexdump of the specified buffer via `debugf`\n\n This is useful to dump a binary buffer for debugging purposes. The hexdump shown\n contains both the hexadecimal and ASCII values, similar to what hex editors do.\n\n Sample output:\n\n <pre>\n 0000  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80   |................|\n 0010  45 67 cd ef aa aa aa aa  aa aa aa aa aa aa aa aa   |Eg..............|\n 0020  9a bc 12 34 80 80 80 80  80 80 80 80 80 80 80 80   |...4............|\n 0030  aa aa aa aa aa aa aa aa  ef 01 67 89 aa aa aa aa   |..........g.....|\n 0040  80 80 80 80 80 80 80 80  00 00 00 00 80 80 80 80   |................|\n </pre>\n\n @param[in] buffer \tBuffer to dump\n @param[in] size \t\tSize of the buffer in bytes"]
#[inline]
pub fn hexdump<T>(_buffer: &[T]) {
    if cfg!(feature = "debug") {
        unsafe { debug_hexdump(_buffer.as_ptr() as _, _buffer.len() as _) }
    }
}
#[doc = "Dump a backtrace (call stack) via `debugf`\n\n This function will dump the current call stack to the debugging channel. It is\n useful to understand where the program is currently executing, and to understand\n the context of an error.\n\n The implementation of this function relies on the lower level `backtrace` and\n `backtrace_symbols` functions, which are implemented in libdragon itself via\n a symbol table embedded in the ROM. See `backtrace_symbols` for more information.\n\n @see `backtrace`\n @see `backtrace_symbols`"]
#[inline]
pub fn backtrace() {
    if cfg!(feature = "debug") {
        unsafe { debug_backtrace() }
    }
}

#[doc(hidden)]
pub struct Debug;

impl core::fmt::Write for Debug {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if cfg!(feature = "debug") {
            use crate::sys::stdio::*;
            unsafe {
                fwrite(s.as_ptr() as _, 1, s.len() as _, (*_impure_ptr)._stderr);
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => { {
        let _ = ::core::fmt::Write::write_fmt(
            &mut $crate::debug::Debug,
            ::core::format_args!($($arg)*)
        );
    } };
}

#[macro_export]
macro_rules! debugln {
    () => { {
        let _ = ::core::fmt::Write::write_str(&mut $crate::debug::Debug, "\n");
    } };
    ($($arg:tt)*) => { {
        let _ = ::core::fmt::Write::write_fmt(
            &mut $crate::debug::Debug,
            ::core::format_args!("{}\n", ::core::format_args!($($arg)*))
        );
    } };
}
