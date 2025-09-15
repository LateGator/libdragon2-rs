use core::ops::*;
use sys::{fgeom::*, fmath::*};

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, PartialOrd, PartialEq)]
pub struct Vec3(pub [f32; 3]);

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, PartialOrd, PartialEq)]
pub struct Vec4(pub [f32; 4]);

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, PartialOrd, PartialEq)]
pub struct Quat(pub [f32; 4]);

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, PartialOrd, PartialEq)]
pub struct Mat4(pub [[f32; 4]; 4]);

#[inline]
pub fn sinf(x: f32) -> f32 {
    unsafe { fm_sinf(x) }
}

#[inline]
pub fn sinf_approx(x: f32, approx: i32) -> f32 {
    unsafe { fm_sinf_approx(x, approx) }
}

#[inline]
pub fn cosf(x: f32) -> f32 {
    unsafe { fm_cosf(x) }
}

#[inline]
pub fn sincosf(x: f32) -> (f32, f32) {
    let mut sin = core::mem::MaybeUninit::uninit();
    let mut cos = core::mem::MaybeUninit::uninit();
    unsafe {
        fm_sincosf(x, sin.as_mut_ptr(), cos.as_mut_ptr());
        (sin.assume_init(), cos.assume_init())
    }
}

#[inline]
pub fn atan2f(y: f32, x: f32) -> f32 {
    unsafe { fm_atan2f(y, x) }
}

#[inline]
pub fn exp(x: f32) -> f32 {
    unsafe { fm_exp(x) }
}

#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[inline]
pub fn lerp_angle(a: f32, b: f32, t: f32) -> f32 {
    unsafe { fm_lerp_angle(a, b, t) }
}

#[inline]
pub fn wrap_angle(angle: f32) -> f32 {
    unsafe { fm_wrap_angle(angle) }
}

pub trait FastMathExt: crate::sealed::Sealed + Sized {
    fn sqrt(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn sin_cos(self) -> (Self, Self);
    fn atan2(self, other: f32) -> Self;
    fn exp(self) -> Self;
}

impl FastMathExt for f32 {
    #[inline]
    fn sqrt(self) -> f32 {
        unsafe { core::intrinsics::sqrtf32(self) }
    }
    #[inline]
    fn sin(self) -> f32 {
        sinf(self)
    }
    #[inline]
    fn cos(self) -> f32 {
        cosf(self)
    }
    #[inline]
    fn sin_cos(self) -> (f32, f32) {
        sincosf(self)
    }
    #[inline]
    fn atan2(self, other: f32) -> f32 {
        atan2f(self, other)
    }
    #[inline]
    fn exp(self) -> f32 {
        exp(self)
    }
}

impl Vec3 {
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z])
    }
    #[inline]
    pub const fn splat(n: f32) -> Self {
        Self::new(n, n, n)
    }
    #[inline]
    pub const fn as_ptr(&self) -> *const fm_vec3_t {
        self as *const _ as _
    }
    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut fm_vec3_t {
        self as *mut _ as _
    }
    #[inline]
    pub const fn x(&self) -> f32 {
        self.0[0]
    }
    #[inline]
    pub const fn y(&self) -> f32 {
        self.0[1]
    }
    #[inline]
    pub const fn z(&self) -> f32 {
        self.0[2]
    }
    #[inline]
    pub const fn set_x(&mut self, x: f32) {
        self.0[0] = x;
    }
    #[inline]
    pub const fn set_y(&mut self, y: f32) {
        self.0[1] = y;
    }
    #[inline]
    pub const fn set_z(&mut self, z: f32) {
        self.0[2] = z;
    }
    #[inline]
    pub const fn dot(&self, rhs: &Self) -> f32 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }
    #[inline]
    pub const fn length_squared(&self) -> f32 {
        self.dot(self)
    }
    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    #[inline]
    pub const fn distance_squared(&self, rhs: &Self) -> f32 {
        (*self).sub(*rhs).length_squared()
    }
    #[inline]
    pub fn distance(&self, rhs: &Self) -> f32 {
        self.distance_squared(rhs).sqrt()
    }
    #[inline]
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len < FM_EPSILON as f32 {
            Self::ZERO
        } else {
            let invlen = 1.0 / len;
            self * invlen
        }
    }
    #[inline]
    pub const fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }
    #[inline]
    pub const fn lerp(&self, rhs: &Self, t: f32) -> Self {
        (*self).add((*rhs).sub(*self)).mul(Self::splat(t))
    }
    #[inline]
    pub fn reflect(&mut self, normal: Self) {
        unsafe { fm_vec3_reflect(self.as_mut_ptr(), self.as_ptr(), normal.as_ptr()) };
    }
    #[inline]
    pub fn reflected(&self, normal: Self) -> Self {
        let mut vec = core::mem::MaybeUninit::uninit();
        unsafe {
            fm_vec3_reflect(vec.as_mut_ptr(), self.as_ptr(), normal.as_ptr());
            Self(vec.assume_init().v)
        }
    }
    #[inline]
    pub fn refract(&mut self, normal: Self, eta: f32) -> bool {
        unsafe { fm_vec3_refract(self.as_mut_ptr(), self.as_ptr(), normal.as_ptr(), eta) }
    }
    #[inline]
    pub fn refracted(&self, normal: Self, eta: f32) -> (Self, bool) {
        let mut vec = core::mem::MaybeUninit::uninit();
        unsafe {
            let res = fm_vec3_refract(vec.as_mut_ptr(), self.as_ptr(), normal.as_ptr(), eta);
            (Self(vec.assume_init().v), res)
        }
    }
}

