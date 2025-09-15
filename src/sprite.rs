use core::{ffi::CStr, marker::PhantomData, ptr::NonNull};

use crate::{
    surface::{Surface, TexFormat},
    sys::sprite::*,
};

#[doc = "Sprite structure.\n\n A \"sprite\" (as saved in a `.sprite` file) is a 2D image with\n metadata attached to them to facilitate drawing it onto N64.\n\n Despite the name, a libdragon sprite is basically the basic format\n to handle assets for images. It is commonly used for handling\n textures, full screen images like splash screens, tile maps,\n font pictures, and even \"real\" 2D sprites.\n\n If the sprite uses a color-indexed format like `FMT_CI4` or `FMT_CI8`,\n the sprite contains also the corresponding palette.\n\n To convert an image file to libdragon's sprite format, use\n the mksprite tool. To load a sprite into memory, use `sprite_load`."]
#[repr(transparent)]
#[derive(Debug)]
pub struct Sprite(pub(crate) NonNull<sprite_t>);

#[doc = "Sprite detail texture information structure.\n\n A \"detail texture\" is a 2D image with metadata attached to it\n to increase the perceived resolution of the main sprite when rendering\n with little to no additional TMEM usage.\n\n If the sprite uses a detail texture, its information can be retreived\n using the `sprite_get_detail_pixels` function.\n\n To include a detail texture to libdragon's sprite format, use\n the mksprite tool with --detail argument.\n\n `rdpq_sprite_upload` automatically uploads detail textures associated with\n the sprite."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Detail {
    #[doc = "Is the detail texture the same as the main surface of the sprite, used for fractal detailing"]
    pub use_main_tex: bool,
    #[doc = "Blend factor of the detail texture in range of 0 to 1"]
    pub blend_factor: f32,
}

#[doc = "Texture sampling parameters for [`tex::upload`].\n\n This structure contains all possible parameters for [`tex::upload`].\n All fields have been made so that the 0 value is always the most\n reasonable default. This means that you can simply initialize the structure\n to 0 and then change only the fields you need (for instance, through a\n compound literal).\n"]
#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct SpriteTexParms(pub(crate) crate::sys::rdpq_tex::rdpq_texparms_t);

impl SpriteTexParms {
    #[doc = "TMEM address where to load the texture (default: 0)"]
    #[inline]
    pub const fn tmem_addr(&self) -> i32 {
        self.0.tmem_addr
    }
    #[doc = "Palette number where TLUT is stored (used only for CI4 textures)"]
    #[inline]
    pub const fn palette(&self) -> i32 {
        self.0.palette
    }
    #[doc = "S direction of texture parameters"]
    #[inline]
    pub const fn s(&self) -> &SpriteTexAxis {
        unsafe { core::mem::transmute(&self.0.s) }
    }
    #[doc = "T direction of texture parameters"]
    #[inline]
    pub const fn t(&self) -> &SpriteTexAxis {
        unsafe { core::mem::transmute(&self.0.t) }
    }
    #[inline]
    pub const fn into_builder(self) -> crate::rdpq::TexParms {
        crate::rdpq::TexParms(self.0)
    }
}

#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct SpriteTexAxis(pub(crate) crate::sys::rdpq_tex::rdpq_texparms_s__bindgen_ty_1);
impl SpriteTexAxis {
    #[doc = "Translation of the texture (in pixels)"]
    #[inline]
    pub const fn translate(&self) -> f32 {
        self.0.translate
    }
    #[doc = "Power of 2 scale modifier of the texture (default: 0). Eg: -2 = make the texture 4 times smaller. Range is [-5..10]"]
    #[inline]
    pub const fn scale_log(&self) -> i32 {
        self.0.scale_log
    }
    #[doc = "Number of repetitions before the texture clamps (default: 1). Use [`REPEAT_INFINITE`] for infinite repetitions (wrapping)"]
    #[inline]
    pub const fn repeats(&self) -> u32 {
        self.0.repeats as u32
    }
    #[doc = "Repetition mode (default: false). If true, the texture mirrors at each repetition"]
    #[inline]
    pub const fn mirror(&self) -> bool {
        self.0.mirror
    }
    #[inline]
    pub const fn into_builder(self) -> crate::rdpq::TexAxis {
        crate::rdpq::TexAxis(self.0)
    }
}

