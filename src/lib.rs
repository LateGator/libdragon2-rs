#![no_std]
#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(temporary_niche_types)]
#![feature(asm_experimental_arch)]
#![feature(allocator_api)]

extern crate alloc as alloc_;

#[doc(hidden)]
pub use cstr::cstr;
#[doc(hidden)]
pub use paste::paste;

pub use sys;
#[macro_use]
mod macros;

pub mod alloc;
pub mod asset;
pub mod audio;
pub mod cpakfs;
pub mod debug;
pub mod display;
pub mod fmath;
pub mod fs;
pub mod graphics;
pub mod io;
pub mod joybus;
pub mod joypad;
pub mod mixer;
pub mod n64;
pub mod rdpq;
pub mod rsp;
pub mod rspq;
pub mod sprite;
pub mod surface;
pub mod ucstr;
pub mod wav64;
pub mod xm64;
pub mod ym64;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    debugln!("{info}");
    loop {}
}

pub(crate) mod sealed {
    pub trait Sealed {}
    impl Sealed for f32 {}
    impl Sealed for crate::fs::File {}
    impl Sealed for crate::joypad::Joypads {}
    impl<'r> Sealed for crate::rdpq::RdpQ<'r> {}
    impl<'s, 'r> Sealed for crate::rdpq::ModeFreeze<'s, 'r> {}
    impl<'s> Sealed for crate::surface::Surface<'s> {}
}

trait Undroppable {
    const ERROR: &'static str;
}

struct DropBomb<T: Undroppable>(core::mem::ManuallyDrop<T>);

impl<T: Undroppable> DropBomb<T> {
    pub fn new(val: T) -> Self {
        Self(core::mem::ManuallyDrop::new(val))
    }
}

impl<T: Undroppable> Drop for DropBomb<T> {
    fn drop(&mut self) {
        const {
            panic!("{}", T::ERROR);
        }
    }
}