impl From<fm_vec3_t> for Vec3 {
    #[inline]
    fn from(value: fm_vec3_t) -> Self {
        unsafe { Self(value.v) }
    }
}

impl From<Vec3> for fm_vec3_t {
    #[inline]
    fn from(value: Vec3) -> Self {
        Self { v: value.0 }
    }
}

impl From<[f32; 3]> for Vec3 {
    #[inline]
    fn from(value: [f32; 3]) -> Self {
        Self(value)
    }
}

impl From<Vec3> for [f32; 3] {
    #[inline]
    fn from(value: Vec3) -> Self {
        value.0
    }
}

impl From<&[f32; 3]> for Vec3 {
    #[inline]
    fn from(value: &[f32; 3]) -> Self {
        Self(*value)
    }
}

impl From<Vec3> for (f32, f32, f32) {
    #[inline]
    fn from(value: Vec3) -> Self {
        (value.0[0], value.0[1], value.0[2])
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    #[inline]
    fn from(value: (f32, f32, f32)) -> Self {
        Self([value.0, value.1, value.2])
    }
}

impl From<&(f32, f32, f32)> for Vec3 {
    #[inline]
    fn from(value: &(f32, f32, f32)) -> Self {
        Self([value.0, value.1, value.2])
    }
}

impl TryFrom<&[f32]> for Vec3 {
    type Error = core::array::TryFromSliceError;
    #[inline]
    fn try_from(value: &[f32]) -> Result<Self, Self::Error> {
        Ok(Self(<_>::try_from(value)?))
    }
}

impl Deref for Vec3 {
    type Target = [f32; 3];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Vec4 {
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0);
    pub const IDENTITY: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self([x, y, z, w])
    }
    #[inline]
    pub const fn as_ptr(&self) -> *const fm_vec4_t {
        self as *const _ as _
    }
    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut fm_vec4_t {
        self as *mut _ as _
    }
    #[inline]
    pub const fn x(&self) -> f32 {
        self.0[0]
    }
    #[inline]
    pub const fn y(&self) -> f32 {
        self.0[1]
    }
    #[inline]
    pub const fn z(&self) -> f32 {
        self.0[2]
    }
    #[inline]
    pub const fn w(&self) -> f32 {
        self.0[3]
    }
    #[inline]
    pub const fn set_x(&mut self, x: f32) {
        self.0[0] = x;
    }
    #[inline]
    pub const fn set_y(&mut self, y: f32) {
        self.0[1] = y;
    }
    #[inline]
    pub const fn set_z(&mut self, z: f32) {
        self.0[2] = z;
    }
    #[inline]
    pub const fn set_w(&mut self, w: f32) {
        self.0[3] = w;
    }
}

impl From<[f32; 4]> for Vec4 {
    #[inline]
    fn from(value: [f32; 4]) -> Self {
        Self(value)
    }
}

