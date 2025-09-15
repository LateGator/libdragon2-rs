use core::{cell::UnsafeCell, marker::PhantomData, ptr::NonNull};

use dragon::{
    fmath::{Quat, Vec3},
    graphics::Color,
    rspq::BlockRef,
};

#[derive(Debug)]
#[repr(transparent)]
pub struct Model(pub(crate) NonNull<sys::T3DModel>);

#[repr(transparent)]
#[derive(Debug)]
pub struct ModelState(pub(crate) sys::T3DModelState);

#[repr(transparent)]
pub struct ModelDrawConf<T>(pub(crate) sys::T3DModelDrawConf, PhantomData<T>);

#[derive(Debug)]
#[repr(transparent)]
pub struct Object(pub(crate) UnsafeCell<sys::T3DObject>);

#[derive(Debug)]
#[repr(transparent)]
pub struct ObjectPart(pub(crate) UnsafeCell<sys::T3DObjectPart>);

#[derive(Debug)]
#[repr(transparent)]
pub struct Material(pub(crate) UnsafeCell<sys::T3DMaterial>);

#[derive(Debug)]
#[repr(transparent)]
pub struct ChunkSkeleton(pub(crate) UnsafeCell<sys::T3DChunkSkeleton>);

#[derive(Debug)]
#[repr(transparent)]
pub struct ChunkBone(pub(crate) UnsafeCell<sys::T3DChunkBone>);

#[derive(Debug)]
#[repr(transparent)]
pub struct ChunkAnim(pub(crate) UnsafeCell<sys::T3DChunkAnim>);

#[derive(Debug)]
#[repr(transparent)]
pub struct AnimChannelMapping(pub(crate) UnsafeCell<sys::T3DAnimChannelMapping>);

#[derive(Debug)]
#[repr(transparent)]
pub struct Bvh(pub(crate) UnsafeCell<sys::T3DBvh>);

#[derive(Debug)]
#[repr(transparent)]
pub struct BvhNode(pub(crate) UnsafeCell<sys::T3DBvhNode>);

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct ModelIter<'m, T>(
    pub(crate) sys::T3DModelIter,
    PhantomData<(&'m Model, &'m T)>,
);

#[derive(Debug, Copy, Clone)]
pub struct VertBuffer<'m>(
    pub(crate) NonNull<crate::VertPacked>,
    pub(crate) usize,
    PhantomData<&'m Model>,
);

pub trait Chunk: crate::sealed::ChunkType {}
impl Chunk for Material {}
impl Chunk for Object {}
impl Chunk for ChunkSkeleton {}
impl Chunk for ChunkAnim {}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(u8)]
pub enum VertexFXFunc {
    None = sys::T3DVertexFX_T3D_VERTEX_FX_NONE as _,
    SphericalUV = sys::T3DVertexFX_T3D_VERTEX_FX_SPHERICAL_UV as _,
    CelShadeColor = sys::T3DVertexFX_T3D_VERTEX_FX_CELSHADE_COLOR as _,
    CelShadeAlpha = sys::T3DVertexFX_T3D_VERTEX_FX_CELSHADE_ALPHA as _,
    Outline = sys::T3DVertexFX_T3D_VERTEX_FX_OUTLINE as _,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(u8)]
pub enum FogMode {
    Default = sys::T3D_FOG_MODE_DEFAULT as _,
    Disabled = sys::T3D_FOG_MODE_DISABLED as _,
    Active = sys::T3D_FOG_MODE_ACTIVE as _,
}

pub type ModelTileFunc<T> =
    extern "C" fn(user_data: T, tile_params: &mut dragon::rdpq::TexParms, tile: dragon::rdpq::Tile);
pub type ModelFilterFunc<T> = extern "C" fn(user_data: T, obj: &Object);
pub type ModelDynTextureFunc<T> = extern "C" fn(
    user_data: T,
    material: &Material,
    tile_params: &mut dragon::rdpq::TexParms,
    tile: dragon::rdpq::Tile,
);

