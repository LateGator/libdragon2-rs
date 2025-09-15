use core::{
    marker::PhantomData,
    sync::atomic::{AtomicBool, Ordering},
};

use dragon::{graphics::Color, rdpq::RdpQ};

pub type Particle = sys::TPXParticle;

#[repr(transparent)]
#[derive(Debug)]
pub struct TinyPX<'r, R>(PhantomData<&'r mut R>);

static TPX_INIT: AtomicBool = AtomicBool::new(false);

#[inline]
pub fn init<'r, R: AsMut<RdpQ<'r>>>(_rdpq: R, matrix_stack_size: u32) -> TinyPX<'r, R> {
    assert_eq!(TPX_INIT.load(Ordering::Relaxed), false);
    TPX_INIT.store(true, Ordering::Relaxed);
    unsafe {
        sys::tpx_init(sys::TPXInitParams {
            matrixStackSize: matrix_stack_size as _,
        });
    }
    TinyPX(PhantomData)
}

impl<'r, R> Drop for TinyPX<'r, R> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            sys::tpx_destroy();
            TPX_INIT.store(false, Ordering::Relaxed);
        }
    }
}

impl<'r, 'v, R: AsMut<RdpQ<'r>>> TinyPX<'r, crate::Tiny3D<'r, 'v, R>> {
    #[inline]
    pub fn state_from_t3d(&mut self) {
        unsafe { sys::tpx_state_from_t3d() };
    }
}

impl<'r, 'v, R: AsMut<RdpQ<'r>>> crate::Tiny3D<'r, 'v, crate::TinyPX<'r, R>> {
    #[inline]
    pub fn state_from_t3d(&mut self) {
        unsafe { sys::tpx_state_from_t3d() };
    }
}

impl<'r, R: AsMut<RdpQ<'r>>> TinyPX<'r, R> {
    #[inline]
    pub fn state_set_scale(&mut self, scale_x: f32, scale_y: f32) {
        unsafe { sys::tpx_state_set_scale(scale_x, scale_y) };
    }
    #[inline]
    pub fn state_set_base_size(&mut self, base_size: u16) {
        unsafe { sys::tpx_state_set_base_size(base_size) };
    }
    #[inline]
    pub fn state_set_tex_params(&mut self, offset_x: i16, mirror_point: u16) {
        unsafe { sys::tpx_state_set_tex_params(offset_x, mirror_point) };
    }
    #[inline]
    pub fn particle_draw(&mut self, particles: &[Particle]) {
        unsafe { sys::tpx_particle_draw(particles.as_ptr() as _, particles.particle_count() as _) };
    }
    #[inline]
    pub fn particle_draw_tex(&mut self, particles: &[Particle]) {
        unsafe {
            sys::tpx_particle_draw_tex(particles.as_ptr() as _, particles.particle_count() as _)
        };
    }
    #[inline]
    pub fn matrix_set(&mut self, mat: &crate::math::Mat4FP, do_multiply: bool) {
        unsafe { sys::tpx_matrix_set(mat.as_ptr(), do_multiply) };
    }
    #[inline]
    pub fn matrix_push(&mut self, mat: &crate::math::Mat4FP) {
        unsafe { sys::tpx_matrix_push(mat.as_ptr()) };
    }
    #[inline]
    pub fn matrix_pop(&mut self, count: i32) {
        unsafe { sys::tpx_matrix_pop(count) };
    }
    #[inline]
    pub fn matrix_push_pos(&mut self, count: i32) {
        unsafe { sys::tpx_matrix_push_pos(count) };
    }
}

pub unsafe trait ParticleBufferExt: crate::sealed::Sealed {
    fn particle_count(&self) -> usize;
    fn pos(&self, idx: usize) -> [i8; 3];
    fn size(&self, idx: usize) -> i8;
    fn color(&self, idx: usize) -> Color;
    fn pos_mut(&mut self, idx: usize) -> &mut [i8; 3];
    fn size_mut(&mut self, idx: usize) -> &mut i8;
    fn color_mut(&mut self, idx: usize) -> &mut Color;
    fn swap(&mut self, idx_a: usize, idx_b: usize);
    fn copy(&mut self, idx_dst: usize, idx_src: usize);
}

