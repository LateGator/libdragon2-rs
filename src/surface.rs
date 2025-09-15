use core::{marker::PhantomData, num::NonZeroU8};

use alloc_::boxed::Box;

use crate::{sys::surface::*, ucstr::UCStr};

#[doc = "A surface buffer for graphics\n\n This structure holds the basic information about a buffer used to hold graphics.\n It is commonly used by graphics routines in libdragon as either a source (eg: texture)\n or a target (eg: framebuffer). It can be used for both CPU-based drawing\n (such as graphics.h) or RDP-basic drawing (such as rdp.h and rdpq.h).\n\n Use #surface_alloc / #surface_free to allocate / free a surface. If you already have\n a memory pointer to a graphics buffer and you just need to wrap it in a #surface_t,\n use #surface_make."]
#[repr(transparent)]
#[derive(Debug)]
pub struct Surface<'s>(pub(crate) surface_t, pub(crate) PhantomData<&'s ()>);

#[doc = "Pixel format enum\n\n This enum defines the pixel formats that can be used for #surface_t buffers.\n The list corresponds to the pixel formats that the RDP can use as textures.\n\n @note Some of these formats can be used by RDP as framebuffer (specifically,\n #FMT_RGBA16, #FMT_RGBA32 and #FMT_CI8).\n @warning the CPU-based graphics library\n graphics.h only accepts surfaces in either #FMT_RGBA16 or #FMT_RGBA32 as\n target buffers, and does not assert."]
#[repr(i8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum TexFormat {
    #[doc = "Placeholder for no format defined"]
    None = tex_format_t_FMT_NONE as i8,
    #[doc = "Format RGBA 5551 (16-bit)"]
    RGBA16 = tex_format_t_FMT_RGBA16 as i8,
    #[doc = "Format RGBA 8888 (32-bit)"]
    RGBA32 = tex_format_t_FMT_RGBA32 as i8,
    #[doc = "Format YUV2 4:2:2 (data interleaved as YUYV)"]
    YUV16 = tex_format_t_FMT_YUV16 as i8,
    #[doc = "Format CI4: color index 4-bit (paletted, 2 indices per byte)"]
    CI4 = tex_format_t_FMT_CI4 as i8,
    #[doc = "Format CI8: color index 8-bit (paletted, 1 index per byte)"]
    CI8 = tex_format_t_FMT_CI8 as i8,
    #[doc = "Format IA4: 3-bit intensity + 1-bit alpha (4-bit per pixel)"]
    IA4 = tex_format_t_FMT_IA4 as i8,
    #[doc = "Format IA8: 4-bit intensity + 4-bit alpha (8-bit per pixel)"]
    IA8 = tex_format_t_FMT_IA8 as i8,
    #[doc = "Format IA16: 8-bit intensity + 8-bit alpha (16-bit per pixel)"]
    IA16 = tex_format_t_FMT_IA16 as i8,
    #[doc = "Format I4: 4-bit intensity (4-bit per pixel)"]
    I4 = tex_format_t_FMT_I4 as i8,
    #[doc = "Format I8: 8-bit intensity (8-bit per pixel)"]
    I8 = tex_format_t_FMT_I8 as i8,
}

impl core::fmt::Display for TexFormat {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.name().fmt(f)
    }
}

impl From<TexFormat> for u32 {
    #[inline]
    fn from(value: TexFormat) -> Self {
        value as _
    }
}

impl TexFormat {
    #[doc = "Return the name of the texture format as a string (for debugging purposes)"]
    #[inline]
    pub fn name(self) -> &'static UCStr {
        unsafe { UCStr::from_ptr(tex_format_name(self as _)) }
    }
    #[inline]
    pub const fn bitdepth(self) -> u32 {
        4 << (self as u32 & 3)
    }
    #[inline]
    pub const fn pixels_to_bytes(self, pixels: u32) -> u32 {
        (((pixels) << (((self as u32) & 3) + 2)) + 7) >> 3
    }
    #[inline]
    pub const fn bytes_to_pixels(self, bytes: u32) -> u32 {
        ((bytes) << 1) >> ((self as u32) & 3)
    }
}