impl Model {
    #[inline]
    pub unsafe fn load(path: &core::ffi::CStr) -> Self {
        Self(unsafe { NonNull::new_unchecked(sys::t3d_model_load(path.as_ptr())) })
    }
    #[inline]
    pub const fn as_ptr(&self) -> *mut sys::T3DModel {
        self.0.as_ptr()
    }
    #[inline]
    pub const fn rspq_block(&self) -> Option<BlockRef<'_>> {
        unsafe { BlockRef::from_raw((*self.0.as_ptr()).userBlock) }
    }
    #[inline]
    pub fn set_rspq_block(&mut self, block: dragon::rspq::Block) {
        let model = self.0.as_ptr();
        unsafe {
            let block = core::mem::ManuallyDrop::new(block).as_raw();
            let block = core::mem::replace(&mut (*model).userBlock, block);
            if !block.is_null() {
                dragon::sys::rspq::rspq_block_free(block);
            }
        }
    }
    #[inline]
    pub fn vertices(&self) -> VertBuffer<'_> {
        let model = self.0.as_ptr();
        unsafe {
            let offset = (*(*model)
                .chunkOffsets
                .as_ptr()
                .add((*model).chunkIdxVertices as usize))
            .offset
                & 0x00FFFFFF;
            let verts = self.as_ptr().cast::<u8>().add(offset as usize);
            let count = ((*model).totalVertCount >> 1) as usize;
            VertBuffer(NonNull::new_unchecked(verts.cast()), count, PhantomData)
        }
    }
    #[inline]
    pub fn skeleton(&self) -> Option<&ChunkSkeleton> {
        let model = self.0.as_ptr();
        unsafe {
            for i in 0..(*model).chunkCount as usize {
                let chunk = (*model).chunkOffsets.as_ptr().add(i);
                if (*chunk).type_ == sys::T3DModelChunkType_T3D_CHUNK_TYPE_SKELETON as _ {
                    let offset = (*chunk).offset & 0x00FFFFFF;
                    return Some(&*self.as_ptr().cast::<u8>().add(offset as usize).cast());
                }
            }
        }
        None
    }
    #[inline]
    pub fn animation_count(&self) -> u32 {
        let model = self.0.as_ptr();
        let mut count = 0;
        unsafe {
            for i in 0..(*model).chunkCount as usize {
                let chunk = (*model).chunkOffsets.as_ptr().add(i);
                if (*chunk).type_ == sys::T3DModelChunkType_T3D_CHUNK_TYPE_ANIM as _ {
                    count += 1;
                }
            }
        }
        count
    }
    pub fn animations(&self) -> alloc::vec::Vec<&ChunkAnim> {
        let count = self.animation_count();
        let mut vec = alloc::vec::Vec::<&ChunkAnim>::with_capacity(count as usize);
        unsafe {
            sys::t3d_model_get_animations(self.as_ptr().cast_const(), vec.as_mut_ptr().cast());
            vec.set_len(count as usize);
        }
        vec
    }
    #[inline]
    pub fn animation(&self, name: &core::ffi::CStr) -> Option<&ChunkAnim> {
        unsafe {
            sys::t3d_model_get_animation(self.0.as_ptr() as _, name.as_ptr())
                .cast::<ChunkAnim>()
                .as_ref()
        }
    }
    #[inline]
    pub fn object(&self, name: &core::ffi::CStr) -> Option<&Object> {
        unsafe {
            sys::t3d_model_get_object(self.0.as_ptr() as _, name.as_ptr())
                .cast::<Object>()
                .as_ref()
        }
    }
    #[inline]
    pub fn object_by_index(&self, index: u32) -> Option<&Object> {
        let model = self.0.as_ptr();
        unsafe {
            if index >= (*model).chunkCount {
                return None;
            }
            let chunk = (*model).chunkOffsets.as_ptr().add(index as usize);
            if (*chunk).type_ != sys::T3DModelChunkType_T3D_CHUNK_TYPE_OBJECT as _ {
                return None;
            }
            let offset = (*chunk).offset & 0x00FFFFFF;
            Some(
                &*self
                    .as_ptr()
                    .cast::<u8>()
                    .add(offset as usize)
                    .cast_const()
                    .cast(),
            )
        }
    }
    #[inline]
    pub fn material(&self, name: &core::ffi::CStr) -> Option<&Material> {
        unsafe {
            sys::t3d_model_get_material(self.0.as_ptr() as _, name.as_ptr())
                .cast::<Material>()
                .as_ref()
        }
    }
    #[inline]
    pub const fn iter<T: Chunk>(&self) -> ModelIter<'_, T> {
        ModelIter(
            sys::T3DModelIter {
                __bindgen_anon_1: sys::T3DModelIter__bindgen_ty_1 {
                    chunk: core::ptr::null_mut(),
                },
                _model: self.as_ptr().cast_const(),
                _idx: 0,
                _chunkType: T::TYPE,
            },
            PhantomData,
        )
    }
    #[inline]
    pub fn bvh(&self) -> Option<&Bvh> {
        let model = self.0.as_ptr();
        unsafe {
            for i in 0..(*model).chunkCount as usize {
                let chunk = (*model).chunkOffsets.as_ptr().add(i);
                if (*chunk).type_ == sys::T3DModelChunkType_T3D_CHUNK_TYPE_BVH as _ {
                    let offset = (*chunk).offset & 0x00FFFFFF;
                    return Some(&*self.as_ptr().cast::<u8>().add(offset as usize).cast());
                }
            }
        }
        None
    }
}

