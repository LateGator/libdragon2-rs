#![no_std]

use core::{
    cell::UnsafeCell,
    marker::PhantomData,
    ptr::NonNull,
    sync::atomic::{AtomicBool, Ordering},
};
use dragon::{
    fmath::{Mat4, Vec3},
    graphics::Color,
    rdpq::RdpQ,
};

use sys::*;

mod anim;
mod math;
mod model;
mod skeleton;
pub mod tpx;

pub use anim::*;
pub use math::*;
pub use model::*;
pub use skeleton::*;
pub use sys;
pub use tpx::*;

pub type VertPacked = T3DVertPacked;

extern crate alloc;

#[repr(transparent)]
#[derive(Debug)]
pub struct Tiny3D<'r, 'v, R>(PhantomData<(&'r mut R, &'v Viewport)>);

#[derive(Debug)]
#[repr(transparent)]
pub struct Viewport(UnsafeCell<T3DViewport>);

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(transparent)]
pub struct SegmentAddress(pub(crate) i32);

bitflags::bitflags! {
    pub struct DrawFlags: u32 {
        const DEPTH = T3DDrawFlags_T3D_FLAG_DEPTH;
        const TEXTURED = T3DDrawFlags_T3D_FLAG_TEXTURED;
        const SHADED = T3DDrawFlags_T3D_FLAG_SHADED;
        const CULL_FRONT = T3DDrawFlags_T3D_FLAG_CULL_FRONT;
        const CULL_BACK = T3DDrawFlags_T3D_FLAG_CULL_BACK;
        const NO_LIGHT = T3DDrawFlags_T3D_FLAG_NO_LIGHT;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum VertexFX {
    None,
    SphericalUV { w: i16, h: i16 },
    CelShadeColor,
    CelShadeAlpha,
    Outline { x: i16, y: i16 },
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Segment {
    _1 = 1,
    _2 = 2,
    _3 = 3,
    _4 = 4,
    _5 = 5,
    _6 = 6,
    SKELETON = 7,
}

static T3D_INIT: AtomicBool = AtomicBool::new(false);

#[inline]
pub fn init<'r, 'v, R: AsMut<RdpQ<'r>>>(_rdpq: R, matrix_stack_size: u32) -> Tiny3D<'r, 'v, R> {
    assert_eq!(T3D_INIT.load(Ordering::Relaxed), false);
    T3D_INIT.store(true, Ordering::Relaxed);
    unsafe {
        t3d_init(T3DInitParams {
            matrixStackSize: matrix_stack_size as _,
        });
    }
    Tiny3D(PhantomData)
}

impl<'r, 'v, R> Drop for Tiny3D<'r, 'v, R> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            t3d_destroy();
            T3D_INIT.store(false, Ordering::Relaxed);
        }
    }
}

#[inline]
pub fn pack_normal(normal: &Vec3) -> u16 {
    unsafe { t3d_vert_pack_normal(normal.as_ptr()) }
}

#[inline]
pub fn indexbuffer_convert(indexes: &mut [i16]) {
    unsafe { t3d_indexbuffer_convert(indexes.as_mut_ptr(), indexes.len() as _) };
}

const SEGMENT_SHIFT: u32 = 8 * 3 + 2;
const SEGOFFSET_MASK: i32 = (1 << SEGMENT_SHIFT) - 1;

#[inline]
pub const fn segment_placeholder(segment_id: Segment) -> SegmentAddress {
    SegmentAddress((segment_id as i32) << SEGMENT_SHIFT)
}

#[inline]
pub const fn segment_address<T>(segment_id: Segment, offset: u16) -> SegmentAddress {
    SegmentAddress(offset as i32 & SEGOFFSET_MASK | segment_placeholder(segment_id).0)
}

impl Segment {
    #[inline]
    pub const fn id(self) -> u8 {
        self as u8
    }
}

impl From<Segment> for u8 {
    #[inline]
    fn from(value: Segment) -> Self {
        value.id()
    }
}

