use core::{cell::UnsafeCell, marker::PhantomData};

use dragon::fmath::{Quat, Vec3};

#[derive(Debug)]
pub struct Skeleton<'m>(
    pub(crate) UnsafeCell<sys::T3DSkeleton>,
    PhantomData<&'m crate::Model>,
);

impl<'m> Skeleton<'m> {
    #[inline]
    pub fn new(model: &'m crate::Model) -> Self {
        Self::new_buffered(model, 1)
    }
    #[inline]
    pub fn new_buffered(model: &'m crate::Model, buffer_count: u32) -> Self {
        Self(
            UnsafeCell::new(unsafe {
                sys::t3d_skeleton_create_buffered(model.as_ptr(), buffer_count as _)
            }),
            PhantomData,
        )
    }
    #[inline]
    pub const fn as_ptr(&self) -> *mut sys::T3DSkeleton {
        self.0.get()
    }
    #[inline]
    pub fn clone_blend(&self) -> Self {
        Self(
            UnsafeCell::new(unsafe { sys::t3d_skeleton_clone(self.0.get() as _, false) }),
            PhantomData,
        )
    }
    #[inline]
    pub fn reset(&self) {
        unsafe { sys::t3d_skeleton_reset(self.0.get() as _) }
    }
    #[inline]
    pub fn update(&self) {
        unsafe {
            let ptr = self.0.get();
            if (*ptr).boneMatricesFP.is_null() {
                return;
            }
            sys::t3d_skeleton_update(ptr);
        }
    }
    #[inline]
    pub fn blend_from(&self, skel_a: &Self, skel_b: &Self, factor: f32) {
        unsafe {
            sys::t3d_skeleton_blend(
                self.0.get() as _,
                skel_a.0.get() as _,
                skel_b.0.get() as _,
                factor,
            )
        }
    }
    #[inline]
    pub fn find_bone(&self, name: &core::ffi::CStr) -> i32 {
        unsafe { sys::t3d_skeleton_find_bone(self.0.get() as _, name.as_ptr()) }
    }
    #[inline]
    pub fn bone_scale(&self, bone: i32) -> Vec3 {
        let ptr = self.0.get();
        unsafe {
            assert!((bone as u32) < (*(*ptr).skeletonRef).boneCount as u32);
            Vec3((*(*ptr).bones.add(bone as usize)).scale.v)
        }
    }
    #[inline]
    pub fn bone_rotation(&self, bone: i32) -> Quat {
        let ptr = self.0.get();
        unsafe {
            assert!((bone as u32) < (*(*ptr).skeletonRef).boneCount as u32);
            Quat((*(*ptr).bones.add(bone as usize)).rotation.v)
        }
    }
    #[inline]
    pub fn bone_position(&self, bone: i32) -> Vec3 {
        let ptr = self.0.get();
        unsafe {
            assert!((bone as u32) < (*(*ptr).skeletonRef).boneCount as u32);
            Vec3((*(*ptr).bones.add(bone as usize)).position.v)
        }
    }
    #[inline]
    pub fn bone_set_scale(&self, bone: i32, scale: Vec3) {
        let ptr = self.0.get();
        unsafe {
            assert!((bone as u32) < (*(*ptr).skeletonRef).boneCount as u32);
            (*(*ptr).bones.add(bone as usize)).scale.v = scale.0;
            (*(*ptr).bones.add(bone as usize)).hasChanged = 1;
        }
    }
    #[inline]
    pub fn bone_set_rotation(&self, bone: i32, rotation: Quat) {
        let ptr = self.0.get();
        unsafe {
            assert!((bone as u32) < (*(*ptr).skeletonRef).boneCount as u32);
            (*(*ptr).bones.add(bone as usize)).rotation.v = rotation.0;
            (*(*ptr).bones.add(bone as usize)).hasChanged = 1;
        }
    }
    #[inline]
    pub fn bone_set_position(&self, bone: i32, position: Vec3) {
        let ptr = self.0.get();
        unsafe {
            assert!((bone as u32) < (*(*ptr).skeletonRef).boneCount as u32);
            (*(*ptr).bones.add(bone as usize)).position.v = position.0;
            (*(*ptr).bones.add(bone as usize)).hasChanged = 1;
        }
    }
}

impl<'m> Clone for Skeleton<'m> {
    #[inline]
    fn clone(&self) -> Self {
        Self(
            UnsafeCell::new(unsafe { sys::t3d_skeleton_clone(self.0.get(), true) }),
            PhantomData,
        )
    }
}

impl<'m> Drop for Skeleton<'m> {
    #[inline]
    fn drop(&mut self) {
        unsafe { sys::t3d_skeleton_destroy(self.0.get()) }
    }
}