impl Drop for Model {
    #[inline]
    fn drop(&mut self) {
        unsafe { sys::t3d_model_free(self.0.as_ptr()) }
    }
}

impl<'m, T: Chunk> Iterator for ModelIter<'m, T> {
    type Item = &'m T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if !sys::t3d_model_iter_next(&mut self.0) {
                return None;
            }
            Some(&*self.0.__bindgen_anon_1.chunk.cast_const().cast::<T>())
        }
    }
}

impl ModelDrawConf<()> {
    #[inline]
    pub const fn empty() -> Self {
        Self::new()
    }
}

impl<T> ModelDrawConf<T> {
    #[inline]
    pub const fn new() -> Self {
        Self(
            sys::T3DModelDrawConf {
                userData: core::ptr::null_mut(),
                tileCb: None,
                filterCb: None,
                dynTextureCb: None,
                matrices: core::ptr::null(),
            },
            PhantomData,
        )
    }
}

impl<T: dragon::n64::InterruptArg> ModelDrawConf<T> {
    #[inline]
    pub unsafe fn user_data(mut self, user_data: T) -> Self {
        self.0.userData = unsafe { user_data.into_ptr() };
        self
    }
    #[inline]
    pub const unsafe fn tile_func(mut self, func: ModelTileFunc<T>) -> Self {
        self.0.tileCb = unsafe { core::mem::transmute(func) };
        self
    }
    #[inline]
    pub const unsafe fn filter_func(mut self, func: ModelFilterFunc<T>) -> Self {
        self.0.filterCb = unsafe { core::mem::transmute(func) };
        self
    }
    #[inline]
    pub const unsafe fn dyn_texture_func(mut self, func: ModelDynTextureFunc<T>) -> Self {
        self.0.dynTextureCb = unsafe { core::mem::transmute(func) };
        self
    }
}

impl<T> Default for ModelDrawConf<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> core::fmt::Debug for ModelDrawConf<T> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_tuple("ModelDrawConf").field(&self.0).finish()
    }
}