impl TryFrom<u8> for Segment {
    type Error = core::num::TryFromIntError;
    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value >= 1 && value <= 7 {
            unsafe { core::mem::transmute(value) }
        } else {
            Err(core::num::NonZeroU8::try_from(0u8).unwrap_err())
        }
    }
}

impl SegmentAddress {
    #[inline]
    pub const fn offset(self, n: i16) -> Self {
        Self((self.0 & !SEGOFFSET_MASK) | (self.0.wrapping_add(n as i32) & SEGOFFSET_MASK))
    }
}

impl<'r, 'v, R: AsMut<RdpQ<'r>>> Tiny3D<'r, 'v, R> {
    #[inline]
    pub fn viewport_attach(&mut self, vp: &'v Viewport) {
        unsafe { t3d_viewport_attach(&vp as *const _ as *mut _) };
    }
    #[inline]
    pub fn frame_start(&mut self) {
        unsafe { t3d_frame_start() };
    }
    #[inline]
    pub fn screen_clear_color(&mut self, color: dragon::graphics::Color) {
        unsafe { t3d_screen_clear_color(color.into_raw()) };
    }
    #[inline]
    pub fn screen_clear_depth(&mut self) {
        unsafe { t3d_screen_clear_depth() };
    }
    #[inline]
    pub fn viewport(&self) -> Option<&'v Viewport> {
        unsafe {
            core::ptr::NonNull::new(t3d_viewport_get()).map(|v| core::mem::transmute(&*v.as_ptr()))
        }
    }
    #[inline]
    pub fn tri_draw(&mut self, v0: u32, v1: u32, v2: u32) {
        unsafe { t3d_tri_draw(v0, v1, v2) };
    }
    #[inline]
    pub fn tri_draw_unindexed(&mut self, base_index: u32, tri_count: u32) {
        unsafe { t3d_tri_draw_unindexed(base_index, tri_count) };
    }
    #[inline]
    pub fn tri_quad_draw_unindexed(&mut self, base_index: u32, quad_count: u32) {
        unsafe { t3d_quad_draw_unindexed(base_index, quad_count) };
    }
    #[inline]
    pub fn tri_draw_strip(&mut self, index_buff: &[i16]) {
        unsafe { t3d_tri_draw_strip(index_buff.as_ptr() as _, index_buff.len() as _) };
    }
    #[inline]
    pub fn tri_draw_strip_and_sync(&mut self, index_buff: &[i16]) {
        unsafe { t3d_tri_draw_strip_and_sync(index_buff.as_ptr() as _, index_buff.len() as _) };
    }
    #[inline]
    pub fn tri_sync(&mut self) {
        unsafe { dragon::rdpq::get().write(T3D_RSP_ID, T3DCmd_T3D_CMD_TRI_SYNC, [0]) };
    }
    #[inline]
    pub fn matrix_set(&mut self, mat: &math::Mat4FP, do_multiply: bool) {
        unsafe { t3d_matrix_set(mat.as_ptr(), do_multiply) };
    }
    #[inline]
    pub fn matrix_push(&mut self, mat: &math::Mat4FP) {
        unsafe { t3d_matrix_push(mat.as_ptr()) };
    }
    #[inline]
    pub unsafe fn matrix_set_segmented(&mut self, mat: SegmentAddress, do_multiply: bool) {
        unsafe { t3d_matrix_set(mat.0 as isize as _, do_multiply) };
    }
    #[inline]
    pub unsafe fn matrix_push_segmented(&mut self, mat: SegmentAddress) {
        unsafe { t3d_matrix_push(mat.0 as isize as _) };
    }
    #[inline]
    pub fn matrix_pop(&mut self, count: i32) {
        unsafe { t3d_matrix_pop(count) };
    }
    #[inline]
    pub fn matrix_push_pos(&mut self, count: i32) {
        unsafe { t3d_matrix_push_pos(count) };
    }
    #[inline]
    pub fn matrix_set_proj(&mut self, mat: &math::Mat4FP) {
        unsafe { t3d_matrix_set_proj(mat.as_ptr()) };
    }
    #[inline]
    pub fn vert_load<B: VertBufferExt>(&mut self, vertices: B, offset: u32) {
        let count = vertices.vert_count();
        if count == 0 {
            return;
        }
        debug_assert!(offset as usize + count <= T3D_VERTEX_CACHE_SIZE as usize);
        unsafe { t3d_vert_load(vertices.as_ptr(), offset, count as _) };
    }
    #[inline]
    pub unsafe fn vert_load_segmented(
        &mut self,
        vertices: SegmentAddress,
        offset: u32,
        count: u32,
    ) {
        unsafe { t3d_vert_load(vertices.0 as isize as _, offset, count) };
    }
    #[inline]
    pub fn light_set_ambient(&mut self, color: &[u8; 4]) {
        unsafe { t3d_light_set_ambient(color.as_ptr()) };
    }
    #[inline]
    pub fn light_set_directional(&mut self, index: i32, color: &[u8; 4], dir: &Vec3) {
        unsafe { t3d_light_set_directional(index, color.as_ptr(), dir.as_ptr()) };
    }
    #[inline]
    pub fn light_set_point(
        &mut self,
        index: i32,
        color: &[u8; 4],
        pos: &Vec3,
        size: f32,
        ignore_normals: bool,
    ) {
        unsafe { t3d_light_set_point(index, color.as_ptr(), pos.as_ptr(), size, ignore_normals) };
    }
    #[inline]
    pub fn light_set_count(&mut self, count: i32) {
        unsafe { t3d_light_set_count(count) };
    }
    #[inline]
    pub fn light_set_exposure(&mut self, exposure: f32) {
        unsafe { t3d_light_set_exposure(exposure) };
    }
    #[inline]
    pub fn fog_set_range(&mut self, near: f32, far: f32) {
        unsafe { t3d_fog_set_range(near, far) };
    }
    #[inline]
    pub fn fog_set_enabled(&mut self, enabled: bool) {
        unsafe {
            dragon::rdpq::get().write(
                T3D_RSP_ID,
                T3DCmd_T3D_CMD_FOG_STATE,
                [if enabled { 0x0B } else { 0x0C }],
            )
        };
    }
    #[inline]
    pub fn state_set_drawflags(&mut self, draw_flags: DrawFlags) {
        unsafe { t3d_state_set_drawflags(draw_flags.bits()) }
    }
    #[inline]
    pub fn state_set_depth_offset(&mut self, offset: i16) {
        unsafe { t3d_state_set_depth_offset(offset) }
    }
    #[inline]
    pub fn state_set_alpha_to_tile(&mut self, enable: bool) {
        unsafe { t3d_state_set_alpha_to_tile(enable) }
    }
    #[inline]
    pub fn state_set_vertex_fx(&mut self, func: VertexFX) {
        let (func, arg0, arg1) = match func {
            VertexFX::None => (T3DVertexFX_T3D_VERTEX_FX_NONE, 0, 0),
            VertexFX::SphericalUV { w, h } => (T3DVertexFX_T3D_VERTEX_FX_SPHERICAL_UV, w, h),
            VertexFX::CelShadeColor => (T3DVertexFX_T3D_VERTEX_FX_CELSHADE_COLOR, 0, 0),
            VertexFX::CelShadeAlpha => (T3DVertexFX_T3D_VERTEX_FX_CELSHADE_ALPHA, 0, 0),
            VertexFX::Outline { x, y } => (T3DVertexFX_T3D_VERTEX_FX_OUTLINE, x, y),
        };
        unsafe { t3d_state_set_vertex_fx(func, arg0, arg1) }
    }
    #[inline]
    pub fn state_set_vertex_fx_scale(&mut self, scale: u16) {
        unsafe { t3d_state_set_vertex_fx_scale(scale) }
    }
    #[inline]
    pub fn segment_set<T>(&mut self, segment_id: Segment, address: NonNull<T>) {
        unsafe { t3d_segment_set(segment_id as _, address.as_ptr() as _) }
    }
    #[inline]
    pub fn model_draw_custom<T>(&mut self, model: &Model, conf: ModelDrawConf<T>) {
        unsafe { sys::t3d_model_draw_custom(model.0.as_ptr() as _, conf.0) };
    }
    #[inline]
    pub fn model_draw(&mut self, model: &Model) {
        unsafe {
            sys::t3d_model_draw_custom(model.0.as_ptr() as _, sys::T3DModelDrawConf::default())
        };
    }
    #[inline]
    pub unsafe fn model_draw_object(&mut self, object: &Object, bone_matrices: &[Mat4FP]) {
        unsafe { sys::t3d_model_draw_object(object.as_ptr(), bone_matrices.as_ptr() as _) };
    }
    #[inline]
    pub fn model_draw_material(&mut self, material: &Material, state: Option<&ModelState>) {
        unsafe {
            sys::t3d_model_draw_material(
                &material.0 as *const _ as *mut _,
                state
                    .map(|p| p as *const _ as *mut _)
                    .unwrap_or_else(|| core::ptr::null_mut()),
            )
        };
    }
    #[inline]
    pub fn model_draw_skinned(&mut self, model: &Model, skeleton: &Skeleton) {
        let mut conf = sys::T3DModelDrawConf::default();
        let skel = skeleton.0.get();
        conf.matrices = if unsafe { (*skel).bufferCount == 1 } {
            unsafe { (*skel).boneMatricesFP }
        } else {
            segment_placeholder(Segment::SKELETON).0 as isize as _
        };
        unsafe { sys::t3d_model_draw_custom(model.0.as_ptr() as _, conf) };
    }
    #[inline]
    pub fn skeleton_use(&mut self, skeleton: &Skeleton) {
        let skel = skeleton.0.get();
        if unsafe { (*skel).bufferCount > 1 } {
            let mat = unsafe {
                NonNull::new_unchecked((*skel).boneMatricesFP.add(
                    (*skel).currentBufferIdx as usize * (*(*skel).skeletonRef).boneCount as usize,
                ))
            };
            self.segment_set(Segment::SKELETON, mat);
        }
    }
}