unsafe impl ParticleBufferExt for [Particle] {
    #[inline]
    fn particle_count(&self) -> usize {
        self.len() << 1
    }
    #[inline]
    fn pos(&self, idx: usize) -> [i8; 3] {
        match (idx & 1) != 0 {
            false => self[idx >> 1].posA,
            true => self[idx >> 1].posB,
        }
    }
    #[inline]
    fn size(&self, idx: usize) -> i8 {
        match (idx & 1) != 0 {
            false => self[idx >> 1].sizeA,
            true => self[idx >> 1].sizeB,
        }
    }
    #[inline]
    fn color(&self, idx: usize) -> Color {
        match (idx & 1) != 0 {
            false => Color::from_array(self[idx >> 1].colorA),
            true => Color::from_array(self[idx >> 1].colorB),
        }
    }
    #[inline]
    fn pos_mut(&mut self, idx: usize) -> &mut [i8; 3] {
        match (idx & 1) != 0 {
            false => &mut self[idx >> 1].posA,
            true => &mut self[idx >> 1].posB,
        }
    }
    #[inline]
    fn size_mut(&mut self, idx: usize) -> &mut i8 {
        match (idx & 1) != 0 {
            false => &mut self[idx >> 1].sizeA,
            true => &mut self[idx >> 1].sizeB,
        }
    }
    #[inline]
    fn color_mut(&mut self, idx: usize) -> &mut Color {
        unsafe {
            core::mem::transmute(match (idx & 1) != 0 {
                false => &mut self[idx >> 1].colorA,
                true => &mut self[idx >> 1].colorB,
            })
        }
    }
    #[inline]
    fn swap(&mut self, idx_a: usize, idx_b: usize) {
        let count = self.particle_count();
        assert!(idx_a < count);
        assert!(idx_b < count);
        unsafe { sys::tpx_buffer_swap(self.as_mut_ptr(), idx_a as _, idx_b as _) };
    }
    #[inline]
    fn copy(&mut self, idx_dst: usize, idx_src: usize) {
        let count = self.particle_count();
        assert!(idx_dst < count);
        assert!(idx_src < count);
        unsafe { sys::tpx_buffer_copy(self.as_mut_ptr(), idx_dst as _, idx_src as _) };
    }
}

impl<'r, R: AsRef<RdpQ<'r>>> ::core::ops::Deref for TinyPX<'r, R> {
    type Target = RdpQ<'r>;
    #[inline]
    fn deref(&self) -> &RdpQ<'r> {
        unsafe { core::mem::transmute(&dragon::rdpq::get()) }
    }
}
impl<'r, R: AsRef<RdpQ<'r>>> ::core::borrow::Borrow<RdpQ<'r>> for TinyPX<'r, R> {
    #[inline]
    fn borrow(&self) -> &RdpQ<'r> {
        unsafe { core::mem::transmute(&dragon::rdpq::get()) }
    }
}
impl<'r, R: AsRef<RdpQ<'r>>> ::core::convert::AsRef<RdpQ<'r>> for TinyPX<'r, R> {
    #[inline]
    fn as_ref(&self) -> &RdpQ<'r> {
        unsafe { core::mem::transmute(&dragon::rdpq::get()) }
    }
}
impl<'r, R: AsRef<RdpQ<'r>> + AsMut<RdpQ<'r>>> ::core::ops::DerefMut for TinyPX<'r, R> {
    #[inline]
    fn deref_mut(&mut self) -> &mut RdpQ<'r> {
        unsafe { core::mem::transmute(&mut dragon::rdpq::get()) }
    }
}
impl<'r, R: AsRef<RdpQ<'r>> + AsMut<RdpQ<'r>>> ::core::borrow::BorrowMut<RdpQ<'r>>
    for TinyPX<'r, R>
{
    #[inline]
    fn borrow_mut(&mut self) -> &mut RdpQ<'r> {
        unsafe { core::mem::transmute(&mut dragon::rdpq::get()) }
    }
}
impl<'r, R: AsMut<RdpQ<'r>>> ::core::convert::AsMut<RdpQ<'r>> for TinyPX<'r, R> {
    #[inline]
    fn as_mut(&mut self) -> &mut RdpQ<'r> {
        unsafe { core::mem::transmute(&mut dragon::rdpq::get()) }
    }
}