impl ModelState {
    #[inline]
    pub const fn new() -> Self {
        use dragon::sys::graphics::color_t;
        Self(sys::T3DModelState {
            lastTextureHashA: 0,
            lastTextureHashB: 0,
            lastFogMode: 0xFF,
            lastRenderFlags: 0,
            lastCC: 0,
            lastPrimColor: color_t {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            },
            lastEnvColor: color_t {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            },
            lastBlendColor: color_t {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            },
            lastVertFXFunc: sys::T3DVertexFX_T3D_VERTEX_FX_NONE as _,
            lastUvGenParams: [0; 2],
            lastOtherMode: 0xF,
            lastBlendMode: 0xFFFFFFFF,
            drawConf: core::ptr::null_mut(),
        })
    }
}

impl Default for ModelState {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Material {
    #[inline]
    pub const fn as_ptr(&self) -> *mut sys::T3DMaterial {
        self.0.get()
    }
    #[inline]
    pub const fn name(&self) -> &core::ffi::CStr {
        unsafe { core::ffi::CStr::from_ptr((*self.0.get()).name) }
    }
    #[inline]
    pub const fn fog_mode(&self) -> FogMode {
        unsafe { core::mem::transmute((*self.0.get()).fogMode) }
    }
    #[inline]
    pub const fn vertex_fx_func(&self) -> VertexFXFunc {
        unsafe { core::mem::transmute((*self.0.get()).vertexFxFunc) }
    }
    #[inline]
    pub const fn prim_color(&self) -> Color {
        Color::from_raw(unsafe { (*self.0.get()).primColor })
    }
    #[inline]
    pub const fn env_color(&self) -> Color {
        Color::from_raw(unsafe { (*self.0.get()).envColor })
    }
    #[inline]
    pub const fn blend_color(&self) -> Color {
        Color::from_raw(unsafe { (*self.0.get()).blendColor })
    }
    #[inline]
    pub const fn set_fog_mode(&self, fog_mode: FogMode) {
        unsafe { (*self.0.get()).fogMode = fog_mode as _ };
    }
    #[inline]
    pub const fn set_vertex_fx_func(&self, func: VertexFXFunc) {
        unsafe { (*self.0.get()).vertexFxFunc = func as _ };
    }
    #[inline]
    pub const fn set_prim_color(&self, color: Color) {
        unsafe { (*self.0.get()).primColor = color.into_raw() };
    }
    #[inline]
    pub const fn set_env_color(&self, color: Color) {
        unsafe { (*self.0.get()).envColor = color.into_raw() };
    }
    #[inline]
    pub const fn set_blend_color(&self, color: Color) {
        unsafe { (*self.0.get()).blendColor = color.into_raw() };
    }
}

impl Object {
    #[inline]
    pub const fn as_ptr(&self) -> *mut sys::T3DObject {
        self.0.get()
    }
    #[inline]
    pub const fn name(&self) -> &core::ffi::CStr {
        unsafe { core::ffi::CStr::from_ptr((*self.0.get()).name) }
    }
    #[inline]
    pub const fn tri_count(&self) -> u16 {
        unsafe { (*self.0.get()).triCount }
    }
    #[inline]
    pub const fn material(&self) -> Option<&Material> {
        unsafe {
            (*self.0.get())
                .material
                .cast_const()
                .cast::<Material>()
                .as_ref()
        }
    }
    #[inline]
    pub const fn rspq_block(&self) -> Option<BlockRef<'_>> {
        unsafe { BlockRef::from_raw((*self.0.get()).userBlock) }
    }
    #[inline]
    pub const fn is_visible(&self) -> bool {
        unsafe { (*self.0.get()).isVisible != 0 }
    }
    #[inline]
    pub const fn user_value_0(&self) -> u8 {
        unsafe { (*self.0.get()).userValue0 }
    }
    #[inline]
    pub const fn user_value_1(&self) -> u8 {
        unsafe { (*self.0.get()).userValue1 }
    }
    #[inline]
    pub const fn aabb_min(&self) -> [i16; 3] {
        unsafe { (*self.0.get()).aabbMin }
    }
    #[inline]
    pub const fn aabb_max(&self) -> [i16; 3] {
        unsafe { (*self.0.get()).aabbMax }
    }
    #[inline]
    pub const fn parts(&self) -> &[ObjectPart] {
        let obj = self.0.get();
        unsafe {
            let count = (*obj).numParts as usize;
            core::slice::from_raw_parts((*obj).parts.as_ptr().cast(), count)
        }
    }
    #[inline]
    pub fn set_rspq_block(&mut self, block: dragon::rspq::Block) {
        let obj = self.0.get();
        unsafe {
            let block = core::mem::ManuallyDrop::new(block).as_raw();
            let block = core::mem::replace(&mut (*obj).userBlock, block);
            if !block.is_null() {
                dragon::sys::rspq::rspq_block_free(block);
            }
        }
    }
    #[inline]
    pub const fn set_user_value_0(&self, value: u8) {
        unsafe { (*self.0.get()).userValue0 = value };
    }
    #[inline]
    pub const fn set_user_value_1(&self, value: u8) {
        unsafe { (*self.0.get()).userValue1 = value };
    }
}

