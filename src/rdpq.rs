use core::{marker::PhantomData, ptr::NonNull};

use crate::{
    graphics::Color,
    rspq::RspQ,
    sprite::Sprite,
    surface::{Surface, TexFormat},
    sys::{rdpq::*, rdpq_macros::*, rdpq_mode::*, rdpq_tex::*},
};

bitflags::bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
    pub struct Config: u32 {
        const AUTOSYNCPIPE = RDPQ_CFG_AUTOSYNCPIPE;
        const AUTOSYNCLOAD = RDPQ_CFG_AUTOSYNCLOAD;
        const AUTOSYNCTILE = RDPQ_CFG_AUTOSYNCTILE;
        const AUTOSCISSOR = RDPQ_CFG_AUTOSCISSOR;
    }
}

impl Default for Config {
    #[inline]
    fn default() -> Self {
        Self::from_bits_retain(RDPQ_CFG_DEFAULT)
    }
}

#[doc = "Tile descriptors.\n\n These are enums that map to integers 0-7, but they can be used in place of the\n integers for code readability."]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Tile {
    _0 = rdpq_tile_t_TILE0 as _,
    _1 = rdpq_tile_t_TILE1 as _,
    _2 = rdpq_tile_t_TILE2 as _,
    _3 = rdpq_tile_t_TILE3 as _,
    _4 = rdpq_tile_t_TILE4 as _,
    _5 = rdpq_tile_t_TILE5 as _,
    _6 = rdpq_tile_t_TILE6 as _,
    _7 = rdpq_tile_t_TILE7 as _,
}

impl Tile {
    pub const INTERNAL: Self = Self::_7;
    #[inline]
    const fn autosync(self) -> u32 {
        1 << self as u32
    }
}

impl From<Tile> for u32 {
    #[inline]
    fn from(value: Tile) -> Self {
        value as _
    }
}

#[inline]
const fn autosync_tmem(n: u32) -> u32 {
    1 << (8 + n)
}

#[doc = "Tile parameters for [`set_tile`].\n\n This structure contains all possible parameters for [`set_tile`].\n All fields have been made so that the 0 value is always the most\n reasonable default (clamped with default scale, no mirroring).\n This means that you can simply initialize the structure to 0 and then\n change only the fields you need (for instance, through a compound literal).\n"]
#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct TileParms(pub(crate) rdpq_tileparms_t);

impl TileParms {
    pub const EMPTY: Self = Self::new();
    #[inline]
    pub const fn new() -> Self {
        Self(rdpq_tileparms_t {
            palette: 0,
            s: TileAxis::new().0,
            t: TileAxis::new().0,
        })
    }
    #[doc = "Optional palette associated to the texture. For textures in [`TexFormat::CI4`] format, specify the palette index (0-15), otherwise use 0."]
    #[inline]
    pub const fn palette(mut self, palette: u8) -> Self {
        self.0.palette = palette;
        self
    }
    #[doc = "S direction of the tile descriptor"]
    #[inline]
    pub const fn s(mut self, s: TileAxis) -> Self {
        self.0.s = s.0;
        self
    }
    #[doc = "S direction of the tile descriptor"]
    #[inline]
    pub const fn t(mut self, t: TileAxis) -> Self {
        self.0.t = t.0;
        self
    }
}

#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct TileAxis(pub(crate) rdpq_tileparms_t__bindgen_ty_1);

impl TileAxis {
    #[inline]
    pub const fn new() -> Self {
        Self(rdpq_tileparms_t__bindgen_ty_1 {
            clamp: false,
            mirror: false,
            mask: 0,
            shift: 0,
        })
    }
    #[doc = "True if texture needs to be clamped. Otherwise wrap the texture around"]
    #[inline]
    pub const fn clamp(mut self, clamp: bool) -> Self {
        self.0.clamp = clamp;
        self
    }
    #[doc = "True if texture needs to be mirrored. Otherwise wrap the texture without mirroring"]
    #[inline]
    pub const fn mirror(mut self, mirror: bool) -> Self {
        self.0.mirror = mirror;
        self
    }
    #[doc = "Power of 2 boundary of the texture in pixels to wrap. (Important note: Mask value of 0 will force clamping to be ON regardless of clamp value);"]
    #[inline]
    pub const fn mask(mut self, mask: u8) -> Self {
        self.0.mask = mask;
        self
    }
    #[doc = "Power of 2 scale of the texture to wrap on. Range is [-5..10];"]
    #[inline]
    pub const fn shift(mut self, shift: i8) -> Self {
        self.0.shift = shift;
        self
    }
}

pub const REPEAT_INFINITE: u32 = crate::sys::rdpq_tex::REPEAT_INFINITE;

#[doc = "Texture sampling parameters for [`tex::upload`].\n\n This structure contains all possible parameters for [`tex::upload`].\n All fields have been made so that the 0 value is always the most\n reasonable default. This means that you can simply initialize the structure\n to 0 and then change only the fields you need (for instance, through a\n compound literal).\n"]
#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct TexParms(pub(crate) rdpq_texparms_t);

impl TexParms {
    pub const EMPTY: Self = Self::new();
    #[inline]
    pub const fn new() -> Self {
        Self(rdpq_texparms_t {
            tmem_addr: 0,
            palette: 0,
            s: TexAxis::new().0,
            t: TexAxis::new().0,
        })
    }
    #[doc = "TMEM address where to load the texture (default: 0)"]
    #[inline]
    pub const fn tmem_addr(mut self, tmem_addr: i32) -> Self {
        self.0.tmem_addr = tmem_addr;
        self
    }
    #[doc = "Palette number where TLUT is stored (used only for CI4 textures)"]
    #[inline]
    pub const fn palette(mut self, palette: i32) -> Self {
        self.0.palette = palette;
        self
    }
    #[doc = "S direction of texture parameters"]
    #[inline]
    pub const fn s(mut self, s: TexAxis) -> Self {
        self.0.s = s.0;
        self
    }
    #[doc = "T direction of texture parameters"]
    #[inline]
    pub const fn t(mut self, t: TexAxis) -> Self {
        self.0.t = t.0;
        self
    }
}

#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct TexAxis(pub(crate) rdpq_texparms_s__bindgen_ty_1);
impl TexAxis {
    #[inline]
    pub const fn new() -> Self {
        Self(rdpq_texparms_s__bindgen_ty_1 {
            translate: 0.0,
            scale_log: 0,
            repeats: 0.0,
            mirror: false,
        })
    }
    #[doc = "Translation of the texture (in pixels)"]
    #[inline]
    pub const fn translate(mut self, translate: f32) -> Self {
        self.0.translate = translate;
        self
    }
    #[doc = "Power of 2 scale modifier of the texture (default: 0). Eg: -2 = make the texture 4 times smaller. Range is [-5..10]"]
    #[inline]
    pub const fn scale_log(mut self, scale_log: i32) -> Self {
        self.0.scale_log = scale_log;
        self
    }
    #[doc = "Number of repetitions before the texture clamps (default: 1). Use [`REPEAT_INFINITE`] for infinite repetitions (wrapping)"]
    #[inline]
    pub const fn repeats(mut self, repeats: u32) -> Self {
        self.0.repeats = repeats as f32;
        self
    }
    #[doc = "Repetition mode (default: false). If true, the texture mirrors at each repetition"]
    #[inline]
    pub const fn mirror(mut self, mirror: bool) -> Self {
        self.0.mirror = mirror;
        self
    }
}

#[doc = "Blitting parameters for `rdpq_tex_blit`.\n\n This structure contains all possible parameters for `rdpq_tex_blit`.\n The various fields have been designed so that the 0 value is always the most\n reasonable default. This means that you can simply initialize the structure\n to 0 and then change only the fields you need (for instance, through a\n compound literal).\n\n See `rdpq_tex_blit` for several examples."]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Default)]
pub struct BlitParms(pub(crate) rdpq_blitparms_t);

impl BlitParms {
    pub const EMPTY: Self = Self::new();
    #[inline]
    pub const fn new() -> Self {
        Self(rdpq_blitparms_s {
            tile: 0,
            s0: 0,
            t0: 0,
            width: 0,
            height: 0,
            flip_x: false,
            flip_y: false,
            cx: 0,
            cy: 0,
            scale_x: 0.0,
            scale_y: 0.0,
            theta: 0.0,
            filtering: false,
            nx: 0,
            ny: 0,
        })
    }
    #[doc = "Base tile descriptor to use (default: TILE_0); notice that two tiles will often be used to do the upload (tile and tile+1)."]
    #[inline]
    pub const fn tile(mut self, tile: Tile) -> Self {
        self.0.tile = tile as _;
        self
    }
    #[doc = "Source sub-rect top-left X coordinate"]
    #[inline]
    pub const fn s0(mut self, s0: i32) -> Self {
        self.0.s0 = s0;
        self
    }
    #[doc = "Source sub-rect top-left Y coordinate"]
    #[inline]
    pub const fn t0(mut self, t0: i32) -> Self {
        self.0.t0 = t0;
        self
    }
    #[doc = "Source sub-rect width. If 0, the width of the surface is used"]
    #[inline]
    pub const fn width(mut self, width: i32) -> Self {
        self.0.width = width;
        self
    }
    #[doc = "Source sub-rect height. If 0, the height of the surface is used"]
    #[inline]
    pub const fn height(mut self, height: i32) -> Self {
        self.0.height = height;
        self
    }
    #[doc = "Flip horizontally. If true, the source sub-rect is treated as horizontally flipped (so flipping is performed before all other transformations)"]
    #[inline]
    pub const fn flip_x(mut self, flip_x: bool) -> Self {
        self.0.flip_x = flip_x;
        self
    }
    #[doc = "Flip vertically. If true, the source sub-rect is treated as vertically flipped (so flipping is performed before all other transformations)"]
    #[inline]
    pub const fn flip_y(mut self, flip_y: bool) -> Self {
        self.0.flip_y = flip_y;
        self
    }
    #[doc = "Transformation center (aka \"hotspot\") X coordinate, relative to (s0, t0). Used for all transformations"]
    #[inline]
    pub const fn cx(mut self, cx: i32) -> Self {
        self.0.cx = cx;
        self
    }
    #[doc = "Transformation center (aka \"hotspot\") X coordinate, relative to (s0, t0). Used for all transformations"]
    #[inline]
    pub const fn cy(mut self, cy: i32) -> Self {
        self.0.cy = cy;
        self
    }
    #[doc = "Horizontal scale factor to apply to the surface. If 0, no scaling is performed (the same as 1.0f). If negative, horizontal flipping is applied"]
    #[inline]
    pub const fn scale_x(mut self, scale_x: f32) -> Self {
        self.0.scale_x = scale_x;
        self
    }
    #[doc = "Vertical scale factor to apply to the surface. If 0, no scaling is performed (the same as 1.0f). If negative, vertical flipping is applied"]
    #[inline]
    pub const fn scale_y(mut self, scale_y: f32) -> Self {
        self.0.scale_y = scale_y;
        self
    }
    #[doc = "Rotation angle in radians"]
    #[inline]
    pub const fn theta(mut self, theta: f32) -> Self {
        self.0.theta = theta;
        self
    }
    #[doc = "True if texture filtering is enabled (activates workaround for filtering artifacts when splitting textures in chunks)"]
    #[inline]
    pub const fn filtering(mut self, filtering: bool) -> Self {
        self.0.filtering = filtering;
        self
    }
    #[doc = "Texture horizontal repeat count. If 0, no repetition is performed (the same as 1)"]
    #[inline]
    pub const fn nx(mut self, nx: i32) -> Self {
        self.0.nx = nx;
        self
    }
    #[doc = "Texture vertical repeat count. If 0, no repetition is performed (the same as 1)"]
    #[inline]
    pub const fn ny(mut self, ny: i32) -> Self {
        self.0.ny = ny;
        self
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct RdpQ<'r>(PhantomData<&'r mut RspQ>);

static_wrapper! { RdpQ<'r> => RspQ { RspQ(()) } }

#[doc = "Initialize the RDPQ library.\n\n This should be called by the initialization functions of the higher-level\n libraries using RDPQ to emit RDP commands, and/or by the application main\n if the application itself calls rdpq functions.\n\n It is safe to call this function multiple times (it does nothing), so that\n multiple independent libraries using rdpq can call `rdpq_init` with no side\n effects."]
#[inline]
pub fn init<'r>(_rspq: &'r mut RspQ) -> RdpQ<'r> {
    unsafe { rdpq_init() };
    RdpQ(PhantomData)
}

pub unsafe fn get<'r>() -> RdpQ<'r> {
    RdpQ(PhantomData)
}

#[inline(always)]
fn _carg(value: impl Into<u32>, mask: u32, shift: u32) -> u32 {
    (value.into() & mask) << shift
}

impl<'r> RdpQ<'r> {
    #[doc = "Set the configuration of the RDPQ module.\n\n This function allows you to change the configuration of rdpq to enable/disable\n features. This is useful mainly for advanced users that want to manually tune\n RDP programming, disabling some automatisms performed by rdpq.\n\n The configuration is a bitmask that can be composed using the `RDPQ_CFG_*` macros.\n\n To enable or disable specific configuration options use `rdpq_config_enable` or\n `rdpq_config_disable`.\n\n @param cfg         The new configuration to set\n @return            The previous configuration\n\n @see `rdpq_config_enable`\n @see `rdpq_config_disable`"]
    #[inline]
    pub fn config_set(&mut self, cfg: Config) -> Config {
        Config::from_bits_retain(unsafe { rdpq_config_set(cfg.bits()) })
    }
    #[doc = "Enable a specific set of configuration flags\n\n This function allows you to modify the configuration of rdpq activating a specific\n set of features. It can be useful to temporarily modify the configuration and then\n restore it.\n\n @param cfg_enable_bits    Configuration flags to enable\n @return                   The previous configuration\n\n @see `rdpq_config_set`\n @see `rdpq_config_disable`"]
    #[inline]
    pub fn config_enable(&mut self, cfg: Config) -> Config {
        Config::from_bits_retain(unsafe { rdpq_config_enable(cfg.bits()) })
    }
    #[doc = "Disable a specific set of configuration flags\n\n This function allows you to modify the configuration of rdpq disabling a specific\n set of features. It can be useful to temporarily modify the configuration and then\n restore it.\n\n @code{.c}\n      // Disable automatic scissor generation\n      uint32_t old_cfg = rdpq_config_disable(RDPQ_CFG_AUTOSCISSOR);\n\n      // This will change the render target but will NOT issue a corresponding SET_SCISSOR.\n      // This is dangerous as the currently-configured scissor might allow to draw outside of\n      // the surface boundary, but an advanced user will know if this is correct.\n      rdpq_set_color_image(surface);\n\n      [...]\n\n      // Restore the previous configuration\n      rdpq_config_set(old_cfg);\n @endcode\n\n @param cfg_disable_bits   Configuration flags to disable\n @return                   The previous configuration\n\n @see `rdpq_config_set`\n @see `rdpq_config_enable`"]
    #[inline]
    pub fn config_disable(&mut self, cfg: Config) -> Config {
        Config::from_bits_retain(unsafe { rdpq_config_disable(cfg.bits()) })
    }
    #[doc = "Low level function to set the green and blue components of the chroma key"]
    #[inline]
    pub fn set_chromakey_parms(
        &mut self,
        color: Color,
        edge_r: i32,
        edge_g: i32,
        edge_b: i32,
        width_r: i32,
        width_g: i32,
        width_b: i32,
    ) {
        let fsr = 1.0 / edge_r as f32;
        let fsg = 1.0 / edge_g as f32;
        let fsb = 1.0 / edge_b as f32;
        let sr = (fsr as f32 * 255.0) as u8;
        let sg = (fsg as f32 * 255.0) as u8;
        let sb = (fsb as f32 * 255.0) as u8;
        let fwr = width_r as f32 * fsr;
        let fwg = width_g as f32 * fsg;
        let fwb = width_b as f32 * fsb;
        let wr = (fwr * 255.0) as u16;
        let wg = (fwg * 255.0) as u16;
        let wb = (fwb * 255.0) as u16;

        unsafe {
            __rdpq_write8_syncchange(
                RDPQ_CMD_SET_KEY_R,
                0,
                _carg(wr, 0xFFF, 16) | _carg(color.r, 0xFF, 8) | _carg(sr, 0xFF, 0),
                AUTOSYNC_PIPE,
            );
            __rdpq_write8_syncchange(
                RDPQ_CMD_SET_KEY_GB,
                _carg(wg, 0xFFF, 12) | _carg(wb, 0xFFF, 0),
                _carg(color.g, 0xFF, 24)
                    | _carg(sg, 0xFF, 16)
                    | _carg(color.b, 0xFF, 8)
                    | _carg(sb, 0xFF, 0),
                AUTOSYNC_PIPE,
            );
        }
    }
    #[doc = "Low level functions to set the matrix coefficients for texture format conversion"]
    #[inline]
    pub fn set_yuv_parms(&mut self, k0: u16, k1: u16, k2: u16, k3: u16, k4: u16, k5: u16) {
        unsafe {
            __rdpq_write8_syncchange(
                RDPQ_CMD_SET_CONVERT,
                _carg(k0, 0x1FF, 13) | _carg(k1, 0x1FF, 4) | ((k2 as u32 & 0x1FF) >> 5),
                _carg(k2, 0x1F, 27)
                    | _carg(k3, 0x1FF, 18)
                    | _carg(k4, 0x1FF, 9)
                    | _carg(k5, 0x1FF, 0),
                AUTOSYNC_PIPE,
            );
        }
    }
    #[doc = "Configure a scissoring rectangle in screen coordinates (RDP command: SET_SCISSOR)\n\nThis function is used to configure a scissor region that the RDP with adhere to\nwhile drawing primitives (triangles or rectangles). Any points that fall outside\nof the specified scissoring rectangle will be ignored.\n\nThe scissoring capability is also the only one that prevents the RDP from drawing\noutside of the current framebuffer (color surface) extents. As such, rdpq actually\ncalls `rdpq_set_scissor` automatically any time a new render target is configured\n(eg: via `rdpq_attach` or `rdpq_set_color_image`), because forgetting to do so might\neasily cause crashes.\n\nBecause `rdpq_set_color_image` will configure a scissoring region automatically,\nit is normally not required to call this function. Use this function if you want\nto restrict drawing to a smaller area of the framebuffer.\n\nThe scissoring rectangle is defined using unsigned coordinates, and thus negative\ncoordinates will always be clipped. Rectangle-drawing primitives do not allow to\nspecify them at all, but triangle-drawing primitives do.\n\n@param[in]   x0      Top-left X coordinate of the rectangle\n@param[in]   y0      Top-left Y coordinate of the rectangle\n@param[in]   x1      Bottom-right *exclusive* X coordinate of the rectangle\n@param[in]   y1      Bottom-right *exclusive* Y coordinate of the rectangle\n\n@see `rdpq_attach`\n@see `rdpq_set_color_image`"]
    #[inline]
    pub fn set_scissor(&mut self, x0: f32, y0: f32, x1: f32, y1: f32) {
        let x0fx = (x0 * 4.0) as i32;
        let y0fx = (y0 * 4.0) as i32;
        let x1fx = (x1 * 4.0) as i32;
        let y1fx = (y1 * 4.0) as i32;
        debug_assert!(x0fx <= x1fx, "x1 must be greater or equal to x0");
        debug_assert!(y0fx <= y1fx, "y1 must be greater or equal to y0");
        debug_assert!(x0fx >= 0, "x0 must be positive");
        debug_assert!(y0fx >= 0, "y0 must be positive");
        debug_assert!(x1fx <= 0xFFF, "x1 must be less than 1024");
        debug_assert!(y1fx <= 0xFFF, "y1 must be less than 1024");
        unsafe {
            __rdpq_set_scissor(
                _carg(x0fx as u32, 0xFFF, 12) | _carg(y0fx as u32, 0xFFF, 0),
                _carg(x1fx as u32, 0xFFF, 12) | _carg(y1fx as u32, 0xFFF, 0),
            );
        }
    }
    #[doc = "Set a fixed Z value to be used instead of a per-pixel value (RDP command; SET_PRIM_DEPTH)\n\n When using z-buffering, normally the Z value used for z-buffering is\n calculated by interpolating the Z of each vertex onto each pixel.\n The RDP allows for usage of a fixed Z value instead, for special\n effects like particles or decals.\n\n This function allows to configure the RDP register that\n holds the fixed Z value. It is then necessary to activate this\n special RDP mode: either manually turning on SOM_ZSOURCE_PRIM via\n `rdpq_change_other_modes_raw`.\n\n For beginners, it is suggested to use the mode API instead, via\n `rdpq_mode_zoverride`.\n\n @param[in] prim_z     Fixed Z value (in range 0..0x7FFF)\n @param[in] prim_dz    Delta Z value (must be a signed power of two).\n                       Pass 0 initially, and increment to next power of two\n                       in case of problems with objects with the same Z.\n\n @note Pending further investigation of the exact usage of this function,\n       and specifically the prim_dz parameter, rdpq does not currently\n       offer a higher-level function (`rdpq_set_prim_depth`)."]
    #[inline]
    pub fn set_prim_depth_raw(&mut self, prim_z: u16, prim_dz: i16) {
        debug_assert!(prim_z <= 0x7FFF, "prim_z must be in [0..0x7FFF]");
        debug_assert_eq!(
            prim_dz & -prim_dz,
            if prim_dz >= 0 { prim_dz } else { -prim_dz },
            "prim_dz must be a power of 2"
        );
        unsafe {
            __rdpq_write8(
                RDPQ_CMD_SET_PRIM_DEPTH,
                0,
                _carg(prim_z, 0xFFFF, 16) | _carg(prim_dz as u32, 0xFFFF, 0),
            );
        }
    }
    #[doc = "Load a portion of a texture into TMEM (RDP command: LOAD_TILE)\n\nThis is the main command to load data from RDRAM into TMEM. It is\nnormally used to load a texture (or a portion of it), before using\nit for drawing.\n\n@note Beginners are advised to use the rdpq texture API (rdpq_tex.h), \nfor instance `rdpq_tex_upload` that takes care of everything required.\n\nBefore calling `rdpq_load_tile`, the tile must have been configured\nusing `rdpq_set_tile` to specify the TMEM address and pitch, and the \ntexture in RDRAM must have been set via `rdpq_set_texture_image`.\n\nIn addition to loading TMEM, this command also records into the\ntile descriptor the extents of the loaded texture (that is, the\ntexture coordinates), so that subsequence draw commands can still\nrefer to original texture's coordinates to draw. For instance,\nif you have a large 512x128 texture and you load only a small\nportion into TMEM, for instance the rectangle at coordinates\n(16,16) - (48,48), the RDP will remember (through the tile descriptor)\nthat the TMEM contains that specific rectangle, and subsequent\ntriangles or rectangles commands can specify S,T texture\ncoordinates within the range (16,16)-(48,48).\n\nIf the portion being loaded is consecutive in RDRAM (rather\nthan being a rectangle within a wider image), prefer using\n`rdpq_load_block` for increased performance.\n\n@param[in]   tile        Tile descriptor to use (TILE0-TILE7).\n@param[in]   s0          Upper-left X coordinate of the portion of the texture to load (integer or float).\nRange: 0-1024\n@param[in]   t0          Upper-left Y coordinate of the portion of the texture to load (integer or float),\nRange: 0-1024\n@param[in]   s1          Bottom-right X coordinate of the portion of the texture to load (integer or float),\nRange: 0-1024\n@param[in]   t1          Bottom-right Y coordinate of the portion of the texture to load (integer or float),\nRange: 0-1024\n\n@see `rdpq_tex_upload`\n@see `rdpq_set_texture_image`\n@see `rdpq_load_block`\n@see `rdpq_set_tile`\n@see `rdpq_load_tile_fx`"]
    #[inline]
    pub fn load_tile(&mut self, tile: Tile, s0: f32, t0: f32, s1: f32, t1: f32) {
        let s0 = (s0 * 4.0) as i32;
        let t0 = (t0 * 4.0) as i32;
        let s1 = (s1 * 4.0) as i32;
        let t1 = (t1 * 4.0) as i32;
        debug_assert!(
            s0 >= 0 && t0 >= 0 && s1 >= 0 && t1 >= 0,
            "texture coordinates must be positive"
        );
        debug_assert!(
            s0 < 1024 * 4 && t0 < 1024 * 4 && s1 < 1024 * 4 && t1 < 1024 * 4,
            "texture coordinates must be smaller than 1024"
        );
        self.load_tile_fx(tile, s0 as u16, t0 as u16, s1 as u16, t1 as u16);
    }
    #[doc = "Load a portion of a texture into TMEM -- fixed point version (RDP command: LOAD_TILE)\n\n This function is similar to `rdpq_load_tile`, but coordinates can be specified\n in fixed point format (0.10.2). Refer to `rdpq_load_tile` for increased performance\n\n @note Beginners are advised to use the rdpq texture API (rdpq_tex.h),\n for instance `rdpq_tex_upload` that takes care of everything required.\n\n\n @param[in]   tile        Tile descriptor to use (TILE0-TILE7).\n @param[in]   s0          Upper-left X coordinate of the portion of the texture to load (fx 0.10.2).\n                          Range: 0-4096\n @param[in]   t0          Upper-left Y coordinate of the portion of the texture to load (fx 0.10.2),\n                          Range: 0-4096\n @param[in]   s1          Bottom-right X coordinate of the portion of the texture to load (fx 0.10.2),\n                          Range: 0-4096\n @param[in]   t1          Bottom-right Y coordinate of the portion of the texture to load (fx 0.10.2),\n                          Range: 0-4096\n\n @see `rdpq_load_tile`\n @see `rdpq_tex_upload`"]
    #[inline]
    pub fn load_tile_fx(&mut self, tile: Tile, s0: u16, t0: u16, s1: u16, t1: u16) {
        unsafe {
            __rdpq_write8_syncchangeuse(
                RDPQ_CMD_LOAD_TILE,
                _carg(s0, 0xFFF, 12) | _carg(t0, 0xFFF, 0),
                _carg(tile, 0x7, 24) | _carg(s1 - 4, 0xFFF, 12) | _carg(t1 - 4, 0xFFF, 0),
                autosync_tmem(0) | tile.autosync(),
                tile.autosync(),
            );
        }
    }
    #[doc = "Load a palette of colors into TMEM (RDP command: LOAD_TLUT)\n\n This command is used to load a palette into TMEM. TMEM can hold up\n to 256 16-bit colors in total to be used as palette, and they must be\n stored in the upper half of TMEM. These colors are arranged as a single\n 256-color palette when drawing `FMT_CI8` images, or 16 16-colors palettes\n when drawing `FMT_CI4` images.\n\n Storage of colors in TMEM is a bit wasteful, as each color is replicated\n four times (in fact, 256 colors * 16-bit + 4 = 2048 bytes, which is\n in fact half of TMEM). This command should be preferred for palette\n loading as it automatically handles this replication.\n\n Loading a palette manually is a bit involved. It requires configuring\n the palette in RDRAM via `rdpq_set_texture_image`, and also configure a\n tile descriptor with the TMEM destination address (via `rdpq_set_tile`).\n Instead, prefer using the simpler rdpq texture API (rdpq_tex.h), via\n `rdpq_tex_upload_tlut`.\n\n @param[in] tile         Tile descriptor to use (TILE0-TILE7). This is used\n                         to extract the destination TMEM address (all other fields\n                         of the descriptor are ignored).\n @param[in] color_idx    Index of the first color to load into TMEM (0-255).\n                         This is a 16-bit offset into the RDRAM buffer\n                         set via `rdpq_set_texture_image`.\n @param[in] num_colors   Number of colors to load (1-256).\n\n @see `rdpq_tex_upload_tlut`"]
    #[inline]
    pub fn load_tlut_raw(&mut self, tile: Tile, color_idx: i32, num_colors: i32) {
        unsafe {
            __rdpq_write8_syncchangeuse(
                RDPQ_CMD_LOAD_TLUT,
                _carg(color_idx as u32, 0xFF, 14),
                _carg(tile, 0x7, 24) | _carg((color_idx + num_colors - 1) as u32, 0xFF, 14),
                autosync_tmem(0),
                tile.autosync(),
            );
        }
    }