pub unsafe trait VertBufferExt: sealed::Sealed {
    fn vert_count(&self) -> usize;
    fn as_ptr(&self) -> *const crate::VertPacked;
    fn pos(&self, idx: usize) -> [i16; 3];
    fn uv(&self, idx: usize) -> [i16; 2];
    fn color(&self, idx: usize) -> Color;
    fn norm(&self, idx: usize) -> u16;
    fn pos_mut(&mut self, idx: usize) -> &mut [i16; 3];
    fn uv_mut(&mut self, idx: usize) -> &mut [i16; 2];
    fn color_mut(&mut self, idx: usize) -> &mut Color;
    fn norm_mut(&mut self, idx: usize) -> &mut u16;
}

unsafe impl VertBufferExt for [VertPacked] {
    #[inline]
    fn as_ptr(&self) -> *const crate::VertPacked {
        self.as_ptr()
    }
    #[inline]
    fn vert_count(&self) -> usize {
        self.len() << 1
    }
    #[inline]
    fn pos(&self, idx: usize) -> [i16; 3] {
        match (idx & 1) != 0 {
            false => self[idx >> 1].posA,
            true => self[idx >> 1].posB,
        }
    }
    #[inline]
    fn uv(&self, idx: usize) -> [i16; 2] {
        match (idx & 1) != 0 {
            false => self[idx >> 1].stA,
            true => self[idx >> 1].stB,
        }
    }
    #[inline]
    fn color(&self, idx: usize) -> Color {
        match (idx & 1) != 0 {
            false => Color::from_u32(self[idx >> 1].rgbaA),
            true => Color::from_u32(self[idx >> 1].rgbaB),
        }
    }
    #[inline]
    fn norm(&self, idx: usize) -> u16 {
        match (idx & 1) != 0 {
            false => self[idx >> 1].normA,
            true => self[idx >> 1].normB,
        }
    }
    #[inline]
    fn pos_mut(&mut self, idx: usize) -> &mut [i16; 3] {
        match (idx & 1) != 0 {
            false => &mut self[idx >> 1].posA,
            true => &mut self[idx >> 1].posB,
        }
    }
    #[inline]
    fn uv_mut(&mut self, idx: usize) -> &mut [i16; 2] {
        match (idx & 1) != 0 {
            false => &mut self[idx >> 1].stA,
            true => &mut self[idx >> 1].stB,
        }
    }
    #[inline]
    fn color_mut(&mut self, idx: usize) -> &mut Color {
        unsafe {
            core::mem::transmute(match (idx & 1) != 0 {
                false => &mut self[idx >> 1].rgbaA,
                true => &mut self[idx >> 1].rgbaB,
            })
        }
    }
    #[inline]
    fn norm_mut(&mut self, idx: usize) -> &mut u16 {
        match (idx & 1) != 0 {
            false => &mut self[idx >> 1].normA,
            true => &mut self[idx >> 1].normB,
        }
    }
}

