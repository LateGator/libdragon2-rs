//! @brief Software routines for manipulating graphics in a display context.
//!
//! The graphics subsystem is responsible for software manipulation of a display
//! context as returned from the @ref display.  All of the functions use a pure
//! software drawing method and are thus much slower than hardware sprite support.
//! However, they are slightly more flexible and offer no hardware limitations
//! in terms of sprite size.
//!
//! Code wishing to draw to the screen should first acquire a display context
//! using `display_get`.  Once the display context is acquired, code may draw to
//! the context using any of the graphics functions present.  Wherever practical,
//! two versions of graphics functions are available: a transparent variety and
//! a non-transparent variety.  Code that wishes to display sprites without
//! transparency can get a slight performance boost by using the non-transparent
//! variety of calls since no software alpha blending needs to occur.  Once
//! code has finished drawing to the display context, it can be displayed to the
//! screen using `display_show`.
//!
//! The graphics subsystem makes use of the same contexts as the @ref rdp.  Thus,
//! with careful coding, both hardware and software routines can be used to draw
//! to the display context with no ill effects.  The colors returned by
//! `graphics_make_color` and `graphics_convert_color` are also compatible with both
//! hardware and software graphics routines.

use core::ffi::CStr;

#[allow(unused_imports)]
use crate::{display::Gamma, surface::TexFormat};
use crate::{sprite::Sprite, surface::Surface, sys::graphics::*};

#[doc = "Generic color structure"]
#[repr(C, align(4))]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Color {
    #[doc = "Red component"]
    pub r: u8,
    #[doc = "Green component"]
    pub g: u8,
    #[doc = "Blue component"]
    pub b: u8,
    #[doc = "Alpha component"]
    pub a: u8,
}

impl Color {
    pub const BLACK: Self = Self::rgba32(0, 0, 0, 255);
    pub const RED: Self = Self::rgba32(255, 0, 0, 255);
    pub const GREEN: Self = Self::rgba32(0, 255, 0, 255);
    pub const BLUE: Self = Self::rgba32(0, 0, 255, 255);
    pub const YELLOW: Self = Self::rgba32(255, 255, 0, 255);
    pub const MAGENTA: Self = Self::rgba32(255, 0, 255, 255);
    pub const CYAN: Self = Self::rgba32(0, 255, 255, 255);
    pub const WHITE: Self = Self::rgba32(255, 255, 255, 255);