    #[doc = "Configure the extents of a tile descriptor (RDP command: SET_TILE_SIZE)\n\nThis function allows to set the extents (s0,s1 - t0,t1) of a tile descriptor.\nNormally, it is not required to call this function because extents are\nautomatically configured when `rdpq_load_tile` is called to load contents\nin TMEM. This function is mostly useful when loading contents using\n`rdpq_load_block`, or when reinterpreting existing contents of TMEM.\n\nFor beginners, it is suggest to use the rdpq texture API (rdpq_tex.h)\nwhich automatically configures tile descriptors correctly: for instance,\n`rdpq_tex_upload`.\n\n@param[in] tile          Tile descriptor (TILE0-TILE7)\n@param[in] s0            Top-left X texture coordinate to store in the descriptor (integer or float).\nRange: 0-1024 (inclusive)\n@param[in] t0            Top-left Y texture coordinate to store in the descriptor (integer or float).\nRange: 0-1024 (inclusive)\n@param[in] s1            Bottom-right *exclusive* X texture coordinate to store in the descriptor (integer or float).\nRange: 0-1024 (inclusive)\n@param[in] t1            Bottom-right *exclusive* Y texture coordinate to store in the descriptor (integer or float).\nRange: 0-1024 (inclusive)\n\n@see `rdpq_tex_upload`\n@see `rdpq_set_tile_size_fx`"]
    #[inline]
    pub fn set_tile_size(&mut self, tile: Tile, s0: f32, t0: f32, s1: f32, t1: f32) {
        self.set_tile_size_fx(
            tile,
            (s0 * 4.0) as u16,
            (t0 * 4.0) as u16,
            (s1 * 4.0) as u16,
            (t1 * 4.0) as u16,
        );
    }
    #[doc = "Configure the extents of a tile descriptor -- fixed point version (RDP command: SET_TILE_SIZE)\n\n This function is similar to `rdpq_set_tile_size`, but coordinates must be\n specified using fixed point numbers (10.2).\n\n @param tile              Tile descriptor (TILE0-TILE7)\n @param[in] s0            Top-left X texture coordinate to store in the descriptor (fx 10.2)\n @param[in] t0            Top-left Y texture coordinate to store in the descriptor (fx 10.2)\n @param[in] s1            Bottom-right *exclusive* X texture coordinate to store in the descriptor (fx 10.2)\n @param[in] t1            Bottom-right *exclusive* Y texture coordinate to store in the descriptor (fx 10.2)\n\n @see `rdpq_tex_upload`\n @see `rdpq_set_tile_size`"]
    #[inline]
    pub fn set_tile_size_fx(&mut self, tile: Tile, s0: u16, t0: u16, s1: u16, t1: u16) {
        debug_assert!(
            s0 <= 1024 * 4 && t0 <= 1024 * 4 && s1 <= 1024 * 4 && t1 <= 1024 * 4,
            "texture coordinates must be smaller than 1024"
        );

        unsafe {
            __rdpq_write8_syncchange(
                RDPQ_CMD_SET_TILE_SIZE,
                _carg(s0, 0xFFF, 12) | _carg(t0, 0xFFF, 0),
                _carg(tile, 0x7, 24) | _carg(s1 - 4, 0xFFF, 12) | _carg(t1 - 4, 0xFFF, 0),
                tile.autosync(),
            );
        }
    }
    #[doc = "Low level function to load a texture image into TMEM in a single memory transfer"]
    #[inline]
    pub fn load_block_fx(&mut self, tile: Tile, s0: u16, t0: u16, num_texels: u16, dxt: u16) {
        unsafe {
            __rdpq_write8_syncchangeuse(
                RDPQ_CMD_LOAD_BLOCK,
                _carg(s0, 0xFFF, 12) | _carg(t0, 0xFFF, 0),
                _carg(tile, 0x7, 24) | _carg(num_texels - 1, 0xFFF, 12) | _carg(dxt, 0xFFF, 0),
                autosync_tmem(0),
                tile.autosync(),
            )
        }
    }
    #[doc = "Load a texture image into TMEM with a single contiguous memory transfer (RDP command: LOAD_BLOCK)\n\n This is a command alternative to `rdpq_load_tile` to load data from\n RDRAM into TMEM. It is faster than `rdpq_load_tile` but only allows\n to transfer a consecutive block of data; the block can cover multiple\n lines, but not a sub-rectangle of the texture image.\n\n @note Beginners are advised to use the rdpq texture API (rdpq_tex.h),\n for instance `rdpq_tex_upload` that takes care of everything required,\n including using `rdpq_load_block` for performance whenever possible.\n\n Before calling `rdpq_load_block`, the tile must have been configured\n using `rdpq_set_tile` to specify the TMEM address, and the texture\n in RDRAM must have been set via `rdpq_set_texture_image`.\n\n @note It is important to notice that the RDP will interpret the tile pitch\n       configured in the tile descriptor with a different semantic: it is\n       used as a number of texels that must be skipped between lines\n       in RDRAM. Normally, for a compact texture, it should then be set to zero\n       in the call to `rdpq_set_tile`. Instead, The *real* pitch of the texture\n       in TMEM must be provided to `rdpq_load_block` itself.\n\n After the call to `rdpq_load_block`, it is not possible to reuse the tile\n descriptor for performing a draw. So a new tile descriptor should be configured\n from scratch using `rdpq_set_tile`.\n\n The maximum number of texels that can be transferred by a single call is\n 2048. This allows to fill the TMEM only if a 16-bit or 32-bit texture is used.\n If you need to load a 4-bit or 8-bit texture, consider configuring the tile\n descriptor as 16-bit and adjusting the number of texels accordingly. For instance,\n to transfer a 80x64 4-bit texture (5120 texels), do the transfer as if it was a\n 20x64 16-bit texture (1280 texels). It doesn't matter if you lie to the RDP\n during the texture load: what it matters is that the tile descriptor that you will\n later use for drawing is configured with the correct pixel format.\n\n @param[in] tile          Tile descriptor (TILE0-TILE7)\n @param[in] s0            Top-left X texture coordinate to load\n @param[in] t0            Top-left Y texture coordinate to load\n @param[in] num_texels    Number of texels to load (max: 2048)\n @param[in] tmem_pitch    Pitch of the texture in TMEM (in bytes)\n\n @see `rdpq_load_tile`\n @see `rdpq_load_block_fx`\n @see `rdpq_set_tile`\n @see `rdpq_tex_upload`"]
    #[inline]
    pub fn load_block(&mut self, tile: Tile, s0: u16, t0: u16, num_texels: u16, tmem_pitch: u16) {
        debug_assert!(
            num_texels <= 2048,
            "invalid num_texels {num_texels}: must be smaller than 2048"
        );
        debug_assert_eq!(
            tmem_pitch % 8,
            0,
            "invalid tmem_pitch {tmem_pitch}: must be multiple of 8"
        );
        let words = tmem_pitch / 8;
        self.load_block_fx(tile, s0, t0, num_texels, (2048 + words - 1) / words);
    }
    #[doc = "Enqueue a RDP SET_TILE command (full version)\n @param[in] tile Tile descriptor index (0-7)\n @param[in] format Texture format for the tile. Cannot be 0. Should correspond to X_get_format in `surface_t` or `sprite_t`;\n @param[in] tmem_addr Address in tmem where the texture is (or will be loaded). Must be multiple of 8;\n @param[in] tmem_pitch Pitch of the texture in tmem in bytes. Must be multiple of 8. Should correspond to srtide in `surface_t`;\n @param[in] parms Additional optional parameters for the tile. Can be left NULL or all 0. More information about the struct is in `rdpq_tileparms_t`"]
    #[inline]
    pub fn set_tile(
        &mut self,
        tile: Tile,
        format: TexFormat,
        tmem_addr: i32,
        tmem_pitch: u16,
        parms: Option<&TileParms>,
    ) {
        #[inline]
        const fn automem_reuse(offset: u16) -> u32 {
            (0x4000 | (offset / 8)) as u32
        }
        let parms = parms
            .inspect(|parms| {
                let parms = &parms.0;
                debug_assert!(
                    parms.s.shift >= -5 && parms.s.shift <= 10,
                    "invalid s shift {}: must be in [-5..10]",
                    parms.s.shift
                );
                debug_assert!(
                    parms.t.shift >= -5 && parms.t.shift <= 10,
                    "invalid t shift {}: must be in [-5..10]",
                    parms.t.shift
                );
                debug_assert!(
                    parms.palette < 16,
                    "invalid palette {}: must be in [0..15]",
                    parms.palette
                );
            })
            .unwrap_or(&TileParms::EMPTY)
            .0;
        let tmem_addr = tmem_addr as u32;
        let (func, reuse, cmd_id, tmem_addr) =
            if (tmem_addr & (RDPQ_AUTOTMEM | automem_reuse(0))) != 0 {
                (
                    __rdpq_fixup_write8_syncchange as unsafe extern "C" fn(u32, u32, u32, u32),
                    (tmem_addr & automem_reuse(0)) != 0,
                    RDPQ_CMD_AUTOTMEM_SET_TILE,
                    tmem_addr & !(RDPQ_AUTOTMEM | automem_reuse(0)),
                )
            } else {
                debug_assert_eq!(
                    tmem_addr % 8,
                    0,
                    "invalid tmem_addr {tmem_addr}: must be multiple of 8"
                );
                (
                    __rdpq_write8_syncchange as _,
                    false,
                    RDPQ_CMD_SET_TILE,
                    tmem_addr / 8,
                )
            };
        debug_assert_eq!(
            tmem_pitch % 8,
            0,
            "invalid tmem_pitch {tmem_pitch}: must be multiple of 8"
        );
        unsafe {
            func(
                cmd_id,
                _carg(format, 0x1F, 19)
                    | _carg(reuse, 0x1, 18)
                    | _carg(tmem_pitch / 8, 0x1FF, 9)
                    | _carg(tmem_addr, 0x1FF, 0),
                _carg(tile, 0x7, 24)
                    | _carg(parms.palette, 0xF, 20)
                    | _carg(parms.t.clamp | (parms.t.mask == 0), 0x1, 19)
                    | _carg(parms.t.mirror, 0x1, 18)
                    | _carg(parms.t.mask, 0xF, 14)
                    | _carg(parms.t.shift as u8, 0xF, 10)
                    | _carg(parms.s.clamp | (parms.s.mask == 0), 0x1, 9)
                    | _carg(parms.s.mirror, 0x1, 8)
                    | _carg(parms.s.mask, 0xF, 4)
                    | _carg(parms.s.shift as u8, 0xF, 0),
                tile.autosync(),
            );
        }
    }
    #[doc = "Configure the auto-TMEM feature of `rdpq_set_tile`\n\n This function is used to manage the auto-TMEM allocation feature for\n `rdpq_set_tile`. It allows to keep track of the allocated space in TMEM,\n which can be a simplification. It is used by the rdpq_tex module\n (eg: `rdpq_tex_upload`).\n\n The feature works like this:\n   - First, start auto-TMEM via rdpq_set_tile_autotmem(0)\n   - Load a texture and configure a tile for it. When configuring the tile,\n     pass `RDPQ_AUTOTMEM` as tmem_addr. This will allocate the texture in the\n     first available space.\n   - Call `rdpq_set_tile_autotmem` again passing the number of used bytes in\n     TMEM. Notice that rdpq can't know this by itself.\n   - Continue loading the other textures/mipmaps just like before, with\n     `RDPQ_AUTOTMEM`.\n   - If the TMEM is full, a RSP assertion will be triggered.\n   - When you are done, call `rdpq_set_tile_autotmem` passing -1 to finish.\n     This allows reentrant calls to work, and also helps generating errors\n     in case of misuses.\n\n While this API might seem as a small simplification over manually tracking\n TMEM allocation, it might help modularizing the code, and also allows to\n record rspq blocks that handle texture loading without hardcoding the\n TMEM position.\n\n @note This function is part of the raw API. For a higher-level API on texture\n loading, see `rdpq_tex_upload`.\n\n @param tmem_bytes     0: begin, -1: end, >0: number of additional bytes\n                       that were used in TMEM.\n\n @see `rdpq_set_tile`\n @see `rdpq_tex_upload`"]
    #[inline]
    pub fn set_tile_autotmem(&mut self, tmem_bytes: i16) {
        unsafe { rdpq_set_tile_autotmem(tmem_bytes) }
    }
    #[doc = "Enqueue a SET_FILL_COLOR RDP command.\n\n This command is used to configure the color used by RDP when running in FILL mode\n (`rdpq_set_mode_fill`) and normally used by `rdpq_fill_rectangle`.\n\n Notice that `rdpq_set_mode_fill` automatically calls this function, because in general\n it makes no sense to configure the FILL mode without also setting a FILL color.\n\n @code{.c}\n      // Fill top half of the screen in red\n      rdpq_set_mode_fill(RGBA32(255, 0, 0, 0));\n      rdpq_fill_rectangle(0, 0, 320, 120);\n\n      // Fill bottom half of the screen in blue.\n      // No need to change mode again (it's already in fill mode),\n      // so just change the fill color.\n      rdpq_set_fill_color(RGBA32(0, 0, 255, 0));\n      rdpq_fill_rectangle(0, 120, 320, 240);\n @endcode\n\n @param[in]    color   The color to use to fill\n\n @see `rdpq_set_mode_fill`"]
    #[inline]
    pub fn set_fill_color(&mut self, c: Color) {
        unsafe { __rdpq_set_fill_color(c.into_u32()) }
    }
    #[doc = "Enqueue a SET_FILL_COLOR RDP command to draw a striped pattern.\n\n This command is similar to `rdpq_set_fill_color`, but allows to configure\n two colors, and creates a fill pattern that alternates horizontally between\n them every 2 pixels (creating vertical stripes).\n\n This command relies on a low-level hack of how RDP works in filling primitives,\n so there is no configuration knob: it only works with RGBA 16-bit target\n buffers, it only allows two colors, and the vertical stripes are exactly\n 2 pixel width.\n\n @param[in]   color1      Color of the first vertical stripe\n @param[in]   color2      Color of the second vertical stripe\n\n @see `rdpq_set_fill_color`\n"]
    #[inline]
    pub fn set_fill_color_stripes(&mut self, c1: Color, c2: Color) {
        unsafe {
            __rdpq_write8_syncchange(
                RDPQ_CMD_SET_FILL_COLOR,
                0,
                ((c1.into_u16() as u32) << 16) | c2.into_u16() as u32,
                AUTOSYNC_PIPE,
            )
        }
    }
    #[doc = "Set the RDP FOG blender register\n\n This function sets the internal RDP FOG register, part of the blender unit.\n As the name implies, this register is normally used as part of fog calculation,\n but it is actually a generic color register that can be used in custom\n blender formulas.\n\n Another similar blender register is the BLEND register, configured via\n `rdpq_set_blend_color`.\n\n See `RDPQ_BLENDER` and `RDPQ_BLENDER2` on how to configure\n the blender (typically, via `rdpq_mode_blender`).\n\n @param[in] color             Color to set the FOG register to\n\n @see `RDPQ_BLENDER`\n @see `RDPQ_BLENDER2`\n @see `rdpq_set_blend_color`\n @see `rdpq_mode_blender`"]
    #[inline]
    pub fn set_fog_color(&mut self, color: Color) {
        unsafe {
            __rdpq_write8_syncchange(RDPQ_CMD_SET_FOG_COLOR, 0, color.into_u32(), AUTOSYNC_PIPE)
        }
    }
    #[doc = "Set the RDP BLEND blender register\n\n This function sets the internal RDP BLEND register, part of the blender unit.\n As the name implies, this register is normally used as part of fog calculation,\n but it is actually a generic color register that can be used in custom\n blender formulas.\n\n Another similar blender register is the FOG register, configured via\n `rdpq_set_fog_color`.\n\n See `RDPQ_BLENDER` and `RDPQ_BLENDER2` on how to configure\n the blender (typically, via `rdpq_mode_blender`).\n\n @param[in] color             Color to set the BLEND register to\n\n @see `RDPQ_BLENDER`\n @see `RDPQ_BLENDER2`\n @see `rdpq_set_fog_color`\n @see `rdpq_mode_blender`"]
    #[inline]
    pub fn set_blend_color(&mut self, color: Color) {
        unsafe {
            __rdpq_write8_syncchange(RDPQ_CMD_SET_BLEND_COLOR, 0, color.into_u32(), AUTOSYNC_PIPE)
        }
    }
    #[doc = "Set the RDP PRIM combiner register (color only) (RDP command: SET_PRIM_COLOR)\n\n This function sets the internal RDP PRIM register, part of the\n color combiner unit. Naming aside, it is a generic color register that\n can be used in custom color combiner formulas.\n\n Another similar blender register is the ENV register, configured via\n `rdpq_set_env_color`.\n\n See `RDPQ_COMBINER1` and `RDPQ_COMBINER2` on how to configure\n the color combiner (typicall, via `rdpq_mode_combiner`).\n\n If you wish to set PRIM LOD or PRIM MIN LOD values of the PRIM register,\n see `rdpq_set_prim_lod_frac`, `rdpq_set_detail_factor` or `rdpq_set_prim_register_raw`.\n\n @param[in] color             Color to set the PRIM register to\n\n @see `RDPQ_COMBINER1`\n @see `RDPQ_COMBINER2`\n @see `rdpq_set_env_color`\n @see `rdpq_mode_combiner`\n @see `rdpq_set_prim_lod_frac`\n @see `rdpq_set_detail_factor`\n @see `rdpq_set_prim_register_raw`\n"]
    #[inline]
    pub fn set_prim_color(&mut self, color: Color) {
        unsafe {
            __rdpq_fixup_write8_syncchange(
                RDPQ_CMD_SET_PRIM_COLOR_COMPONENT,
                0,
                color.into_u32(),
                0,
            )
        }
    }
    #[doc = "Set the detail/sharpen blending factor (RDP command: SET_PRIM_COLOR (partial))\n\n This function sets the internal minimum clamp for LOD fraction, that is used for\n determining the interpolation blend factor of a detail or sharpen texture at high\n magnification.\n\n Range is [0..1] where 0 means no influence, and 1 means full influence.\n The range is internally inverted and converted to [0..31] for the RDP hardware\n\n @param[in] value             Value to set the register to in range [0..1]\n\n @see `RDPQ_COMBINER1`\n @see `RDPQ_COMBINER2`\n @see `rdpq_mode_combiner`\n"]
    #[inline]
    pub fn set_detail_factor(&mut self, value: f32) {
        let conv = ((1.0 - value) * 31.0) as i8 as i32 as u32;
        unsafe {
            __rdpq_fixup_write8_syncchange(
                RDPQ_CMD_SET_PRIM_COLOR_COMPONENT,
                ((conv & 0x1F) << 8) | (2 << 16),
                0,
                0,
            )
        }
    }
    #[doc = "Set the RDP PRIM LOD FRAC combiner register (RDP command: SET_PRIM_COLOR (partial))\n\n This function sets the internal Level of Detail fraction for primitive register,\n that is used for custom linear interpolation between any two colors in a Color Combiner.\n\n See `RDPQ_COMBINER1` and `RDPQ_COMBINER2` on how to configure\n the color combiner (typicall, via `rdpq_mode_combiner`).\n\n If you wish to set PRIM MIN LOD value, see `rdpq_set_detail_factor`.\n\n @param[in] value             Value to set the PRIM LOD register to in range [0..255]\n\n @see `RDPQ_COMBINER1`\n @see `RDPQ_COMBINER2`\n @see `rdpq_mode_combiner`\n @see `rdpq_set_detail_factor`\n"]
    #[inline]
    pub fn set_prim_lod_frac(&mut self, value: u8) {
        unsafe {
            __rdpq_fixup_write8_syncchange(
                RDPQ_CMD_SET_PRIM_COLOR_COMPONENT,
                value as u32 | (1 << 16),
                0,
                0,
            );
        }
    }
    #[doc = "Set the RDP PRIM combiner register (raw version) (RDP command: SET_PRIM_COLOR)\n\n This function sets the internal RDP PRIM register, part of the\n color combiner unit. Naming aside, it is a generic color register that\n can be used in custom color combiner formulas.\n\n It also sets the PRIM LOD FRAC and PRIM MIN LOD FRAC values for the PRIM register\n For more information, see `rdpq_set_prim_lod_frac`, `rdpq_set_detail_factor`.\n\n Another similar blender register is the ENV register, configured via\n `rdpq_set_env_color`.\n\n See `RDPQ_COMBINER1` and `RDPQ_COMBINER2` on how to configure\n the color combiner (typicall, via `rdpq_mode_combiner`).\n\n If you wish to set PRIM COLOR or PRIM LOD or PRIM MIN LOD values individually,\n see `rdpq_set_prim_lod_frac`, `rdpq_set_detail_factor` or `rdpq_set_prim_color`.\n\n @param[in] color             Color to set the PRIM register to\n @param[in] minlod            Minimum LOD fraction to set the PRIM register to\n @param[in] primlod           Primitive LOD fraction to set the PRIM register to\n\n @see `RDPQ_COMBINER1`\n @see `RDPQ_COMBINER2`\n @see `rdpq_set_env_color`\n @see `rdpq_set_prim_color`\n @see `rdpq_set_prim_lod_frac`\n @see `rdpq_set_detail_factor`\n"]
    #[inline]
    pub fn set_prim_register_raw(&mut self, color: Color, minlod: u8, primlod: u8) {
        unsafe {
            __rdpq_write8(
                RDPQ_CMD_SET_PRIM_COLOR,
                ((minlod as u32 & 0x1F) << 8) | primlod as u32,
                color.into_u32(),
            );
        }
    }
    #[doc = "Set the RDP ENV combiner register (RDP command: SET_ENV_COLOR)\n\n This function sets the internal RDP ENV register, part of the\n color combiner unit. Naming aside, it is a generic color register that\n can be used in custom color combiner formulas.\n\n Another similar blender register is the PRIM register, configured via\n `rdpq_set_prim_color`.\n\n See `RDPQ_COMBINER1` and `RDPQ_COMBINER2` on how to configure\n the color combiner (typically, via `rdpq_mode_combiner`).\n\n @param[in] color             Color to set the ENV register to\n\n @see `RDPQ_COMBINER1`\n @see `RDPQ_COMBINER2`\n @see `rdpq_set_prim_color`\n @see `rdpq_mode_combiner`\n"]
    #[inline]
    pub fn set_env_color(&mut self, color: Color) {
        unsafe {
            __rdpq_write8_syncchange(RDPQ_CMD_SET_ENV_COLOR, 0, color.into_u32(), AUTOSYNC_PIPE);
        }
    }
    #[doc = "Configure the framebuffer to render to (RDP command: SET_COLOR_IMAGE)\n\n This command is used to specify the render target that the RDP will draw to.\n\n Calling this function also automatically configures scissoring (via\n `rdpq_set_scissor`), so that all draw commands are clipped within the buffer,\n to avoid overwriting memory around it. Use `rdpq_config_disable(RDPQ_CFG_AUTOSCISSOR)`\n if you need to disable this behavior.\n\n If you have a raw pointer instead of a `surface_t`, you can use `surface_make` to create\n a temporary surface structure to pass the information to `rdpq_set_color_image`.\n\n If the passed surface is NULL, rdpq will be detached from the render target. If\n a drawing command is issued without a render target, it will be silently\n ignored (but the validator will flag it as an error).\n\n The only valid formats for a surface to be used as a render target are: `FMT_RGBA16`,\n `FMT_RGBA32`, and `FMT_I8`.\n\n @param[in]  surface   Surface to set as render target\n\n @see `rdpq_set_color_image_raw`"]
    #[inline]
    pub fn set_color_image(&mut self, surface: &mut Surface) {
        unsafe {
            rdpq_set_color_image(surface.as_raw_mut());
        }
    }
    #[doc = "Configure the Z-buffer to use (RDP command: SET_Z_IMAGE)\n\n This commands is used to specify the Z-buffer that will be used by RDP for the next\n rendering commands.\n\n The surface must have the same width and height of the surface set as render target\n (via `rdpq_set_color_image` or `rdpq_set_color_image_raw`). The color format should be\n FMT_RGBA16, even though Z values will be written to it.\n\n If the passed surface is NULL, rdpq will be detached from the Z buffer. If\n a drawing command using Z is issued without a Z buffer, the behaviour will be\n undefined (but the validator will flag it as an error).\n\n @param surface      Surface to set as Z buffer\n\n @see `rdpq_set_z_image_raw`"]
    #[inline]
    pub fn set_z_image(&mut self, surface: &mut Surface) {
        unsafe {
            rdpq_set_z_image(surface.as_raw_mut());
        }
    }
    #[doc = "Configure the texture to use (RDP command: SET_TEX_IMAGE)\n\n This commands is used to specify the texture image that will be used by RDP for\n the next load commands (`rdpq_load_tile` and `rdpq_load_block`).\n\n The surface must have the same width and height of the surface set as render target\n (via `rdpq_set_color_image` or `rdpq_set_color_image_raw`). The color format should be\n `FMT_RGBA16`, even though Z values will be written to it.\n\n @param surface      Surface to set as texture\n\n @see `rdpq_set_texture_image_raw`"]
    #[inline]
    pub fn set_texture_image(&mut self, surface: &Surface) {
        unsafe {
            rdpq_set_texture_image(surface.as_raw());
        }
    }
    #[doc = "Low-level version of `rdpq_set_color_image`, with address lookup capability.\n\n This is a low-level version of `rdpq_set_color_image`, that exposes the address lookup\n capability. It allows to either pass a direct buffer, or to use a buffer already stored\n in the address lookup table, adding optionally an offset. See `rdpq_set_lookup_address`\n for more information.\n\n RDP a physical constraint of 64-byte alignment for render targets, so make sure to respect\n that while configuring a buffer. The validator will flag such a mistake.\n\n @param index        Index in the rdpq lookup table of the buffer to set as render target.\n @param offset       Byte offset to add to the buffer stored in the lookup table. Notice that\n                     if index is 0, this can be a physical address to a buffer (use\n                     `PhysicalAddr` to convert a C pointer to a physical address).\n @param format       Format of the buffer. Only `FMT_RGBA32`, `FMT_RGBA16` or `FMT_I8` are\n                     possible to use as a render target.\n @param width        Width of the buffer in pixel\n @param height       Height of the buffer in pixel\n @param stride       Stride of the buffer in bytes (length of a row)\n\n @see `rdpq_set_color_image`\n @see `rdpq_set_lookup_address`"]
    #[inline]
    pub unsafe fn set_color_image_raw(
        &mut self,
        index: u8,
        offset: u32,
        format: TexFormat,
        width: u32,
        height: u32,
        stride: u32,
    ) {
        debug_assert!(
            matches!(
                format,
                TexFormat::RGBA32 | TexFormat::RGBA16 | TexFormat::I8 | TexFormat::CI8
            ),
            "Image format is not supported as color image: {format}\nIt must be RGBA32, RGBA16, I8 or CI8"
        );
        debug_assert!(
            index <= 15,
            "Lookup address index out of range [0,15]: {index}"
        );

        unsafe {
            __rdpq_set_color_image(
                _carg(format, 0x1F, 19)
                    | _carg(format.bytes_to_pixels(stride) - 1, 0x3FF, 0)
                    | _carg(height - 1, 0x1FF, 10),
                _carg(index, 0xF, 28) | (offset & 0xFFFFFF) | _carg((height - 1) >> 9, 0x1, 31),
                _carg(0u32, 0xFFF, 12) | _carg(0u32, 0xFFF, 0), // for set_scissor
                _carg(width * 4, 0xFFF, 12) | _carg(height * 4, 0xFFF, 0),
            ); // for set_scissor
        }
    }
    #[doc = "Low-level version of `rdpq_set_z_image`, with address lookup capability.\n\n This is a low-level version of `rdpq_set_z_image`, that exposes the address lookup\n capability. It allows to either pass a direct buffer, or to use a buffer already stored\n in the address lookup table, adding optionally an offset. See `rdpq_set_lookup_address`\n for more information.\n\n RDP a physical constraint of 64-byte alignment for render targets, so make sure to respect\n that while configuring a buffer. The validator will flag such a mistake.\n\n @param index        Index in the rdpq lookup table of the buffer to set as render target.\n @param offset       Byte offset to add to the buffer stored in the lookup table. Notice that\n                     if index is 0, this can be a physical address to a buffer (use\n                     `PhysicalAddr` to convert a C pointer to a physical address).\n\n @see `rdpq_set_z_image`\n @see `rdpq_set_lookup_address`"]
    #[inline]
    pub unsafe fn set_z_image_raw(&mut self, index: u8, offset: u32) {
        debug_assert!(
            index <= 15,
            "Lookup address index out of range [0,15]: {index}"
        );
        unsafe {
            __rdpq_fixup_write8_syncchange(
                RDPQ_CMD_SET_Z_IMAGE,
                0,
                _carg(index, 0xF, 28) | (offset & 0xFFFFFF),
                AUTOSYNC_PIPE,
            );
        }
    }
    #[doc = "Low-level version of `rdpq_set_texture_image`, with address lookup capability.\n\n This is a low-level version of `rdpq_set_texture_image`, that exposes the address lookup\n capability. It allows to either pass a direct buffer, or to use a buffer already stored\n in the address lookup table, adding optionally an offset. See `rdpq_set_lookup_address`\n for more information.\n\n RDP a physical constraint of 8-byte alignment for textures, so make sure to respect\n that while configuring a buffer. The validator will flag such a mistake.\n\n @param index        Index in the rdpq lookup table of the buffer to set as texture image.\n @param offset       Byte offset to add to the buffer stored in the lookup table. Notice that\n                     if index is 0, this can be a physical address to a buffer (use\n                     `PhysicalAddr` to convert a C pointer to a physical address).\n @param format       Format of the texture (`tex_format_t`)\n @param width        Width of the texture in pixel (max 1024)\n @param height       Height of the texture in pixel (max 1024)\n\n @see `rdpq_set_texture_image`\n @see `rdpq_set_lookup_address`"]
    #[inline]
    pub unsafe fn set_texture_image_raw(
        &mut self,
        index: u8,
        offset: u32,
        format: TexFormat,
        width: u16,
        height: u16,
    ) {
        debug_assert!(
            width <= 1024,
            "Texture width out of range [1,1024]: {width}"
        );
        debug_assert!(
            height <= 1024,
            "Texture height out of range [1,1024]: {height}"
        );
        debug_assert!(
            index <= 15,
            "Lookup address index out of range [0,15]: {index}"
        );
        // NOTE: we also encode the texture height in the command (split in two halves...)
        // to help the validator to a better job. The RDP hardware ignores those bits.
        unsafe {
            __rdpq_fixup_write8_syncchange(
                RDPQ_CMD_SET_TEXTURE_IMAGE,
                _carg(format, 0x1F, 19) | _carg(width - 1, 0x3FF, 0) | _carg(height - 1, 0x1FF, 10),
                _carg(index, 0xF, 28) | (offset & 0xFFFFFF) | _carg((height - 1) >> 9, 0x1, 31),
                AUTOSYNC_PIPE,
            );
        }
    }
    #[doc = "Load a block of memory to TMEM with a single contiguous memory transfer.\n\n Loads data from RDRAM to TMEM but takes in byte offsets and sizes unlike `rdpq_load_block`.\n\n @param[in] offset    Destination TMEM offset\n @param[in] buffer    Pointer to data in RDRAM to load\n @param[in] size      Number of bytes to load (max: 4096)\n\n @see `rdpq_load_block`\n @see `rdpq_load_block_fx`"]
    #[inline]
    pub fn load_block_linear(&mut self, offset: i32, buffer: &[u8]) {
        let addr = crate::n64::physical_addr(unsafe {
            NonNull::new_unchecked(buffer.as_ptr() as *mut u8)
        });
        let size = buffer.len();
        assert_eq!(
            offset & 7,
            0,
            "invalid TMEM offset {offset}: must be 8-byte aligned"
        );
        assert_eq!(
            addr & 7,
            0,
            "invalid buffer: 0x{addr:08x}, must be 8-byte aligned"
        );
        assert_eq!(size & 7, 0, "invalid size {size}: must be a multiple of 8");
        assert!(size <= 4096, "invalid size {size}: must fit TMEM");
        unsafe {
            self.set_texture_image_raw(0, addr, TexFormat::RGBA16, 8, (size / 8) as u16);
        }
        self.set_tile(Tile::INTERNAL, TexFormat::RGBA16, offset, 0, None);
        self.load_block(Tile::INTERNAL, 0, 0, (size / 2) as u16, 16);
    }
    #[doc = "Store an address into the rdpq lookup table\n\n This function is for advanced usages, it is not normally required to call it.\n\n This function modifies the internal RDPQ address lookup table, by storing\n an address into on of the available slots.\n\n The lookup table is used to allow for an indirect access to surface pointers.\n For instance, some library code might want to record a block that manipulates\n several surfaces, but without saving the actual surface pointers within the\n block. Instead, all commands referring to a surface, will actually refer to\n an index into the lookup table. The caller of the block will then store\n the actual buffer pointers in the table, before playing back the block.\n\n While recording, you can create a placeholder surface via `surface_make_placeholder` or\n `surface_make_placeholder_linear` that is just an \"index\" into the lookup\n table.\n\n @code{.c}\n      // Create placeholder surfaces with indices 3 and 4\n      surface_t tex1 = surface_make_placeholder_linear(3, FMT_RGBA16, 32, 32);\n      surface_t tex2 = surface_make_placeholder_linear(4, FMT_RGBA16, 32, 32);\n\n      // Start recording a block.\n      rspq_block_begin();\n      rdpq_set_mode_standard();\n\n      // Load texture from lookup table (slot 3) and draw it to the screen\n      rdpq_set_texture_image(&tex1);\n      rdpq_load_tile(0, 0, 32, 32);\n      rdpq_texture_rectangle(0, 0, 32, 32);\n\n      // Load texture from lookup table (slot 4) and draw it to the screen\n      rdpq_set_texture_image(&tex2);\n      rdpq_load_tile(0, 0, 32, 32);\n      rdpq_texture_rectangle(32, 0, 64, 32);\n\n      rspq_block_t *bl = rspq_block_end();\n\n      [...]\n\n      // Set two real textures into the the lookup table and call the block\n      rdpq_set_lookup_address(3, robot->buffer);\n      rdpq_set_lookup_address(4, dragon->buffer);\n      rspq_block_run(bl);\n @endcode\n\n @note RDP has some alignment constraints: color and Z buffers must be 64-byte aligned,\n       and textures must be 8-byte aligned.\n\n @param index           Index of the slot in the table. Available slots are 1-15\n                        (slot 0 is reserved).\n @param rdram_addr      Pointer of the buffer to store into the address table.\n"]
    #[inline]
    pub unsafe fn set_lookup_address(&mut self, index: u8, rdram_addr: NonNull<u8>) {
        debug_assert!(
            index > 0 && index <= 15,
            "Lookup address index out of range [1,15]: {index}"
        );
        unsafe {
            __rdpq_write8(
                RDPQ_CMD_SET_LOOKUP_ADDRESS,
                (index as u32) << 2,
                crate::n64::physical_addr(rdram_addr),
            )
        }
    }
    #[doc = "Schedule a RDP SYNC_PIPE command.\n\n This command must be sent before changing the RDP pipeline configuration (eg: color\n combiner, blender, colors, etc.) if the RDP is currently drawing.\n\n Normally, you do not need to call this function because rdpq automatically\n emits sync commands whenever necessary. You must call this function only\n if you have disabled autosync for SYNC_PIPE (see `RDPQ_CFG_AUTOSYNCPIPE`).\n\n @note No software emulator currently requires this command, so manually\n       sending SYNC_PIPE should be developed on real hardware."]
    #[inline]
    pub fn sync_pipe(&mut self) {
        unsafe { rdpq_sync_pipe() }
    }
    #[doc = "Schedule a RDP SYNC_TILE command.\n\n This command must be sent before changing a RDP tile configuration if the\n RDP is currently drawing using that same tile.\n\n Normally, you do not need to call this function because rdpq automatically\n emits sync commands whenever necessary. You must call this function only\n if you have disabled autosync for SYNC_TILE (see `RDPQ_CFG_AUTOSYNCTILE`).\n\n @note No software emulator currently requires this command, so manually\n       sending SYNC_TILE should be developed on real hardware."]
    #[inline]
    pub fn sync_tile(&mut self) {
        unsafe { rdpq_sync_tile() }
    }
    #[doc = "Schedule a RDP SYNC_LOAD command.\n\n This command must be sent before loading an area of TMEM if the\n RDP is currently drawing using that same area.\n\n Normally, you do not need to call this function because rdpq automatically\n emits sync commands whenever necessary. You must call this function only\n if you have disabled autosync for SYNC_LOAD (see `RDPQ_CFG_AUTOSYNCLOAD`).\n\n @note No software emulator currently requires this command, so manually\n       sending SYNC_LOAD should be developed on real hardware."]
    #[inline]
    pub fn sync_load(&mut self) {
        unsafe { rdpq_sync_load() }
    }
    #[doc = "Schedule a RDP SYNC_FULL command and register a callback when it is done.\n\n This function schedules a RDP SYNC_FULL command into the RSP queue. This\n command basically forces the RDP to finish drawing everything that has been\n sent to it before it, and then generate an interrupt when it is done.\n\n This is normally useful at the end of the frame. For instance, it is used\n internally by `rdpq_detach_wait` to make sure RDP is finished drawing on\n the target display before detaching it.\n\n The function can be passed an optional callback that will be called\n when the RDP interrupt triggers. This can be useful to perform some operations\n asynchronously.\n\n @param      callback  A callback to invoke under interrupt when the RDP\n                       is finished drawing, or NULL if no callback is necessary.\n @param      arg       Opaque argument that will be passed to the callback.\n\n @see `rspq_wait`\n @see `rdpq_fence`\n"]
    #[inline]
    pub fn sync_full<T: crate::n64::InterruptArg>(&mut self, func: T::Fn, data: T) {
        unsafe { rdpq_sync_full(Some(T::cast_fn(func)), data.into_ptr()) }
    }
    #[doc = "Low-level function to set the rendering mode register.\n\n This function enqueues a low-level SET_OTHER_MODES RDP command that changes\n the RDP render mode, setting it to a new value\n\n This function is very low level and requires very good knowledge of internal\n RDP state management. Moreover, it completely overwrites any existing\n configuration for all bits, so it must be used with caution within a block.\n\n @note If possible, prefer using the RDPQ mode API (defined in rdpq_mode.h),\n that expose a higher level API for changing the RDP modes\n\n @param      mode     The new render mode. See the RDP_RM\n"]
    #[inline]
    pub fn set_other_modes_raw(&mut self, mode: SOM) {
        let mode = mode.bits();
        unsafe { __rdpq_set_other_modes((mode >> 32) as u32 & 0x00FFFFFF, mode as u32) }
    }
    #[doc = "Low-level function to partly change the rendering mode register.\n\n This function is very low level and requires very good knowledge of internal\n RDP state management.\n\n It allows to partially change the RDP render mode register, enqueuing a\n command that will modify only the requested bits. This function\n is to be preferred to `rdpq_set_other_modes_raw` as it preservers existing\n render mode for all the other bits, so it allows for easier composition.\n\n @note If possible, prefer using the RDPQ mode API (defined in rdpq_mode.h),\n that expose a higher level API for changing the RDP modes\n\n @param[in] mask          Mask of bits of the SOM register that must be changed\n @param[in] val           New value for the bits selected by the mask.\n"]
    #[inline]
    pub fn change_other_modes_raw(&mut self, mask: SOMMask, val: SOM) {
        let mask = mask.bits();
        let hi = (mask >> 32) as u32;
        let lo = mask as u32;
        let val = val.bits();
        unsafe {
            if hi != 0 {
                __rdpq_change_other_modes(0, !hi, (val >> 32) as u32);
            }
            if lo != 0 {
                __rdpq_change_other_modes(4, !lo, val as u32);
            }
        }
    }
    #[doc = "Read the current render mode register.\n\n This function executes a full sync (`rspq_wait`) and then extracts the\n current raw render mode from the RSP state. This should be used only\n for debugging purposes.\n\n @return     THe current value of the render mode register."]
    #[inline]
    pub fn other_modes_raw(&mut self) -> SOM {
        SOM::from_bits_retain(unsafe { rdpq_get_other_modes_raw() })
    }
    #[doc = "Low-level function to change the RDP combiner.\n\n This function enqueues a low-level SET_COMBINE RDP command that changes\n the RDP combiner, setting it to a new value.\n You can use `RDPQ_COMBINER1` and `RDPQ_COMBINER2` to create\n the combiner settings for respectively a 1-pass or 2-pass combiner.\n\n @note Prefer using `rdpq_mode_combiner` (part of the RDPQ mode API), as it better\n handles integration with other render mode changes.\n\n @param      comb     The new combiner setting\n\n @see `rdpq_mode_combiner`\n @see `RDPQ_COMBINER1`\n @see `RDPQ_COMBINER2`\n"]
    #[inline]
    pub fn set_combiner_raw(&mut self, comb: u64) {
        unsafe {
            __rdpq_write8_syncchange(
                RDPQ_CMD_SET_COMBINE_MODE_RAW,
                (comb >> 32) as u32 & 0x00FFFFFF,
                comb as u32,
                AUTOSYNC_PIPE,
            );
        }
    }
    #[doc = "Read the current combiner register.\n\n This function executes a full sync (`rspq_wait`) and then extracts the\n current raw combiner from the RSP state. This should be used only\n for debugging purposes.\n\n @return     THe current value of the combiner register."]
    #[inline]
    pub fn get_combiner_raw(&self) -> u64 {
        unsafe { rdpq_get_combiner_raw() }
    }
    #[doc = "Add a fence to synchronize RSP with RDP commands.\n\n This function schedules a fence in the RSP queue that makes RSP waits until\n all previously enqueued RDP commands have finished executing. This is useful\n in the rare cases in which you need to post-process the output of RDP with RSP\n commands.\n\n Notice that the RSP will spin-lock waiting for RDP to become idle, so, if\n possible, call rdpq_fence as late as possible, to allow for parallel RDP/RSP\n execution for the longest possible time.\n\n Notice that this does not block the CPU in any way; the CPU will just\n schedule the fence command in the RSP queue and continue execution. If you\n need to block the CPU until the RDP is done, check `rspq_wait` or `rdpq_sync_full`\n instead.\n\n @see `rdpq_sync_full`\n @see `rspq_wait`"]
    #[inline]
    pub fn fence(&self) {
        unsafe { rdpq_fence() }
    }
    #[doc = "Send to the RDP a buffer of RDP commands from RDRAM\n\n This command can be used to execute raw RDP commands from RDRAM. It is\n normally not necessary to call this function as normal rdpq functions will\n simply enqueue the commands in the RSP queue, but there can be cases\n where commands have been prepared in RAM somehow (especially, for compatibility\n with existing code that assembled RDP commands in RDRAM, or to playback\n RDP command lists prepared with offline tools).\n\n This function fully interoperates with the rest of RDPQ, so you can freely\n intermix it with standard rdpq calls.\n\n @param buffer        Pointer to the buffer containing RDP commands\n @param size          Size of the buffer, in bytes (must be a multiple of 8)\n\n @note This function cannot be called within a block."]
    #[inline]
    pub unsafe fn exec(&mut self, buffer: &[u64]) {
        unsafe { rdpq_exec(buffer.as_ptr() as _, buffer.len() as _) }
    }
    #[doc = "Enqueue a callback that will be called after the RSP and the RDP have\n        finished processing all commands enqueued until now.\n\n This function is similar to `rspq_call_deferred`, but it also guarantees\n that the callback is called after the RDP has finished processing all\n commands enqueued until now.\n\n For example:\n\n @code{.c}\n      // Draw a green rectangle\n      rdpq_mode_set_fill(RGBA(0,255,0,0));\n      rdpq_fill_rectangle(10, 10, 100, 100);\n\n      // Enqueue a callback. The callback is guaranteed to be called\n      // after the RSP has finished prepared the RDP command list for the\n      // filled rectangle. It is possible that the RDP would still\n      // be processing the rectangle when the callback is called.\n      rspq_call_deferred(my_callback1, NULL);\n\n      // Enqueue a callback. The callback is guaranteed to be called\n      // after the rectangle has been fully drawn to the target buffer, so\n      // that for instance the callback could readback the green pixels.\n      rdpq_call_deferred(my_callback2, NULL);\n @endcode\n\n @param func          Callback function to call\n @param arg           Argument to pass to the callback function"]
    #[inline]
    pub fn call_deferred<T: crate::n64::InterruptArg>(&mut self, func: T::Fn, data: T) {
        unsafe { rdpq_call_deferred(Some(T::cast_fn(func)), data.into_ptr()) }
    }
    #[doc = "Enqueue a RSP command that also generates RDP commands.\n\nThis function is similar to `rspq_write`: it enqueues a RSP command in the\nRSP command queue for later execution by RSP. The main difference is that\nthis macro also declares that the RSP command is going to generate RDP\ncommands as part of its execution.\n\nRSP commands in overlays can generate RDP commands by including rsp_rdqp.inc\nand calling RDPQ_Send (or RDPQ_Write8 / RDPQ_Write16 / RDPQ_Finalize). If\nthey do, they must enqueued using `rdpq_write` instead of `rspq_write`.\n\nIt is important to know that the RSP command is going to generate RDP commands\nbecause the space for them needs to be allocated in the static buffer in\nblocks. When wrongly using `rspq_write` instead of `rdpq_write`, the command\nwill work correctly outside of blocks but might fail in surprising ways\nwhen called within blocks.\n\nIn some cases, it is not possible to know beforehand how many RDP commands \nwill be generated. In these case, @p num_rdp_commands should be the maximum\npossible value in words. If the number is quite high and potentially\nunbounded, pass the special value `-1`.\n\n@param num_rdp_commands    Maximum number of RDP 8-byte commands that will be\ngenerated by the RSP command. Use -1 if the number\nis unbounded and potentially high. \n@param ovl_id              ID of the overlay for the command (see `rspq_write`)\n@param cmd_id              ID of the command (see `rspq_write`)\n\n@see `rspq_write`\n\n@note Some RDP commands are made of multiple 64 bit words. For the purpose\nof `rdpq_write`, please treat @p num_rdp_commands as it was the\n\"number of 64-bit words\". So for instance if the RSP command generates\na single RDP TEXTURE_RECTANGLE command, pass 2 as @p num_rdp_commands."]
    #[inline(always)]
    pub unsafe fn rdp_write<const N: usize>(
        &mut self,
        num_rdp_commands: i32,
        ovl_id: u32,
        cmd_id: u32,
        args: [u32; N],
    ) {
        unsafe {
            if num_rdp_commands != 0 && !rspq_block.is_null() {
                __rdpq_block_reserve(num_rdp_commands);
            }
            crate::rspq::RspQ(()).write(ovl_id, cmd_id, args);
        }
    }
}

