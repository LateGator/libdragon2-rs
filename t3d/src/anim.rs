use core::{
    cell::{Cell, UnsafeCell},
    marker::PhantomData,
};

use dragon::fmath::{Quat, Vec3};

use crate::Skeleton;

#[derive(Debug)]
#[repr(transparent)]
pub struct Anim<'m, 's>(
    pub(crate) UnsafeCell<sys::T3DAnim>,
    PhantomData<&'s Skeleton<'m>>,
);

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(u8)]
pub enum AnimTarget {
    Translation = sys::T3D_ANIM_TARGET_TRANSLATION as _,
    ScaleXYZ = sys::T3D_ANIM_TARGET_SCALE_XYZ as _,
    ScaleS = sys::T3D_ANIM_TARGET_SCALE_S as _,
    Rotation = sys::T3D_ANIM_TARGET_ROTATION as _,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(i32)]
pub enum AnimUpdate {
    None = 0,
    Changed = 1,
    Looped = 2,
}

impl<'m, 's> Anim<'m, 's> {
    #[inline]
    pub fn new(model: &'m crate::Model, name: &core::ffi::CStr) -> Self {
        Self(
            UnsafeCell::new(unsafe { sys::t3d_anim_create(model.as_ptr() as _, name.as_ptr()) }),
            PhantomData,
        )
    }
    #[inline]
    pub const fn as_ptr(&self) -> *mut sys::T3DAnim {
        self.0.get()
    }
    #[inline]
    pub unsafe fn attach(&self, skeleton: &'s crate::Skeleton) {
        unsafe { sys::t3d_anim_attach(self.as_ptr(), skeleton.as_ptr() as _) }
    }
    #[inline]
    pub fn attach_pos(
        &self,
        target_idx: u32,
        target: &'s Cell<Vec3>,
        update_flag: &'s Cell<AnimUpdate>,
    ) {
        unsafe {
            sys::t3d_anim_attach_pos(
                self.as_ptr(),
                target_idx,
                target.as_ptr().cast(),
                update_flag.as_ptr().cast(),
            )
        }
    }
    #[inline]
    pub fn attach_rot(
        &self,
        target_idx: u32,
        target: &'s Cell<Quat>,
        update_flag: &'s Cell<AnimUpdate>,
    ) {
        unsafe {
            sys::t3d_anim_attach_rot(
                self.as_ptr(),
                target_idx,
                target.as_ptr().cast(),
                update_flag.as_ptr().cast(),
            )
        }
    }
    #[inline]
    pub fn attach_scale(
        &self,
        target_idx: u32,
        target: &'s Cell<Vec3>,
        update_flag: &'s Cell<AnimUpdate>,
    ) {
        unsafe {
            sys::t3d_anim_attach_scale(
                self.as_ptr(),
                target_idx,
                target.as_ptr().cast(),
                update_flag.as_ptr().cast(),
            )
        }
    }
    #[inline]
    pub unsafe fn update(&self, delta_time: f32) {
        unsafe { sys::t3d_anim_update(self.as_ptr(), delta_time) }
    }
    #[inline]
    pub fn set_time(&self, time: f32) {
        unsafe { sys::t3d_anim_set_time(self.as_ptr(), time) }
    }
    #[inline]
    pub fn set_speed(&self, speed: f32) {
        unsafe { (*self.as_ptr()).speed = speed.max(0.0) };
    }
    #[inline]
    pub fn set_playing(&self, is_playing: bool) {
        unsafe { (*self.as_ptr()).isPlaying = is_playing as _ };
    }
    #[inline]
    pub fn set_looping(&self, r#loop: bool) {
        unsafe { (*self.as_ptr()).isLooping = r#loop as _ };
    }
}

impl<'m, 's> Drop for Anim<'m, 's> {
    #[inline]
    fn drop(&mut self) {
        unsafe { sys::t3d_anim_destroy(self.0.get()) }
    }
}