impl ObjectPart {
    #[inline]
    pub const fn as_ptr(&self) -> *mut sys::T3DObjectPart {
        self.0.get()
    }
    #[inline]
    pub fn vertices(&self) -> VertBuffer<'_> {
        let part = self.0.get();
        unsafe {
            let verts = (*part).vert.add((*part).vertDestOffset as usize);
            let count = ((*part).vertLoadCount >> 1) as usize;
            VertBuffer(NonNull::new_unchecked(verts.cast()), count, PhantomData)
        }
    }
}

impl ChunkAnim {
    #[inline]
    pub const fn as_ptr(&self) -> *mut sys::T3DChunkAnim {
        self.0.get()
    }
    #[inline]
    pub const fn name(&self) -> &core::ffi::CStr {
        unsafe { core::ffi::CStr::from_ptr((*self.0.get()).name) }
    }
    #[inline]
    pub const fn duration(&self) -> f32 {
        unsafe { (*self.0.get()).duration }
    }
    #[inline]
    pub const fn keyframe_count(&self) -> u32 {
        unsafe { (*self.0.get()).keyframeCount }
    }
    #[inline]
    pub const fn channels_quat(&self) -> u16 {
        unsafe { (*self.0.get()).channelsQuat }
    }
    #[inline]
    pub const fn channels_scalar(&self) -> u16 {
        unsafe { (*self.0.get()).channelsScalar }
    }
    #[inline]
    pub const fn file_path(&self) -> &core::ffi::CStr {
        unsafe { core::ffi::CStr::from_ptr((*self.0.get()).filePath) }
    }
    #[inline]
    pub const fn channels(&self) -> &[AnimChannelMapping] {
        let anim = self.0.get();
        unsafe {
            let count = (*anim).channelsQuat as usize + (*anim).channelsScalar as usize;
            core::slice::from_raw_parts((*anim).channelMappings.as_ptr().cast(), count)
        }
    }
    #[inline]
    pub const fn set_duration(&self, duration: f32) {
        unsafe { (*self.0.get()).duration = duration };
    }
}

