use dragon::fmath::{Mat4, Quat, Vec3, Vec4};

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Vec4FP([i16; 4], [u16; 4]);

#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Mat4FP(pub [Vec4FP; 4]);

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Frustum(pub [Vec4; 6]);

#[inline(always)]
pub const fn f32_to_fixed(val: f32) -> i32 {
    (val * 65536.0) as i32
}

#[inline(always)]
pub const fn fixed_to_f32(i: i16, f: u16) -> f32 {
    i as f32 + f as f32 * (1.0 / 65536.0)
}

impl Vec4FP {
    #[inline]
    pub const fn as_ptr(&self) -> *const sys::T3DVec4FP {
        self as *const _ as _
    }
    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut sys::T3DVec4FP {
        self as *mut _ as _
    }
    #[inline]
    pub const fn set_fixed(&mut self, row: usize, val: i32) {
        self.0[row] = (val >> 16) as i16;
        self.1[row] = val as u16;
    }
    #[inline]
    pub const fn set_float(&mut self, row: usize, val: f32) {
        self.set_fixed(row, f32_to_fixed(val));
    }
    #[inline]
    pub const fn float(&mut self, row: usize) -> f32 {
        fixed_to_f32(self.0[row], self.1[row])
    }
}

impl From<sys::T3DVec4FP> for Vec4FP {
    #[inline]
    fn from(value: sys::T3DVec4FP) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl From<Vec4FP> for sys::T3DVec4FP {
    #[inline]
    fn from(value: Vec4FP) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

pub fn mat4_rot_from_dir(dir: &Vec3, up: &Vec3) -> Mat4 {
    let mut mat = core::mem::MaybeUninit::uninit();
    unsafe {
        sys::t3d_mat4_rot_from_dir(mat.as_mut_ptr(), dir.as_ptr(), up.as_ptr());
        mat.assume_init().into()
    }
}

impl Mat4FP {
    pub const IDENTITY: Self = Self([
        Vec4FP([1, 0, 0, 0], [0, 0, 0, 0]),
        Vec4FP([0, 1, 0, 0], [0, 0, 0, 0]),
        Vec4FP([0, 0, 1, 0], [0, 0, 0, 0]),
        Vec4FP([0, 0, 0, 1], [0, 0, 0, 0]),
    ]);
    #[inline]
    pub fn from_srt(scale: &Vec3, quat: &Quat, translate: &Vec3) -> Self {
        let mut mat = core::mem::MaybeUninit::uninit();
        unsafe {
            sys::t3d_mat4fp_from_srt(
                mat.as_mut_ptr(),
                scale.0.as_ptr(),
                quat.0.as_ptr(),
                translate.0.as_ptr(),
            );
            mat.assume_init().into()
        }
    }
    #[inline]
    pub fn from_srt_euler(scale: &Vec3, euler: &[f32; 3], translate: &Vec3) -> Self {
        let mut mat = core::mem::MaybeUninit::uninit();
        unsafe {
            sys::t3d_mat4fp_from_srt_euler(
                mat.as_mut_ptr(),
                scale.0.as_ptr(),
                euler.as_ptr(),
                translate.0.as_ptr(),
            );
            mat.assume_init().into()
        }
    }
    #[inline]
    pub fn from_mat4(mat: &Mat4) -> Self {
        let mut mfp = core::mem::MaybeUninit::uninit();
        unsafe {
            sys::t3d_mat4_to_fixed(mfp.as_mut_ptr(), mat.as_ptr());
            mfp.assume_init().into()
        }
    }
    #[inline]
    pub fn from_mat4_3x4(mat: &Mat4) -> Self {
        let mut mfp = core::mem::MaybeUninit::uninit();
        unsafe {
            sys::t3d_mat4_to_fixed_3x4(mfp.as_mut_ptr(), mat.as_ptr());
            mfp.assume_init().into()
        }
    }
    #[inline]
    pub const fn as_ptr(&self) -> *const sys::T3DMat4FP {
        &self.0 as *const _ as _
    }
    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut sys::T3DMat4FP {
        &mut self.0 as *mut _ as _
    }
    #[inline]
    pub const fn set_float(&mut self, column: usize, row: usize, val: f32) {
        self.0[column].set_float(row, val);
    }
    #[inline]
    pub const fn set_pos(&mut self, pos: &Vec3) {
        self.set_float(3, 0, pos.x());
        self.set_float(3, 1, pos.y());
        self.set_float(3, 2, pos.z());
    }
    #[inline]
    pub const fn float(&mut self, column: usize, row: usize) -> f32 {
        self.0[column].float(row)
    }
}

impl From<sys::T3DMat4FP> for Mat4FP {
    #[inline]
    fn from(value: sys::T3DMat4FP) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl From<Mat4FP> for sys::T3DMat4FP {
    #[inline]
    fn from(value: Mat4FP) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl Frustum {
    #[inline]
    pub fn from_mat4(mat: &Mat4) -> Self {
        let mut frustum = core::mem::MaybeUninit::uninit();
        unsafe {
            sys::t3d_mat4_to_frustum(frustum.as_mut_ptr(), mat.as_ptr());
            frustum.assume_init().into()
        }
    }
    #[inline]
    pub const fn as_ptr(&self) -> *const sys::T3DFrustum {
        &self.0 as *const _ as _
    }
    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut sys::T3DFrustum {
        &mut self.0 as *mut _ as _
    }
    #[inline]
    pub fn scale(&mut self, scale: f32) {
        unsafe { sys::t3d_frustum_scale(self.as_mut_ptr(), scale) }
    }
    #[inline]
    pub fn vs_aabb(&self, min: &Vec3, max: &Vec3) -> bool {
        unsafe { sys::t3d_frustum_vs_aabb(self.as_ptr(), min.as_ptr(), max.as_ptr()) }
    }
    #[inline]
    pub fn vs_aabb_s16(&self, min: &[i16; 3], max: &[i16; 3]) -> bool {
        unsafe { sys::t3d_frustum_vs_aabb_s16(self.as_ptr(), min.as_ptr(), max.as_ptr()) }
    }
    #[inline]
    pub fn vs_sphere(&self, center: &Vec3, radius: f32) -> bool {
        unsafe { sys::t3d_frustum_vs_sphere(self.as_ptr(), center.as_ptr(), radius) }
    }
}

impl From<sys::T3DFrustum> for Frustum {
    #[inline]
    fn from(value: sys::T3DFrustum) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl From<Frustum> for sys::T3DFrustum {
    #[inline]
    fn from(value: Frustum) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}
