use core::{cell::UnsafeCell, marker::PhantomData};

use dragon::fmath::{Quat, Vec3};

use crate::Skeleton;

#[derive(Debug)]
#[repr(transparent)]
pub struct Anim<'m, 's, 't>(
    pub(crate) UnsafeCell<sys::T3DAnim>,
    PhantomData<(&'s Skeleton<'m>, &'t mut ())>,
);

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(u8)]
pub enum AnimTarget {
    Translation = sys::T3D_ANIM_TARGET_TRANSLATION as _,
    ScaleXYZ = sys::T3D_ANIM_TARGET_SCALE_XYZ as _,
    ScaleS = sys::T3D_ANIM_TARGET_SCALE_S as _,
    Rotation = sys::T3D_ANIM_TARGET_ROTATION as _,
}

impl<'m, 's, 't> Anim<'m, 's, 't> {
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
    pub fn attach_pos(&self, target_idx: u32, target: &'t mut Vec3, update_flag: &'t mut i32) {
        unsafe {
            sys::t3d_anim_attach_pos(self.as_ptr(), target_idx, target.as_mut_ptr(), update_flag)
        }
    }
    #[inline]
    pub fn attach_rot(&self, target_idx: u32, target: &'t mut Quat, update_flag: &'t mut i32) {
        unsafe {
            sys::t3d_anim_attach_rot(self.as_ptr(), target_idx, target.as_mut_ptr(), update_flag)
        }
    }
    #[inline]
    pub fn attach_scale(&self, target_idx: u32, target: &'t mut Vec3, update_flag: &'t mut i32) {
        unsafe {
            sys::t3d_anim_attach_scale(self.as_ptr(), target_idx, target.as_mut_ptr(), update_flag)
        }
    }
    #[inline]
    pub fn update(&self, delta_time: f32) {
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

impl<'m, 's, 't> Drop for Anim<'m, 's, 't> {
    #[inline]
    fn drop(&mut self) {
        unsafe { sys::t3d_anim_destroy(self.0.get()) }
    }
}