impl From<Vec4> for [f32; 4] {
    #[inline]
    fn from(value: Vec4) -> Self {
        value.0
    }
}

impl From<&[f32; 4]> for Vec4 {
    #[inline]
    fn from(value: &[f32; 4]) -> Self {
        Self(*value)
    }
}

impl From<Vec4> for (f32, f32, f32, f32) {
    #[inline]
    fn from(value: Vec4) -> Self {
        (value.0[0], value.0[1], value.0[2], value.0[3])
    }
}

impl From<(f32, f32, f32, f32)> for Vec4 {
    #[inline]
    fn from(value: (f32, f32, f32, f32)) -> Self {
        Self([value.0, value.1, value.2, value.3])
    }
}

impl From<&(f32, f32, f32, f32)> for Vec4 {
    #[inline]
    fn from(value: &(f32, f32, f32, f32)) -> Self {
        Self([value.0, value.1, value.2, value.3])
    }
}

impl TryFrom<&[f32]> for Vec4 {
    type Error = core::array::TryFromSliceError;
    #[inline]
    fn try_from(value: &[f32]) -> Result<Self, Self::Error> {
        Ok(Self(<_>::try_from(value)?))
    }
}

impl From<fm_vec4_t> for Vec4 {
    #[inline]
    fn from(value: fm_vec4_t) -> Self {
        unsafe { Self(value.v) }
    }
}

impl From<Vec4> for fm_vec4_t {
    #[inline]
    fn from(value: Vec4) -> Self {
        Self { v: value.0 }
    }
}

impl From<Vec3> for Vec4 {
    #[inline]
    fn from(value: Vec3) -> Self {
        Self::new(value.x(), value.y(), value.z(), 1.0)
    }
}

impl From<Vec4> for Vec3 {
    #[inline]
    fn from(value: Vec4) -> Self {
        Self::new(value.x(), value.y(), value.z())
    }
}