impl Clone for Viewport {
    #[inline]
    fn clone(&self) -> Self {
        Self(UnsafeCell::new(unsafe { *self.0.get() }))
    }
}

impl Viewport {
    #[inline]
    pub const fn with_area(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self(UnsafeCell::new(T3DViewport {
            _isCamProjDirty: true,
            size: [width, height],
            guardBandScale: 2,
            _matCameraFP: T3DMat4FP {
                m: [T3DVec4FP {
                    i: [0; 4],
                    f: [0; 4],
                }; 4],
            },
            _matProjFP: T3DMat4FP {
                m: [T3DVec4FP {
                    i: [0; 4],
                    f: [0; 4],
                }; 4],
            },
            matCamera: T3DMat4 { m: [[0.0; 4]; 4] },
            matProj: T3DMat4 { m: [[0.0; 4]; 4] },
            matCamProj: T3DMat4 { m: [[0.0; 4]; 4] },
            viewFrustum: T3DFrustum {
                planes: [T3DVec4 { v: [0.0; 4] }; 6],
            },
            offset: [x, y],
            useRejection: 0,
            _normScaleW: 0.0,
        }))
    }
    #[inline]
    pub const fn new(width: i32, height: i32) -> Self {
        Self::with_area(0, 0, width, height)
    }
    #[inline]
    pub fn for_display(display: &dragon::display::Display) -> Self {
        Self::new(display.width() as _, display.height() as _)
    }
    #[inline]
    pub const fn set_area(&self, x: i32, y: i32, width: i32, height: i32) {
        let vp = self.0.get();
        unsafe {
            (*vp).offset[0] = x;
            (*vp).offset[1] = y;
            (*vp).size[0] = width;
            (*vp).size[1] = height;
        }
    }
    #[inline]
    pub fn set_perspective(&self, fov: f32, aspect_ratio: f32, near: f32, far: f32) {
        unsafe {
            t3d_viewport_set_perspective(
                &self.0 as *const _ as *mut _,
                fov,
                aspect_ratio,
                near,
                far,
            )
        }
    }
    #[inline]
    pub fn set_projection(&self, fov: f32, near: f32, far: f32) {
        unsafe { t3d_viewport_set_projection(&self.0 as *const _ as *mut _, fov, near, far) }
    }
    #[inline]
    pub fn set_ortho(&self, left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) {
        unsafe {
            t3d_viewport_set_ortho(
                &self.0 as *const _ as *mut _,
                left,
                right,
                bottom,
                top,
                near,
                far,
            )
        }
    }
    #[inline]
    pub const fn set_w_normalize(&self, near: f32, far: f32) {
        unsafe { (*self.0.get())._normScaleW = 2.0 / (far + near) };
    }
    #[inline]
    pub fn look_at(&self, eye: &Vec3, target: &Vec3, up: &Vec3) {
        unsafe {
            t3d_viewport_look_at(
                &self.0 as *const _ as *mut _,
                eye.as_ptr(),
                target.as_ptr(),
                up.as_ptr(),
            )
        }
    }
    #[inline]
    pub fn set_view_matrix(&self, mat: &Mat4) {
        unsafe { t3d_viewport_set_view_matrix(&self.0 as *const _ as *mut _, mat.as_ptr()) }
    }
    #[inline]
    pub fn calc_viewspace_pos(&self, pos: &Vec3) -> Vec3 {
        let mut out = core::mem::MaybeUninit::uninit();
        unsafe {
            t3d_viewport_calc_viewspace_pos(
                &self.0 as *const _ as *mut _,
                out.as_mut_ptr(),
                pos.as_ptr(),
            );
            out.assume_init().into()
        }
    }
}