impl<'r> RdpQ<'r> {
    #[doc = "Attach the RDP to a color surface (and optionally a Z buffer)\n\n This function configures the new render targets the RDP will draw to. It accepts\n both a color buffer and optionally a Z buffer, both of which in terms of\n surface_t pointers.\n\n For instance, it can be used with framebuffers acquired by calling `display_get`,\n or to render to an offscreen buffer created with `surface_alloc` or `surface_make`.\n\n This function should be called before any rendering operations to ensure that the RDP\n has a valid render target to operate on. It also resets the scissor rectangle\n to match the buffer being passed, so that the whole buffer will be writable\n after attaching to it.\n\n The previous render targets are stored away in a small stack, so that they can be\n restored later when `rdpq_detach` is called. This allows to temporarily switch\n rendering to an offscreen surface, and then restore the main render target.\n\n @param[in] surf_color\n            The surface to render to. Supported formats are: `FMT_RGBA32`, `FMT_RGBA16`,\n            `FMT_CI8`, `FMT_I8`.\n @param[in] surf_z\n            The Z-buffer to render to (can be NULL if no Z-buffer is required).\n            The only supported format is `FMT_RGBA16`.\n\n @see `display_get`\n @see `surface_alloc`"]
    #[must_use]
    #[inline]
    pub fn attach<'t>(
        &'t mut self,
        surf_color: &'t mut Surface,
        surf_z: Option<&'t mut Surface>,
    ) -> Attachment<'t, 'r> {
        unsafe {
            crate::sys::rdpq_attach::rdpq_attach(
                surf_color.as_raw_mut(),
                surf_z
                    .map(|s| s.as_raw_mut() as *mut _)
                    .unwrap_or_else(core::ptr::null_mut),
            )
        }
        Attachment(PhantomData)
    }
    #[doc = "Attach the RDP to a surface and clear it\n\n This function is similar to `rdpq_attach`, but it also clears the surface\n to full black (color 0) immediately after attaching. If a z-buffer is\n specified, it is also cleared (to `ZBUF_MAX`).\n\n This function is just a shortcut for calling `rdpq_attach`, `rdpq_clear` and\n `rdpq_clear_z`.\n\n @param[in] surf_color\n            The surface to render to.\n @param[in] surf_z\n            The Z-buffer to render to (can be NULL if no Z-buffer is required).\n\n @see `display_get`\n @see `surface_alloc`\n @see `rdpq_clear`\n @see `rdpq_clear_z`"]
    #[must_use]
    #[inline]
    pub fn attach_clear<'t>(
        &'t mut self,
        surf_color: &'t mut Surface,
        surf_z: Option<&'t mut Surface>,
    ) -> Attachment<'t, 'r> {
        unsafe {
            crate::sys::rdpq_attach::rdpq_attach_clear(
                surf_color.as_raw_mut(),
                surf_z
                    .map(|s| s.as_raw_mut() as *mut _)
                    .unwrap_or_else(core::ptr::null_mut),
            )
        }
        Attachment(PhantomData)
    }
    #[doc = "Clear the current render target with the specified color.\n\n Note that this function will respect the current scissor rectangle, if\n configured.\n\n @param[in] color\n            Color to use to clear the surface"]
    pub fn clear(&mut self, color: Color) {
        unsafe { crate::sys::rdpq_attach::__rdpq_clear(&color.into_raw()) }
    }
    #[doc = "Reset the current Z buffer to a given value.\n\n This function clears the Z-buffer with the specified packed 16-bit value. This\n value is composed as follows:\n\n * The top 16-bit contains the Z value in a custom floating point format.\n * The bottom 2-bits (plus the 2 hidden bits) contain the Delta-Z value. The\n   Delta-Z value to use while clearing does not matter in practice for\n   normal Z buffer usages, so it can be left as 0.\n\n The default value to use for clearing the Z-buffer is `ZBUF_MAX`. To set the\n clear value to a custom Z value, use the `ZBUF_VAL` macro.\n\n Note that this function will respect the current scissor rectangle, if\n configured.\n\n @param[in] z\n            Value to reset the Z buffer to"]
    pub fn clear_z(&mut self, z: u16) {
        unsafe { crate::sys::rdpq_attach::__rdpq_clear_z(&z) }
    }
    #[doc = "Check if the RDP is currently attached to a surface\n\n @return true if it is attached, false otherwise."]
    pub fn is_attached(&self) -> bool {
        unsafe { crate::sys::rdpq_attach::rdpq_is_attached() }
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct AttachedSurface<'s>(&'s Surface<'s>);

wrapper_ref! { AttachedSurface<'s> => Surface<'s> { self => &self.0 } }

#[repr(transparent)]
#[derive(Debug)]
pub struct Attachment<'s, 'r>(PhantomData<&'s mut RdpQ<'r>>);

static_wrapper! { Attachment<'s, 'r> => RdpQ<'r> { RdpQ(PhantomData) } }

impl<'s, 'r> Attachment<'s, 'r> {
    #[doc = "Get the surface that is currently attached to the RDP\n\n @return A pointer to the surface that is currently attached to the RDP,\n         or NULL if none is attached.\n\n @see `rdpq_attach`"]
    pub fn surface(&self) -> AttachedSurface<'_> {
        AttachedSurface(unsafe {
            core::mem::transmute(crate::sys::rdpq_attach::rdpq_get_attached())
        })
    }
    #[doc = "Detach the RDP from the current surface, and restore the previous one\n\n This function detaches the RDP from the current surface. Using a small internal\n stack, the previous render target is restored (if any).\n\n Notice that `rdpq_detach` does not wait for the RDP to finish rendering, like any\n other rdpq function. If you need to ensure that the RDP has finished rendering,\n either call `rspq_wait` afterwards, or use the `rdpq_detach_wait` function.\n\n A common use case is detaching from the main framebuffer (obtained via `display_get`),\n and then displaying it via `display_show`. For this case, consider using\n `rdpq_detach_show` which basically schedules the `display_show` to happen automatically\n without blocking the CPU.\n\n @see `rdpq_attach`\n @see `rdpq_detach_show`\n @see `rdpq_detach_wait`"]
    #[inline]
    pub fn detach(self) {
        unsafe { crate::sys::rdpq_attach::rdpq_detach_cb(None, core::ptr::null_mut()) }
    }
    #[doc = "Detach the RDP from the current framebuffer, and show it on screen\n\n This function runs a `rdpq_detach` on the surface, and then schedules in\n background for the surface to be displayed on screen after the RDP has\n finished drawing to it.\n\n The net result is similar to calling `rdpq_detach_wait` and then `display_show`\n manually, but it is more efficient because it does not block the CPU. Thus,\n if this function is called at the end of the frame, the CPU can immediately\n start working on the next one (assuming there is a free framebuffer available).\n\n @see `rdpq_detach_wait`\n @see `display_show`"]
    #[inline]
    pub fn detach_show(self) {
        unsafe { crate::sys::rdpq_attach::rdpq_detach_show() }
    }
    #[doc = "Detach the RDP from the current surface, waiting for RDP to finish drawing.\n\n This function is similar to `rdpq_detach`, but also waits for the RDP to finish\n drawing to the surface.\n\n @see `rdpq_detach`"]
    #[inline]
    pub fn detach_wait(self) {
        self.detach();
        crate::rspq::RspQ(()).wait()
    }
    #[doc = "Detach the RDP from the current surface, and call a callback when\n        the RDP has finished drawing to it.\n\n This function is similar to `rdpq_detach`: it does not block the CPU, but\n schedules for a callback to be called (under interrupt) when the RDP has\n finished drawing to the surface.\n\n @param[in] cb\n            Callback that will be called when the RDP has finished drawing to the surface.\n @param[in] arg\n            Argument to the callback.\n\n @see `rdpq_detach`"]
    #[inline]
    pub fn detach_cb<T: crate::n64::InterruptArg>(&mut self, func: T::Fn, data: T) {
        unsafe { crate::sys::rdpq_attach::rdpq_detach_cb(Some(T::cast_fn(func)), data.into_ptr()) }
    }
}

impl<'s, 'r> crate::Undroppable for &mut Attachment<'s, 'r> {
    const ERROR: &'static str = "Finish the attachment with Attachment::detach, Attachment::detach_cb, Attachment::show, or Attachment::wait";
}

impl<'s, 'r> Drop for Attachment<'s, 'r> {
    #[inline]
    fn drop(&mut self) {
        let _ = crate::DropBomb::new(self);
    }
}

#[doc = "Create a packed Z-buffer value for a given Z value.\n\nThis macro can be used to convert a floating point Z value in range `[0..1]` to a packed Z value that can be written as-is in the Z-buffer, for instance via [`RdpQ::clear_z`].\n\nNotice that this macro sets Delta-Z to 0 in the packed Z value, since it is not possible to fully configure Delta-Z via `rdpq_clear_z` anyway."]
#[inline]
pub fn zbuf_val(f: f32) -> u16 {
    unsafe { __rdpq_zfp14(f) << 2 }
}

#[doc = "Texture filtering types"]
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Filter {
    #[doc = "Point filtering (aka nearest)"]
    Point = rdpq_filter_s_FILTER_POINT as _,
    #[doc = "Bilinear filtering"]
    Bilinear = rdpq_filter_s_FILTER_BILINEAR as _,
    #[doc = "Median filtering"]
    Median = rdpq_filter_s_FILTER_MEDIAN as _,
}

#[doc = "Dithering configuration\n\n RDP can optionally perform dithering on RGB and Alpha channel of the texture.\n The dithering is performed by the blender unit, which is also in charge of\n adapting the pixel color depth to that of the framebuffer. Dithering is\n a good way to reduce the mach banding effect created by color depth\n reduction.\n\n The blender in fact will reduce the RGB components of the pixel (coming\n from the color combiner) to 5-bit when the framebuffer is 16-bit. If the\n framebuffer is 32-bit, the blender formula will be calculated with 8-bit\n per channel, so no dithering is required.\n\n On the other hand, the alpha channels (used as multiplicative factors\n in the blender formulas) will always be reduced to 5-bit depth, even if\n the framebuffer is 32-bit. If you see banding artifacts in transparency levels\n of blended polygons, you may want to activate dithering on the alpha channel.\n\n It is important to notice that the VI can optionally run an \"dither filter\"\n on the final image, while sending it to the video output. This\n algorithm tries to recover color depth precision by averaging lower bits\n in neighborhood pixels, and reducing the small noise created by dithering.\n `display_init` currently activates it by default on all 16-bit display modes,\n if passed `FILTERS_DEDITHER` or `FILTERS_RESAMPLE_ANTIALIAS_DEDITHER`.\n\n If you are using an emulator, make sure it correctly emulates the VI\n dither filter to judge the quality of the final image. For instance,\n the RDP plugin parallel-RDP (based on Vulkan) emulates it very accurately,\n so emulators like Ares, dgb-n64 or simple64 will produce a picture closer to\n real hardware.\n\n The supported dither algorithms are:\n\n   * `SQUARE` (aka \"magic square\"). This is a custom dithering\n     algorithm, designed to work best with the VI dither filter. When\n     using it, the VI will reconstruct a virtually perfect 32-bit image\n     even though the framebuffer is only 16-bit.\n   * `BAYER`: standard Bayer dithering. This algorithm looks\n     better than the magic square when the VI dither filter is disabled,\n     or in some specific scenarios like large blended polygons. Make\n     sure to test it as well.\n   * `INVSQUARE` and `INVBAYER`: these are the same algorithms, but using\n     an inverse (symmetrical) pattern. They can be selected for alpha\n     channels to avoid making transparency phase with color dithering,\n     which is sometimes awkward.\n   * `NOISE`: random noise dithering. The dithering is performed\n     by perturbing the lower bit of each pixel with random noise.\n     This will create a specific visual effect as it changes from frame to\n     frame even on still images; it is especially apparent when used on\n     alpha channel as it can affect transparency. It is more commonly used\n     as a graphic effect rather than an actual dithering.\n   * `NONE`: disable dithering.\n\n While the RDP hardware allows to configure different dither algorithms\n for RGB and Alpha channels, unfortunately not all combinations are\n available. This enumerator defines the available combinations. For\n instance, `DITHER_BAYER_NOISE` selects the Bayer dithering for the\n RGB channels, and the noise dithering for alpha channel."]
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Dither {
    #[doc = "Dithering: RGB=Square, Alpha=Square"]
    SquareSquare = rdpq_dither_s_DITHER_SQUARE_SQUARE as _,
    #[doc = "Dithering: RGB=Square, Alpha=InvSquare"]
    SquareInvSquare = rdpq_dither_s_DITHER_SQUARE_INVSQUARE as _,
    #[doc = "Dithering: RGB=Square, Alpha=Noise"]
    SquareNoise = rdpq_dither_s_DITHER_SQUARE_NOISE as _,
    #[doc = "Dithering: RGB=Square, Alpha=None"]
    SquareNone = rdpq_dither_s_DITHER_SQUARE_NONE as _,
    #[doc = "Dithering: RGB=Bayer, Alpha=Bayer"]
    BayerBayer = rdpq_dither_s_DITHER_BAYER_BAYER as _,
    #[doc = "Dithering: RGB=Bayer, Alpha=InvBayer"]
    BayerInvBayer = rdpq_dither_s_DITHER_BAYER_INVBAYER as _,
    #[doc = "Dithering: RGB=Bayer, Alpha=Noise"]
    BayerNoise = rdpq_dither_s_DITHER_BAYER_NOISE as _,
    #[doc = "Dithering: RGB=Bayer, Alpha=None"]
    BayerNone = rdpq_dither_s_DITHER_BAYER_NONE as _,
    #[doc = "Dithering: RGB=Noise, Alpha=Square"]
    NoiseSquare = rdpq_dither_s_DITHER_NOISE_SQUARE as _,
    #[doc = "Dithering: RGB=Noise, Alpha=InvSquare"]
    NoiseInvSquare = rdpq_dither_s_DITHER_NOISE_INVSQUARE as _,
    #[doc = "Dithering: RGB=Noise, Alpha=Noise"]
    NoiseNoise = rdpq_dither_s_DITHER_NOISE_NOISE as _,
    #[doc = "Dithering: RGB=Noise, Alpha=None"]
    NoiseNone = rdpq_dither_s_DITHER_NOISE_NONE as _,
    #[doc = "Dithering: RGB=None, Alpha=Bayer"]
    NoneBayer = rdpq_dither_s_DITHER_NONE_BAYER as _,
    #[doc = "Dithering: RGB=None, Alpha=InvBayer"]
    NoneInvBayer = rdpq_dither_s_DITHER_NONE_INVBAYER as _,
    #[doc = "Dithering: RGB=None, Alpha=Noise"]
    NoneNoise = rdpq_dither_s_DITHER_NONE_NOISE as _,
    #[doc = "Dithering: RGB=None, Alpha=None"]
    NoneNone = rdpq_dither_s_DITHER_NONE_NONE as _,
}

#[doc = "Types of palettes supported by RDP"]
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum TLUT {
    #[doc = "No palette"]
    None = rdpq_tlut_s_TLUT_NONE as _,
    #[doc = "Palette made of `FMT_RGBA16` colors"]
    RGBA16 = rdpq_tlut_s_TLUT_RGBA16 as _,
    #[doc = "Palette made of `FMT_IA16` colors"]
    IA16 = rdpq_tlut_s_TLUT_IA16 as _,
}

impl crate::surface::TexFormat {
    #[doc = "Converts the specified texture format to the TLUT mode that is needed to draw a texture of this format"]
    #[inline]
    pub const fn tlut(self) -> TLUT {
        match self {
            Self::CI4 | Self::CI8 => TLUT::RGBA16,
            _ => TLUT::None,
        }
    }
}

#[doc = "Types of mipmap supported by RDP"]
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Mipmap {
    #[doc = "Mipmap disabled"]
    None = rdpq_mipmap_s_MIPMAP_NONE as _,
    #[doc = "Choose the nearest mipmap level"]
    Nearest = rdpq_mipmap_s_MIPMAP_NEAREST as _,
    #[doc = "Interpolate between the two nearest mipmap levels (also known as \"trilinear\")"]
    Interpolate = rdpq_mipmap_s_MIPMAP_INTERPOLATE as _,
    #[doc = "Interpolate between the two nearest mipmap levels (also known as \"trilinear\") with sharpening enabled"]
    Sharpen = rdpq_mipmap_s_MIPMAP_INTERPOLATE_SHARPEN as _,
    #[doc = "Interpolate between the two nearest mipmap levels (also known as \"trilinear\") with detail texture enabled"]
    Detail = rdpq_mipmap_s_MIPMAP_INTERPOLATE_DETAIL as _,
    #[doc = "Special mipmap mode that must be used for SHC textures"]
    SHQ = rdpq_mipmap_s_MIPMAP_INTERPOLATE_SHQ as _,
}

#[doc = "Types of antialiasing supported by RDP"]
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Antialias {
    #[doc = "No antialiasing"]
    None = rdpq_antialias_s_AA_NONE as _,
    #[doc = "Standard antialiasing"]
    Standard = rdpq_antialias_s_AA_STANDARD as _,
    #[doc = "Reduced antialiasing"]
    Reduced = rdpq_antialias_s_AA_REDUCED as _,
}

#[doc = "Types of Z-buffering modes supported by RDP\n\n See `rdpq_mode_zmode` for more information."]
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum ZMode {
    #[doc = "Standard Z-buffer mode"]
    Standard = rdpq_zmode_s_ZMODE_STANDARD as _,
    #[doc = "Z-buffer mode for interpenetrating surfaces"]
    Interpenetrating = rdpq_zmode_s_ZMODE_INTERPENETRATING as _,
    #[doc = "Z-buffer mode for decal surfaces"]
    Decal = rdpq_zmode_s_ZMODE_DECAL as _,
}

trait BoolExt {
    fn choose<T>(self, yes: T, no: T) -> T;
}
impl BoolExt for bool {
    #[inline(always)]
    fn choose<T>(self, yes: T, no: T) -> T {
        match self {
            true => yes,
            false => no,
        }
    }
}

#[inline(always)]
fn mode_change_som(mask: u64, val: u64) {
    unsafe {
        if (mask >> 32) != 0 {
            __rdpq_fixup_mode3(
                RDPQ_CMD_MODIFY_OTHER_MODES,
                0 | (1 << 15),
                !(mask >> 32) as u32,
                (val >> 32) as u32,
            );
        }
        if (mask as u32) != 0 {
            __rdpq_fixup_mode3(
                RDPQ_CMD_MODIFY_OTHER_MODES,
                4 | (1 << 15),
                !mask as u32,
                val as u32,
            );
        }
    }
}

impl<'r> RdpQ<'r> {
    #[doc = "Push the current render mode into the stack\n\n This function allows to push the current render mode into an internal stack.\n It allows to temporarily modify the render mode, and later recover its value.\n\n This is effective on all render mode changes that can be modified via\n rdpq_mode_* function. It does not affect other RDP configurations such as\n the various colors.\n\n The stack has 4 slots (including the current one)."]
    #[inline]
    pub fn mode_push(&mut self) -> ModeFrame<'_, 'r> {
        unsafe { rdpq_mode_push() }
        ModeFrame(PhantomData)
    }
    #[doc = "Start a batch of RDP mode changes\n\n This function can be used as an optimization when changing render mode\n and/or multiple render states. It allows to batch the changes, so that\n RDP hardware registers are updated only once.\n\n To use it, put a call to `rdpq_mode_begin` and `rdpq_mode_end` around\n the mode functions that you would like to batch. For instance:\n\n @code{.c}\n      rdpq_mode_begin();\n          rdpq_set_mode_standard();\n          rdpq_mode_mipmap(MIPMAP_INTERPOLATE, 2);\n          rdpq_mode_dithering(DITHER_SQUARE_SQUARE);\n          rdpq_mode_blender(RDPQ_BLENDING_MULTIPLY);\n      rdpq_mode_end();\n @endcode\n\n The only effect of using `rdpq_mode_begin` is more efficient RSP\n and RDP usage, there is no semantic change in the way RDP is\n programmed when `rdpq_mode_end` is called.\n\n @note The functions affected by `rdpq_mode_begin` / `rdpq_mode_end`\n       are just those that are part of the mode API (that is,\n       `rdpq_set_mode_*` and `rdpq_mode_*`). Any other function\n       is not batched and will be issued immediately."]
    #[inline]
    pub fn mode_begin(&mut self) -> ModeFreeze<'_, 'r> {
        unsafe { rdpq_mode_begin() }
        ModeFreeze(PhantomData)
    }
}

pub trait RdpQModes: crate::sealed::Sealed {
    #[doc = "Reset render mode to standard.\n\n This is the most basic and general mode reset function. It configures the RDP\n processor in a standard and very basic way:\n\n   * Basic texturing (without shading)\n   * No dithering, antialiasing, blending, etc.\n\n You can further configure the mode by calling one of the many functions\n in the mode API (`rdpq_mode_*`)."]
    #[inline]
    fn set_mode_standard(&mut self) {
        unsafe { rdpq_set_mode_standard() }
    }
    #[doc = "Reset render mode to FILL type.\n\n This function sets the render mode type to FILL, which is used to quickly\n fill portions of the screens with a solid color. The specified color is\n configured via `rdpq_set_fill_color`, and can be changed later.\n\n Notice that in FILL mode most of the RDP features are disabled, so all other\n render modes settings (rdpq_mode_* functions) do not work.\n\n @param[in]  color  The fill color to use"]
    #[inline]
    fn set_mode_fill(&mut self, color: Color) {
        unsafe { __rdpq_set_mode_fill() };
        RdpQ(PhantomData).set_fill_color(color);
    }
    #[doc = "Reset render mode to COPY type.\n\n This function sets the render mode type to COPY, which is used to quickly\n blit bitmaps. In COPY mode, only texture rectangles (aka \"sprites\") can be\n drawn and no advanced render mode features are working (rdpq_mode_* functions).\n\n The only available feature is transparency: pixels with alpha set to 0 can\n optionally be discarded during blit, so that the target buffer contents is\n not overwritten for those pixels. This is implemented using alpha compare.\n\n The COPY mode is approximately 4 times faster at drawing than the standard\n mode, so make sure to enable it whenever it is possible.\n\n @note The COPY mode only works with 16-bpp framebuffers. It will trigger a\n       hardware crash (!) on 32-bpp framebuffers, so avoid using it. The\n       validator will warn you about this anyway.\n\n @param[in]  transparency   If true, pixels with alpha set to 0 are not drawn\n\n @see `rdpq_set_mode_standard`"]
    #[inline]
    fn set_mode_copy(&mut self, transparency: bool) {
        unsafe { rdpq_set_mode_copy(transparency) }
    }
    #[doc = "Reset render mode to YUV mode.\n\n This is a helper function to configure a render mode for YUV conversion.\n In addition of setting the render mode, this function also configures a\n combiner (given that YUV conversion happens also at the combiner level),\n and set standard YUV parameters (for BT.601 TV Range).\n\n After setting the YUV mode, you can load YUV textures to TMEM (using a\n surface with `FMT_YUV16`), and then draw them on the screen as part of\n triangles or rectangles.\n\n @param[in] bilinear      If true, YUV textures will also be filtered with\n                          bilinear interpolation (note: this will require\n                          2-cycle mode so it will be twice as slow)."]
    #[inline]
    fn set_mode_yuv(&mut self, bilinear: bool) {
        unsafe { rdpq_set_mode_yuv(bilinear) }
    }
    #[doc = "Activate antialiasing\n\n This function can be used to enable/disable antialias at the RDP level.\n There are two different kinds of antialias on N64:\n\n   * Antialias on internal edges: this is fully performed by RDP.\n   * Antialias on external edges: this is prepared by RDP but is actually\n     performed as a post-processing filter by VI.\n\n This function activates both kinds of antialias, but to display correctly\n the second type, make sure that you did pass `FILTERS_RESAMPLE_ANTIALIAS` or\n `FILTERS_RESAMPLE_ANTIALIAS_DEDITHER` to `display_init`.\n\n On the other hand, if you want to make sure that no antialias is performed,\n disable antialias with `rdpq_mode_antialias(false)` (which is the default\n for `rdpq_set_mode_standard`), and that will make sure that the VI will not\n do anything to the image, even if `display_init` was called with\n `FILTERS_RESAMPLE_ANTIALIAS` or `FILTERS_RESAMPLE_ANTIALIAS_DEDITHER`.\n\n @note Antialiasing internally uses the blender unit. If you already\n       configured a formula via `rdpq_mode_blender`, antialias will just\n       rely on that one to correctly blend pixels with the framebuffer. It is\n       thus important that a custom formula configured via `rdpq_mode_blender`\n       does blend with the background somehow.\n\n @param mode        Antialiasing mode to use (or AA_NONE to disable)"]
    #[inline]
    fn mode_antialias(&mut self, mode: Antialias) {
        mode_change_som(
            SOM_AA_ENABLE as u64 | SOMX_AA_REDUCED,
            (mode != Antialias::None).choose(SOM_AA_ENABLE as _, 0)
                | (mode == Antialias::Reduced).choose(SOMX_AA_REDUCED, 0),
        )
    }
    #[doc = "Configure the color combiner\n\n This function allows to configure the color combiner formula to be used.\n The color combiner is the internal RDP hardware unit that mixes inputs\n from textures, colors and other sources and produces a RGB/Alpha value,\n that is then sent to the blender unit. If the blender is disabled (eg:\n the polygon is solid), the value produced by the combiner is the one\n that will be written into the framebuffer.\n\n For common use cases, rdpq offers ready-to-use macros that you can pass\n to `rdpq_mode_combiner`: `RDPQ_COMBINER_FLAT`, `RDPQ_COMBINER_SHADE`,\n `RDPQ_COMBINER_TEX`, `RDPQ_COMBINER_TEX_FLAT`, `RDPQ_COMBINER_TEX_SHADE`.\n\n For example, to draw a texture rectangle modulated with a flat color:\n\n @code{.c}\n      // Reset to standard rendering mode.\n      rdpq_set_mode_standard();\n\n      // Configure the combiner\n      rdpq_mode_combiner(RDPQ_COMBINER_TEX_FLAT);\n\n      // Configure the flat color that will modulate the texture\n      rdpq_set_prim_color(RGBA32(192, 168, 74, 255));\n\n      // Upload a texture into TMEM (tile descriptor `4`)\n      rdpq_tex_upload(TILE4, &texture, 0);\n\n      // Draw the rectangle\n      rdpq_texture_rectangle(TILE4,\n          0, 0, 32, 16,     // x0, y0, x1, y1\n          0, 0, 1.0, 1.0f   // s, t, ds, dt\n      );\n @endcode\n\n Alternatively, you can use your own combiner formulas, created with either\n `RDPQ_COMBINER1` (one pass) or `RDPQ_COMBINER2` (two passes). See the respective\n documentation for all the details on how to create a custom formula.\n\n When using a custom formula, you must take into account that some render states\n also rely on the combiner to work. Specifically:\n\n  * Mipmap (`rdpq_mode_mipmap`): when activating interpolated mipmapping\n    (`MIPMAP_INTERPOLATE`, also known as \"trilinear filterig\"), a dedicated\n    color combiner pass is needed, so if you set a custom formula, it has to be\n    a one-pass formula. Otherwise, a RSP assertion will trigger.\n  * Fog (`rdpq_mode_fog`): fogging is generally made by substituting the alpha\n    component of the shade color with a depth value, which is then used in\n    the blender formula (eg: `RDPQ_FOG_STANDARD`). The only interaction with the\n    color combiner is that the SHADE alpha component should not be used as\n    a modulation factor in the combiner, otherwise you get wrong results\n    (if you then use the alpha for blending). rdpq automatically adjusts\n    standard combiners using shade (`RDPQ_COMBINER_SHADE` and `RDPQ_COMBINER_TEX_SHADE`)\n    when fog is enabled, but for custom combiners it is up to the user to\n    take care of that.\n\n @param comb      The combiner formula to configure\n\n @see `RDPQ_COMBINER1`\n @see `RDPQ_COMBINER2`\n\n @note For programmers with previous RDP programming experience: this function\n       makes sure that the current cycle type can work correctly with the\n       specified combiner formula. Specifically, it switches automatically\n       between 1-cycle and 2-cycle depending on the formula being set and the\n       blender unit configuration, and also automatically adapts combiner\n       formulas to the required cycle mode. See the documentation in rdpq.c\n       for more information."]
    #[inline]
    fn mode_combiner(&mut self, comb: Combiner) {
        let comb = comb.into_inner();
        if (comb & RDPQ_COMBINER_2PASS as u64) != 0 {
            unsafe {
                __rdpq_fixup_mode(
                    RDPQ_CMD_SET_COMBINE_MODE_2PASS,
                    (comb >> 32) as u32 & 0x00FFFFFF,
                    comb as u32,
                );
            }
        } else {
            let mut comb1_mask = RDPQ_COMB1_MASK;
            if ((comb >> 0) & 7) == 1 {
                comb1_mask ^= 1 << 0;
            }
            if ((comb >> 3) & 7) == 1 {
                comb1_mask ^= 1 << 3;
            }
            if ((comb >> 6) & 7) == 1 {
                comb1_mask ^= 1 << 6;
            }
            if ((comb >> 18) & 7) == 1 {
                comb1_mask ^= 1 << 18;
            }
            if ((comb >> 21) & 7) == 1 {
                comb1_mask ^= 1 << 21;
            }
            if ((comb >> 24) & 7) == 1 {
                comb1_mask ^= 1 << 24;
            }
            if ((comb >> 32) & 31) == 1 {
                comb1_mask ^= 1 << 32;
            }
            if ((comb >> 37) & 15) == 1 {
                comb1_mask ^= 1 << 37;
            }

            unsafe {
                __rdpq_fixup_mode4(
                    RDPQ_CMD_SET_COMBINE_MODE_1PASS,
                    (comb >> 32) as u32 & 0x00FFFFFF,
                    comb as u32,
                    (comb1_mask >> 32) as u32 & 0x00FFFFFF,
                    comb1_mask as u32,
                );
            }
        }
    }
    #[doc = "Configure the formula to use for blending.\n\n This function can be used to configure the formula used\n in the blender unit.\n\n The standard blending formulas are:\n\n  * `RDPQ_BLENDER_MULTIPLY`: multiplicative alpha blending using texture's alpha\n  * `RDPQ_BLENDER_MULTIPLY_CONST`: multiplicative alpha blending using a constant alpha\n  * `RDPQ_BLENDER_ADDITIVE`: additive alpha blending (mostly broken).\n\n Normally, you would use `RDPQ_BLENDER_MULTIPLY` when your source texture has\n an internal alpha channel that you want to use for blending. Otherwise, if\n you just want to add a fixed-level semi-transparency to an existing texture,\n use `RDPQ_BLENDER_MULTIPLY_CONST`.\n\n `RDPQ_BLENDER_ADDITIVE` is mostly broken on RDP, as it doesn't handle correctly\n overflowing values. Basically, values up to 1.5 are correctly clamped to 1,\n but values above 1.5 are wrapped back to 0, which makes the mode almost useless.\n\n It is possible to also create custom formulas. The blender unit\n allows for up to two passes. Use `RDPQ_BLENDER` to create a one-pass\n blending formula, or `RDPQ_BLENDER2` to create a two-pass formula.\n\n Please notice that two-pass formulas are not compatible with fogging\n (`rdpq_mode_fog`). Also notice that rdpq_mode assumes that any formula\n that you set here (either one-pass or two-passes) does blend with the\n background. If you want to use a formula that does not blend with the\n background, set it via `rdpq_mode_fog`, otherwise you might get incorrect\n results when using anti-alias (see `rdpq_mode_antialias`).\n\n The following example shows how to draw a texture rectangle using\n a fixed blending value of 0.5 (ignoring the alpha channel of the\n texture):\n\n @code{.c}\n      // Set standard mode\n      rdpq_set_mode_standard();\n\n      // Use blending with a constant semi-transparency value\n      rdpq_mode_blender(RDPQ_BLENDER_MULTIPLY_CONST);\n\n      // Configure the blending value to 128 (0.5). Notice that RGB\n      // values are ignored in this formula.\n      rdpq_set_fog_color(RGBA32(0,0,0, 128));\n\n      // Load a texture into TMEM\n      rdpq_tex_upload(TILE0, texture, 0);\n\n      // Draw it\n      rdpq_texture_rectangle(TILE0,\n          0, 0, 64, 64,   // x0,y0 - x1,y1\n          0, 0, 1.0, 1.0  // s0,t0 - ds,dt\n      );\n @endcode\n\n @param blend          Blending formula created with `RDPQ_BLENDER`,\n                       or 0 to disable.\n\n @see `rdpq_mode_fog`\n @see `RDPQ_BLENDER`\n @see `RDPQ_BLENDER_MULTIPLY`\n @see `RDPQ_BLENDER_MULTIPLY_CONST`\n @see `RDPQ_BLENDER_ADDITIVE`"]
    #[inline]
    fn mode_blender(&mut self, blend: Blender) {
        let mut blend = blend.into_inner();
        if blend != 0 {
            blend |= SOM_BLENDING;
        }
        unsafe { __rdpq_fixup_mode(RDPQ_CMD_SET_BLENDING_MODE, 0, blend) };
    }
    #[doc = "Enable or disable fog\n\n This function enables fog on RDP. Fog on RDP is simulated in the\n following way:\n\n  * The T&L pipeline must calculate a depth information for each\n    vertex of the primitive and put it into the alpha channel of\n    the per-vertex color. This is outside of the scope of rdpq,\n    so rdpq assumes that this has already been done when\n    `rdpq_mode_fog` is called.\n  * The RDP blender unit is programmed to modulate a \"fog color\"\n    with the polygon pixel, using SHADE_ALPHA as interpolation\n    factor. Since SHADE_ALPHA contains a depth information, the\n    farther the object, the stronger it will assume the fog color.\n\n To enable fog, pass `RDPQ_FOG_STANDARD` to this function, and\n call `rdpq_set_fog_color` to configure the fog color. This is\n the standard fogging formula.\n\n If you want, you can instead build a custom fogging formula\n using `RDPQ_BLENDER`. Notice that rdpq_mode assumes that the formula\n that you set with rdpq_mode_fog does not blend with the background; for\n that, use `rdpq_mode_blender`.\n\n To disable fog, call `rdpq_mode_fog` passing 0.\n\n @note Fogging uses one pass of the blender unit (the first),\n       so this can coexist with a blending formula (`rdpq_mode_blender`)\n       as long as it's a single pass one (created via `RDPQ_BLENDER`).\n       If a two-pass blending formula (`RDPQ_BLENDER2`) was set with\n       `rdpq_mode_blender`, fogging cannot be used.\n\n @param fog            Fog formula created with `RDPQ_BLENDER`,\n                       or 0 to disable.\n\n @see `RDPQ_FOG_STANDARD`\n @see `rdpq_set_fog_color`\n @see `RDPQ_BLENDER`\n @see `rdpq_mode_blender`"]
    #[inline]
    fn mode_fog(&mut self, fog: Blender) {
        let mut fog = fog.into_inner();
        if fog != 0 {
            fog |= SOM_BLENDING;
            assert!(
                (fog & SOMX_BLEND_2PASS) == 0,
                "Fogging cannot be used with two-pass blending formulas"
            );
        }
        unsafe {
            mode_change_som(SOMX_FOG, if fog != 0 { SOMX_FOG } else { 0 });
            __rdpq_fixup_mode(RDPQ_CMD_SET_FOG_MODE, 0, fog);
        }
    }
    #[doc = "Change dithering mode\n\n This function allows to change the dithering algorithm performed by\n RDP on RGB and alpha channels. Note that by default, `rdpq_set_mode_standard`\n disables any dithering.\n\n See `rdpq_dither_t` for an explanation of how RDP applies dithering and\n how the different dithering algorithms work.\n\n @param dither    Dithering to perform\n\n @see `rdpq_dither_t`"]
    #[inline]
    fn mode_dithering(&mut self, dither: Dither) {
        mode_change_som(
            SOM_RGBDITHER_MASK | SOM_ALPHADITHER_MASK,
            (dither as u64) << SOM_ALPHADITHER_SHIFT,
        );
    }
    #[doc = "Activate alpha compare feature\n\n This function activates the alpha compare feature. It allows to do per-pixel\n rejection (masking) depending on the value of the alpha component of the pixel.\n The value output from the combiner is compared with a configured threshold\n and if the value is lower, the pixel is not written to the framebuffer.\n\n Moreover, RDP also support a random noise alpha compare mode, where the threshold\n value is calculated as a random number for each pixel. This can be used for special\n graphic effects.\n\n @note Alpha compare becomes more limited if antialiasing is enabled (both full and reduced,\n       see `rdpq_mode_antialias`). In that case, any threshold value not equal to 0 will\n       internally be treated as if 255 was specified. This implies that noise-based\n       alpha compare is not supported under this condition.\n\n @param threshold          Threshold value. All pixels whose alpha is less than this threshold\n                           will not be drawn. Use 0 to disable. Use a negative value for\n                           activating the noise-based alpha compare."]
    #[inline]
    fn mode_alphacompare(&mut self, threshold: i32) {
        if threshold == 0 {
            mode_change_som(SOM_ALPHACOMPARE_MASK as _, 0);
        } else if threshold > 0 {
            mode_change_som(SOM_ALPHACOMPARE_MASK as _, SOM_ALPHACOMPARE_THRESHOLD as _);
            RdpQ(PhantomData).set_blend_color(Color::rgba32(0, 0, 0, threshold as u8));
        } else {
            mode_change_som(SOM_ALPHACOMPARE_MASK as _, SOM_ALPHACOMPARE_NOISE as _);
        }
    }
    #[doc = "Activate z-buffer usage\n\n Activate usage of Z-buffer. The Z-buffer surface must be configured\n via `rdpq_set_z_image`.\n\n It is possible to separately activate the depth comparison\n (*reading* from the Z-buffer) and the Z update (*writing* to\n the Z-buffer).\n\n @param compare     True if per-pixel depth test must be performed\n @param update      True if per-pixel depth write must be performed\n\n @see `rdpq_set_z_image`"]
    #[inline]
    fn mode_zbuf(&mut self, compare: bool, update: bool) {
        mode_change_som(
            (SOM_Z_COMPARE | SOM_Z_WRITE) as _,
            (compare.choose(SOM_Z_COMPARE, 0) | update.choose(SOM_Z_WRITE, 0)) as _,
        )
    }
    #[doc = "Set a fixed override of Z value\n\n This function activates a special mode in which RDP will use a fixed value\n of Z for the next drawn primitives. This works with both rectangles\n (`rdpq_fill_rectangle` and `rdpq_texture_rectangle`) and triangles\n (`rdpq_triangle`).\n\n If a triangle is drawn with per-vertex Z while the Z-override is active,\n the per-vertex Z will be ignored.\n\n @param enable    Enable/disable the Z-override mode\n @param z         Z value to use (range 0..1)\n @param deltaz    DeltaZ value to use.\n\n @see `rdpq_set_prim_depth_raw`"]
    #[inline]
    fn mode_zoverride(&mut self, enable: bool, z: f32, deltaz: i16) {
        if enable {
            RdpQ(PhantomData).set_prim_depth_raw((z * 0x7FFF as f32) as u16, deltaz);
        }
        mode_change_som(
            SOM_ZSOURCE_PRIM as _,
            enable.choose(SOM_ZSOURCE_PRIM, 0) as _,
        );
    }
    #[doc = "Configure the Z buffering mode\n\n This function allows to tune the internal Z buffer formula to obtain several\n different effects. In addition to the standard operating mode (`ZMODE_STANDARD`),\n there are two special modes that can be activated:\n\n  * `ZMODE_DECAL`: this mode can be used to draw polygons that are coplanar with\n    already drawn polygons, normally called \"decals\". NOTE: this will never\n    be bulletproof. If you still get some Z-fighting flickering in this mode,\n    try to subdivide the background polygons so that they share vertices\n    exactly with the decal.\n  * `ZMODE_INTERPENETRATING`: this mode can be used to reduce z-fighting when\n    two objects intersect each other, and anti-aliasing is enabled. A common\n    case can be objects like trees positioned slightly under the terrain.\n\n @param mode      Z-buffering mode to use\n\n @see `rdpq_zmode_t`"]
    #[inline]
    fn mode_zmode(&mut self, mode: rdpq_zmode_t) {
        mode_change_som(SOM_ZMODE_MASK as _, (mode as u64) << SOM_ZMODE_SHIFT);
    }
    #[doc = "Activate palette lookup during drawing\n\n This function allows to enable / disable palette lookup during\n drawing. Uploading a palette to TMEM can be done via `rdpq_tex_upload_tlut`,\n or lower-level functions such as `rdpq_load_tlut_raw`.\n\n @param tlut     Palette type, or 0 to disable.\n\n @see `rdpq_tex_upload`\n @see `rdpq_tex_upload_tlut`\n @see `rdpq_tlut_t`"]
    #[inline]
    fn mode_tlut(&mut self, tlut: TLUT) {
        mode_change_som(SOM_TLUT_MASK, (tlut as u64) << SOM_TLUT_SHIFT);
    }
    #[doc = "Activate texture filtering\n\n This function allows to configure the kind of texture filtering that will be used\n while sampling textures.\n\n Available in render modes: standard, copy.\n\n @param filt      Texture filtering type\n\n @see `rdpq_filter_t`"]
    #[inline]
    fn mode_filter(&mut self, filt: rdpq_filter_t) {
        mode_change_som(SOM_SAMPLE_MASK, (filt as u64) << SOM_SAMPLE_SHIFT);
    }
    #[doc = "Activate mip-mapping.\n\n This function can be used to turn on mip-mapping.\n\n To use mip-mapping, you must have prepared multiple textures in TMEM. The\n simplest way is to let mksprite generate the mipmaps automatically (via\n the --mipmap option), so they get embedded within the sprite file; in this\n case, mipmaps are automatically uploaded to TMEM when you call `rdpq_sprite_upload`,\n and this function is also called automatically for you by `rdpq_sprite_upload`.\n\n Alternatively, you can upload the mipmaps manually to TMEM using `rdpq_tex_multi_begin`,\n `rdpq_tex_upload`, and `rdpq_tex_multi_end`. You must configure multiple consecutive\n tiles in TMEM, each one containing a mipmap level, and then call this function\n to activate mip-mapping and specifying how many levels you want to use.\n\n If you manually draw screen-space triangles via `rdpq_triangle` when mipmap\n is active via `rdpq_mode_mipmap`, pass 0 to the number of mipmaps in\n `rdpq_trifmt_t`, as the number of levels set here will win over it.\n\n @note Mip-mapping is not compatible with two-pass combiner formulas. if you\n       do so, you will hit a RSP assertion.\n\n @param mode          Mipmapping mode (use `MIPMAP_NONE` to disable)\n @param num_levels    Number of mipmap levels to use. Pass 0 when setting MIPMAP_NONE."]
    #[inline]
    fn mode_mipmap(&mut self, mode: Mipmap, mut num_levels: u32) {
        if mode == Mipmap::None {
            num_levels = 0;
        }
        if num_levels > 0 {
            num_levels -= 1;
        }
        mode_change_som(
            SOM_TEXTURE_LOD
                | SOMX_LOD_INTERP_MASK
                | SOMX_NUMLODS_MASK
                | SOM_TEXTURE_SHARPEN
                | SOM_TEXTURE_DETAIL,
            ((mode as u64) << 32) | (num_levels as u64) << SOMX_NUMLODS_SHIFT,
        );
    }
    #[doc = "Activate perspective correction for textures\n\n This function enables or disables the perspective correction for texturing.\n Perspective correction does not slow down rendering, and thus it is basically\n free.\n\n To be able to use perspective correction, make sure to pass the Z and W values\n in the triangle vertices.\n\n @param perspective       True to activate perspective correction, false to disable it."]
    #[inline]
    fn mode_persp(&mut self, perspective: bool) {
        mode_change_som(SOM_TEXTURE_PERSP, perspective.choose(SOM_TEXTURE_PERSP, 0));
    }
}

impl<'r> RdpQModes for RdpQ<'r> {}
impl<'s, 'r> RdpQModes for ModeFreeze<'s, 'r> {}

#[repr(transparent)]
#[derive(Debug)]
pub struct ModeFrame<'s, 'r>(PhantomData<&'s mut RdpQ<'r>>);

static_wrapper! { ModeFrame<'s, 'r> => RdpQ<'r> { RdpQ(PhantomData) } }

impl<'s, 'r> ModeFrame<'s, 'r> {
    #[inline]
    pub fn pop(self) {}
}

impl<'s, 'r> Drop for ModeFrame<'s, 'r> {
    #[doc = "Pop the current render mode from the stack\n\n This function allows to pop a previously pushed render mode from the stack,\n setting it as current again."]
    #[inline]
    fn drop(&mut self) {
        unsafe { rdpq_mode_pop() }
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct ModeFreeze<'s, 'r>(PhantomData<&'s mut RdpQ<'r>>);

static_wrapper! { ModeFreeze<'s, 'r> => RdpQ<'r> { RdpQ(PhantomData) } }

impl<'s, 'r> ModeFreeze<'s, 'r> {
    #[inline]
    pub fn end(self) {}
}

impl<'s, 'r> Drop for ModeFreeze<'s, 'r> {
    #[doc = "Finish a batch of RDP mode changes\n\n This function completes a batch of changes started with `rdpq_mode_begin`.\n\n @see `rdpq_mode_begin`"]
    #[inline]
    fn drop(&mut self) {
        unsafe { rdpq_mode_end() }
    }
}

impl<'r> RdpQ<'r> {
    #[doc = "Upload a sprite to TMEM, making it ready for drawing\n\n This function will upload a sprite to TMEM, making it ready for drawing.\n It is similar to `rdpq_tex_upload` which can be used for any surface, but\n it builds upon it with sprite-specific features:\n\n  * If the sprite contains mipmaps, the whole mipmap chain is uploaded to TMEM\n    as well. Moreover, mipmaps are automatically enabled in the render mode\n    (via `rdpq_mode_mipmap`).\n  * If the sprite contains a palette, it is uploaded to TMEM as well, and the\n    palette is also activated in the render mode (via `rdpq_mode_tlut`).\n  * If the sprite contains pre-configured texture parameters (eg: mirroring,\n    wrapping, translation, etc), they are automatically configured in the\n    RDP for drawing (on the specified RDP tile descriptor).\n\n After calling this function, the specified tile descriptor will be ready\n to be used in drawing primitives like `rdpq_triangle` or `rdpq_texture_rectangle`.\n\n This function is meant for sprites that can be loaded in full into TMEM; it\n will assert if the sprite does not fit TMEM. For larger sprites, either\n use `rdpq_sprite_blit` to directly draw then (handling partial uploads transparently),\n or use `rdpq_tex_upload_sub` to manually upload a smaller portion of the sprite.\n\n To load multiple sprites in TMEM at once (for instance, for multitexturing),\n you can manually specify the @p parms->tmem_addr for the second sprite, or\n call `rdpq_tex_multi_begin` / `rdpq_tex_multi_end` around multiple calls to\n `rdpq_sprite_upload`. For instance:\n\n @code{.c}\n      // Load multiple sprites in TMEM, with auto-TMEM allocation.\n      rdpq_tex_multi_begin();\n      rdpq_sprite_upload(TILE0, sprite0, NULL);\n      rdpq_sprite_upload(TILE1, sprite1, NULL);\n      rdpq_tex_multi_end();\n @endcode\n\n To speed up loading of a sprite, you can record the loading sequence in\n a rspq block and replay it any time later. For instance:\n\n @code{.c}\n      sprite_t *hero = sprite_load(\"rom:/hero.sprite\");\n\n      // Record the loading sequence in a rspq block\n      rspq_block_begin();\n          rdpq_sprite_upload(TILE0, hero, NULL);\n      rspq_block_t *hero_load = rspq_block_end();\n\n      // Later, load the sprite\n      rspq_block_run(hero_load);\n\n      // Remember to free the block when you don't need it anymore\n      rspq_wait();     // wait until RSP is idle\n      rspq_block_free(hero_load);\n      sprite_free(hero);\n @endcode\n\n @param tile      Tile descriptor that will be initialized with this sprite\n @param sprite    Sprite to upload\n @param parms     Texture upload parameters to use\n @return          Number of bytes used in TMEM for this sprite (excluding palette)\n\n @see `rdpq_tex_upload`\n @see `rdpq_tex_upload_sub`\n @see `rdpq_sprite_blit`"]
    #[inline]
    pub fn sprite_upload(&mut self, tile: Tile, sprite: &Sprite, parms: Option<&TexParms>) -> u32 {
        unsafe {
            crate::sys::rdpq_sprite::rdpq_sprite_upload(
                tile as _,
                sprite.as_raw(),
                parms.map(|p| &p.0 as _).unwrap_or_else(core::ptr::null),
            ) as _
        }
    }
    #[doc = "Blit a sprite to the active framebuffer\n\n This function will perform a blit of a sprite to the active framebuffer,\n with several features like source rectangle selection, scaling, rotation, etc.\n\n The function is similar to `rdpq_tex_blit`, but it works on a sprite rather than\n a generic surface. In addition to the standard features of `rdpq_tex_blit`,\n it will also handle sprite-specific features:\n\n  * If the sprite contains a palette, it is uploaded to TMEM as well, and the\n    palette is also activated in the render mode (via `rdpq_mode_tlut`).\n  * If the sprite is optimized (via mksprite --optimize), the upload function\n    will be faster.\n\n Just like `rdpq_tex_blit`, this function is designed to work with sprites of\n arbitrary sizes; those that won't fit in TMEM will be automatically split\n in multiple chunks to perform the requested operation.\n\n Please refer to `rdpq_tex_blit` for a full overview of the features.\n\n @param sprite    Sprite to blit\n @param x0        X coordinate on the framebuffer where to draw the surface\n @param y0        Y coordinate on the framebuffer where to draw the surface\n @param parms     Parameters for the blit operation (or NULL for default)"]
    #[inline]
    pub fn sprite_blit(&mut self, sprite: &Sprite, x0: f32, y0: f32, parms: Option<&BlitParms>) {
        unsafe {
            crate::sys::rdpq_sprite::rdpq_sprite_blit(
                sprite.as_raw(),
                x0,
                y0,
                parms.map(|p| &p.0 as _).unwrap_or_else(core::ptr::null),
            ) as _
        }
    }
}

impl<'r> RdpQ<'r> {
    #[doc = "Load a texture into TMEM\n\n This function helps loading a texture into TMEM, which normally involves:\n\n   * Configuring a tile descriptor (via `rdpq_set_tile`)\n   * Setting the source texture image (via `rdpq_set_texture_image`)\n   * Loading the texture (via `rdpq_load_tile` or `rdpq_load_block`)\n\n After calling this function, the specified tile descriptor will be ready\n to be used in drawing primitives like `rdpq_triangle` or `rdpq_texture_rectangle`.\n\n If the texture uses a palette (`FMT_CI8` or `FMT_CI4`), the tile descriptor\n will be by default pointing to palette 0. In the case of `FMT_CI4`, this\n might not be the correct palette; to specify a different palette number,\n add .palette = X to the tex parms. Before drawing a texture with palette,\n remember to call `rdpq_mode_tlut` to activate palette mode.\n\n If you want to load a portion of a texture rather than the full texture,\n use `rdpq_tex_upload_sub`, or alternatively create a sub-surface using\n `surface_make_sub` and pass it to `rdpq_tex_upload`. See `rdpq_tex_upload_sub`\n for an example of both techniques.\n\n @param tile       Tile descriptor that will be initialized with this texture\n @param tex        Surface containing the texture to load\n @param parms      All optional parameters on where to load the texture and how to sample it. Refer to `rdpq_texparms_t` for more information.\n @return           Number of bytes used in TMEM for this texture\n\n @see `rdpq_tex_upload_sub`\n @see `surface_make_sub`"]
    #[inline]
    pub fn tex_upload(&mut self, tile: Tile, tex: &Surface, parms: Option<&TexParms>) -> u32 {
        unsafe {
            rdpq_tex_upload(
                tile as _,
                &tex.0,
                parms.map(|p| &p.0 as _).unwrap_or_else(core::ptr::null),
            ) as _
        }
    }
    #[doc = "Load a portion of texture into TMEM\n\n This function is similar to [`Self::tex_upload`], but only loads a portion of a texture\n in TMEM. The portion is specified as a rectangle (with exclusive bounds) that must\n be contained within the original texture.\n\n Notice that, after calling this function, you must draw the polygon using texture\n coordinates that are contained within the loaded ones. For instance:\n\n @code{.c}\n      // Load a 32x32 sprite starting at position (100,100) in the\n      // \"spritemap\" surface.\n      rdpq_tex_upload_sub(TILE2, spritemap, 0, 100, 100, 132, 132);\n\n      // Draw the sprite. Notice that we must refer to it using the\n      // original texture coordinates, even if just that portion is in TMEM.\n      rdpq_texture_rectangle(TILE2,\n          pos_x, pos_y, pos_x+32, pos_y+32,   // screen coordinates of the sprite\n          100, 100,                           // texture coordinates\n          1.0, 1.0);                          // texture increments (= no scaling)\n @endcode\n\n An alternative to this function is to call `surface_make_sub` on the texture\n to create a sub-surface, and then call rdpq_tex_upload on the sub-surface.\n The same data will be loaded into TMEM but this time the RDP ignores that\n you are loading a portion of a larger texture:\n\n @code{.c}\n      // Create a sub-surface of spritemap texture. No memory allocations\n      // or pixel copies are performed, this is just a rectangular \"window\"\n      // into the original texture.\n      surface_t hero = surface_make_sub(spritemap, 100, 100, 32, 32);\n\n      // Load the sub-surface. Notice that the RDP is unaware that it is\n      // a sub-surface; it will think that it is a whole texture.\n      rdpq_tex_upload(TILE2, &hero, 0);\n\n      // Draw the sprite. Notice that we must refer to it using\n      // texture coordinates (0,0).\n      rdpq_texture_rectangle(TILE2,\n          pos_x, pos_y, pos_x+32, pos_y+32,   // screen coordinates of the sprite\n          0, 0,                               // texture coordinates\n          1.0, 1.0);                          // texture increments (= no scaling)\n @endcode\n\n The only limit of this second solution is that the sub-surface pointer must\n be 8-byte aligned (like all RDP textures), so it can only be used if the\n rectangle that needs to be loaded respects such constraint as well.\n\n\n @param tile       Tile descriptor that will be initialized with this texture\n @param tex        Surface containing the texture to load\n @param parms      All optional parameters on where to load the texture and how to sample it. Refer to `rdpq_texparms_t` for more information.\n @param s0         Top-left X coordinate of the rectangle to load\n @param t0         Top-left Y coordinate of the rectangle to load\n @param s1         Bottom-right *exclusive* X coordinate of the rectangle\n @param t1         Bottom-right *exclusive* Y coordinate of the rectangle\n @return int       Number of bytes used in TMEM for this texture\n\n @see `rdpq_tex_upload`\n @see `surface_make_sub`"]
    #[inline]
    pub fn tex_upload_sub(
        &mut self,
        tile: Tile,
        tex: &Surface,
        parms: Option<&TexParms>,
        s0: i32,
        t0: i32,
        s1: i32,
        t1: i32,
    ) -> u32 {
        unsafe {
            rdpq_tex_upload_sub(
                tile as _,
                &tex.0,
                parms.map(|p| &p.0 as _).unwrap_or_else(core::ptr::null),
                s0,
                t0,
                s1,
                t1,
            ) as _
        }
    }
    #[doc = "Load one or more palettes into TMEM\n\n This function allows to load one or more palettes into TMEM.\n\n When using palettes, the upper half of TMEM is allocated to them. There is room\n for 256 colors in total, which allows for one palette for a CI8 texture, or up\n to 16 palettes for CI4 textures.\n\n @param tlut          Pointer to the first color entry to load (must be 8-byte aligned)\n @param color_idx     Index of the first color entry in TMEM (0-255)\n @param num_colors    Number of color entries to load (1-256)"]
    #[inline]
    pub fn tex_upload_tlut(&mut self, tlut: &[u16], color_idx: i32) {
        unsafe { rdpq_tex_upload_tlut(tlut.as_ptr() as _, color_idx, tlut.len() as _) }
    }
    #[doc = "Reuse a portion of the previously uploaded texture to TMEM\n\n When a texture has been uploaded, its possible to reuse it for multiple tiles\n without increasing TMEM usage. This function provides a way to achieve this while also\n configuring your own texture parameters for the reused texture.\n\n This sub-variant also allows to specify what part of the uploaded texture must be reused.\n For example, after uploading a 64x64 texture (or a 64x64 sub texture of a larger surface),\n you can reuse an existing portion of it, like (16,16)-(48,48) or (0,0)-(8,32).\n Restrictions of rdpq_texparms_t apply just when reusing just as well as for uploading a texture.\n\n Sub-rectangle must be within the bounds of the texture reused and be 8-byte aligned,\n not all starting positions are valid for different formats.\n\n Starting horizontal position s0 must be 8-byte aligned, meaning for different image formats\n you can use TEX_FORMAT_BYTES2PIX(fmt, bytes) with bytes being in multiples of 8.\n Starting vertical position t0 must be in multiples of 2 pixels due to TMEM arrangement.\n\n Leaving parms to NULL will copy the previous' texture texparms.\n\n NOTE: This function must be executed in a multi-upload block right after the reused texture has been\n uploaded.\n\n @param tile       Tile descriptor that will be initialized with reused texture\n @param parms      All optional parameters on how to sample reused texture. Refer to `rdpq_texparms_t` for more information.\n @param s0         Top-left X coordinate of the rectangle to reuse\n @param t0         Top-left Y coordinate of the rectangle to reuse\n @param s1         Bottom-right *exclusive* X coordinate of the rectangle\n @param t1         Bottom-right *exclusive* Y coordinate of the rectangle\n @return int       Number of bytes used in TMEM for this texture (always 0)"]
    #[inline]
    pub fn tex_reuse_sub(
        &mut self,
        tile: Tile,
        parms: Option<&TexParms>,
        s0: i32,
        t0: i32,
        s1: i32,
        t1: i32,
    ) -> u32 {
        unsafe {
            rdpq_tex_reuse_sub(
                tile as _,
                parms.map(|p| &p.0 as _).unwrap_or_else(core::ptr::null),
                s0,
                t0,
                s1,
                t1,
            ) as _
        }
    }
    #[doc = "Reuse the previously uploaded texture to TMEM\n\n When a texture has been uploaded, its possible to reuse it for multiple tiles\n without increasing TMEM usage. This function provides a way to achieve this while also\n configuring your own texture parameters for the reused texture.\n\n This full-variant will use the whole texture that was previously uploaded.\n Leaving parms to NULL will copy the previous' texture texparms.\n\n NOTE: This function must be executed in a multi-upload block right after the reused texture has been\n uploaded.\n\n @param tile       Tile descriptor that will be initialized with reused texture\n @param parms      All optional parameters on how to sample reused texture. Refer to `rdpq_texparms_t` for more information.\n @return int       Number of bytes used in TMEM for this texture (always 0)"]
    #[inline]
    pub fn tex_reuse(&mut self, tile: Tile, parms: Option<&TexParms>) -> u32 {
        unsafe {
            rdpq_tex_reuse(
                tile as _,
                parms.map(|p| &p.0 as _).unwrap_or_else(core::ptr::null),
            ) as _
        }
    }
    #[doc = "Begin a multi-texture upload\n\n This function begins a multi-texture upload, with automatic TMEM layout.\n There are two main cases where you may want to squeeze multiple textures\n within TMEM: when loading mipmaps, and when using multi-texturing.\n\n After calling `rdpq_tex_multi_begin`, you can call `rdpq_tex_upload` multiple\n times in sequence, without manually specifying a TMEM address. The functions\n will start filling TMEM from the beginning, in sequence.\n\n If the TMEM becomes full and is unable to fullfil a load, an assertion\n will be issued.\n\n @note When calling `rdpq_tex_upload` or `rdpq_tex_upload_sub` in this mode,\n       do not specify a TMEM address in the parms structure, as the actual\n       address is automatically calculated.\n\n @see `rdpq_tex_upload`\n @see `rdpq_tex_upload_sub`\n @see `rdpq_tex_multi_end`"]
    #[must_use]
    #[inline]
    pub fn tex_multi_begin(&mut self) -> TexMulti<'_, 'r> {
        unsafe { rdpq_tex_multi_begin() }
        TexMulti(PhantomData)
    }
    #[doc = "Blit a surface to the active framebuffer\n\n This is the highest level function for drawing an arbitrary-sized surface\n to the screen, possibly scaling and rotating it.\n\n It handles all the required steps to blit the entire contents of a surface\n to the framebuffer, that is:\n\n   * Logically split the surface in chunks that fit the TMEM\n   * Calculate an appropriate scaling factor for each chunk\n   * Load each chunk into TMEM (via `rdpq_tex_upload`)\n   * Draw each chunk to the framebuffer (via `rdpq_texture_rectangle` or `rdpq_triangle`)\n\n Note that this function only performs the actual blits, it does not\n configure the rendering mode or handle palettes. Before calling this\n function, make sure to configure the render mode via\n `rdpq_set_mode_standard` (or `rdpq_set_mode_copy` if no scaling and pixel\n format conversion is required). If the surface uses a palette, you also\n need to load the palette using `rdpq_tex_upload_tlut`.\n\n This function is able to perform many different complex transformations. The\n implementation has been tuned to try to be as fast as possible for simple\n blits, but it scales up nicely for more complex operations.\n\n The parameters that describe the transformations to perform are passed in\n the @p parms structure. The structure contains a lot of fields, but it has\n been designed so that most of them can be simply initalized to zero to\n disable advanced behaviors (and thus simply left unmentioned in an inline\n initialization).\n\n For instance, this blits a large image to the screen, aligning it to the\n top-left corner (eg: a splashscreen).\n\n @code{.c}\n     rdpq_tex_blit(splashscreen, 0, 0, NULL);\n @endcode\n\n This is the same, but the image will be centered on the screen. To do this,\n we specify the center of the screen as position, and then we set the hotspost\n of the image (\"cx\" and \"cy\" fields) to its center:\n\n @code{.c}\n      rdpq_tex_blit(splashscreen, 320/2, 160/2, &(rdpq_blitparms_t){\n          .cx = splashscreen->width / 2,\n          .cy = splashscreen->height / 2,\n      });\n @endcode\n\n This examples scales a 64x64 image to 256x256, putting its center near the\n top-left of the screen (so part of resulting image will be offscreen):\n\n @code{.c}\n      rdpq_tex_blit(splashscreen, 20, 20, &(rdpq_blitparms_t){\n          .cx = splashscreen->width / 2, .cy = splashscreen->height / 2,\n          .scale_x = 4.0f, .scale_y = 4.0f,\n      });\n @endcode\n\n This example assumes that the surface is a spritemap with frames of size\n 32x32. It selects the sprite at row 4, column 2, and draws it centered\n at position 100,100 on the screen applying a rotation of 45 degrees around its center:\n\n @code{.c}\n     rdpq_tex_blit(splashscreen, 100, 100, &(rdpq_blitparms_t){\n          .s0 = 32*2, .t0 = 32*4,\n          .width = 32, .height = 32,\n          .cx = 16, .cy = 16,\n          .theta = M_PI/4,\n     });\n @endcode\n\n @param surf           Surface to draw\n @param x0             X coordinate on the framebuffer where to draw the surface\n @param y0             Y coordinate on the framebuffer where to draw the surface\n @param parms          Parameters for the blit operation (or NULL for default)"]
    #[inline]
    pub fn blit(&mut self, surf: &Surface, x0: f32, y0: f32, parms: Option<&BlitParms>) {
        unsafe {
            rdpq_tex_blit(
                &surf.0,
                x0,
                y0,
                parms.map(|p| &p.0 as _).unwrap_or_else(core::ptr::null),
            );
        }
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct TexMulti<'s, 'r>(PhantomData<&'s mut RdpQ<'r>>);

static_wrapper! { TexMulti<'s, 'r> => RdpQ<'r> { RdpQ(PhantomData) } }

impl<'s, 'r> TexMulti<'s, 'r> {
    #[doc = "Load a texture into TMEM\n\n This function helps loading a texture into TMEM, which normally involves:\n\n   * Configuring a tile descriptor (via `rdpq_set_tile`)\n   * Setting the source texture image (via `rdpq_set_texture_image`)\n   * Loading the texture (via `rdpq_load_tile` or `rdpq_load_block`)\n\n After calling this function, the specified tile descriptor will be ready\n to be used in drawing primitives like `rdpq_triangle` or `rdpq_texture_rectangle`.\n\n If the texture uses a palette (`FMT_CI8` or `FMT_CI4`), the tile descriptor\n will be by default pointing to palette 0. In the case of `FMT_CI4`, this\n might not be the correct palette; to specify a different palette number,\n add .palette = X to the tex parms. Before drawing a texture with palette,\n remember to call `rdpq_mode_tlut` to activate palette mode.\n\n If you want to load a portion of a texture rather than the full texture,\n use `rdpq_tex_upload_sub`, or alternatively create a sub-surface using\n `surface_make_sub` and pass it to `rdpq_tex_upload`. See `rdpq_tex_upload_sub`\n for an example of both techniques.\n\n @param tile       Tile descriptor that will be initialized with this texture\n @param tex        Surface containing the texture to load\n @param parms      All optional parameters on where to load the texture and how to sample it. Refer to `rdpq_texparms_t` for more information.\n @return           Number of bytes used in TMEM for this texture\n\n @see `rdpq_tex_upload_sub`\n @see `surface_make_sub`"]
    #[inline]
    pub fn tex_upload(&mut self, tile: Tile, tex: &Surface, parms: Option<&TexParms>) -> u32 {
        (**self).tex_upload(tile, tex, parms)
    }
    #[doc = "Load a portion of texture into TMEM\n\n This function is similar to [`Self::tex_upload`], but only loads a portion of a texture\n in TMEM. The portion is specified as a rectangle (with exclusive bounds) that must\n be contained within the original texture.\n\n Notice that, after calling this function, you must draw the polygon using texture\n coordinates that are contained within the loaded ones. For instance:\n\n @code{.c}\n      // Load a 32x32 sprite starting at position (100,100) in the\n      // \"spritemap\" surface.\n      rdpq_tex_upload_sub(TILE2, spritemap, 0, 100, 100, 132, 132);\n\n      // Draw the sprite. Notice that we must refer to it using the\n      // original texture coordinates, even if just that portion is in TMEM.\n      rdpq_texture_rectangle(TILE2,\n          pos_x, pos_y, pos_x+32, pos_y+32,   // screen coordinates of the sprite\n          100, 100,                           // texture coordinates\n          1.0, 1.0);                          // texture increments (= no scaling)\n @endcode\n\n An alternative to this function is to call `surface_make_sub` on the texture\n to create a sub-surface, and then call rdpq_tex_upload on the sub-surface.\n The same data will be loaded into TMEM but this time the RDP ignores that\n you are loading a portion of a larger texture:\n\n @code{.c}\n      // Create a sub-surface of spritemap texture. No memory allocations\n      // or pixel copies are performed, this is just a rectangular \"window\"\n      // into the original texture.\n      surface_t hero = surface_make_sub(spritemap, 100, 100, 32, 32);\n\n      // Load the sub-surface. Notice that the RDP is unaware that it is\n      // a sub-surface; it will think that it is a whole texture.\n      rdpq_tex_upload(TILE2, &hero, 0);\n\n      // Draw the sprite. Notice that we must refer to it using\n      // texture coordinates (0,0).\n      rdpq_texture_rectangle(TILE2,\n          pos_x, pos_y, pos_x+32, pos_y+32,   // screen coordinates of the sprite\n          0, 0,                               // texture coordinates\n          1.0, 1.0);                          // texture increments (= no scaling)\n @endcode\n\n The only limit of this second solution is that the sub-surface pointer must\n be 8-byte aligned (like all RDP textures), so it can only be used if the\n rectangle that needs to be loaded respects such constraint as well.\n\n\n @param tile       Tile descriptor that will be initialized with this texture\n @param tex        Surface containing the texture to load\n @param parms      All optional parameters on where to load the texture and how to sample it. Refer to `rdpq_texparms_t` for more information.\n @param s0         Top-left X coordinate of the rectangle to load\n @param t0         Top-left Y coordinate of the rectangle to load\n @param s1         Bottom-right *exclusive* X coordinate of the rectangle\n @param t1         Bottom-right *exclusive* Y coordinate of the rectangle\n @return int       Number of bytes used in TMEM for this texture\n\n @see `rdpq_tex_upload`\n @see `surface_make_sub`"]
    #[inline]
    pub fn tex_upload_sub(
        &mut self,
        tile: Tile,
        tex: &Surface,
        parms: Option<&TexParms>,
        s0: i32,
        t0: i32,
        s1: i32,
        t1: i32,
    ) -> u32 {
        (**self).tex_upload_sub(tile, tex, parms, s0, t0, s1, t1)
    }
    #[doc = "Finish a multi-texture upload\n\n This function finishes a multi-texture upload. See `rdpq_tex_multi_begin`\n for more information.\n\n @returns The number of bytes used in TMEM for this multi-texture upload\n\n @see `rdpq_tex_multi_begin`."]
    #[inline]
    pub fn end(self) -> u32 {
        unsafe { rdpq_tex_multi_end() as _ }
    }
}

impl<'s, 'r> crate::Undroppable for &mut TexMulti<'s, 'r> {
    const ERROR: &'static str = "Finish the multi-texture upload with TexMulti::end";
}

impl<'s, 'r> Drop for TexMulti<'s, 'r> {
    #[inline]
    fn drop(&mut self) {
        let _ = crate::DropBomb::new(self);
    }
}

pub trait TriOffsets {
    #[doc = "Index of the position component within the vertex arrays.\n\n For instance, if `pos_offset == 4`, `v1[4]` and `v1[5]` must be the X and Y\n coordinates of the first vertex."]
    const POS: u32;
    #[doc = "Index of the shade component within the vertex arrays.\n\n For instance, if `shade_offset == 4`, `v1[4]`, `v1[5]`, `v1[6]`, `v1[7]` must be\n the R, G, B, A values associated to the first vertex. If shade_offset is less\n than 0, no shade component will be used to draw the triangle."]
    const SHADE: Option<u32>;
    #[doc = "Index of the texture component within the vertex arrays.\n\n For instance, if `tex_offset == 4`, `v1[4]`, `v1[5]`, `v1[6]` must be the S, T, W\n values associated to the first vertex. If tex_offset is less than 0, no texture\n component will be used to draw the triangle."]
    const TEX: Option<u32>;
    #[doc = "Index of the depth component within the vertex array.\n\n For instance, if `z_offset == 4`, `v1[4]` must be the Z coordinate of the first\n vertex. If z_offset is less than 0, no depth component will be used to\n draw the triangle."]
    const Z: Option<u32>;
}

#[doc = "Format descriptor of a triangle\n\n This structure holds the parameters required to draw triangles.\n It contains both a description of the vertex format, and some\n configuration parameters for the triangle rasterizer.\n\n This library provides a few predefined formats (such as `TRIFMT_FILL`,\n `TRIFMT_TEX`, etc.) but you are free to define your own format.\n\n There is no overhead in using a custom format or even switching\n format from a triangle to another (besides the required mode changes),\n so feel free to define as many formats are required for your application.\n\n Refer to `rdpq_triangle` for a description of the different vertex\n components."]
#[repr(C)]
pub struct TriFormat<O: TriOffsets> {
    pos_offset: u32,
    shade_offset: i32,
    #[doc = "If true, draw the triangle with flat shading (instead of gouraud shading).\n\n This parameter is ignored if the shade component does not exist (`shade_offset < 0`).\n Normally, gouraud shading is used to draw triangles, which means that the shading\n of each vertex is interpolated across the triangle. If flat shading is enabled, the\n shading of the first vertex is used for the whole triangle."]
    pub shade_flat: bool,
    tex_offset: i32,
    #[doc = "RDP tile descriptor that describes the texture (0-7).\n\n This parameter is ignored if the texture component does not exist (`tex_offset < 0`).\n In case of multi-texturing, `tile + 1` will be used for the second texture.\n Notice that the tile descriptor must be configured before drawing the triangle."]
    pub tex_tile: Tile,
    #[doc = "Number of mipmaps to use for the texture.\n\n This parameter is ignored if the texture component does not exist (`tex_offset < 0`),\n or if mipmapping has not been configured.\n\n Notice that when using the mode API (`rdpq_mode_mipmap`), the number of mipmaps\n is specified there, so this parameter should be left to zero."]
    pub tex_mipmaps: u32,
    z_offset: i32,
    _offsets: PhantomData<O>,
}
const _: () = {
    assert!(size_of::<TriFormat<tri::Fill>>() == size_of::<crate::sys::rdpq_tri::rdpq_trifmt_t>());
};

impl<O: TriOffsets> Default for TriFormat<O> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<O: TriOffsets> Clone for TriFormat<O> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            pos_offset: self.pos_offset,
            shade_offset: self.shade_offset,
            shade_flat: self.shade_flat,
            tex_offset: self.tex_offset,
            tex_tile: self.tex_tile,
            tex_mipmaps: self.tex_mipmaps,
            z_offset: self.z_offset,
            _offsets: self._offsets,
        }
    }
}

impl<O: TriOffsets> Copy for TriFormat<O> {}

impl<O: TriOffsets> TriFormat<O> {
    #[inline]
    pub const fn new() -> Self {
        TriFormat {
            pos_offset: O::POS,
            shade_offset: match O::SHADE {
                Some(o) => o as i32,
                None => -1,
            },
            shade_flat: false,
            tex_offset: match O::TEX {
                Some(o) => o as i32,
                None => -1,
            },
            tex_tile: Tile::_0,
            tex_mipmaps: 0,
            z_offset: match O::Z {
                Some(o) => o as i32,
                None => -1,
            },
            _offsets: PhantomData,
        }
    }
    #[doc = "If true, draw the triangle with flat shading (instead of gouraud shading).\n\n This parameter is ignored if the shade component does not exist (`shade_offset < 0`).\n Normally, gouraud shading is used to draw triangles, which means that the shading\n of each vertex is interpolated across the triangle. If flat shading is enabled, the\n shading of the first vertex is used for the whole triangle."]
    #[inline]
    pub fn shade_flat(mut self, shade_flat: bool) -> Self {
        self.shade_flat = shade_flat;
        self
    }
    #[doc = "RDP tile descriptor that describes the texture (0-7).\n\n This parameter is ignored if the texture component does not exist (`tex_offset < 0`).\n In case of multi-texturing, `tile + 1` will be used for the second texture.\n Notice that the tile descriptor must be configured before drawing the triangle."]
    #[inline]
    pub fn tex_tile(mut self, tile: Tile) -> Self {
        self.tex_tile = tile;
        self
    }
    #[doc = "Number of mipmaps to use for the texture.\n\n This parameter is ignored if the texture component does not exist (`tex_offset < 0`),\n or if mipmapping has not been configured.\n\n Notice that when using the mode API (`rdpq_mode_mipmap`), the number of mipmaps\n is specified there, so this parameter should be left to zero."]
    #[inline]
    pub fn tex_mipmaps(mut self, tex_mipmaps: u32) -> Self {
        self.tex_mipmaps = tex_mipmaps;
        self
    }
    #[inline]
    pub const fn minimum_array_size() -> usize {
        let mut size = O::POS as usize + 2;
        if let Some(shade) = O::SHADE {
            let s = shade as usize + 4;
            if s > size {
                size = s;
            }
        }
        if let Some(tex) = O::TEX {
            let s = tex as usize + 3;
            if s > size {
                size = s;
            }
        }
        if let Some(z) = O::Z {
            let s = z as usize + 1;
            if s > size {
                size = s;
            }
        }
        size
    }
}

impl<'r> RdpQ<'r> {
    #[doc = "Draw a triangle (RDP command: TRI_*)\n\n This function allows to draw a triangle into the framebuffer using RDP, in screen coordinates.\n RDP does not handle transform and lightning, so it only reasons of screen level coordinates.\n\n Each vertex of a triangle is made of up to 4 components:\n\n   * Position. 2 values: X, Y. The values must be in screen coordinates, that is they refer\n     to the framebuffer pixels. Fractional values allow for subpixel precision. Supported\n     range is [-4096..4095] (numbers outside that range will be clamped).\n   * Depth. 1 value: Z. Supported range in [0..1].\n   * Shade. 4 values: R, G, B, A. The values must be in the 0..1 range.\n   * Texturing. 3 values: S, T, INV_W. The values S,T address the texture specified by the tile\n     descriptor. INV_W is the inverse of the W vertex coordinate in clip space (after\n     projection), a value commonly used to do the final perspective division. This value is\n     required to do perspective-corrected texturing.\n\n Only the position is mandatory, all other components are optionals, depending on the kind of\n triangle that needs to be drawn. For instance, specifying only position and shade will allow\n to draw a gouraud-shaded triangle with no texturing and no z-buffer usage.\n\n The vertex components must be provided via arrays of floating point values. The order of\n the components within the array is flexible, and can be specified at call time via the\n `rdpq_trifmt_t` structure.\n\n Notice that it is important to configure the correct render modes before calling this function.\n Specifically:\n\n    * To use the depth component, you must activate the z-buffer via `rdpq_mode_zbuf`.\n    * To use the shade component, you must configure a color combiner formula via `rdpq_mode_combiner`.\n      The formula must use the SHADE slot, to specify the exact pixel formula that will combine the\n      per-pixel color value with other components, like the texture.\n    * To use the texturing component, you must configure a color combiner formula via `rdpq_mode_combiner`\n      that uses the TEX0 (and/or TEX1) slot, such as `RDPQ_COMBINER_TEX` or `RDPQ_COMBINER_SHADE`,\n      to specify the exact pixel formula that will combine the per-pixel color value with other\n      components, like the shade. Moreover, you can activate perspective texturing via `rdpq_mode_persp`.\n\n If you fail to activate a specific render mode for a provided component, the component will be ignored\n by RDP. For instance, if you provide S,T,W but do not configure a combiner formula that accesses\n TEX0, the texture will not be rendered. On the contrary, if you activate a specific render mode\n but then fail to provide the component (eg: activate z buffering but then fail to provide a depth\n component), RDP will fall into undefined behavior that can vary from nothing being rendered, garbage\n on the screen or even a freeze. The rdpq validator will do its best to help you catching these mistakes,\n so remember to activate it via `rdpq_debug_start` whenever you get a surprising result.\n\n For instance, this code snippet will draw a filled triangle, with a flat green color:\n\n @code\n      // Reset to standard rendering mode.\n      rdpq_set_mode_standard();\n\n      // Configure the combiner for flat-color rendering\n      rdpq_mode_combiner(RDPQ_COMBINER_FLAT);\n\n      // Configure the flat color\n      rdpq_set_prim_color(RGBA32(0, 255, 0, 255));\n\n      // Draw the triangle\n      float v1[] = { 100, 100 };\n      float v2[] = { 200, 200 };\n      float v3[] = { 100, 200 };\n      rdpq_triangle(&TRIFMT_FILL, v1, v2, v3);\n @endcode\n\n The three vertices (v1, v2, v3) can be provided in any order (clockwise or counter-clockwise). The\n function will render the triangle in any case (so back-face culling must be handled before calling\n it).\n\n @param fmt            Format of the triangle being drawn. This structure specifies the order of the\n                       components within the vertex arrays, and also some additional rasterization\n                       parameters. You can pass one of the predefined formats (`TRIFMT_FILL`,\n                       `TRIFMT_TEX`, etc.), or a custom one.\n @param v1             Array of components for vertex 1\n @param v2             Array of components for vertex 2\n @param v3             Array of components for vertex 3"]
    pub fn triangle<O: TriOffsets, const N: usize>(
        &mut self,
        fmt: &TriFormat<O>,
        v1: &[f32; N],
        v2: &[f32; N],
        v3: &[f32; N],
    ) {
        const {
            assert!(N >= TriFormat::<O>::minimum_array_size());
        }
        unsafe {
            crate::sys::rdpq_tri::rdpq_triangle(
                core::mem::transmute(fmt),
                v1.as_ptr(),
                v2.as_ptr(),
                v3.as_ptr(),
            )
        }
    }
}

pub mod tri {
    pub struct Offsets<const POS: u32, const SHADE: i32, const TEX: i32, const Z: i32>(());

    impl<const POS: u32, const SHADE: i32, const TEX: i32, const Z: i32> super::TriOffsets
        for Offsets<POS, SHADE, TEX, Z>
    {
        const POS: u32 = POS;
        const SHADE: Option<u32> = const { if SHADE >= 0 { Some(SHADE as u32) } else { None } };
        const TEX: Option<u32> = const { if TEX >= 0 { Some(TEX as u32) } else { None } };
        const Z: Option<u32> = const { if Z >= 0 { Some(Z as u32) } else { None } };
    }

    #[doc = "Format descriptor for a solid-filled triangle.\n\n Vertex array format: `(float){X, Y}` (2 floats)\n\n Given that only position is provided, the triangle is drawn with a solid color,\n which is the output of the color combiner. See `rdpq_mode_combiner` for more\n information.\n\n A common choice for a combiner formula is `RDPQ_COMBINER_FLAT`, that will\n simply output whatever color is configured via `rdpq_set_prim_color`."]
    pub type Fill = Offsets<0, -1, -1, -1>;
    #[doc = "Format descriptor for a shaded triangle.\n\n Vertex array format: `(float){X, Y, R, G, B, A}` (6 floats)\n\n The suggested standard color combiner for this format is `RDPQ_COMBINER_SHADE`."]
    pub type Shade = Offsets<0, 2, -1, -1>;
    #[doc = "Format descriptor for a textured triangle.\n\n Vertex array format: `(float){X, Y, S, T, INV_W}` (5 floats)\n\n The suggested standard color combiner for this format is `RDPQ_COMBINER_TEX`."]
    pub type Tex = Offsets<0, -1, 2, -1>;
    #[doc = "Format descriptor for a shaded, textured triangle.\n\n Vertex array format: `(float){X, Y, R, G, B, A, S, T, INV_W}` (9 floats)\n\n The suggested standard color combiner for this format is `RDPQ_COMBINER_TEX_SHADE`."]
    pub type ShadeTex = Offsets<0, 2, 6, -1>;
    #[doc = "Format descriptor for a solid-filled, z-buffered triangle.\n\n Vertex array format: `(float){X, Y, Z}` (3 floats)"]
    pub type ZBuf = Offsets<0, -1, -1, 2>;
    #[doc = "Format descriptor for a z-buffered, shaded triangle.\n\n Vertex array format: `(float){X, Y, Z, R, G, B, A}` (7 floats)"]
    pub type ZBufShade = Offsets<0, 3, -1, 2>;
    #[doc = "Format descriptor for a z-buffered, textured triangle.\n\n Vertex array format: `(float){X, Y, Z, S, T, INV_W}` (6 floats)"]
    pub type ZBufTex = Offsets<0, -1, 3, 2>;
    #[doc = "Format descriptor for a z-buffered, shaded, textured triangle.\n\n Vertex array format: `(float){X, Y, Z, R, G, B, A, S, T, INV_W}` (10 floats)"]
    pub type ZBufShadeTex = Offsets<0, 3, 7, 2>;
}

bitflags::bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
    pub struct SOM: u64 {
        #[doc = "RDPQ special state: fogging is enabled"]
        const X_FOG = SOMX_FOG as u64;

        #[doc = "Atomic: serialize command execution "]
        const ATOMIC_PRIM = SOM_ATOMIC_PRIM as u64;

        #[doc = "Set cycle-type: 1cyc"]
        const CYCLE_1 = SOM_CYCLE_1 as u64;
        #[doc = "Set cycle-type: 2cyc"]
        const CYCLE_2 = SOM_CYCLE_2 as u64;
        #[doc = "Set cycle-type: copy"]
        const CYCLE_COPY = SOM_CYCLE_COPY as u64;
        #[doc = "Set cycle-type: fill"]
        const CYCLE_FILL = SOM_CYCLE_FILL as u64;

        #[doc = "Texture: enable perspective correction"]
        const TEXTURE_PERSP = SOM_TEXTURE_PERSP as u64;
        #[doc = "Texture: enable 'detail'"]
        const TEXTURE_DETAIL = SOM_TEXTURE_DETAIL as u64;
        #[doc = "Texture: enable 'sharpen'"]
        const TEXTURE_SHARPEN = SOM_TEXTURE_SHARPEN as u64;
        #[doc = "Texture: enable LODs."]
        const TEXTURE_LOD = SOM_TEXTURE_LOD as u64;

        #[doc = "TLUT: no palettes"]
        const TLUT_NONE = SOM_TLUT_NONE as u64;
        #[doc = "TLUT: draw with palettes in format RGB16"]
        const TLUT_RGBA16 = SOM_TLUT_RGBA16 as u64;
        #[doc = "TLUT: draw with palettes in format IA16"]
        const TLUT_IA16 = SOM_TLUT_IA16 as u64;

        #[doc = "Texture sampling: point sampling (1x1)"]
        const SAMPLE_POINT = SOM_SAMPLE_POINT as u64;
        #[doc = "Texture sampling: bilinear interpolation (2x2)"]
        const SAMPLE_BILINEAR = SOM_SAMPLE_BILINEAR as u64;
        #[doc = "Texture sampling: mid-texel average (2x2)"]
        const SAMPLE_MEDIAN = SOM_SAMPLE_MEDIAN as u64;

        #[doc = "Texture Filter, cycle 0 (TEX0): standard fetching (for RGB)"]
        const TF0_RGB = SOM_TF0_RGB as u64;
        #[doc = "Texture Filter, cycle 0 (TEX0): fetch nearest and do first step of color conversion (for YUV)"]
        const TF0_YUV = SOM_TF0_YUV as u64;
        #[doc = "Texture Filter, cycle 1 (TEX1): standard fetching (for RGB)"]
        const TF1_RGB = SOM_TF1_RGB as u64;
        #[doc = "Texture Filter, cycle 1 (TEX1): fetch nearest and do first step of color conversion (for YUV)"]
        const TF1_YUV = SOM_TF1_YUV as u64;
        #[doc = "Texture Filter, cycle 1 (TEX1): don't fetch, and instead do color conversion on TEX0 (allows YUV with bilinear filtering)"]
        const TF1_YUVTEX0 = SOM_TF1_YUVTEX0 as u64;

        #[doc = "RGB Dithering: square filter"]
        const RGBDITHER_SQUARE = SOM_RGBDITHER_SQUARE as u64;
        #[doc = "RGB Dithering: bayer filter"]
        const RGBDITHER_BAYER = SOM_RGBDITHER_BAYER as u64;
        #[doc = "RGB Dithering: noise"]
        const RGBDITHER_NOISE = SOM_RGBDITHER_NOISE as u64;
        #[doc = "RGB Dithering: none"]
        const RGBDITHER_NONE = SOM_RGBDITHER_NONE as u64;

        #[doc = "Alpha Dithering: same as RGB"]
        const ALPHADITHER_SAME = SOM_ALPHADITHER_SAME as u64;
        #[doc = "Alpha Dithering: invert pattern compared to RG"]
        const ALPHADITHER_INVERT = SOM_ALPHADITHER_INVERT as u64;
        #[doc = "Alpha Dithering: noise"]
        const ALPHADITHER_NOISE = SOM_ALPHADITHER_NOISE as u64;
        #[doc = "Alpha Dithering: none"]
        const ALPHADITHER_NONE = SOM_ALPHADITHER_NONE as u64;

        #[doc = "RDPQ special state: mimap interpolation (aka trilinear) requested"]
        const X_LOD_INTERPOLATE = SOMX_LOD_INTERPOLATE as u64;
        #[doc = "RDPQ special state: mimap interpolation for SHC texture format"]
        const X_LOD_INTERPOLATE_SHQ = SOMX_LOD_INTERPOLATE_SHQ as u64;
        #[doc = "RDPQ special state: reduced antialiasing is enabled"]
        const X_AA_REDUCED = SOMX_AA_REDUCED as u64;
        #[doc = "RDPQ special state: render mode update is frozen (see #rdpq_mode_begin)"]
        const X_UPDATE_FREEZE = SOMX_UPDATE_FREEZE as u64;

        #[doc = "RDPQ special state: record that the blender is made of 2 passes"]
        const X_BLEND_2PASS = SOMX_BLEND_2PASS as u64;

        #[doc = "Activate blending for all pixels"]
        const BLENDING = SOM_BLENDING as u64;

        #[doc = "Blender IN_ALPHA is the output of the combiner output (default)"]
        const BLALPHA_CC = SOM_BLALPHA_CC as u64;
        #[doc = "Blender IN_ALPHA is the coverage of the current pixel"]
        const BLALPHA_CVG = SOM_BLALPHA_CVG as u64;
        #[doc = "Blender IN_ALPHA is the product of the combiner output and the coverage"]
        const BLALPHA_CVG_TIMES_CC = SOM_BLALPHA_CVG_TIMES_CC as u64;

        #[doc = "Z-mode: opaque surface"]
        const ZMODE_OPAQUE = SOM_ZMODE_OPAQUE as u64;
        #[doc = "Z-mode: interprenating surfaces"]
        const ZMODE_INTERPENETRATING = SOM_ZMODE_INTERPENETRATING as u64;
        #[doc = "Z-mode: transparent surface"]
        const ZMODE_TRANSPARENT = SOM_ZMODE_TRANSPARENT as u64;
        #[doc = "Z-mode: decal surface"]
        const ZMODE_DECAL = SOM_ZMODE_DECAL as u64;

        #[doc = "Activate Z-buffer write"]
        const Z_WRITE = SOM_Z_WRITE as u64;

        #[doc = "Activate Z-buffer compare"]
        const Z_COMPARE = SOM_Z_COMPARE as u64;

        #[doc = "Z-source: per-pixel Z"]
        const ZSOURCE_PIXEL = SOM_ZSOURCE_PIXEL as u64;
        #[doc = "Z-source: fixed value"]
        const ZSOURCE_PRIM = SOM_ZSOURCE_PRIM as u64;

        #[doc = "Alpha Compare: disable"]
        const ALPHACOMPARE_NONE = SOM_ALPHACOMPARE_NONE as u64;
        #[doc = "Alpha Compare: use blend alpha as threshold"]
        const ALPHACOMPARE_THRESHOLD = SOM_ALPHACOMPARE_THRESHOLD as u64;
        #[doc = "Alpha Compare: use noise as threshold"]
        const ALPHACOMPARE_NOISE = SOM_ALPHACOMPARE_NOISE as u64;

        #[doc = "Enable reads from framebuffer"]
        const READ_ENABLE = SOM_READ_ENABLE as u64;
        #[doc = "Enable anti-alias"]
        const AA_ENABLE = SOM_AA_ENABLE as u64;

        #[doc = "Coverage: add and clamp to 7 (full)"]
        const COVERAGE_DEST_CLAMP = SOM_COVERAGE_DEST_CLAMP as u64;
        #[doc = "Coverage: add and wrap from 0"]
        const COVERAGE_DEST_WRAP = SOM_COVERAGE_DEST_WRAP as u64;
        #[doc = "Coverage: force 7 (full)"]
        const COVERAGE_DEST_ZAP = SOM_COVERAGE_DEST_ZAP as u64;
        #[doc = "Coverage: save (don't write)"]
        const COVERAGE_DEST_SAVE = SOM_COVERAGE_DEST_SAVE as u64;

        #[doc = "Update color buffer only on coverage overflow"]
        const COLOR_ON_CVG_OVERFLOW = SOM_COLOR_ON_CVG_OVERFLOW as u64;
    }
}

impl SOM {
    #[doc = "Rdpq extension: number of LODs shift"]
    pub const X_NUMLODS_SHIFT: u32 = SOMX_NUMLODS_SHIFT;
    #[doc = "Cycle-type shift"]
    pub const CYCLE_SHIFT: u32 = SOM_CYCLE_SHIFT;
    #[doc = "Texture: LODs shift"]
    pub const TEXTURE_LOD_SHIFT: u32 = SOM_TEXTURE_LOD_SHIFT;
    #[doc = "TLUT mask shift"]
    pub const TLUT_SHIFT: u32 = SOM_TLUT_SHIFT;
    #[doc = "Texture sampling mask shift"]
    pub const SAMPLE_SHIFT: u32 = SOM_SAMPLE_SHIFT;
    #[doc = "Texture filter mask shift"]
    pub const TF_SHIFT: u32 = SOM_TF_SHIFT;
    #[doc = "RGB Dithering mask shift"]
    pub const RGBDITHER_SHIFT: u32 = SOM_RGBDITHER_SHIFT;
    #[doc = "Alpha Dithering mask shift"]
    pub const ALPHADITHER_SHIFT: u32 = SOM_ALPHADITHER_SHIFT;
    #[doc = "RDPQ special state: shift for LOD interpolation formulas"]
    pub const X_LOD_INTERP_SHIFT: u32 = SOMX_LOD_INTERP_SHIFT;
    #[doc = "Blender alpha configuration shift"]
    pub const BLALPHA_SHIFT: u32 = SOM_BLALPHA_SHIFT;
    #[doc = "Z-mode mask shift"]
    pub const ZMODE_SHIFT: u32 = SOM_ZMODE_SHIFT;
    #[doc = "Z-buffer write bit shift"]
    pub const Z_WRITE_SHIFT: u32 = SOM_Z_WRITE_SHIFT;
    #[doc = "Z-buffer compare bit shift"]
    pub const Z_COMPARE_SHIFT: u32 = SOM_Z_COMPARE_SHIFT;
    #[doc = "Z-source mask shift"]
    pub const ZSOURCE_SHIFT: u32 = SOM_ZSOURCE_SHIFT;
    #[doc = "Alpha Compare mask shift"]
    pub const ALPHACOMPARE_SHIFT: u32 = SOM_ALPHACOMPARE_SHIFT;
    #[doc = "Coverage mask shift"]
    pub const COVERAGE_DEST_SHIFT: u32 = SOM_COVERAGE_DEST_SHIFT;
    #[inline]
    pub fn insert_blender(&mut self, blender: Blender) {
        self.insert(SOM::from_bits_retain(blender.into_inner() as u64));
    }
}

bitflags::bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
    pub struct SOMMask: u64 {
        #[doc = "Rdpq extension: number of LODs"]
        const X_NUMLODS = SOMX_NUMLODS_MASK as u64;
        #[doc = "RDPQ special state: fogging is enabled"]
        const X_FOG = SOMX_FOG as u64;
        #[doc = "Atomic: serialize command execution "]
        const ATOMIC_PRIM = SOM_ATOMIC_PRIM as u64;
        #[doc = "Cycle-type mask"]
        const CYCLE = SOM_CYCLE_MASK as u64;
        #[doc = "Texture: enable perspective correction"]
        const TEXTURE_PERSP = SOM_TEXTURE_PERSP as u64;
        #[doc = "Texture: enable 'detail'"]
        const TEXTURE_DETAIL = SOM_TEXTURE_DETAIL as u64;
        #[doc = "Texture: enable 'sharpen'"]
        const TEXTURE_SHARPEN = SOM_TEXTURE_SHARPEN as u64;
        #[doc = "Texture: enable LODs."]
        const TEXTURE_LOD = SOM_TEXTURE_LOD as u64;
        #[doc = "TLUT mask"]
        const TLUT = SOM_TLUT_MASK as u64;
        #[doc = "Texture sampling mask"]
        const SAMPLE = SOM_SAMPLE_MASK as u64;
        #[doc = "Texture Filter mask"]
        const TF = SOM_TF_MASK as u64;
        #[doc = "RGB Dithering mask"]
        const RGBDITHER = SOM_RGBDITHER_MASK as u64;
        #[doc = "Alpha Dithering mask"]
        const ALPHADITHER = SOM_ALPHADITHER_MASK as u64;
        #[doc = "RDPQ special state: mask for LOD interpolation formulas"]
        const X_LOD_INTERP = SOMX_LOD_INTERP_MASK as u64;
        #[doc = "RDPQ special state: reduced antialiasing is enabled"]
        const X_AA_REDUCED = SOMX_AA_REDUCED as u64;
        #[doc = "RDPQ special state: render mode update is frozen (see #rdpq_mode_begin)"]
        const X_UPDATE_FREEZE = SOMX_UPDATE_FREEZE as u64;
        #[doc = "Blender: mask of settings related to pass 0"]
        const BLEND0 = SOM_BLEND0_MASK as u64;
        #[doc = "Blender: mask of settings related to pass 1"]
        const BLEND1 = SOM_BLEND1_MASK as u64;
        #[doc = "Blender: mask of all settings"]
        const BLEND = SOM_BLEND_MASK as u64;
        #[doc = "RDPQ special state: record that the blender is made of 2 passes"]
        const X_BLEND_2PASS = SOMX_BLEND_2PASS as u64;
        #[doc = "Activate blending for all pixels"]
        const BLENDING = SOM_BLENDING as u64;
        #[doc = "Blender alpha configuration mask"]
        const BLALPHA = SOM_BLALPHA_MASK as u64;
        #[doc = "Z-mode mask"]
        const ZMODE = SOM_ZMODE_MASK as u64;
        #[doc = "Activate Z-buffer write"]
        const Z_WRITE = SOM_Z_WRITE as u64;
        #[doc = "Activate Z-buffer compare"]
        const Z_COMPARE = SOM_Z_COMPARE as u64;
        #[doc = "Z-source mask"]
        const ZSOURCE = SOM_ZSOURCE_MASK as u64;
        #[doc = "Alpha Compare mask"]
        const ALPHACOMPARE = SOM_ALPHACOMPARE_MASK as u64;
        #[doc = "Enable reads from framebuffer"]
        const READ_ENABLE = SOM_READ_ENABLE as u64;
        #[doc = "Enable anti-alias"]
        const AA_ENABLE = SOM_AA_ENABLE as u64;
        #[doc = "Coverage mask"]
        const COVERAGE_DEST = SOM_COVERAGE_DEST_MASK as u64;
        #[doc = "Update color buffer only on coverage overflow"]
        const COLOR_ON_CVG_OVERFLOW = SOM_COLOR_ON_CVG_OVERFLOW as u64;
    }
}

macro_rules! impl_macro_enums {
    (#[repr($repr:ident)] $($item:item)*) => {
        $(
            #[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
            #[doc(hidden)]
            #[repr($repr)]
            $item
        )*
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! _rdpq_mode_input {
    (<$ty:ty>::0) => {
        (<$ty>::Zero)
    };
    (<$ty:ty>::1) => {
        (<$ty>::One)
    };
    (<$ty:ty>::$ident:ident) => {
        (<$ty>::$ident)
    };
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(transparent)]
pub struct Combiner(pub(crate) u64);

impl Combiner {
    #[inline]
    pub const unsafe fn new_unchecked(combiner: u64) -> Self {
        Self(combiner)
    }
    #[inline]
    pub const fn into_inner(self) -> u64 {
        self.0
    }
    #[inline]
    pub const fn comb0(self) -> u64 {
        self.0 & RDPQ_COMB0_MASK
    }
    #[inline]
    pub const fn comb1(self) -> u64 {
        self.0 & RDPQ_COMB1_MASK
    }
}

impl From<Combiner> for u64 {
    #[inline]
    fn from(value: Combiner) -> Self {
        value.into_inner()
    }
}

impl_macro_enums! { #[repr(u64)]
pub enum Comb1RGBSubA {
    Tex0 = _RDPQ_COMB1_RGB_SUBA_TEX0 as _,
    Prim = _RDPQ_COMB1_RGB_SUBA_PRIM as _,
    Shade = _RDPQ_COMB1_RGB_SUBA_SHADE as _,
    Env = _RDPQ_COMB1_RGB_SUBA_ENV as _,
    One = _RDPQ_COMB1_RGB_SUBA_ONE as _,
    Noise = _RDPQ_COMB1_RGB_SUBA_NOISE as _,
    Zero = _RDPQ_COMB1_RGB_SUBA_ZERO as _,
}
pub enum Comb2ARGBSubA {
    Tex0 = _RDPQ_COMB2A_RGB_SUBA_TEX0 as _,
    Tex1 = _RDPQ_COMB2A_RGB_SUBA_TEX1 as _,
    Prim = _RDPQ_COMB2A_RGB_SUBA_PRIM as _,
    Shade = _RDPQ_COMB2A_RGB_SUBA_SHADE as _,
    Env = _RDPQ_COMB2A_RGB_SUBA_ENV as _,
    One = _RDPQ_COMB2A_RGB_SUBA_ONE as _,
    Noise = _RDPQ_COMB2A_RGB_SUBA_NOISE as _,
    Zero = _RDPQ_COMB2A_RGB_SUBA_ZERO as _,
}
pub enum Comb2BRGBSubA {
    Combined = _RDPQ_COMB2B_RGB_SUBA_COMBINED as _,
    Tex1 = _RDPQ_COMB2B_RGB_SUBA_TEX1 as _,
    Tex0Bug = _RDPQ_COMB2B_RGB_SUBA_TEX0_BUG as _,
    Prim = _RDPQ_COMB2B_RGB_SUBA_PRIM as _,
    Shade = _RDPQ_COMB2B_RGB_SUBA_SHADE as _,
    Env = _RDPQ_COMB2B_RGB_SUBA_ENV as _,
    One = _RDPQ_COMB2B_RGB_SUBA_ONE as _,
    Noise = _RDPQ_COMB2B_RGB_SUBA_NOISE as _,
    Zero = _RDPQ_COMB2B_RGB_SUBA_ZERO as _,
}
pub enum Comb1RGBSubB {
    Tex0 = _RDPQ_COMB1_RGB_SUBB_TEX0 as _,
    Prim = _RDPQ_COMB1_RGB_SUBB_PRIM as _,
    Shade = _RDPQ_COMB1_RGB_SUBB_SHADE as _,
    Env = _RDPQ_COMB1_RGB_SUBB_ENV as _,
    KeyCenter = _RDPQ_COMB1_RGB_SUBB_KEYCENTER as _,
    K4 = _RDPQ_COMB1_RGB_SUBB_K4 as _,
    Zero = _RDPQ_COMB1_RGB_SUBB_ZERO as _,
}
pub enum Comb2ARGBSubB {
    Tex0 = _RDPQ_COMB2A_RGB_SUBB_TEX0 as _,
    Tex1 = _RDPQ_COMB2A_RGB_SUBB_TEX1 as _,
    Prim = _RDPQ_COMB2A_RGB_SUBB_PRIM as _,
    Shade = _RDPQ_COMB2A_RGB_SUBB_SHADE as _,
    Env = _RDPQ_COMB2A_RGB_SUBB_ENV as _,
    KeyCenter = _RDPQ_COMB2A_RGB_SUBB_KEYCENTER as _,
    K4 = _RDPQ_COMB2A_RGB_SUBB_K4 as _,
    Zero = _RDPQ_COMB2A_RGB_SUBB_ZERO as _,
}
pub enum Comb2BRGBSubB {
    Combined = _RDPQ_COMB2B_RGB_SUBB_COMBINED as _,
    Tex1 = _RDPQ_COMB2B_RGB_SUBB_TEX1 as _,
    Tex0Bug = _RDPQ_COMB2B_RGB_SUBB_TEX0_BUG as _,
    Prim = _RDPQ_COMB2B_RGB_SUBB_PRIM as _,
    Shade = _RDPQ_COMB2B_RGB_SUBB_SHADE as _,
    Env = _RDPQ_COMB2B_RGB_SUBB_ENV as _,
    KeyCenter = _RDPQ_COMB2B_RGB_SUBB_KEYCENTER as _,
    K4 = _RDPQ_COMB2B_RGB_SUBB_K4 as _,
    Zero = _RDPQ_COMB2B_RGB_SUBB_ZERO as _,
}
pub enum Comb1RGBMul {
    Tex0 = _RDPQ_COMB1_RGB_MUL_TEX0 as _,
    Prim = _RDPQ_COMB1_RGB_MUL_PRIM as _,
    Shade = _RDPQ_COMB1_RGB_MUL_SHADE as _,
    Env = _RDPQ_COMB1_RGB_MUL_ENV as _,
    KeyScale = _RDPQ_COMB1_RGB_MUL_KEYSCALE as _,
    Tex0Alpha = _RDPQ_COMB1_RGB_MUL_TEX0_ALPHA as _,
    PrimAlpha = _RDPQ_COMB1_RGB_MUL_PRIM_ALPHA as _,
    ShadeAlpha = _RDPQ_COMB1_RGB_MUL_SHADE_ALPHA as _,
    EnvAlpha = _RDPQ_COMB1_RGB_MUL_ENV_ALPHA as _,
    LodFrac = _RDPQ_COMB1_RGB_MUL_LOD_FRAC as _,
    PrimLodFrac = _RDPQ_COMB1_RGB_MUL_PRIM_LOD_FRAC as _,
    K5 = _RDPQ_COMB1_RGB_MUL_K5 as _,
    Zero = _RDPQ_COMB1_RGB_MUL_ZERO as _,
}
pub enum Comb2ARGBMul {
    Tex0 = _RDPQ_COMB2A_RGB_MUL_TEX0 as _,
    Tex1 = _RDPQ_COMB2A_RGB_MUL_TEX1 as _,
    Prim = _RDPQ_COMB2A_RGB_MUL_PRIM as _,
    Shade = _RDPQ_COMB2A_RGB_MUL_SHADE as _,
    Env = _RDPQ_COMB2A_RGB_MUL_ENV as _,
    KeyScale = _RDPQ_COMB2A_RGB_MUL_KEYSCALE as _,
    Tex0Alpha = _RDPQ_COMB2A_RGB_MUL_TEX0_ALPHA as _,
    Tex1Alpha = _RDPQ_COMB2A_RGB_MUL_TEX1_ALPHA as _,
    PrimAlpha = _RDPQ_COMB2A_RGB_MUL_PRIM_ALPHA as _,
    ShadeAlpha = _RDPQ_COMB2A_RGB_MUL_SHADE_ALPHA as _,
    EnvAlpha = _RDPQ_COMB2A_RGB_MUL_ENV_ALPHA as _,
    LodFrac = _RDPQ_COMB2A_RGB_MUL_LOD_FRAC as _,
    PrimLodFrac = _RDPQ_COMB2A_RGB_MUL_PRIM_LOD_FRAC as _,
    K5 = _RDPQ_COMB2A_RGB_MUL_K5 as _,
    Zero = _RDPQ_COMB2A_RGB_MUL_ZERO as _,
}
pub enum Comb2BRGBMul {
    Combined = _RDPQ_COMB2B_RGB_MUL_COMBINED as _,
    Tex1 = _RDPQ_COMB2B_RGB_MUL_TEX1 as _,
    Tex0Bug = _RDPQ_COMB2B_RGB_MUL_TEX0_BUG as _,
    Prim = _RDPQ_COMB2B_RGB_MUL_PRIM as _,
    Shade = _RDPQ_COMB2B_RGB_MUL_SHADE as _,
    Env = _RDPQ_COMB2B_RGB_MUL_ENV as _,
    KeyScale = _RDPQ_COMB2B_RGB_MUL_KEYSCALE as _,
    CombinedAlpha = _RDPQ_COMB2B_RGB_MUL_COMBINED_ALPHA as _,
    Tex1Alpha = _RDPQ_COMB2B_RGB_MUL_TEX1_ALPHA as _,
    Tex0Alpha = _RDPQ_COMB2B_RGB_MUL_TEX0_ALPHA as _,
    PrimAlpha = _RDPQ_COMB2B_RGB_MUL_PRIM_ALPHA as _,
    ShadeAlpha = _RDPQ_COMB2B_RGB_MUL_SHADE_ALPHA as _,
    EnvAlpha = _RDPQ_COMB2B_RGB_MUL_ENV_ALPHA as _,
    LodFrac = _RDPQ_COMB2B_RGB_MUL_LOD_FRAC as _,
    PrimLodFrac = _RDPQ_COMB2B_RGB_MUL_PRIM_LOD_FRAC as _,
    K5 = _RDPQ_COMB2B_RGB_MUL_K5 as _,
    Zero = _RDPQ_COMB2B_RGB_MUL_ZERO as _,
}
pub enum Comb1RGBAdd {
    Tex0 = _RDPQ_COMB1_RGB_ADD_TEX0 as _,
    Prim = _RDPQ_COMB1_RGB_ADD_PRIM as _,
    Shade = _RDPQ_COMB1_RGB_ADD_SHADE as _,
    Env = _RDPQ_COMB1_RGB_ADD_ENV as _,
    One = _RDPQ_COMB1_RGB_ADD_ONE as _,
    Zero = _RDPQ_COMB1_RGB_ADD_ZERO as _,
}
pub enum Comb2ARGBAdd {
    Tex0 = _RDPQ_COMB2A_RGB_ADD_TEX0 as _,
    Tex1 = _RDPQ_COMB2A_RGB_ADD_TEX1 as _,
    Prim = _RDPQ_COMB2A_RGB_ADD_PRIM as _,
    Shade = _RDPQ_COMB2A_RGB_ADD_SHADE as _,
    Env = _RDPQ_COMB2A_RGB_ADD_ENV as _,
    One = _RDPQ_COMB2A_RGB_ADD_ONE as _,
    Zero = _RDPQ_COMB2A_RGB_ADD_ZERO as _,
}
pub enum Comb2BRGBAdd {
    Combined = _RDPQ_COMB2B_RGB_ADD_COMBINED as _,
    Tex1 = _RDPQ_COMB2B_RGB_ADD_TEX1 as _,
    Tex0Bug = _RDPQ_COMB2B_RGB_ADD_TEX0_BUG as _,
    Prim = _RDPQ_COMB2B_RGB_ADD_PRIM as _,
    Shade = _RDPQ_COMB2B_RGB_ADD_SHADE as _,
    Env = _RDPQ_COMB2B_RGB_ADD_ENV as _,
    One = _RDPQ_COMB2B_RGB_ADD_ONE as _,
    Zero = _RDPQ_COMB2B_RGB_ADD_ZERO as _,
}
pub enum Comb1AlphaAddSub {
    Tex0 = _RDPQ_COMB1_ALPHA_ADDSUB_TEX0 as _,
    Prim = _RDPQ_COMB1_ALPHA_ADDSUB_PRIM as _,
    Shade = _RDPQ_COMB1_ALPHA_ADDSUB_SHADE as _,
    Env = _RDPQ_COMB1_ALPHA_ADDSUB_ENV as _,
    One = _RDPQ_COMB1_ALPHA_ADDSUB_ONE as _,
    Zero = _RDPQ_COMB1_ALPHA_ADDSUB_ZERO as _,
}
pub enum Comb2AAlphaAddSub {
    Tex0 = _RDPQ_COMB2A_ALPHA_ADDSUB_TEX0 as _,
    Tex1 = _RDPQ_COMB2A_ALPHA_ADDSUB_TEX1 as _,
    Prim = _RDPQ_COMB2A_ALPHA_ADDSUB_PRIM as _,
    Shade = _RDPQ_COMB2A_ALPHA_ADDSUB_SHADE as _,
    Env = _RDPQ_COMB2A_ALPHA_ADDSUB_ENV as _,
    One = _RDPQ_COMB2A_ALPHA_ADDSUB_ONE as _,
    Zero = _RDPQ_COMB2A_ALPHA_ADDSUB_ZERO as _,
}
pub enum Comb2BAlphaAddSub {
    Combined = _RDPQ_COMB2B_ALPHA_ADDSUB_COMBINED as _,
    Tex1 = _RDPQ_COMB2B_ALPHA_ADDSUB_TEX1 as _,
    Prim = _RDPQ_COMB2B_ALPHA_ADDSUB_PRIM as _,
    Shade = _RDPQ_COMB2B_ALPHA_ADDSUB_SHADE as _,
    Env = _RDPQ_COMB2B_ALPHA_ADDSUB_ENV as _,
    One = _RDPQ_COMB2B_ALPHA_ADDSUB_ONE as _,
    Zero = _RDPQ_COMB2B_ALPHA_ADDSUB_ZERO as _,
}
pub enum Comb1AlphaMul {
    LodFrac = _RDPQ_COMB1_ALPHA_MUL_LOD_FRAC as _,
    Tex0 = _RDPQ_COMB1_ALPHA_MUL_TEX0 as _,
    Prim = _RDPQ_COMB1_ALPHA_MUL_PRIM as _,
    Shade = _RDPQ_COMB1_ALPHA_MUL_SHADE as _,
    Env = _RDPQ_COMB1_ALPHA_MUL_ENV as _,
    PrimLodFrac = _RDPQ_COMB1_ALPHA_MUL_PRIM_LOD_FRAC as _,
    Zero = _RDPQ_COMB1_ALPHA_MUL_ZERO as _,
}
pub enum Comb2AAlphaMul {
    LodFrac = _RDPQ_COMB2A_ALPHA_MUL_LOD_FRAC as _,
    Tex0 = _RDPQ_COMB2A_ALPHA_MUL_TEX0 as _,
    Tex1 = _RDPQ_COMB2A_ALPHA_MUL_TEX1 as _,
    Prim = _RDPQ_COMB2A_ALPHA_MUL_PRIM as _,
    Shade = _RDPQ_COMB2A_ALPHA_MUL_SHADE as _,
    Env = _RDPQ_COMB2A_ALPHA_MUL_ENV as _,
    PrimLodFrac = _RDPQ_COMB2A_ALPHA_MUL_PRIM_LOD_FRAC as _,
    Zero = _RDPQ_COMB2A_ALPHA_MUL_ZERO as _,
}
pub enum Comb2BAlphaMul {
    LodFrac = _RDPQ_COMB2B_ALPHA_MUL_LOD_FRAC as _,
    Tex1 = _RDPQ_COMB2B_ALPHA_MUL_TEX1 as _,
    Prim = _RDPQ_COMB2B_ALPHA_MUL_PRIM as _,
    Shade = _RDPQ_COMB2B_ALPHA_MUL_SHADE as _,
    Env = _RDPQ_COMB2B_ALPHA_MUL_ENV as _,
    PrimLodFrac = _RDPQ_COMB2B_ALPHA_MUL_PRIM_LOD_FRAC as _,
    Zero = _RDPQ_COMB2B_ALPHA_MUL_ZERO as _,
}
}

#[macro_export]
macro_rules! rdpq_combiner {
    (($ra1:tt, $rb1:tt, $rc1:tt, $rd1:tt $(,)?) , ($aa1:tt, $ab1:tt, $ac1:tt, $ad1:tt $(,)?) $(,)?) => {
        const {
            let _ra1 = $crate::_rdpq_mode_input!(<$crate::rdpq::Comb1RGBSubA>::$ra1) as u64;
            let _rb1 = $crate::_rdpq_mode_input!(<$crate::rdpq::Comb1RGBSubB>::$rb1) as u64;
            let _rc1 = $crate::_rdpq_mode_input!(<$crate::rdpq::Comb1RGBMul>::$rc1) as u64;
            let _rd1 = $crate::_rdpq_mode_input!(<$crate::rdpq::Comb1RGBAdd>::$rd1) as u64;
            let _aa1 = $crate::_rdpq_mode_input!(<$crate::rdpq::Comb1AlphaAddSub>::$aa1) as u64;
            let _ab1 = $crate::_rdpq_mode_input!(<$crate::rdpq::Comb1AlphaAddSub>::$ab1) as u64;
            let _ac1 = $crate::_rdpq_mode_input!(<$crate::rdpq::Comb1AlphaMul>::$ac1) as u64;
            let _ad1 = $crate::_rdpq_mode_input!(<$crate::rdpq::Comb1AlphaAddSub>::$rd1) as u64;
            let _comb = (_ra1 << 52)
                | (_rb1 << 28)
                | (_rc1 << 47)
                | (_rd1 << 15)
                | (_ra1 << 37)
                | (_rb1 << 24)
                | (_rc1 << 32)
                | (_rd1 << 6)
                | (_aa1 << 44)
                | (_ab1 << 12)
                | (_ac1 << 41)
                | (_ad1 << 9)
                | (_aa1 << 21)
                | (_ab1 << 3)
                | (_ac1 << 18)
                | (_ad1 << 0);
            unsafe { $crate::rdpq::Combiner::new_unchecked(_comb) }
        }
    };
    (($ra1:tt, $rb1:tt, $rc1:tt, $rd1:tt $(,)?) , ($aa1:tt, $ab1:tt, $ac1:tt, $ad1:tt $(,)?) ,
     ($ca2:tt, $cb2:tt, $cc2:tt, $cd2:tt $(,)?) , ($aa2:tt, $ab2:tt, $ac2:tt, $ad2:tt $(,)?) $(,)?) => {
        const {
            let _ra1 = $crate::_rdpq_mode_input!(<$crate::rdpq::Comb2ARGBSubA>::$ra1) as u64;
            let _rb1 = $crate::_rdpq_mode_input!(<$crate::rdpq::Comb2ARGBSubB>::$rb1) as u64;
            let _rc1 = $crate::_rdpq_mode_input!(<$crate::rdpq::Comb2ARGBMul>::$rc1) as u64;
            let _rd1 = $crate::_rdpq_mode_input!(<$crate::rdpq::Comb2ARGBAdd>::$rd1) as u64;
            let _aa1 = $crate::_rdpq_mode_input!(<$crate::rdpq::Comb2AAlphaAddSub>::$aa1) as u64;
            let _ab1 = $crate::_rdpq_mode_input!(<$crate::rdpq::Comb2AAlphaAddSub>::$ab1) as u64;
            let _ac1 = $crate::_rdpq_mode_input!(<$crate::rdpq::Comb2AAlphaMul>::$ac1) as u64;
            let _ad1 = $crate::_rdpq_mode_input!(<$crate::rdpq::Comb2AAlphaAddSub>::$rd1) as u64;
            let _ra2 = $crate::_rdpq_mode_input!(<$crate::rdpq::Comb2BRGBSubA>::$ra2) as u64;
            let _rb2 = $crate::_rdpq_mode_input!(<$crate::rdpq::Comb2BRGBSubB>::$rb2) as u64;
            let _rc2 = $crate::_rdpq_mode_input!(<$crate::rdpq::Comb2BRGBMul>::$rc2) as u64;
            let _rd2 = $crate::_rdpq_mode_input!(<$crate::rdpq::Comb2BRGBAdd>::$rd2) as u64;
            let _aa2 = $crate::_rdpq_mode_input!(<$crate::rdpq::Comb2BAlphaAddSub>::$aa2) as u64;
            let _ab2 = $crate::_rdpq_mode_input!(<$crate::rdpq::Comb2BAlphaAddSub>::$ab2) as u64;
            let _ac2 = $crate::_rdpq_mode_input!(<$crate::rdpq::Comb2BAlphaMul>::$ac2) as u64;
            let _ad2 = $crate::_rdpq_mode_input!(<$crate::rdpq::Comb2BAlphaAddSub>::$rd2) as u64;
            let _comb = $crate::sys::rdpq_macros::RDPQ_COMBINER_2PASS as u64
                | (_ra1 << 52)
                | (_rb1 << 28)
                | (_rc1 << 47)
                | (_rd1 << 15)
                | (_ra2 << 37)
                | (_rb2 << 24)
                | (_rc2 << 32)
                | (_rd2 << 6)
                | (_aa1 << 44)
                | (_ab1 << 12)
                | (_ac1 << 41)
                | (_ad1 << 9)
                | (_aa2 << 21)
                | (_ab2 << 3)
                | (_ac2 << 18)
                | (_ad2 << 0);
            unsafe { $crate::rdpq::Combiner::new_unchecked(_comb) }
        }
    };
}

pub const COMBINER_FLAT: Combiner = rdpq_combiner!((0, 0, 0, Prim), (0, 0, 0, Prim));
pub const COMBINER_SHADE: Combiner = rdpq_combiner!((0, 0, 0, Shade), (0, 0, 0, Shade));
pub const COMBINER_TEX: Combiner = rdpq_combiner!((0, 0, 0, Tex0), (0, 0, 0, Tex0));
pub const COMBINER_TEX_FLAT: Combiner = rdpq_combiner!((Tex0, 0, Prim, 0), (Tex0, 0, Prim, 0));
pub const COMBINER_TEX_SHADE: Combiner = rdpq_combiner!((Tex0, 0, Shade, 0), (Tex0, 0, Shade, 0));

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(transparent)]
pub struct Blender(pub(crate) u32);

impl Blender {
    #[inline]
    pub const unsafe fn new_unchecked(blender: u32) -> Self {
        Self(blender)
    }
    #[inline]
    pub const fn into_inner(self) -> u32 {
        self.0
    }
}

impl From<Blender> for u32 {
    #[inline]
    fn from(value: Blender) -> Self {
        value.into_inner()
    }
}

impl_macro_enums! { #[repr(u32)]
pub enum SOMBlend1PM {
    InRGB = _RDPQ_SOM_BLEND1_A_IN_RGB,
    MemoryRGB = _RDPQ_SOM_BLEND1_A_MEMORY_RGB,
    BlendRGB = _RDPQ_SOM_BLEND1_A_BLEND_RGB,
    FogRGB = _RDPQ_SOM_BLEND1_A_FOG_RGB,
}
pub enum SOMBlend1A {
    InAlpha = _RDPQ_SOM_BLEND1_B1_IN_ALPHA,
    FogAlpha = _RDPQ_SOM_BLEND1_B1_FOG_ALPHA,
    ShadeAlpha = _RDPQ_SOM_BLEND1_B1_SHADE_ALPHA,
    Zero = _RDPQ_SOM_BLEND1_B1_ZERO,
}
pub enum SOMBlend1B {
    InvMuxAlpha = _RDPQ_SOM_BLEND1_B2_INV_MUX_ALPHA,
    MemoryCvg = _RDPQ_SOM_BLEND1_B2_MEMORY_CVG,
    One = _RDPQ_SOM_BLEND1_B2_ONE,
    Zero = _RDPQ_SOM_BLEND1_B2_ZERO,
}
pub enum SOMBlend2APM {
    InRGB = _RDPQ_SOM_BLEND2A_A_IN_RGB,
    BlendRGB = _RDPQ_SOM_BLEND2A_A_BLEND_RGB,
    FogRGB = _RDPQ_SOM_BLEND2A_A_FOG_RGB,
}
pub enum SOMBlend2AA {
    InAlpha = _RDPQ_SOM_BLEND2A_B1_IN_ALPHA,
    FogAlpha = _RDPQ_SOM_BLEND2A_B1_FOG_ALPHA,
    ShadeAlpha = _RDPQ_SOM_BLEND2A_B1_SHADE_ALPHA,
    Zero = _RDPQ_SOM_BLEND2A_B1_ZERO,
}
pub enum SOMBlend2AB {
    InvMuxAlpha = _RDPQ_SOM_BLEND2A_B2_INV_MUX_ALPHA,
}
pub enum SOMBlend2BPM {
    Cycle1RGB = _RDPQ_SOM_BLEND2B_A_CYCLE1_RGB,
    MemoryRGB = _RDPQ_SOM_BLEND2B_A_MEMORY_RGB,
    BlendRGB = _RDPQ_SOM_BLEND2B_A_BLEND_RGB,
    FogRGB = _RDPQ_SOM_BLEND2B_A_FOG_RGB,
}
pub enum SOMBlend2BA {
    InAlpha = _RDPQ_SOM_BLEND2B_B1_IN_ALPHA,
    FogAlpha = _RDPQ_SOM_BLEND2B_B1_FOG_ALPHA,
    ShadeAlpha = _RDPQ_SOM_BLEND2B_B1_SHADE_ALPHA,
    Zero = _RDPQ_SOM_BLEND2B_B1_ZERO,
}
pub enum SOMBlend2BB {
    InvMuxAlpha = _RDPQ_SOM_BLEND2B_B2_INV_MUX_ALPHA,
    MemoryCvg = _RDPQ_SOM_BLEND2B_B2_MEMORY_CVG,
    One = _RDPQ_SOM_BLEND2B_B2_ONE,
    Zero = _RDPQ_SOM_BLEND2B_B2_ZERO,
}
}

impl SOMBlend1PM {
    #[inline]
    pub const fn extra(self) -> u32 {
        match self {
            Self::MemoryRGB => SOM_READ_ENABLE,
            _ => 0,
        }
    }
}

impl SOMBlend2BPM {
    #[inline]
    pub const fn extra(self) -> u32 {
        match self {
            Self::MemoryRGB => SOM_READ_ENABLE,
            _ => 0,
        }
    }
}

impl SOMBlend1B {
    #[inline]
    pub const fn extra(self) -> u32 {
        match self {
            Self::MemoryCvg => SOM_READ_ENABLE,
            _ => 0,
        }
    }
}

impl SOMBlend2BB {
    #[inline]
    pub const fn extra(self) -> u32 {
        match self {
            Self::MemoryCvg => SOM_READ_ENABLE,
            _ => 0,
        }
    }
}

#[macro_export]
macro_rules! rdpq_blender {
    (($p1:tt, $a1:tt, $m1:tt, $b1:tt $(,)?) $(,)?) => {
        const {
            let _p1 = $crate::_rdpq_mode_input!(<$crate::rdpq::SOMBlend1PM>::$p1);
            let _a1 = $crate::_rdpq_mode_input!(<$crate::rdpq::SOMBlend1A>::$a1);
            let _m1 = $crate::_rdpq_mode_input!(<$crate::rdpq::SOMBlend1PM>::$m1);
            let _b1 = $crate::_rdpq_mode_input!(<$crate::rdpq::SOMBlend1B>::$b1);
            let _blend = ((_p1 as u32) << 30)
                | ((_a1 as u32) << 26)
                | ((_m1 as u32) << 22)
                | ((_b1 as u32) << 18);
            let _blend = _blend | (_blend >> 2) | _p1.extra() | _m1.extra() | _b1.extra();
            unsafe { $crate::rdpq::Blender::new_unchecked(_blend) }
        }
    };
    (($p1:tt, $a1:tt, $m1:tt, $b1:tt $(,)?) , ($p2:tt, $a2:tt, $m2:tt, $b2:tt $(,)?) $(,)?) => {
        const {
            let _p1 = $crate::_rdpq_mode_input!(<$crate::rdpq::SOMBlend1PM>::$p1);
            let _a1 = $crate::_rdpq_mode_input!(<$crate::rdpq::SOMBlend1A>::$a1);
            let _m1 = $crate::_rdpq_mode_input!(<$crate::rdpq::SOMBlend1PM>::$m1);
            let _b1 = $crate::_rdpq_mode_input!(<$crate::rdpq::SOMBlend1B>::$b1);
            let _p2 = $crate::_rdpq_mode_input!(<$crate::rdpq::SOMBlend2BPM>::$p2);
            let _a2 = $crate::_rdpq_mode_input!(<$crate::rdpq::SOMBlend2BA>::$a2);
            let _m2 = $crate::_rdpq_mode_input!(<$crate::rdpq::SOMBlend2BPM>::$m2);
            let _b2 = $crate::_rdpq_mode_input!(<$crate::rdpq::SOMBlend2BB>::$b2);
            let _blend = $crate::sys::rdpq_macros::SOMX_BLEND_2PASS
                | ((_p1 as u32) << 30)
                | ((_a1 as u32) << 26)
                | ((_m1 as u32) << 22)
                | ((_b1 as u32) << 18)
                | ((_p2 as u32) << 28)
                | ((_a2 as u32) << 24)
                | ((_m2 as u32) << 20)
                | ((_b2 as u32) << 16)
                | _p1.extra()
                | _m1.extra()
                | _b1.extra()
                | _p2.extra()
                | _m2.extra()
                | _b2.extra();
            unsafe { $crate::rdpq::Blender::new_unchecked(_blend) }
        }
    };
}

pub const FOG_STANDARD: Blender = rdpq_blender!((InRGB, ShadeAlpha, FogRGB, InvMuxAlpha));
pub const BLENDER_MULTIPLY: Blender = rdpq_blender!((InRGB, InAlpha, MemoryRGB, InvMuxAlpha));
pub const BLENDER_MULTIPLY_CONST: Blender =
    rdpq_blender!((InRGB, FogAlpha, MemoryRGB, InvMuxAlpha));
pub const BLENDER_ADDITIVE: Blender = rdpq_blender!((InRGB, InAlpha, MemoryRGB, One));