impl Sprite {
    #[doc = "Load a sprite from a filesystem (eg: ROM)\n\n This function loads a full sprite from a filesystem. Notice that there is no\n streaming support, so the file is fully loaded into RDRAM, in its final\n uncompressed format.\n\n sprite_load internally uses the asset API (`asset_load`), so the sprite file\n is transparently uncompressed if needed.\n\n @param fn           Filename of the sprite, including filesystem specifier.\n                     For instance: \"rom:/hero.sprite\" to load from DFS.\n @return sprite_t*   The loaded sprite"]
    #[inline]
    pub fn load(filename: &CStr) -> Self {
        unsafe { Self(NonNull::new(sprite_load(filename.as_ptr())).unwrap()) }
    }
    #[doc = "Load a sprite from a buffer\n\n This function loads a sprite from a buffer corresponding to sprite\n file data in memory. The function also performs any necessary processing\n to load the sprite file data.\n\n sprite_load_buf functions in-place which means it does not allocate another\n buffer for the loaded sprite. So, sprite_free will not remove the sprite data\n from memory. This means that the input buffer must be freed manually after\n sprite_free is called.\n\n @param buf           Pointer to the sprite file data\n @param sz            Size of the sprite file buffer\n @return sprite_t*    The loaded sprite"]
    #[inline]
    pub unsafe fn load_buf(buf: &'static [u8]) -> Self {
        unsafe { Self(NonNull::new(sprite_load_buf(buf.as_ptr() as _, buf.len() as _)).unwrap()) }
    }
}

impl Drop for Sprite {
    #[doc = "Deallocate a sprite"]
    #[inline]
    fn drop(&mut self) {
        unsafe {
            sprite_free(self.as_raw());
        }
    }
}