impl Deref for Vec4 {
    type Target = [f32; 4];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Quat {
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0);
    pub const IDENTITY: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self([x, y, z, w])
    }
    #[inline]
    pub fn from_euler(euler: &[f32; 3]) -> Self {
        let mut quat = core::mem::MaybeUninit::uninit();
        unsafe {
            fm_quat_from_euler(quat.as_mut_ptr(), euler.as_ptr());
            Self(quat.assume_init().v)
        }
    }
    #[inline]
    pub fn from_euler_zyx(x: f32, y: f32, z: f32) -> Self {
        let mut quat = core::mem::MaybeUninit::uninit();
        unsafe {
            fm_quat_from_euler_zyx(quat.as_mut_ptr(), x, y, z);
            Self(quat.assume_init().v)
        }
    }
    #[inline]
    pub fn from_axis_angle(axis: &Vec3, angle: f32) -> Self {
        let (s, c) = angle.sin_cos();
        Self::new(axis.x() * s, axis.y() * s, axis.z() * s, c)
    }
    #[inline]
    pub const fn as_ptr(&self) -> *const fm_quat_t {
        self as *const _ as _
    }
    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut fm_quat_t {
        self as *mut _ as _
    }
    #[inline]
    pub const fn x(&self) -> f32 {
        self.0[0]
    }
    #[inline]
    pub const fn y(&self) -> f32 {
        self.0[1]
    }
    #[inline]
    pub const fn z(&self) -> f32 {
        self.0[2]
    }
    #[inline]
    pub const fn w(&self) -> f32 {
        self.0[3]
    }
    #[inline]
    pub const fn set_x(&mut self, x: f32) {
        self.0[0] = x;
    }
    #[inline]
    pub const fn set_y(&mut self, y: f32) {
        self.0[1] = y;
    }
    #[inline]
    pub const fn set_z(&mut self, z: f32) {
        self.0[2] = z;
    }
    #[inline]
    pub const fn set_w(&mut self, w: f32) {
        self.0[3] = w;
    }
    #[inline]
    pub const fn neg(mut self) -> Self {
        self.0[0] = -self.0[0];
        self.0[1] = -self.0[1];
        self.0[2] = -self.0[2];
        self.0[3] = -self.0[3];
        self
    }
    #[inline]
    pub const fn add(mut self, rhs: Self) -> Self {
        self.0[0] = self.0[0] + rhs.0[0];
        self.0[1] = self.0[1] + rhs.0[1];
        self.0[2] = self.0[2] + rhs.0[2];
        self.0[3] = self.0[3] + rhs.0[3];
        self
    }
    #[inline]
    pub const fn sub(mut self, rhs: Self) -> Self {
        self.0[0] = self.0[0] - rhs.0[0];
        self.0[1] = self.0[1] - rhs.0[1];
        self.0[2] = self.0[2] - rhs.0[2];
        self.0[3] = self.0[3] - rhs.0[3];
        self
    }
    #[inline]
    pub fn multiply(&mut self, rhs: &Self) {
        unsafe { fm_quat_mul(self.as_mut_ptr(), self.as_ptr(), rhs.as_ptr()) };
    }
    #[inline]
    pub fn mul(self, rhs: Self) -> Self {
        let mut quat = core::mem::MaybeUninit::uninit();
        unsafe {
            fm_quat_mul(quat.as_mut_ptr(), self.as_ptr(), rhs.as_ptr());
            Self(quat.assume_init().v)
        }
    }
    #[inline]
    pub fn rotate(&mut self, axis: &Vec3, angle: f32) {
        unsafe { fm_quat_rotate(self.as_mut_ptr(), self.as_ptr(), axis.as_ptr(), angle) };
    }
    #[inline]
    pub fn rotated(self, axis: Vec3, angle: f32) -> Self {
        let mut quat = core::mem::MaybeUninit::uninit();
        unsafe {
            fm_quat_rotate(quat.as_mut_ptr(), self.as_ptr(), axis.as_ptr(), angle);
            Self(quat.assume_init().v)
        }
    }
    #[inline]
    pub const fn dot(&self, rhs: &Self) -> f32 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z() + self.w() * rhs.w()
    }
    #[inline]
    pub const fn inverse(&self) -> Self {
        let inv_mag2 = 1.0 / self.dot(self);
        Self::new(
            -self.x() * inv_mag2,
            -self.y() * inv_mag2,
            -self.z() * inv_mag2,
            self.w() * inv_mag2,
        )
    }
    #[inline]
    pub fn divide(&mut self, rhs: &Self) {
        self.multiply(&rhs.inverse());
    }
    #[inline]
    pub fn div(self, rhs: Self) -> Self {
        self.mul(rhs.inverse())
    }
    #[inline]
    pub fn normalize(self) -> Self {
        let len = self.dot(&self).sqrt();
        if len < FM_EPSILON as f32 {
            Self::ZERO
        } else {
            let invlen = 1.0 / len;
            Self::new(
                self.x() * invlen,
                self.y() * invlen,
                self.z() * invlen,
                self.w() * invlen,
            )
        }
    }
    #[inline]
    pub fn nlerp(&mut self, rhs: &Self, t: f32) {
        unsafe { fm_quat_nlerp(self.as_mut_ptr(), self.as_ptr(), rhs.as_ptr(), t) };
    }
    #[inline]
    pub fn nlerped(self, rhs: &Self, t: f32) -> Self {
        let mut quat = core::mem::MaybeUninit::uninit();
        unsafe {
            fm_quat_nlerp(quat.as_mut_ptr(), self.as_ptr(), rhs.as_ptr(), t);
            Self(quat.assume_init().v)
        }
    }
    #[inline]
    pub fn slerp(&mut self, rhs: &Self, t: f32) {
        unsafe { fm_quat_slerp(self.as_mut_ptr(), self.as_ptr(), rhs.as_ptr(), t) };
    }
    #[inline]
    pub fn slerped(self, rhs: &Self, t: f32) -> Self {
        let mut quat = core::mem::MaybeUninit::uninit();
        unsafe {
            fm_quat_slerp(quat.as_mut_ptr(), self.as_ptr(), rhs.as_ptr(), t);
            Self(quat.assume_init().v)
        }
    }
}

impl From<[f32; 4]> for Quat {
    #[inline]
    fn from(value: [f32; 4]) -> Self {
        Self(value)
    }
}

impl From<Quat> for [f32; 4] {
    #[inline]
    fn from(value: Quat) -> Self {
        value.0
    }
}

impl From<&[f32; 4]> for Quat {
    #[inline]
    fn from(value: &[f32; 4]) -> Self {
        Self(*value)
    }
}