impl AnimChannelMapping {
    #[inline]
    pub const fn as_ptr(&self) -> *mut sys::T3DAnimChannelMapping {
        self.0.get()
    }
    #[inline]
    pub const fn target_idx(&self) -> u16 {
        unsafe { (*self.0.get()).targetIdx }
    }
    #[inline]
    pub const fn target_type(&self) -> crate::AnimTarget {
        unsafe { core::mem::transmute((*self.0.get()).targetType) }
    }
    #[inline]
    pub const fn attribute_idx(&self) -> u8 {
        unsafe { (*self.0.get()).attributeIdx }
    }
    #[inline]
    pub const fn quant_scale(&self) -> f32 {
        unsafe { (*self.0.get()).quantScale }
    }
    #[inline]
    pub const fn quant_offset(&self) -> f32 {
        unsafe { (*self.0.get()).quantOffset }
    }
    #[inline]
    pub const fn set_quant_scale(&self, scale: f32) {
        unsafe { (*self.0.get()).quantScale = scale };
    }
    #[inline]
    pub const fn set_quant_offset(&self, offset: f32) {
        unsafe { (*self.0.get()).quantOffset = offset };
    }
}

impl ChunkSkeleton {
    #[inline]
    pub const fn as_ptr(&self) -> *mut sys::T3DChunkSkeleton {
        self.0.get()
    }
    #[inline]
    pub const fn bones(&self) -> &[ChunkBone] {
        let skel = self.0.get();
        unsafe {
            let count = (*skel).boneCount as usize;
            core::slice::from_raw_parts((*skel).bones.as_ptr().cast(), count)
        }
    }
}

impl ChunkBone {
    #[inline]
    pub const fn as_ptr(&self) -> *mut sys::T3DChunkBone {
        self.0.get()
    }
    #[inline]
    pub const fn parent_idx(&self) -> u16 {
        unsafe { (*self.0.get()).parentIdx }
    }
    #[inline]
    pub const fn depth(&self) -> u16 {
        unsafe { (*self.0.get()).depth }
    }
    #[inline]
    pub const fn scale(&self) -> Vec3 {
        Vec3(unsafe { (*self.0.get()).scale.v })
    }
    #[inline]
    pub const fn rotation(&self) -> Quat {
        Quat(unsafe { (*self.0.get()).rotation.v })
    }
    #[inline]
    pub const fn position(&self) -> Vec3 {
        Vec3(unsafe { (*self.0.get()).position.v })
    }
    #[inline]
    pub const fn set_scale(&self, scale: Vec3) {
        unsafe { (*self.0.get()).scale.v = scale.0 };
    }
    #[inline]
    pub const fn set_rotation(&self, rotation: Quat) {
        unsafe { (*self.0.get()).rotation.v = rotation.0 };
    }
    #[inline]
    pub const fn set_position(&self, position: Vec3) {
        unsafe { (*self.0.get()).position.v = position.0 };
    }
}

impl Bvh {
    #[inline]
    pub const fn as_ptr(&self) -> *mut sys::T3DBvh {
        self.0.get()
    }
    #[inline]
    pub fn query_frustum(&self, frustum: &crate::Frustum) {
        unsafe { sys::t3d_model_bvh_query_frustum(self.0.get().cast_const(), frustum.as_ptr()) };
    }
    #[inline]
    pub const fn nodes(&self) -> &[BvhNode] {
        let bvh = self.0.get();
        unsafe {
            let count = (*bvh).nodeCount as usize;
            core::slice::from_raw_parts((*bvh).nodes.as_ptr().cast(), count)
        }
    }
}

impl BvhNode {
    #[inline]
    pub const fn as_ptr(&self) -> *mut sys::T3DBvhNode {
        self.0.get()
    }
    #[inline]
    pub const fn aabb_min(&self) -> [i16; 3] {
        unsafe { (*self.0.get()).aabbMin }
    }
    #[inline]
    pub const fn aabb_max(&self) -> [i16; 3] {
        unsafe { (*self.0.get()).aabbMax }
    }
}