    #[doc = "Create a `Color` from the R,G,B,A components in the RGBA16 range (that is: RGB in 0-31, A in 0-1)"]
    #[inline]
    pub const fn rgba16(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: (r << 3) | (r >> 3),
            g: (g << 3) | (g >> 3),
            b: (b << 3) | (b >> 3),
            a: if a != 0 { 0xFF } else { 0 },
        }
    }
    #[doc = "Create a `Color` from the R,G,B,A components in the RGBA32 range (0-255)."]
    #[inline]
    pub const fn rgba32(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    #[doc = "Create a `Color` from the R,G,B,A components in the linear 16bit color space (that is: RGB in 0-31 squared, A in 0-1), use with [`Gamma::Correct`] display."]
    #[inline]
    pub const fn linear16(r: u8, g: u8, b: u8, a: u8) -> Self {
        let r = r as u32;
        let g = g as u32;
        let b = b as u32;
        let r = (r << 3) | (r >> 3);
        let g = (g << 3) | (g >> 3);
        let b = (b << 3) | (b >> 3);
        Self {
            r: ((r * r) >> 8) as u8,
            g: ((g * g) >> 8) as u8,
            b: ((b * b) >> 8) as u8,
            a: if a != 0 { 0xFF } else { 0 },
        }
    }
    #[doc = "Create a `Color` from the R,G,B,A components in the linear color space (0-255 squared), use with [`Gamma::Correct`] display."]
    #[inline]
    pub const fn linear32(r: u8, g: u8, b: u8, a: u8) -> Self {
        let r = r as u32;
        let g = g as u32;
        let b = b as u32;
        Self {
            r: ((r * r) >> 8) as u8,
            g: ((g * g) >> 8) as u8,
            b: ((b * b) >> 8) as u8,
            a,
        }
    }
    #[doc = "Convert a `Color` to the 16-bit packed format used by a [`TexFormat::RGBA16`] surface (RGBA 5551)"]
    #[inline]
    pub const fn into_u16(self) -> u16 {
        (((self.r as u16) >> 3) << 11)
            | (((self.g as u16) >> 3) << 6)
            | (((self.b as u16) >> 3) << 1)
            | ((self.a as u16) >> 7)
    }
    #[doc = "Convert a `Color` to the 32-bit packed format used by a [`TexFormat::RGBA32`] surface (RGBA 8888)"]
    #[inline]
    pub const fn into_u32(self) -> u32 {
        unsafe { core::mem::transmute::<_, u32>(self).to_be() }
    }
    #[doc = "Create a `Color` from the 16-bit packed format used by a [`TexFormat::RGBA16`] surface (RGBA 5551)"]
    #[inline]
    pub const fn from_u16(c: u16) -> Self {
        let r = ((c >> 11) & 0x1F) as u8;
        let g = ((c >> 6) & 0x1F) as u8;
        let b = ((c >> 1) & 0x1F) as u8;
        let a = if (c & 0x1) != 0 { 0xFF } else { 0 };
        Self::rgba32(
            (r << 3) | (r >> 2),
            (g << 3) | (g >> 2),
            (b << 3) | (b >> 2),
            a,
        )
    }
    #[doc = "Create a `Color` from the 32-bit packed format used by a [`TexFormat::RGBA32`] surface (RGBA 8888)"]
    #[inline]
    pub const fn from_u32(c: u32) -> Self {
        Self::rgba32((c >> 24) as u8, (c >> 16) as u8, (c >> 8) as u8, c as u8)
    }
    #[doc = "Convert a color structure to a 32-bit representation of an RGBA color\n\n This function is similar to `color_to_packed16` and `color_to_packed32`, but\n automatically picks the version matching with the current display configuration.\n Notice that this might be wrong if you are drawing to an arbitrary surface rather\n than a framebuffer.\n\n @note In 16 bpp mode, this function will return a packed 16-bit color\n in BOTH the lower 16 bits and the upper 16 bits. In general, this is not necessary.\n However, for drawing with the old deprecated RDP API (in particular,\n rdp_set_primitive_color), this is still required.\n\n @deprecated By switching to the rdpq API, this function should not be required\n anymore. Please avoid using it in new code if possible.\n\n @param[in] color\n            A color structure representing an RGBA color\n\n @return a 32-bit representation of the color suitable for blitting in software or hardware"]
    #[inline]
    pub fn convert(self) -> u32 {
        unsafe { graphics_convert_color(self.into_raw()) }
    }
    #[doc = "Return a packed 32-bit representation of an RGBA color\n\n This is exactly the same as calling `graphics_convert_color(RGBA32(r,g,b,a))`.\n Refer to `graphics_convert_color` for more information.\n\n @deprecated By switching to the rdpq API, this function should not be required\n anymore. Use `RGBA32` or `RGBA16` instead. Please avoid using it in new code if possible.\n\n @param[in] r\n            8-bit red value\n @param[in] g\n            8-bit green value\n @param[in] b\n            8-bit blue value\n @param[in] a\n            8-bit alpha value.  Note that 255 is opaque and 0 is transparent\n\n @return a 32-bit representation of the color suitable for blitting in software or hardware\n\n @see `graphics_convert_color`\n"]
    #[inline]
    pub fn make(r: u8, g: u8, b: u8, a: u8) -> u32 {
        Self::rgba32(r, g, b, a).convert()
    }
    #[inline]
    pub const fn from_array(c: [u8; 4]) -> Self {
        Self::rgba32(c[0], c[1], c[2], c[3])
    }
    #[inline]
    pub const fn from_tuple(c: (u8, u8, u8, u8)) -> Self {
        Self::rgba32(c.0, c.1, c.2, c.3)
    }
    #[inline]
    pub const fn from_raw(color: color_t) -> Self {
        unsafe { core::mem::transmute(color) }
    }
    #[inline]
    pub const fn into_raw(self) -> color_t {
        unsafe { core::mem::transmute(self) }
    }
    #[inline]
    pub const fn into_array(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
    #[inline]
    pub const fn into_tuple(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }
}

impl From<Color> for u16 {
    #[inline]
    fn from(c: Color) -> Self {
        c.into_u16()
    }
}

impl From<Color> for u32 {
    #[inline]
    fn from(c: Color) -> Self {
        c.into_u32()
    }
}

impl From<[u8; 4]> for Color {
    #[inline]
    fn from(c: [u8; 4]) -> Self {
        Self::from_array(c)
    }
}

impl From<Color> for [u8; 4] {
    #[inline]
    fn from(c: Color) -> Self {
        c.into_array()
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    #[inline]
    fn from(c: (u8, u8, u8, u8)) -> Self {
        Self::from_tuple(c)
    }
}

impl From<Color> for (u8, u8, u8, u8) {
    #[inline]
    fn from(c: Color) -> Self {
        c.into_tuple()
    }
}

impl From<u16> for Color {
    #[doc = "Create a `Color` from the 16-bit packed format used by a [`TexFormat::RGBA16`] surface (RGBA 5551)"]
    #[inline]
    fn from(c: u16) -> Self {
        Self::from_u16(c)
    }
}

impl From<u32> for Color {
    #[doc = "Create a `Color` from the 32-bit packed format used by a [`TexFormat::RGBA32`] surface (RGBA 8888)"]
    #[inline]
    fn from(c: u32) -> Self {
        Self::from_u32(c)
    }
}

#[doc = "Set the current forecolor and backcolor for text operations\n\n @param[in] forecolor\n            32-bit RGBA color to use as the text color.  Use `graphics_convert_color`\n            or `graphics_make_color` to generate this value.\n @param[in] backcolor\n             32-bit RGBA color to use as the background color for text.  Use\n             `graphics_convert_color` or `graphics_make_color` to generate this value.\n             Note that if the color given is transparent, text can be written over\n             other graphics without background colors showing."]
#[inline]
pub fn set_color(forecolor: u32, backcolor: u32) {
    unsafe { graphics_set_color(forecolor, backcolor) }
}
#[doc = "Set the font to the default."]
#[inline]
pub fn set_default_font() {
    unsafe { graphics_set_default_font() }
}
#[doc = "Set the current font. Should be set before using any of the draw function.\n\n The sprite font should be imported using hslices/vslices according to the amount of characters it has.\n The amount of hslices vs vslices does not matter for this, but it should include the whole ASCII\n range that you will want to use, including characters from the 0 to 32 range. Normally the sprite should have\n 127 slices to cover the normal ASCII range.\n\n During rendering, the slice used will be the same number as the char (eg.: character 'A' will use slice 65).\n\n You can see an example of a sprite font (that has the default font double sized) under examples/customfont.\n\n @param[in] font\n        Sprite font to be used."]
#[inline]
pub fn set_font_sprite(sprite: &'static Sprite) {
    unsafe { graphics_set_font_sprite(sprite.as_raw()) }
}

pub trait GraphicsExt: crate::sealed::Sealed {
    #[doc = "Draw a pixel to a given display context\n\n @note This function does not support transparency for speed purposes.  To draw\n a transparent or translucent pixel, use `graphics_draw_pixel_trans`.\n\n @param[in] surf\n            The currently active display context.\n @param[in] x\n            The x coordinate of the pixel.\n @param[in] y\n            The y coordinate of the pixel.\n @param[in] color\n            The 32-bit RGBA color to draw to the screen.  Use `graphics_convert_color`\n            or `graphics_make_color` to generate this value."]
    fn draw_pixel(&mut self, x: i32, y: i32, color: u32);
    #[doc = "Draw a pixel to a given display context with alpha support\n\n @note This function is much slower than `graphics_draw_pixel` for 32-bit\n pixels due to the need to sample the current pixel to do software alpha-blending.\n\n @param[in] surf\n            The currently active display context.\n @param[in] x\n            The x coordinate of the pixel.\n @param[in] y\n            The y coordinate of the pixel.\n @param[in] color\n            The 32-bit RGBA color to draw to the screen.  Use `graphics_convert_color`\n            or `graphics_make_color` to generate this value."]
    fn draw_pixel_trans(&mut self, x: i32, y: i32, color: u32);
    #[doc = "Draw a line to a given display context\n\n @note This function does not support transparency for speed purposes.  To draw\n a transparent or translucent line, use `graphics_draw_line_trans`.\n\n @param[in] surf\n            The currently active display context.\n @param[in] x0\n            The x coordinate of the start of the line.\n @param[in] y0\n            The y coordinate of the start of the line.\n @param[in] x1\n            The x coordinate of the end of the line.\n @param[in] y1\n            The y coordinate of the end of the line.\n @param[in] color\n            The 32-bit RGBA color to draw to the screen.  Use `graphics_convert_color`\n            or `graphics_make_color` to generate this value."]
    fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32);
    #[doc = "Draw a line to a given display context with alpha support\n\n @note This function is much slower than `graphics_draw_line` for 32-bit\n buffers due to the need to sample the current pixel to do software alpha-blending.\n\n @param[in] surf\n            The currently active display context.\n @param[in] x0\n            The x coordinate of the start of the line.\n @param[in] y0\n            The y coordinate of the start of the line.\n @param[in] x1\n            The x coordinate of the end of the line.\n @param[in] y1\n            The y coordinate of the end of the line.\n @param[in] color\n            The 32-bit RGBA color to draw to the screen.  Use `graphics_convert_color`\n            or `graphics_make_color` to generate this value."]
    fn draw_line_trans(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32);
    #[doc = "Draw a filled rectangle to a display context\n\n @note This function does not support transparency for speed purposes.  To draw\n a transparent or translucent box, use `graphics_draw_box_trans`.\n\n @param[in] surf\n            The currently active display context.\n @param[in] x\n            The x coordinate of the top left of the box.\n @param[in] y\n            The y coordinate of the top left of the box.\n @param[in] width\n            The width of the box in pixels.\n @param[in] height\n            The height of the box in pixels.\n @param[in] color\n            The 32-bit RGBA color to draw to the screen.  Use `graphics_convert_color`\n            or `graphics_make_color` to generate this value."]
    fn draw_box(&mut self, x: i32, y: i32, width: i32, height: i32, color: u32);
    #[doc = "Draw a filled rectangle to a display context\n\n @note This function is much slower than `graphics_draw_box` for 32-bit\n buffers due to the need to sample the current pixel to do software alpha-blending.\n\n @param[in] surf\n            The currently active display context.\n @param[in] x\n            The x coordinate of the top left of the box.\n @param[in] y\n            The y coordinate of the top left of the box.\n @param[in] width\n            The width of the box in pixels.\n @param[in] height\n            The height of the box in pixels.\n @param[in] color\n            The 32-bit RGBA color to draw to the screen.  Use `graphics_convert_color`\n            or `graphics_make_color` to generate this value."]
    fn draw_box_trans(&mut self, x: i32, y: i32, width: i32, height: i32, color: u32);
    #[doc = "Fill the entire screen with a particular color\n\n @note Since this function is designed for blanking the screen, alpha values for\n colors are ignored.\n\n @param[in] surf\n            The currently active display context.\n @param[in] c\n            The 32-bit RGBA color to draw to the screen.  Use `graphics_convert_color`\n            or `graphics_make_color` to generate this value."]
    fn fill_screen(&mut self, c: u32);
    #[doc = "Draw a character to the screen using the built-in font\n\n Draw a character from the built-in font to the screen.  This function does not support alpha blending,\n only binary transparency.  If the background color is fully transparent, the font is drawn with no\n background.  Otherwise, the font is drawn on a fully colored background.  The foreground and background\n can be set using `graphics_set_color`.\n\n @param[in] surf\n            The currently active display context.\n @param[in] x\n            The X coordinate to place the top left pixel of the character drawn.\n @param[in] y\n            The Y coordinate to place the top left pixel of the character drawn.\n @param[in] ch\n            The ASCII character to draw to the screen."]
    fn draw_character(&mut self, x: i32, y: i32, ch: u8);
    #[doc = "Draw a null terminated string to a display context\n\n Draw a string to the screen, following a few simple rules.  Standard ASCII is supported, as well\n as \\\\r, \\\\n, space and tab.  \\\\r and \\\\n will both cause the next character to be rendered one line\n lower and at the x coordinate specified in the parameters.  The tab character inserts five spaces.\n\n This function does not support alpha blending, only binary transparency.  If the background color is\n fully transparent, the font is drawn with no background.  Otherwise, the font is drawn on a fully\n colored background.  The foreground and background can be set using `graphics_set_color`.\n\n @param[in] surf\n            The currently active display context.\n @param[in] x\n            The X coordinate to place the top left pixel of the character drawn.\n @param[in] y\n            The Y coordinate to place the top left pixel of the character drawn.\n @param[in] msg\n            The ASCII null terminated string to draw to the screen."]
    fn draw_text(&mut self, x: i32, y: i32, msg: &CStr);
    #[doc = "Draw a sprite to a display context\n\n Given a sprite structure, this function will draw a sprite to the display context\n with clipping support.\n\n @note This function does not support alpha blending for speed purposes.  For\n alpha blending support, please see `graphics_draw_sprite_trans`\n\n @param[in] surf\n            The currently active display context.\n @param[in] x\n            The X coordinate to place the top left pixel of the sprite.  This can\n            be negative if the sprite is clipped horizontally.\n @param[in] y\n            The Y coordinate to place the top left pixel of the sprite.  This can\n            be negative if the sprite is clipped vertically.\n @param[in] sprite\n            Pointer to a sprite structure to display to the screen."]
    fn draw_sprite(&mut self, x: i32, y: i32, sprite: &Sprite);
    #[doc = "Draw a sprite from a spritemap to a display context\n\n Given a sprite structure, this function will draw a sprite out of a larger spritemap\n to the display context with clipping support.  This function is useful for software\n tilemapping.  If a sprite was generated as a spritemap (it has more than one horizontal\n or vertical slice), this function can display a slice of the sprite as a standalone sprite.\n\n Given a sprite with 3 horizontal slices and 2 vertical slices, the offsets would be as follows:\n\n <pre>\n *---*---*---*\n | 0 | 1 | 2 |\n *---*---*---*\n | 3 | 4 | 5 |\n *---*---*---*\n </pre>\n\n @note This function does not support alpha blending for speed purposes.  For\n alpha blending support, please see `graphics_draw_sprite_trans_stride`\n\n @param[in] surf\n            The currently active display context.\n @param[in] x\n            The X coordinate to place the top left pixel of the sprite.  This can\n            be negative if the sprite is clipped horizontally.\n @param[in] y\n            The Y coordinate to place the top left pixel of the sprite.  This can\n            be negative if the sprite is clipped vertically.\n @param[in] sprite\n            Pointer to a sprite structure to display to the screen.\n @param[in] offset\n            Offset of the sprite to display out of the spritemap.  The offset is counted\n            starting from 0.  The top left sprite in the map is 0, the next one to the right\n            is 1, and so on."]
    fn draw_sprite_stride(&mut self, x: i32, y: i32, sprite: &Sprite, offset: u32);
    #[doc = "Draw a sprite to a display context with alpha transparency\n\n Given a sprite structure, this function will draw a sprite to the display context\n with clipping support.\n\n @note This function supports alpha blending and is much slower for 32-bit sprites.\n If you do not need alpha blending support, please see `graphics_draw_sprite`.\n\n @param[in] surf\n            The currently active display context.\n @param[in] x\n            The X coordinate to place the top left pixel of the sprite.  This can\n            be negative if the sprite is clipped horizontally.\n @param[in] y\n            The Y coordinate to place the top left pixel of the sprite.  This can\n            be negative if the sprite is clipped vertically.\n @param[in] sprite\n            Pointer to a sprite structure to display to the screen."]
    fn draw_sprite_trans(&mut self, x: i32, y: i32, sprite: &Sprite);
    #[doc = "Draw a sprite from a spritemap to a display context\n\n Given a sprite structure, this function will draw a sprite out of a larger spritemap\n to the display context with clipping support.  This function is useful for software\n tilemapping.  If a sprite was generated as a spritemap (it has more than one horizontal\n or vertical slice), this function can display a slice of the sprite as a standalone sprite.\n\n Given a sprite with 3 horizontal slices and 2 vertical slices, the offsets would be as follows:\n\n <pre>\n *---*---*---*\n | 0 | 1 | 2 |\n *---*---*---*\n | 3 | 4 | 5 |\n *---*---*---*\n </pre>\n\n @note This function supports alpha blending and is much slower for 32-bit sprites.\n If you do not need alpha blending support, please see `graphics_draw_sprite_stride`.\n\n @param[in] surf\n            The currently active display context.\n @param[in] x\n            The X coordinate to place the top left pixel of the sprite.  This can\n            be negative if the sprite is clipped horizontally.\n @param[in] y\n            The Y coordinate to place the top left pixel of the sprite.  This can\n            be negative if the sprite is clipped vertically.\n @param[in] sprite\n            Pointer to a sprite structure to display to the screen.\n @param[in] offset\n            Offset of the sprite to display out of the spritemap.  The offset is counted\n            starting from 0.  The top left sprite in the map is 0, the next one to the right\n            is 1, and so on."]
    fn draw_sprite_trans_stride(&mut self, x: i32, y: i32, sprite: &Sprite, offset: u32);
}

impl<'s> GraphicsExt for Surface<'s> {
    #[doc = "Draw a pixel to a given display context\n\n @note This function does not support transparency for speed purposes.  To draw\n a transparent or translucent pixel, use `graphics_draw_pixel_trans`.\n\n @param[in] surf\n            The currently active display context.\n @param[in] x\n            The x coordinate of the pixel.\n @param[in] y\n            The y coordinate of the pixel.\n @param[in] color\n            The 32-bit RGBA color to draw to the screen.  Use `graphics_convert_color`\n            or `graphics_make_color` to generate this value."]
    fn draw_pixel(&mut self, x: i32, y: i32, color: u32) {
        unsafe { graphics_draw_pixel(self.as_raw_mut(), x, y, color) }
    }
    #[doc = "Draw a pixel to a given display context with alpha support\n\n @note This function is much slower than `graphics_draw_pixel` for 32-bit\n pixels due to the need to sample the current pixel to do software alpha-blending.\n\n @param[in] surf\n            The currently active display context.\n @param[in] x\n            The x coordinate of the pixel.\n @param[in] y\n            The y coordinate of the pixel.\n @param[in] color\n            The 32-bit RGBA color to draw to the screen.  Use `graphics_convert_color`\n            or `graphics_make_color` to generate this value."]
    fn draw_pixel_trans(&mut self, x: i32, y: i32, color: u32) {
        unsafe { graphics_draw_pixel_trans(self.as_raw_mut(), x, y, color) }
    }
    #[doc = "Draw a line to a given display context\n\n @note This function does not support transparency for speed purposes.  To draw\n a transparent or translucent line, use `graphics_draw_line_trans`.\n\n @param[in] surf\n            The currently active display context.\n @param[in] x0\n            The x coordinate of the start of the line.\n @param[in] y0\n            The y coordinate of the start of the line.\n @param[in] x1\n            The x coordinate of the end of the line.\n @param[in] y1\n            The y coordinate of the end of the line.\n @param[in] color\n            The 32-bit RGBA color to draw to the screen.  Use `graphics_convert_color`\n            or `graphics_make_color` to generate this value."]
    fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        unsafe { graphics_draw_line(self.as_raw_mut(), x0, y0, x1, y1, color) }
    }
    #[doc = "Draw a line to a given display context with alpha support\n\n @note This function is much slower than `graphics_draw_line` for 32-bit\n buffers due to the need to sample the current pixel to do software alpha-blending.\n\n @param[in] surf\n            The currently active display context.\n @param[in] x0\n            The x coordinate of the start of the line.\n @param[in] y0\n            The y coordinate of the start of the line.\n @param[in] x1\n            The x coordinate of the end of the line.\n @param[in] y1\n            The y coordinate of the end of the line.\n @param[in] color\n            The 32-bit RGBA color to draw to the screen.  Use `graphics_convert_color`\n            or `graphics_make_color` to generate this value."]
    fn draw_line_trans(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        unsafe { graphics_draw_line_trans(self.as_raw_mut(), x0, y0, x1, y1, color) }
    }
    #[doc = "Draw a filled rectangle to a display context\n\n @note This function does not support transparency for speed purposes.  To draw\n a transparent or translucent box, use `graphics_draw_box_trans`.\n\n @param[in] surf\n            The currently active display context.\n @param[in] x\n            The x coordinate of the top left of the box.\n @param[in] y\n            The y coordinate of the top left of the box.\n @param[in] width\n            The width of the box in pixels.\n @param[in] height\n            The height of the box in pixels.\n @param[in] color\n            The 32-bit RGBA color to draw to the screen.  Use `graphics_convert_color`\n            or `graphics_make_color` to generate this value."]
    fn draw_box(&mut self, x: i32, y: i32, width: i32, height: i32, color: u32) {
        unsafe { graphics_draw_box(self.as_raw_mut(), x, y, width, height, color) }
    }
    #[doc = "Draw a filled rectangle to a display context\n\n @note This function is much slower than `graphics_draw_box` for 32-bit\n buffers due to the need to sample the current pixel to do software alpha-blending.\n\n @param[in] surf\n            The currently active display context.\n @param[in] x\n            The x coordinate of the top left of the box.\n @param[in] y\n            The y coordinate of the top left of the box.\n @param[in] width\n            The width of the box in pixels.\n @param[in] height\n            The height of the box in pixels.\n @param[in] color\n            The 32-bit RGBA color to draw to the screen.  Use `graphics_convert_color`\n            or `graphics_make_color` to generate this value."]
    fn draw_box_trans(&mut self, x: i32, y: i32, width: i32, height: i32, color: u32) {
        unsafe { graphics_draw_box_trans(self.as_raw_mut(), x, y, width, height, color) }
    }
    #[doc = "Fill the entire screen with a particular color\n\n @note Since this function is designed for blanking the screen, alpha values for\n colors are ignored.\n\n @param[in] surf\n            The currently active display context.\n @param[in] c\n            The 32-bit RGBA color to draw to the screen.  Use `graphics_convert_color`\n            or `graphics_make_color` to generate this value."]
    fn fill_screen(&mut self, c: u32) {
        unsafe { graphics_fill_screen(self.as_raw_mut(), c) }
    }
    #[doc = "Draw a character to the screen using the built-in font\n\n Draw a character from the built-in font to the screen.  This function does not support alpha blending,\n only binary transparency.  If the background color is fully transparent, the font is drawn with no\n background.  Otherwise, the font is drawn on a fully colored background.  The foreground and background\n can be set using `graphics_set_color`.\n\n @param[in] surf\n            The currently active display context.\n @param[in] x\n            The X coordinate to place the top left pixel of the character drawn.\n @param[in] y\n            The Y coordinate to place the top left pixel of the character drawn.\n @param[in] ch\n            The ASCII character to draw to the screen."]
    fn draw_character(&mut self, x: i32, y: i32, ch: u8) {
        unsafe { graphics_draw_character(self.as_raw_mut(), x, y, ch as _) }
    }
    #[doc = "Draw a null terminated string to a display context\n\n Draw a string to the screen, following a few simple rules.  Standard ASCII is supported, as well\n as \\\\r, \\\\n, space and tab.  \\\\r and \\\\n will both cause the next character to be rendered one line\n lower and at the x coordinate specified in the parameters.  The tab character inserts five spaces.\n\n This function does not support alpha blending, only binary transparency.  If the background color is\n fully transparent, the font is drawn with no background.  Otherwise, the font is drawn on a fully\n colored background.  The foreground and background can be set using `graphics_set_color`.\n\n @param[in] surf\n            The currently active display context.\n @param[in] x\n            The X coordinate to place the top left pixel of the character drawn.\n @param[in] y\n            The Y coordinate to place the top left pixel of the character drawn.\n @param[in] msg\n            The ASCII null terminated string to draw to the screen."]
    fn draw_text(&mut self, x: i32, y: i32, msg: &CStr) {
        unsafe { graphics_draw_text(self.as_raw_mut(), x, y, msg.as_ptr()) }
    }
    #[doc = "Draw a sprite to a display context\n\n Given a sprite structure, this function will draw a sprite to the display context\n with clipping support.\n\n @note This function does not support alpha blending for speed purposes.  For\n alpha blending support, please see `graphics_draw_sprite_trans`\n\n @param[in] surf\n            The currently active display context.\n @param[in] x\n            The X coordinate to place the top left pixel of the sprite.  This can\n            be negative if the sprite is clipped horizontally.\n @param[in] y\n            The Y coordinate to place the top left pixel of the sprite.  This can\n            be negative if the sprite is clipped vertically.\n @param[in] sprite\n            Pointer to a sprite structure to display to the screen."]
    fn draw_sprite(&mut self, x: i32, y: i32, sprite: &Sprite) {
        unsafe { graphics_draw_sprite(self.as_raw_mut(), x, y, sprite.as_raw()) }
    }
    #[doc = "Draw a sprite from a spritemap to a display context\n\n Given a sprite structure, this function will draw a sprite out of a larger spritemap\n to the display context with clipping support.  This function is useful for software\n tilemapping.  If a sprite was generated as a spritemap (it has more than one horizontal\n or vertical slice), this function can display a slice of the sprite as a standalone sprite.\n\n Given a sprite with 3 horizontal slices and 2 vertical slices, the offsets would be as follows:\n\n <pre>\n *---*---*---*\n | 0 | 1 | 2 |\n *---*---*---*\n | 3 | 4 | 5 |\n *---*---*---*\n </pre>\n\n @note This function does not support alpha blending for speed purposes.  For\n alpha blending support, please see `graphics_draw_sprite_trans_stride`\n\n @param[in] surf\n            The currently active display context.\n @param[in] x\n            The X coordinate to place the top left pixel of the sprite.  This can\n            be negative if the sprite is clipped horizontally.\n @param[in] y\n            The Y coordinate to place the top left pixel of the sprite.  This can\n            be negative if the sprite is clipped vertically.\n @param[in] sprite\n            Pointer to a sprite structure to display to the screen.\n @param[in] offset\n            Offset of the sprite to display out of the spritemap.  The offset is counted\n            starting from 0.  The top left sprite in the map is 0, the next one to the right\n            is 1, and so on."]
    fn draw_sprite_stride(&mut self, x: i32, y: i32, sprite: &Sprite, offset: u32) {
        unsafe {
            graphics_draw_sprite_stride(self.as_raw_mut(), x, y, sprite.as_raw(), offset as _)
        }
    }
    #[doc = "Draw a sprite to a display context with alpha transparency\n\n Given a sprite structure, this function will draw a sprite to the display context\n with clipping support.\n\n @note This function supports alpha blending and is much slower for 32-bit sprites.\n If you do not need alpha blending support, please see `graphics_draw_sprite`.\n\n @param[in] surf\n            The currently active display context.\n @param[in] x\n            The X coordinate to place the top left pixel of the sprite.  This can\n            be negative if the sprite is clipped horizontally.\n @param[in] y\n            The Y coordinate to place the top left pixel of the sprite.  This can\n            be negative if the sprite is clipped vertically.\n @param[in] sprite\n            Pointer to a sprite structure to display to the screen."]
    fn draw_sprite_trans(&mut self, x: i32, y: i32, sprite: &Sprite) {
        unsafe { graphics_draw_sprite_trans(self.as_raw_mut(), x, y, sprite.as_raw()) }
    }
    #[doc = "Draw a sprite from a spritemap to a display context\n\n Given a sprite structure, this function will draw a sprite out of a larger spritemap\n to the display context with clipping support.  This function is useful for software\n tilemapping.  If a sprite was generated as a spritemap (it has more than one horizontal\n or vertical slice), this function can display a slice of the sprite as a standalone sprite.\n\n Given a sprite with 3 horizontal slices and 2 vertical slices, the offsets would be as follows:\n\n <pre>\n *---*---*---*\n | 0 | 1 | 2 |\n *---*---*---*\n | 3 | 4 | 5 |\n *---*---*---*\n </pre>\n\n @note This function supports alpha blending and is much slower for 32-bit sprites.\n If you do not need alpha blending support, please see `graphics_draw_sprite_stride`.\n\n @param[in] surf\n            The currently active display context.\n @param[in] x\n            The X coordinate to place the top left pixel of the sprite.  This can\n            be negative if the sprite is clipped horizontally.\n @param[in] y\n            The Y coordinate to place the top left pixel of the sprite.  This can\n            be negative if the sprite is clipped vertically.\n @param[in] sprite\n            Pointer to a sprite structure to display to the screen.\n @param[in] offset\n            Offset of the sprite to display out of the spritemap.  The offset is counted\n            starting from 0.  The top left sprite in the map is 0, the next one to the right\n            is 1, and so on."]
    fn draw_sprite_trans_stride(&mut self, x: i32, y: i32, sprite: &Sprite, offset: u32) {
        unsafe {
            graphics_draw_sprite_trans_stride(self.as_raw_mut(), x, y, sprite.as_raw(), offset as _)
        }
    }
}