impl From<Quat> for (f32, f32, f32, f32) {
    #[inline]
    fn from(value: Quat) -> Self {
        (value.0[0], value.0[1], value.0[2], value.0[3])
    }
}

impl From<(f32, f32, f32, f32)> for Quat {
    #[inline]
    fn from(value: (f32, f32, f32, f32)) -> Self {
        Self([value.0, value.1, value.2, value.3])
    }
}

impl From<&(f32, f32, f32, f32)> for Quat {
    #[inline]
    fn from(value: &(f32, f32, f32, f32)) -> Self {
        Self([value.0, value.1, value.2, value.3])
    }
}

impl TryFrom<&[f32]> for Quat {
    type Error = core::array::TryFromSliceError;
    #[inline]
    fn try_from(value: &[f32]) -> Result<Self, Self::Error> {
        Ok(Self(<_>::try_from(value)?))
    }
}

impl From<fm_quat_t> for Quat {
    #[inline]
    fn from(value: fm_quat_t) -> Self {
        unsafe { Self(value.v) }
    }
}

impl From<Quat> for fm_quat_t {
    #[inline]
    fn from(value: Quat) -> Self {
        Self { v: value.0 }
    }
}

impl From<Quat> for Vec4 {
    #[inline]
    fn from(value: Quat) -> Self {
        Self::new(value.x(), value.y(), value.z(), value.z())
    }
}

impl From<Vec4> for Quat {
    #[inline]
    fn from(value: Vec4) -> Self {
        Self::new(value.x(), value.y(), value.z(), value.z())
    }
}

impl Deref for Quat {
    type Target = [f32; 4];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Mat4 {
    pub const ZERO: Self = Self::new([[0.0; 4]; 4]);
    pub const IDENTITY: Self = Self::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);
    #[inline]
    pub const fn new(n: [[f32; 4]; 4]) -> Self {
        Self(n)
    }
    #[inline]
    pub fn from_axis_angle(axis: &Vec3, angle: f32) -> Self {
        let mut mat = core::mem::MaybeUninit::uninit();
        unsafe {
            fm_mat4_from_axis_angle(mat.as_mut_ptr(), axis.as_ptr(), angle);
            Self(mat.assume_init().m)
        }
    }
    #[inline]
    pub fn from_srt(scale: &Vec3, quat: &Quat, translate: &Vec3) -> Self {
        let mut mat = core::mem::MaybeUninit::uninit();
        unsafe {
            fm_mat4_from_srt(
                mat.as_mut_ptr(),
                scale.as_ptr(),
                quat.as_ptr(),
                translate.as_ptr(),
            );
            Self(mat.assume_init().m)
        }
    }
    #[inline]
    pub fn from_srt_euler(scale: &Vec3, euler: &[f32; 3], translate: &Vec3) -> Self {
        let mut mat = core::mem::MaybeUninit::uninit();
        unsafe {
            fm_mat4_from_srt_euler(
                mat.as_mut_ptr(),
                scale.as_ptr(),
                euler.as_ptr(),
                translate.as_ptr(),
            );
            Self(mat.assume_init().m)
        }
    }
    #[inline]
    pub fn from_rt(quat: &Quat, translate: &Vec3) -> Self {
        Self::from_srt(&Vec3::splat(1.0), quat, translate)
    }
    #[inline]
    pub fn from_rt_euler(euler: &[f32; 3], translate: &Vec3) -> Self {
        Self::from_srt_euler(&Vec3::splat(1.0), euler, translate)
    }
    #[inline]
    pub const fn from_translation(translate: Vec3) -> Self {
        let mut mat = Self::IDENTITY;
        mat.translate(&translate);
        mat
    }
    #[inline]
    pub const fn from_scale(scale: Vec3) -> Self {
        let mut mat = Self::IDENTITY;
        mat.scale(&scale);
        mat
    }
    #[inline]
    pub const fn as_ptr(&self) -> *const fm_mat4_t {
        self as *const _ as _
    }
    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut fm_mat4_t {
        self as *mut _ as _
    }
    #[inline]
    pub const fn scale(&mut self, scale: &Vec3) {
        self.0[0][0] *= scale.x();
        self.0[0][1] *= scale.x();
        self.0[0][2] *= scale.x();
        self.0[0][3] *= scale.x();
        self.0[1][0] *= scale.y();
        self.0[1][1] *= scale.y();
        self.0[1][2] *= scale.y();
        self.0[1][3] *= scale.y();
        self.0[2][0] *= scale.z();
        self.0[2][1] *= scale.z();
        self.0[2][2] *= scale.z();
        self.0[2][3] *= scale.z();
    }
    #[inline]
    pub const fn translate(&mut self, translate: &Vec3) {
        self.0[3][0] *= translate.x();
        self.0[3][1] *= translate.y();
        self.0[3][2] *= translate.z();
    }
    #[inline]
    pub fn rotate(&mut self, rotation: &Quat) {
        unsafe { fm_mat4_rotate(self.as_mut_ptr(), rotation.as_ptr()) }
    }
    #[inline]
    pub fn transpose(&self) -> Self {
        Self(core::array::from_fn(|i| {
            core::array::from_fn(|j| self[j][i])
        }))
    }
    #[inline]
    pub fn det(&self) -> f32 {
        unsafe { fm_mat4_det(self.as_ptr()) }
    }
    #[inline]
    pub fn inverse(&self) -> Self {
        let mut mat = core::mem::MaybeUninit::uninit();
        unsafe {
            fm_mat4_inverse(mat.as_mut_ptr(), self.as_ptr());
            Self(mat.assume_init().m)
        }
    }
    #[inline]
    pub fn invert(&mut self) {
        unsafe { fm_mat4_inverse(self.as_mut_ptr(), self.as_ptr()) }
    }
    #[inline]
    pub fn affine_to_normal(&self) -> Self {
        let mut mat = core::mem::MaybeUninit::uninit();
        unsafe {
            fm_mat4_affine_to_normal_mat(mat.as_mut_ptr(), self.as_ptr());
            Self(mat.assume_init().m)
        }
    }
    #[inline]
    pub fn look(&mut self, eye: &Vec3, dir: &Vec3, up: &Vec3) {
        unsafe { fm_mat4_look(self.as_mut_ptr(), eye.as_ptr(), dir.as_ptr(), up.as_ptr()) }
    }
    #[inline]
    pub fn look_at(&mut self, eye: &Vec3, dir: &Vec3, up: &Vec3) {
        unsafe { fm_mat4_lookat(self.as_mut_ptr(), eye.as_ptr(), dir.as_ptr(), up.as_ptr()) }
    }
    #[inline]
    pub fn mul3(&self, rhs: &Vec3) -> Vec3 {
        Vec3(core::array::from_fn(|i| {
            self[0][i] * rhs.x() + self[1][i] * rhs.y() + self[2][i] * rhs.z()
        }))
    }
}