impl Sprite {
    #[inline]
    pub const fn as_raw(&self) -> *mut sprite_t {
        self.0.as_ptr()
    }
    #[doc = "Get the sprite texture format\n\n @param sprite    The sprite\n @return          The texture format"]
    #[inline]
    pub fn format(&self) -> TexFormat {
        unsafe {
            core::mem::transmute(
                (*self.as_raw()).__bindgen_anon_1.flags & SPRITE_FLAGS_TEXFORMAT as u8,
            )
        }
    }
    #[doc = "Create a `Surface` pointing to the full sprite contents.\n\n This function can be used to pass a full sprite to functions accepting\n a `surface_t`.\n\n Notice that no memory allocations or copies are performed:\n the returned surface will point to the sprite contents.\n\n @param  sprite      The sprite\n @return             The surface pointing to the sprite"]
    #[inline]
    pub fn pixels(&self) -> Surface<'_> {
        unsafe { Surface(sprite_get_pixels(self.as_raw()), PhantomData) }
    }
    #[doc = "Create a surface_t pointing to the contents of a LOD level.\n\n This function can be used to access LOD images within a sprite file.\n It is useful for sprites created by mksprite containing multiple\n mipmap levels.\n\n LOD levels are indexed from 1 upward. 0 refers to the main sprite,\n so calling `sprite_get_lod_pixels(s, 0)` is equivalent to\n `sprite_get_pixels(s)`.\n\n Notice that no memory allocations or copies are performed:\n the returned surface will point to the sprite contents.\n\n @param sprite        The sprite to access\n @param num_level     The number of LOD level. 0 is the main sprite.\n @return surface_t    The surface containing the data."]
    #[inline]
    pub fn lod_pixels(&self, num_level: u8) -> Surface<'_> {
        assert!(num_level < 8);
        unsafe {
            Surface(
                sprite_get_lod_pixels(self.as_raw(), num_level as _),
                PhantomData,
            )
        }
    }
    #[doc = "Create a surface_t pointing to the contents of a detail texture.\n\n This function can be used to access detail texture within a sprite file.\n It is useful for sprites created by mksprite containing one.\n\n If there isn't a detail texture, the returned surface is 0.\n\n Additional detail information such as factor or texparms are accessible\n through the filled sprite_detail_t and rdpq_texparms_t structure.\n If you don't wish to use this information, pass NULL to the info argument(s).\n\n Notice that no memory allocations or copies are performed:\n the returned surface will point to the sprite contents.\n\n @param sprite        The sprite to access\n @param info          The detail information struct to fill if needed\n @param infoparms     The detail texture sampling struct to fill if needed\n @return surface_t    The surface containing the data."]
    #[inline]
    pub fn detail_pixels(&self, info: Option<&Detail>) -> (Surface<'_>, SpriteTexParms) {
        unsafe {
            let mut parms = core::mem::MaybeUninit::uninit();
            let surface = sprite_get_detail_pixels(
                self.as_raw(),
                core::mem::transmute(
                    info.map(|d| d as *const _ as *mut sprite_detail_t)
                        .unwrap_or_else(core::ptr::null_mut),
                ),
                parms.as_mut_ptr(),
            );
            (
                Surface(surface, PhantomData),
                SpriteTexParms(parms.assume_init()),
            )
        }
    }
    #[doc = "Return a surface_t pointing to a specific tile of the spritemap.\n\n A sprite can be used as a spritemap, that is a collection of multiple\n smaller images of equal size, called \"tiles\". In this case, the number\n of tiles is stored in the members `hslices` and `vslices` of the\n sprite structure.\n\n This function allows to get a surface that points to the specific sub-tile,\n so that it can accessed directly.\n\n @param   sprite      The sprite used as spritemap\n @param   h           Horizontal index of the tile to access\n @param   v           Vertical index of the tile to access\n @return              A surface pointing to the tile"]
    #[inline]
    pub fn tile(&self, h: u32, v: u32) -> Surface<'_> {
        unsafe { Surface(sprite_get_tile(self.as_raw(), h as _, v as _), PhantomData) }
    }
    #[doc = "Access the sprite palette (if any)\n\n A sprite can also contain a palette, in case the sprite data is color-indexed\n (that is, the format is either `FMT_CI4` or `FMT_CI8`).\n\n This function returns a pointer to the raw palette data contained in the sprite.\n\n @param   sprite      The sprite to access\n @return              A pointer to the palette data, or NULL if the sprite does not have a palette"]
    #[inline]
    pub fn palette(&self) -> Option<&[u16]> {
        unsafe {
            let palette = NonNull::new(sprite_get_palette(self.as_raw()))?;
            let len = match self.format() {
                TexFormat::CI4 => 16,
                TexFormat::CI8 => 256,
                _ => unreachable!(),
            };
            Some(core::slice::from_raw_parts(palette.as_ptr(), len))
        }
    }
    #[doc = "Get a copy of the RDP texparms, optionally stored within the sprite.\n\n This function allows to obtain the RDP texparms structure stored within the\n sprite, if any. This structure is used by the RDP to set texture properties\n such as wrapping, mirroring, etc. It can be added to the sprite via\n the mksprite tool, using the `--texparms` option.\n\n @param sprite        The sprite to access\n @param parms         The texparms structure to fill\n @return              true if the sprite contain RDP texparms, false otherwise"]
    #[inline]
    pub fn texparms(&self) -> Option<SpriteTexParms> {
        unsafe {
            let mut parms = core::mem::MaybeUninit::uninit();
            match sprite_get_texparms(self.as_raw(), parms.as_mut_ptr()) {
                false => None,
                true => Some(SpriteTexParms(parms.assume_init())),
            }
        }
    }
    #[doc = "Return the number of LOD levels stored within the sprite (including the main image).\n\n @param sprite        The sprite to access\n @return              The number of LOD levels"]
    #[inline]
    pub fn lod_count(&self) -> u32 {
        unsafe { sprite_get_lod_count(self.as_raw()) as _ }
    }
    #[doc = "Return true if the sprite fits in TMEM without splitting\n\n This function returns true if the sprite can be fully uploaded in TMEM\n (including all its LODs, detail texture and palettes).\n\n When working on 3D graphics, each texture must fit into RDP TMEM (4 KiB),\n otherwise it cannot be used. All sprites that are meant to be used as\n textures should fit in TMEM.\n\n In case of 2D graphics, it is more common to have images of arbitrary size.\n They can be drawn with `rdpq_sprite_blit` (accelerated) or `graphics_draw_sprite`\n (CPU) without specific limits (the RDP accelerated\n version does internally need to split the sprite in multiple parts, but\n that is indeed possible).\n\n This function is mostly for debugging purposes, as it can help validating\n whether a sprite can be used as a texture or not.\n\n @param sprite        The sprite to access\n @return              True if the sprite fits TMEM, false otherwise"]
    #[inline]
    pub fn fits_tmem(&self) -> bool {
        unsafe { sprite_fits_tmem(self.as_raw()) }
    }
    #[doc = "Return true if the sprite is in SHQ format\n\n This is a special sprite made of two mipmaps (one I4 and one RGBA16)\n that must be displayed using subtractive blending.\n\n @param sprite        The sprite to access\n @return              True if the sprite is in SHQ format, false otherwise"]
    #[inline]
    pub fn is_shq(&self) -> bool {
        unsafe { sprite_is_shq(self.as_raw()) }
    }
}
