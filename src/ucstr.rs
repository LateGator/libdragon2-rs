use core::{
    ffi::{CStr, FromBytesUntilNulError, c_char},
    mem::transmute,
    slice::{from_raw_parts, from_raw_parts_mut},
    str::Utf8Error,
};

#[repr(transparent)]
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct UCStr(str);

#[macro_export]
macro_rules! ucstr {
    ($str:expr $(,)?) => {
        match $crate::ucstr::UCStr::from_str(::core::concat!($str, "\0")) {
            ::core::option::Option::Some(s) => s,
            None => ::core::unreachable!(),
        }
    };
}

impl UCStr {
    const EMPTY: &Self = ucstr!("");
    #[inline]
    pub const unsafe fn from_raw_parts<'s>(ptr: *const c_char, len: usize) -> &'s Self {
        debug_assert!(!ptr.is_null());
        unsafe { transmute(from_raw_parts(ptr, len)) }
    }
    #[inline]
    pub const unsafe fn from_ptr<'s>(ptr: *const c_char) -> &'s Self {
        debug_assert!(!ptr.is_null());
        unsafe { Self::from_cstr_unchecked(CStr::from_ptr(ptr)) }
    }
    #[inline]
    pub const unsafe fn from_cstr_unchecked<'s>(s: &'s CStr) -> &'s Self {
        unsafe { transmute(core::str::from_utf8_unchecked(s.to_bytes())) }
    }
    #[inline]
    pub const fn from_cstr<'s>(s: &'s CStr) -> Result<&'s Self, Utf8Error> {
        match core::str::from_utf8(s.to_bytes()) {
            Ok(s) => Ok(unsafe { transmute(s) }),
            Err(e) => Err(e),
        }
    }
    #[inline]
    pub const fn from_str<'s>(s: &'s str) -> Option<&'s Self> {
        let b = s.as_bytes();
        let mut i = 0;
        while i < b.len() {
            if b[i] == b'\0' {
                return unsafe { Some(Self::from_raw_parts(s.as_ptr() as _, i)) };
            }
            i += 1;
        }
        None
    }
    #[inline]
    pub const fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }
    #[inline]
    pub const fn as_str(&self) -> &str {
        &self.0
    }
    #[inline]
    pub const fn as_cstr(&self) -> &CStr {
        unsafe {
            CStr::from_bytes_with_nul_unchecked(from_raw_parts(self.as_ptr(), self.0.len() + 1))
        }
    }
}

impl<'s> Default for &'s UCStr {
    #[inline]
    fn default() -> Self {
        UCStr::EMPTY
    }
}

impl<'s> From<&'s UCStr> for &'s str {
    #[inline]
    fn from(value: &'s UCStr) -> Self {
        value.as_str()
    }
}

impl<'s> From<&'s UCStr> for &'s CStr {
    #[inline]
    fn from(value: &'s UCStr) -> Self {
        value.as_cstr()
    }
}

impl<'s> TryFrom<&'s CStr> for &'s UCStr {
    type Error = Utf8Error;
    #[inline]
    fn try_from(value: &'s CStr) -> Result<Self, Self::Error> {
        UCStr::from_cstr(value)
    }
}

impl<'s> TryFrom<&'s str> for &'s UCStr {
    type Error = FromBytesUntilNulError;
    #[inline]
    fn try_from(value: &'s str) -> Result<Self, Self::Error> {
        Ok(unsafe { UCStr::from_cstr_unchecked(CStr::from_bytes_until_nul(value.as_bytes())?) })
    }
}

wrapper! { UCStr => str { self => self.0 } }

impl core::borrow::Borrow<CStr> for UCStr {
    #[inline]
    fn borrow(&self) -> &CStr {
        self.as_cstr()
    }
}
impl core::convert::AsRef<CStr> for UCStr {
    #[inline]
    fn as_ref(&self) -> &CStr {
        self.as_cstr()
    }
}