impl From<fm_mat4_t> for Mat4 {
    #[inline]
    fn from(value: fm_mat4_t) -> Self {
        Self(value.m)
    }
}

impl From<Mat4> for fm_mat4_t {
    #[inline]
    fn from(value: Mat4) -> Self {
        Self { m: value.0 }
    }
}

impl Mul<&Mat4> for &Mat4 {
    type Output = Mat4;
    #[inline]
    fn mul(self, rhs: &Mat4) -> Mat4 {
        Mat4(core::array::from_fn(|i| {
            core::array::from_fn(|j| {
                self[0][j] * rhs[i][0]
                    + self[1][j] * rhs[i][1]
                    + self[2][j] * rhs[i][2]
                    + self[3][j] * rhs[i][3]
            })
        }))
    }
}
impl Mul<&Mat4> for Mat4 {
    type Output = Mat4;
    #[inline]
    fn mul(self, rhs: &Mat4) -> Mat4 {
        (&self).mul(rhs)
    }
}
impl Mul<Mat4> for &Mat4 {
    type Output = Mat4;
    #[inline]
    fn mul(self, rhs: Mat4) -> Mat4 {
        self.mul(&rhs)
    }
}
impl Mul<Mat4> for Mat4 {
    type Output = Mat4;
    #[inline]
    fn mul(self, rhs: Mat4) -> Mat4 {
        self.mul(&rhs)
    }
}
impl Mul<&Vec4> for &Mat4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: &Vec4) -> Vec4 {
        Vec4(core::array::from_fn(|i| {
            self[0][i] * rhs.x()
                + self[1][i] * rhs.y()
                + self[2][i] * rhs.z()
                + self[3][i] * rhs.w()
        }))
    }
}
impl Mul<&Vec4> for Mat4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: &Vec4) -> Vec4 {
        (&self).mul(rhs)
    }
}
impl Mul<Vec4> for &Mat4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: Vec4) -> Vec4 {
        self.mul(&rhs)
    }
}
impl Mul<Vec4> for Mat4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: Vec4) -> Vec4 {
        self.mul(&rhs)
    }
}
impl Mul<&Vec3> for &Mat4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: &Vec3) -> Vec4 {
        Vec4(core::array::from_fn(|i| {
            self[0][i] * rhs.x() + self[1][i] * rhs.y() + self[2][i] * rhs.z() + self[3][i]
        }))
    }
}
impl Mul<&Vec3> for Mat4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: &Vec3) -> Vec4 {
        (&self).mul(rhs)
    }
}
impl Mul<Vec3> for &Mat4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: Vec3) -> Vec4 {
        self.mul(&rhs)
    }
}
impl Mul<Vec3> for Mat4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: Vec3) -> Vec4 {
        self.mul(&rhs)
    }
}