impl Surface<'static> {
    #[doc = "Allocate a new surface in memory\n\n This function allocates a new surface with the specified pixel format,\n width and height. The surface must be freed via #surface_free when it is\n not needed anymore.\n\n A surface allocated via #surface_alloc can be used as a RDP frame buffer\n (passed to #rdpq_attach) because it is guaranteed to have the required\n alignment of 64 bytes, provided it is using one of the formats supported by\n RDP as a framebuffer target (`FMT_RGBA32`, `FMT_RGBA16` or `FMT_I8`).\n\n @param[in]  format   Pixel format of the surface\n @param[in]  width    Width in pixels\n @param[in]  height   Height in pixels\n @return              The initialized surface"]
    #[inline]
    pub fn new(format: TexFormat, width: u16, height: u16) -> Self {
        unsafe { Self(surface_alloc(format as _, width, height), PhantomData) }
    }
    #[doc = "Initialize a surface_t structure with the provided linear buffer.\n\n This function is similar to #surface_make, but it works for images that\n are linearly mapped with no per-line padding or extraneous data.\n\n Compared to #surface_make, it does not accept a stride parameter, and\n calculate the stride from the width and the pixel format.\n\n @param[in] buffer    Pointer to the memory buffer\n @param[in] format    Pixel format\n @param[in] width     Width in pixels\n @param[in] height    Height in pixels\n @return              The initialized surface\n\n @see #surface_make"]
    #[inline]
    pub fn from_buffer(
        buffer: Box<[u8]>,
        format: TexFormat,
        width: u16,
        height: u16,
        stride: u16,
    ) -> Self {
        assert_eq!(
            buffer.len(),
            stride as usize * width as usize * format.bitdepth() as usize
        );
        let buffer = Box::into_raw(buffer) as *mut _;
        Self(
            surface_t {
                flags: format as _,
                width,
                height,
                stride,
                buffer,
            },
            PhantomData,
        )
    }
    #[doc = "Initialize a surface_t structure with the provided linear buffer.\n\n This function is similar to #surface_make, but it works for images that\n are linearly mapped with no per-line padding or extraneous data.\n\n Compared to #surface_make, it does not accept a stride parameter, and\n calculate the stride from the width and the pixel format.\n\n @param[in] buffer    Pointer to the memory buffer\n @param[in] format    Pixel format\n @param[in] width     Width in pixels\n @param[in] height    Height in pixels\n @return              The initialized surface\n\n @see #surface_make"]
    #[inline]
    pub fn from_buffer_linear(
        buffer: Box<[u8]>,
        format: TexFormat,
        width: u16,
        height: u16,
    ) -> Self {
        Self::from_buffer(
            buffer,
            format,
            width,
            height,
            format.pixels_to_bytes(width as _) as _,
        )
    }

    #[doc = "Create a placeholder surface, that can be used during rdpq block recording.\n\n When recording a rspq block (via #rspq_block_begin / #rspq_block_end) it might\n be useful sometimes to issue draw commands that refer to a surface, but\n allowing the actual surface to change later at any time.\n\n See #rdpq_set_lookup_address for more information.\n\n @note A placeholder surface holds a NULL pointer to the actual bytes. Make sure\n       not to use it anywhere else but with rdpq.\n\n @param index     Index that will be used to lookup the surface at playback time\n @param format    Pixel format\n @param width     Width of the surface in pixels\n @param height    Height of the surface in pixels\n @param stride    Stride of the surface in bytes\n @return surface_t    The initialized placeholder surface\n\n @see #surface_make_placeholder_linear\n @see #rdpq_set_lookup_address"]
    #[inline]
    pub fn new_placeholder(
        index: NonZeroU8,
        format: TexFormat,
        width: u16,
        height: u16,
        stride: u16,
    ) -> Self {
        let buffer = core::ptr::null_mut();
        let flags = format as u16 | (((index.get() as u16) << 8) & SURFACE_FLAGS_TEXINDEX as u16);
        Self(
            surface_t {
                flags,
                width,
                height,
                stride,
                buffer,
            },
            PhantomData,
        )
    }
    #[doc = "Create a linear placeholder surface, that can be used during rdpq block recording.\n\n This function is similar to #surface_make_placeholder, but it creates\n a surface that is linearly mapped with no per-line padding or extraneous data.\n (so the stride is automatically deduced from the width).\n\n @param index     Index that will be used to lookup the surface at playback time\n @param format    Pixel format\n @param width     Width of the surface in pixels\n @param height    Height of the surface in pixels\n @return surface_t    The initialized placeholder surface\n\n @see #surface_make_placeholder"]
    #[inline]
    pub fn new_placeholder_linear(
        index: NonZeroU8,
        format: TexFormat,
        width: u16,
        height: u16,
    ) -> Self {
        Self::new_placeholder(
            index,
            format,
            width,
            height,
            format.pixels_to_bytes(width as _) as _,
        )
    }
}