impl core::fmt::Debug for UCStr {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl core::fmt::Display for UCStr {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct UCStringArray<const N: usize> {
    buffer: [u8; N],
    len: u32,
}

impl<const N: usize> Default for UCStringArray<N> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> UCStringArray<N> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            buffer: unsafe { core::mem::MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }
    #[inline]
    pub const unsafe fn from_raw_unchecked(buffer: [u8; N], len: u32) -> Self {
        Self { buffer, len }
    }
    #[inline]
    pub const fn capacity(&self) -> usize {
        let cap = N.saturating_sub(1);
        if cap > u32::MAX as usize {
            u32::MAX as usize
        } else {
            cap
        }
    }
    #[inline]
    pub const fn len(&self) -> usize {
        self.len as _
    }
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len != 0
    }
    #[inline]
    pub const fn clear(&mut self) {
        if N > 0 {
            self.buffer[0] = b'\0';
        }
        self.len = 0;
    }
    #[inline]
    pub const fn from_ucstr(s: &UCStr) -> Option<Self> {
        let s = s.as_cstr().to_bytes_with_nul();
        if s.len() > Self::new().capacity() {
            None
        } else {
            let buffer = unsafe {
                let mut buffer = core::mem::MaybeUninit::<[u8; N]>::uninit();
                core::ptr::copy_nonoverlapping(s.as_ptr(), buffer.as_mut_ptr() as *mut _, s.len());
                buffer.assume_init()
            };
            Some(Self {
                buffer,
                len: (s.len() - 1) as _,
            })
        }
    }
    #[inline]
    pub const fn grow(&mut self, len: usize) -> usize {
        match self.try_grow(len) {
            Some(len) => len,
            None => panic!("UCStringArray overflow"),
        }
    }
    #[inline]
    pub const fn try_grow(&mut self, len: usize) -> Option<usize> {
        match self.len().checked_add(len) {
            Some(newlen) if newlen <= self.capacity() => {
                Some(core::mem::replace(&mut self.len, newlen as _) as _)
            }
            _ => None,
        }
    }
    pub const fn try_push(&mut self, c: char) -> bool {
        assert!(c != '\0', "null terminators not allowed in UCStringArray");
        let cl = c.len_utf8();
        let Some(i) = self.try_grow(cl) else {
            return false;
        };
        unsafe {
            c.encode_utf8(
                self.buffer
                    .split_at_mut_unchecked(i)
                    .1
                    .split_at_mut_unchecked(cl)
                    .0,
            );
            self.buffer.as_mut_ptr().add(self.len as usize).write(b'\0');
        }
        true
    }
    pub const fn try_push_ucstr(&mut self, s: &UCStr) -> bool {
        let cl = s.as_str().len();
        let Some(i) = self.try_grow(cl) else {
            return false;
        };
        let s = s.as_cstr().to_bytes_with_nul();
        unsafe {
            core::ptr::copy_nonoverlapping(s.as_ptr(), self.buffer.as_mut_ptr().add(i), s.len());
        }
        true
    }
    #[inline]
    pub const fn push(&mut self, c: char) {
        if !self.try_push(c) {
            panic!("UCStringArray overflow");
        }
    }
    #[inline]
    pub const fn push_ucstr(&mut self, s: &UCStr) {
        if !self.try_push_ucstr(s) {
            panic!("UCStringArray overflow");
        }
    }
    #[inline]
    pub const fn as_ptr(&self) -> *const u8 {
        if N == 0 {
            UCStr::EMPTY.as_ptr()
        } else {
            self.buffer.as_ptr()
        }
    }
    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut u8 {
        assert!(
            N > 0,
            "mutable references to zero length UCStringArray not allowed"
        );
        self.buffer.as_mut_ptr()
    }
    #[inline]
    pub const fn as_ucstr(&self) -> &UCStr {
        if N == 0 {
            UCStr::EMPTY
        } else {
            unsafe { transmute(from_raw_parts(self.as_ptr(), self.len as _)) }
        }
    }
    #[inline]
    pub const fn as_mut_ucstr(&mut self) -> &mut UCStr {
        assert!(
            N > 0,
            "mutable references to zero length UCStringArray not allowed"
        );
        unsafe { transmute(from_raw_parts_mut(self.as_mut_ptr(), self.len as _)) }
    }
}

impl<'s, const N: usize> From<&'s UCStringArray<N>> for &'s str {
    #[inline]
    fn from(value: &'s UCStringArray<N>) -> Self {
        value.as_str()
    }
}

impl<'s, const N: usize> From<&'s UCStringArray<N>> for &'s CStr {
    #[inline]
    fn from(value: &'s UCStringArray<N>) -> Self {
        value.as_cstr()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct TooLongError(());

impl core::fmt::Display for TooLongError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        "string is too long".fmt(f)
    }
}

impl<'s, const N: usize> TryFrom<&'s UCStr> for UCStringArray<N> {
    type Error = TooLongError;
    #[inline]
    fn try_from(value: &'s UCStr) -> Result<Self, Self::Error> {
        Self::from_ucstr(value).ok_or_else(|| TooLongError(()))
    }
}

impl<const N: usize> core::ops::Deref for UCStringArray<N> {
    type Target = UCStr;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ucstr()
    }
}
impl<const N: usize> core::ops::DerefMut for UCStringArray<N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_ucstr()
    }
}
impl<const N: usize> core::borrow::Borrow<UCStr> for UCStringArray<N> {
    #[inline]
    fn borrow(&self) -> &UCStr {
        self.as_ucstr()
    }
}
impl<const N: usize> core::convert::AsRef<UCStr> for UCStringArray<N> {
    #[inline]
    fn as_ref(&self) -> &UCStr {
        self.as_ucstr()
    }
}
impl<const N: usize> core::borrow::Borrow<str> for UCStringArray<N> {
    #[inline]
    fn borrow(&self) -> &str {
        self.as_ucstr()
    }
}
impl<const N: usize> core::convert::AsRef<str> for UCStringArray<N> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_ucstr()
    }
}
impl<const N: usize> core::borrow::Borrow<CStr> for UCStringArray<N> {
    #[inline]
    fn borrow(&self) -> &CStr {
        self.as_cstr()
    }
}
impl<const N: usize> core::convert::AsRef<CStr> for UCStringArray<N> {
    #[inline]
    fn as_ref(&self) -> &CStr {
        self.as_cstr()
    }
}
impl<const N: usize> core::borrow::BorrowMut<UCStr> for UCStringArray<N> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut UCStr {
        self.as_mut_ucstr()
    }
}
impl<const N: usize> core::convert::AsMut<UCStr> for UCStringArray<N> {
    #[inline]
    fn as_mut(&mut self) -> &mut UCStr {
        self.as_mut_ucstr()
    }
}
impl<const N: usize> core::borrow::BorrowMut<str> for UCStringArray<N> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut str {
        self.as_mut_ucstr()
    }
}
impl<const N: usize> core::convert::AsMut<str> for UCStringArray<N> {
    #[inline]
    fn as_mut(&mut self) -> &mut str {
        self.as_mut_ucstr()
    }
}

impl<const N: usize> core::fmt::Debug for UCStringArray<N> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_ucstr().fmt(f)
    }
}

impl<const N: usize> core::fmt::Display for UCStringArray<N> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_ucstr().fmt(f)
    }
}