impl Deref for Mat4 {
    type Target = [[f32; 4]; 4];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Mat4 {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

macro_rules! impl_vec_ops {
    ($ty:ty [$($elems:expr),+ $(,)?]) => {
impl DerefMut for $ty {
    #[inline] fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
impl Neg for $ty {
    type Output = $ty;
    #[inline] fn neg(self) -> $ty { self.neg() }
}

impl Add for $ty {
    type Output = $ty;
    #[inline] fn add(self, rhs: $ty) -> $ty { self.add(rhs) }
}
impl Sub for $ty {
    type Output = $ty;
    #[inline] fn sub(self, rhs: $ty) -> $ty { self.sub(rhs) }
}
impl Mul for $ty {
    type Output = $ty;
    #[inline] fn mul(self, rhs: $ty) -> $ty { self.mul(rhs) }
}
impl Div for $ty {
    type Output = $ty;
    #[inline] fn div(self, rhs: $ty) -> $ty { self.div(rhs) }
}

impl Add<f32> for $ty {
    type Output = $ty;
    #[inline] fn add(mut self, rhs: f32) -> $ty { $(self[$elems] = self[$elems] + rhs;)+ self }
}
impl Sub<f32> for $ty {
    type Output = $ty;
    #[inline] fn sub(mut self, rhs: f32) -> $ty { $(self[$elems] = self[$elems] - rhs;)+ self }
}
impl Mul<f32> for $ty {
    type Output = $ty;
    #[inline] fn mul(mut self, rhs: f32) -> $ty { $(self[$elems] = self[$elems] * rhs;)+ self }
}
impl Div<f32> for $ty {
    type Output = $ty;
    #[inline] fn div(mut self, rhs: f32) -> $ty { let m = 1.0 / rhs; $(self[$elems] = self[$elems] * m;)+ self }
}

impl Add<$ty> for f32 {
    type Output = $ty;
    #[inline] fn add(self, mut rhs: $ty) -> $ty { $(rhs[$elems] = self + rhs[$elems];)+ rhs }
}
impl Sub<$ty> for f32 {
    type Output = $ty;
    #[inline] fn sub(self, mut rhs: $ty) -> $ty { $(rhs[$elems] = self - rhs[$elems];)+ rhs }
}
impl Mul<$ty> for f32 {
    type Output = $ty;
    #[inline] fn mul(self, mut rhs: $ty) -> $ty { $(rhs[$elems] = self * rhs[$elems];)+ rhs }
}
impl Div<$ty> for f32 {
    type Output = $ty;
    #[inline] fn div(self, mut rhs: $ty) -> $ty { $(rhs[$elems] = self / rhs[$elems];)+ rhs }
}

impl Add<&$ty> for &$ty {
    type Output = $ty;
    #[inline] fn add(self, rhs: &$ty) -> $ty { (*self).add(*rhs) }
}
impl Sub<&$ty> for &$ty {
    type Output = $ty;
    #[inline] fn sub(self, rhs: &$ty) -> $ty { (*self).sub(*rhs) }
}
impl Mul<&$ty> for &$ty {
    type Output = $ty;
    #[inline] fn mul(self, rhs: &$ty) -> $ty { (*self).mul(*rhs) }
}
impl Div<&$ty> for &$ty {
    type Output = $ty;
    #[inline] fn div(self, rhs: &$ty) -> $ty { (*self).div(*rhs) }
}

impl Add<&$ty> for $ty {
    type Output = $ty;
    #[inline] fn add(self, rhs: &$ty) -> $ty { self.add(*rhs) }
}
impl Sub<&$ty> for $ty {
    type Output = $ty;
    #[inline] fn sub(self, rhs: &$ty) -> $ty { self.sub(*rhs) }
}
impl Mul<&$ty> for $ty {
    type Output = $ty;
    #[inline] fn mul(self, rhs: &$ty) -> $ty { self.mul(*rhs) }
}
impl Div<&$ty> for $ty {
    type Output = $ty;
    #[inline] fn div(self, rhs: &$ty) -> $ty { self.div(*rhs) }
}

impl Add<$ty> for &$ty {
    type Output = $ty;
    #[inline] fn add(self, rhs: $ty) -> $ty { (*self).add(rhs) }
}
impl Sub<$ty> for &$ty {
    type Output = $ty;
    #[inline] fn sub(self, rhs: $ty) -> $ty { (*self).sub(rhs) }
}
impl Mul<$ty> for &$ty {
    type Output = $ty;
    #[inline] fn mul(self, rhs: $ty) -> $ty { (*self).mul(rhs) }
}
impl Div<$ty> for &$ty {
    type Output = $ty;
    #[inline] fn div(self, rhs: $ty) -> $ty { (*self).div(rhs) }
}

impl Add<&$ty> for f32 {
    type Output = $ty;
    #[inline] fn add(self, rhs: &$ty) -> $ty { self.add(*rhs) }
}
impl Sub<&$ty> for f32 {
    type Output = $ty;
    #[inline] fn sub(self, rhs: &$ty) -> $ty { self.sub(*rhs) }
}
impl Mul<&$ty> for f32 {
    type Output = $ty;
    #[inline] fn mul(self, rhs: &$ty) -> $ty { self.mul(*rhs) }
}
impl Div<&$ty> for f32 {
    type Output = $ty;
    #[inline] fn div(self, rhs: &$ty) -> $ty { self.div(*rhs) }
}

impl Add<f32> for &$ty {
    type Output = $ty;
    #[inline] fn add(self, rhs: f32) -> $ty { *self + rhs }
}
impl Sub<f32> for &$ty {
    type Output = $ty;
    #[inline] fn sub(self, rhs: f32) -> $ty { *self - rhs }
}
impl Mul<f32> for &$ty {
    type Output = $ty;
    #[inline] fn mul(self, rhs: f32) -> $ty { *self * rhs }
}
impl Div<f32> for &$ty {
    type Output = $ty;
    #[inline] fn div(self, rhs: f32) -> $ty { *self / rhs }
}
    };
}

macro_rules! impl_const_vec_ops {
    ($ty:ty [$($elems:expr),+ $(,)?]) => {

impl $ty {
    #[inline]
    pub const fn neg(mut self) -> Self {
        $(self.0[$elems] = -self.0[$elems];)+
        self
    }
    #[inline]
    pub const fn add(mut self, rhs: Self) -> Self {
        $(self.0[$elems] = self.0[$elems] + rhs.0[$elems];)+
        self
    }
    #[inline]
    pub const fn sub(mut self, rhs: Self) -> Self {
        $(self.0[$elems] = self.0[$elems] - rhs.0[$elems];)+
        self
    }
    #[inline]
    pub const fn mul(mut self, rhs: Self) -> Self {
        $(self.0[$elems] = self.0[$elems] * rhs.0[$elems];)+
        self
    }
    #[inline]
    pub const fn div(mut self, rhs: Self) -> Self {
        $(self.0[$elems] = self.0[$elems] / rhs.0[$elems];)+
        self
    }
}
impl_vec_ops! { $ty [$($elems),+] }
    }
}

impl_const_vec_ops! { Vec3 [0, 1, 2] }
impl_const_vec_ops! { Vec4 [0, 1, 2, 3] }
impl_vec_ops! { Quat [0, 1, 2, 3] }