impl<'r, 'v, R: AsRef<RdpQ<'r>>> ::core::ops::Deref for Tiny3D<'r, 'v, R> {
    type Target = RdpQ<'r>;
    #[inline]
    fn deref(&self) -> &RdpQ<'r> {
        unsafe { core::mem::transmute(&dragon::rdpq::get()) }
    }
}
impl<'r, 'v, R: AsRef<RdpQ<'r>>> ::core::borrow::Borrow<RdpQ<'r>> for Tiny3D<'r, 'v, R> {
    #[inline]
    fn borrow(&self) -> &RdpQ<'r> {
        unsafe { core::mem::transmute(&dragon::rdpq::get()) }
    }
}
impl<'r, 'v, R: AsRef<RdpQ<'r>>> ::core::convert::AsRef<RdpQ<'r>> for Tiny3D<'r, 'v, R> {
    #[inline]
    fn as_ref(&self) -> &RdpQ<'r> {
        unsafe { core::mem::transmute(&dragon::rdpq::get()) }
    }
}
impl<'r, 'v, R: AsRef<RdpQ<'r>> + AsMut<RdpQ<'r>>> ::core::ops::DerefMut for Tiny3D<'r, 'v, R> {
    #[inline]
    fn deref_mut(&mut self) -> &mut RdpQ<'r> {
        unsafe { core::mem::transmute(&mut dragon::rdpq::get()) }
    }
}
impl<'r, 'v, R: AsRef<RdpQ<'r>> + AsMut<RdpQ<'r>>> ::core::borrow::BorrowMut<RdpQ<'r>>
    for Tiny3D<'r, 'v, R>
{
    #[inline]
    fn borrow_mut(&mut self) -> &mut RdpQ<'r> {
        unsafe { core::mem::transmute(&mut dragon::rdpq::get()) }
    }
}
impl<'r, 'v, R: AsMut<RdpQ<'r>>> ::core::convert::AsMut<RdpQ<'r>> for Tiny3D<'r, 'v, R> {
    #[inline]
    fn as_mut(&mut self) -> &mut RdpQ<'r> {
        unsafe { core::mem::transmute(&mut dragon::rdpq::get()) }
    }
}

mod sealed {
    pub trait Sealed {}
    impl Sealed for [crate::VertPacked] {}
    impl Sealed for [crate::Particle] {}
    impl<'m> Sealed for crate::VertBuffer<'m> {}

    pub trait ChunkType {
        const TYPE: i8;
    }
    impl ChunkType for crate::Material {
        const TYPE: i8 = sys::T3DModelChunkType_T3D_CHUNK_TYPE_MATERIAL as _;
    }
    impl ChunkType for crate::Object {
        const TYPE: i8 = sys::T3DModelChunkType_T3D_CHUNK_TYPE_OBJECT as _;
    }
    impl ChunkType for crate::ChunkSkeleton {
        const TYPE: i8 = sys::T3DModelChunkType_T3D_CHUNK_TYPE_SKELETON as _;
    }
    impl ChunkType for crate::ChunkAnim {
        const TYPE: i8 = sys::T3DModelChunkType_T3D_CHUNK_TYPE_ANIM as _;
    }
}