impl<'s> Surface<'s> {
    #[doc = "Initialize a surface_t structure, pointing to a rectangular portion of another\n        surface.\n\n The surface returned by this function will point to a portion of the buffer of\n the parent surface, and will have of course the same pixel format.\n\n @param[in]  parent   Parent surface that will be pointed to\n @param[in]  x0       X coordinate of the top-left corner within the parent surface\n @param[in]  y0       Y coordinate of the top-left corner within the parent surface\n @param[in]  width    Width of the surface that will be returned\n @param[in]  height   Height of the surface that will be returned\n @return              The initialized surface"]
    #[inline]
    pub fn make_sub(&self, x0: u16, y0: u16, width: u16, height: u16) -> Self {
        assert!(x0 as u32 + width as u32 <= self.width() as u32);
        assert!(y0 as u32 + height as u32 <= self.height() as u32);
        unsafe {
            Self(
                surface_make_sub(&self.0 as *const _ as *mut _, x0, y0, width, height),
                PhantomData,
            )
        }
    }
    #[doc = "Initialize a surface_t structure, pointing to a rectangular portion of another\n        surface.\n\n The surface returned by this function will point to a portion of the buffer of\n the parent surface, and will have of course the same pixel format.\n\n @param[in]  parent   Parent surface that will be pointed to\n @param[in]  x0       X coordinate of the top-left corner within the parent surface\n @param[in]  y0       Y coordinate of the top-left corner within the parent surface\n @param[in]  width    Width of the surface that will be returned\n @param[in]  height   Height of the surface that will be returned\n @return              The initialized surface"]
    #[inline]
    pub fn make_sub_mut(&mut self, x0: u16, y0: u16, width: u16, height: u16) -> Surface<'_> {
        assert!(x0 as u32 + width as u32 <= self.width() as u32);
        assert!(y0 as u32 + height as u32 <= self.height() as u32);
        unsafe {
            Self(
                surface_make_sub(&mut self.0, x0, y0, width, height),
                PhantomData,
            )
        }
    }

    #[doc = "Returns the pixel format of a surface"]
    #[inline]
    pub fn format(&self) -> TexFormat {
        unsafe { core::mem::transmute((self.0.flags & SURFACE_FLAGS_TEXFORMAT as u16) as i8) }
    }
    #[doc = "Checks whether this surface owns the buffer that it contains."]
    #[inline]
    pub fn has_owned_buffer(&self) -> bool {
        (self.0.flags & SURFACE_FLAGS_OWNEDBUFFER as u16) != 0
    }
    #[doc = "Returns the lookup index of a placeholder surface\n\n If the surface is a placeholder, this function returns the associated lookup\n index that will be used to retrieve the actual surface at playback time.\n Otherwise, if it is a normal surface, this function will return 0.\n\n @param surface   Placeholder surface\n @return int      The lookup index of the placeholder surface, or 0 if it is a normal surface"]
    #[inline]
    pub fn placeholder_index(&self) -> Option<NonZeroU8> {
        NonZeroU8::new(((self.0.flags & SURFACE_FLAGS_TEXINDEX as u16) >> 8) as u8)
    }
    #[doc = "Gets the width in pixels"]
    #[inline]
    pub fn width(&self) -> u16 {
        self.0.width
    }
    #[doc = "Gets the height in pixels"]
    #[inline]
    pub fn height(&self) -> u16 {
        self.0.height
    }
    #[doc = "Gets the stride in bytes (length of a row)"]
    #[inline]
    pub fn stride(&self) -> u16 {
        self.0.stride
    }
    #[doc = "Gets the pixel buffer"]
    #[inline]
    pub fn buffer(&self) -> Option<&[u8]> {
        if self.0.buffer.is_null() {
            return None;
        }
        unsafe {
            Some(core::slice::from_raw_parts(
                self.0.buffer as *const _,
                self.0.width as usize * self.0.stride as usize,
            ))
        }
    }
    #[doc = "Gets the mutable pixel buffer"]
    #[inline]
    pub fn mut_buffer(&mut self) -> Option<&mut [u8]> {
        if self.0.buffer.is_null() {
            return None;
        }
        unsafe {
            Some(core::slice::from_raw_parts_mut(
                self.0.buffer as *mut _,
                self.0.width as usize * self.0.stride as usize,
            ))
        }
    }
    #[inline]
    pub const fn as_raw(&self) -> &surface_t {
        &self.0
    }
    #[inline]
    pub const fn as_raw_mut(&mut self) -> &mut surface_t {
        &mut self.0
    }
}

impl<'s> Drop for Surface<'s> {
    #[doc = "Free the buffer allocated in a surface.\n\n This function should be called after a surface allocated via #surface_alloc is not\n needed anymore.\n\n Calling this function on surfaces allocated via #surface_make or #surface_make_sub\n (that is, surfaces initialized with an existing buffer pointer) has no effect but\n clearing the contents of the surface structure.\n\n @param[in]  surface   The surface to free"]
    #[inline]
    fn drop(&mut self) {
        unsafe {
            surface_free(&mut self.0);
        }
    }
}
