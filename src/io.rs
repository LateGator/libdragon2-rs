use core::num::NonZeroI32;

use crate::ucstr::UCStr;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Error(NonZeroI32);

impl embedded_io::Error for Error {
    fn kind(&self) -> embedded_io::ErrorKind {
        use crate::sys::stdio::*;
        use embedded_io::ErrorKind::*;
        match self.0.get() as u32 {
            ENOENT => NotFound,
            EPERM | EACCES => PermissionDenied,
            ECONNREFUSED => ConnectionRefused,
            ECONNRESET => ConnectionReset,
            ECONNABORTED => ConnectionAborted,
            ENOTCONN => NotConnected,
            EADDRINUSE => AddrInUse,
            EADDRNOTAVAIL => AddrNotAvailable,
            EPIPE => BrokenPipe,
            EEXIST => AlreadyExists,
            EINVAL => InvalidInput,
            EIO => InvalidData,
            ETIMEDOUT => TimedOut,
            EINTR => Interrupted,
            ENOSYS => Unsupported,
            ENOMEM => OutOfMemory,
            ENOSPC => WriteZero,
            _ => Other,
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;

impl Error {
    #[inline]
    pub const fn code(self) -> i32 {
        self.0.get()
    }
    #[inline]
    pub fn desc(self) -> &'static UCStr {
        unsafe {
            let e = crate::sys::stdio::strerror(self.0.get());
            if e.is_null() {
                return crate::ucstr!("Unknown error");
            }
            UCStr::from_ptr(e)
        }
    }
    #[inline]
    pub fn from_raw(code: i32) -> Error {
        Self(NonZeroI32::new(code).unwrap_or_else(|| unsafe { NonZeroI32::new_unchecked(-1) }))
    }
    pub fn new(kind: embedded_io::ErrorKind) -> Self {
        use crate::sys::stdio::*;
        use embedded_io::ErrorKind::*;
        Self::from_raw(
            (match kind {
                NotFound => ENOENT,
                PermissionDenied => EPERM,
                ConnectionRefused => ECONNREFUSED,
                ConnectionReset => ECONNRESET,
                ConnectionAborted => ECONNABORTED,
                NotConnected => ENOTCONN,
                AddrInUse => EADDRINUSE,
                AddrNotAvailable => EADDRNOTAVAIL,
                BrokenPipe => EPIPE,
                AlreadyExists => EEXIST,
                InvalidInput => EINVAL,
                InvalidData => EIO,
                TimedOut => ETIMEDOUT,
                Interrupted => EINTR,
                Unsupported => ENOSYS,
                OutOfMemory => ENOMEM,
                WriteZero => ENOSPC,
                _ => u32::MAX,
            }) as i32,
        )
    }
    #[inline]
    pub fn from_errno() -> Error {
        Self::from_raw(unsafe { *crate::sys::stdio::__errno() })
    }
    #[inline]
    pub(crate) fn catch(ret: i32) -> Result<()> {
        match ret {
            0 => Ok(()),
            _ => Err(Self::from_errno()),
        }
    }
    #[inline]
    pub(crate) fn catch_negative(ret: i32) -> Result<i32> {
        match ret {
            ret if ret >= 0 => Ok(ret),
            _ => Err(Self::from_errno()),
        }
    }
}

impl core::fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.desc().fmt(f)
    }
}