impl<'m> VertBuffer<'m> {
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.1 != 0
    }
    #[inline]
    pub const fn as_ptr(&self) -> *mut crate::VertPacked {
        self.0.as_ptr()
    }
    #[inline]
    fn vertex(&self, idx: usize) -> *mut crate::VertPacked {
        let i = idx >> 1;
        if i >= self.1 {
            panic!(
                "index out of bounds: the len is {} but the index is {idx}",
                self.1 << 1
            );
        }
        unsafe { self.0.add(i).as_ptr() }
    }
    #[inline]
    pub fn set_pos(&self, idx: usize, pos: [i16; 3]) {
        let vert = self.vertex(idx);
        unsafe {
            match (idx & 1) != 0 {
                false => (*vert).posA = pos,
                true => (*vert).posB = pos,
            }
        }
    }
    #[inline]
    pub fn set_uv(&self, idx: usize, uv: [i16; 2]) {
        let vert = self.vertex(idx);
        unsafe {
            match (idx & 1) != 0 {
                false => (*vert).stA = uv,
                true => (*vert).stB = uv,
            }
        }
    }
    #[inline]
    pub fn set_color(&self, idx: usize, color: Color) {
        let vert = self.vertex(idx);
        unsafe {
            match (idx & 1) != 0 {
                false => (*vert).rgbaA = color.into_u32(),
                true => (*vert).rgbaB = color.into_u32(),
            }
        }
    }
    #[inline]
    pub fn set_norm(&self, idx: usize, norm: u16) {
        let vert = self.vertex(idx);
        unsafe {
            match (idx & 1) != 0 {
                false => (*vert).normA = norm,
                true => (*vert).normB = norm,
            }
        }
    }
}

unsafe impl<'m> crate::VertBufferExt for VertBuffer<'m> {
    #[inline]
    fn vert_count(&self) -> usize {
        self.1 << 1
    }
    #[inline]
    fn as_ptr(&self) -> *const crate::VertPacked {
        self.as_ptr()
    }
    #[inline]
    fn pos(&self, idx: usize) -> [i16; 3] {
        let vert = self.vertex(idx);
        unsafe {
            match (idx & 1) != 0 {
                false => (*vert).posA,
                true => (*vert).posB,
            }
        }
    }
    #[inline]
    fn uv(&self, idx: usize) -> [i16; 2] {
        let vert = self.vertex(idx);
        unsafe {
            match (idx & 1) != 0 {
                false => (*vert).stA,
                true => (*vert).stB,
            }
        }
    }
    #[inline]
    fn color(&self, idx: usize) -> Color {
        let vert = self.vertex(idx);
        unsafe {
            match (idx & 1) != 0 {
                false => Color::from_u32((*vert).rgbaA),
                true => Color::from_u32((*vert).rgbaB),
            }
        }
    }
    #[inline]
    fn norm(&self, idx: usize) -> u16 {
        let vert = self.vertex(idx);
        unsafe {
            match (idx & 1) != 0 {
                false => (*vert).normA,
                true => (*vert).normB,
            }
        }
    }
    #[inline]
    fn pos_mut(&mut self, idx: usize) -> &mut [i16; 3] {
        let vert = self.vertex(idx);
        unsafe {
            match (idx & 1) != 0 {
                false => &mut (*vert).posA,
                true => &mut (*vert).posB,
            }
        }
    }
    #[inline]
    fn uv_mut(&mut self, idx: usize) -> &mut [i16; 2] {
        let vert = self.vertex(idx);
        unsafe {
            match (idx & 1) != 0 {
                false => &mut (*vert).stA,
                true => &mut (*vert).stB,
            }
        }
    }
    #[inline]
    fn color_mut(&mut self, idx: usize) -> &mut Color {
        let vert = self.vertex(idx);
        unsafe {
            core::mem::transmute(match (idx & 1) != 0 {
                false => &mut (*vert).rgbaA,
                true => &mut (*vert).rgbaB,
            })
        }
    }
    #[inline]
    fn norm_mut(&mut self, idx: usize) -> &mut u16 {
        let vert = self.vertex(idx);
        unsafe {
            match (idx & 1) != 0 {
                false => &mut (*vert).normA,
                true => &mut (*vert).normB,
            }
        }
    }
}
