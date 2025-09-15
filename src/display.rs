pub use crate::sys::display::VI_CRT_MARGIN;
use crate::{surface::Surface, sys::display::*};

#[doc = "Video resolution structure\n\n This structure allows to configure the video resolution, which includes both\n the framebuffer size and some parameters of how the framebuffer is displayed\n on the screen (aspect ratio, TV overscan margins, etc.).\n\n Most users should just use one of the pre-defined constants (such as\n #RESOLUTION_320x240), but it is possible to configure custom resolutions\n by manually filling fields in this structure."]
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Resolution(pub(crate) resolution_t);

#[doc = "Valid interlace modes"]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Interlace {
    #[doc = "Video output is not interlaced"]
    Off = interlace_mode_t_INTERLACE_OFF,
    #[doc = "Video output is interlaced and buffer is swapped on odd and even fields"]
    Half = interlace_mode_t_INTERLACE_HALF,
    #[doc = "Video output is interlaced and buffer is swapped only on even fields"]
    Full = interlace_mode_t_INTERLACE_FULL,
}

#[doc = "Valid bit depths"]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum BitDepth {
    #[doc = "16 bits per pixel (5-5-5-1)"]
    _16Bpp = bitdepth_t_DEPTH_16_BPP,
    #[doc = "32 bits per pixel (8-8-8-8)"]
    _32Bpp = bitdepth_t_DEPTH_32_BPP,
}

#[doc = "Valid gamma correction settings"]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Gamma {
    #[doc = "Uncorrected gamma.\n\n This is the default settings, and should be used with assets\n built by libdragon tools"]
    None = gamma_t_GAMMA_NONE,
    #[doc = "Corrected gamma.\n\n It should be used on a 32-bit framebuffer, only when assets have been\n produced in linear color space and accurate blending is important"]
    Correct = gamma_t_GAMMA_CORRECT,
    #[doc = "Corrected gamma with hardware dithered output"]
    Dither = gamma_t_GAMMA_CORRECT_DITHER,
}
#[doc = "Valid display filter options.\n\n Libdragon uses preconfigured options for enabling certain\n combinations of Video Interface filters due to a large number of wrong/invalid configurations\n with very strict conditions, and to simplify the options for the user.\n\n Like for example antialiasing requiring resampling; dedithering not working with\n resampling, unless always fetching; always enabling divot filter under AA etc.\n\n The options below provide all possible configurations that are deemed useful in development."]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Filters {
    #[doc = "All display filters are disabled"]
    Disabled = filter_options_t_FILTERS_DISABLED,
    #[doc = "Resize the output image with a bilinear filter.\n In general, VI is in charge of resizing the framebuffer to fit the virtual\n TV resolution (which is always 640x480 on NTSC/MPAL, 640x576 on PAL).\n This option enables a bilinear interpolation that can be used during this resize."]
    Resample = filter_options_t_FILTERS_RESAMPLE,
    #[doc = "Reconstruct a 32-bit output from dithered 16-bit framebuffer."]
    Dedither = filter_options_t_FILTERS_DEDITHER,
    #[doc = "Resize the output image with a bilinear filter (see #FILTERS_RESAMPLE).\n Add a video interface anti-aliasing pass with a divot filter.\n To be able to see correct anti-aliased output, this display filter must be enabled,\n along with anti-aliased rendering of surfaces."]
    ResampleAntialias = filter_options_t_FILTERS_RESAMPLE_ANTIALIAS,
    #[doc = "Resize the output image with a bilinear filter (see #FILTERS_RESAMPLE).\n Add a video interface anti-aliasing pass with a divot filter (see #FILTERS_RESAMPLE_ANTIALIAS).\n Reconstruct a 32-bit output from dithered 16-bit framebuffer."]
    ResampleAntialiasDedither = filter_options_t_FILTERS_RESAMPLE_ANTIALIAS_DEDITHER,
}

#[allow(non_upper_case_globals)]
impl Resolution {
    #[doc = "256x240 mode, stretched to 4:3, no borders"]
    pub const _256x240: Resolution = Self::new(256, 240, Interlace::Off, 0.0, 0.0, false);
    #[doc = "320x240 mode, no borders"]
    pub const _320x240: Resolution = Self::new(320, 240, Interlace::Off, 0.0, 0.0, false);
    #[doc = "512x240 mode, stretched to 4:3, no borders"]
    pub const _512x240: Resolution = Self::new(512, 240, Interlace::Off, 0.0, 0.0, false);
    #[doc = "640x240 mode, stretched to 4:3, no borders"]
    pub const _640x240: Resolution = Self::new(640, 240, Interlace::Off, 0.0, 0.0, false);
    #[doc = "512x480 mode, interlaced, stretched to 4:3, no borders"]
    pub const _512x480: Resolution = Self::new(512, 480, Interlace::Half, 0.0, 0.0, false);
    #[doc = "640x480 mode, interlaced, no borders"]
    pub const _640x480: Resolution = Self::new(640, 480, Interlace::Half, 0.0, 0.0, false);
    #[inline]
    pub const fn new(
        width: i32,
        height: i32,
        interlaced: Interlace,
        aspect_ratio: f32,
        overscan_margin: f32,
        pal60: bool,
    ) -> Self {
        let interlaced = interlaced as _;
        Self(resolution_t {
            width,
            height,
            interlaced,
            aspect_ratio,
            overscan_margin,
            pal60,
        })
    }
    #[inline]
    pub fn width(&self) -> i32 {
        self.0.width
    }
    #[inline]
    pub fn height(&self) -> i32 {
        self.0.height
    }
    #[inline]
    pub fn interlaced(&self) -> Interlace {
        unsafe { core::mem::transmute(self.0.interlaced) }
    }
    #[inline]
    pub fn aspect_ratio(&self) -> f32 {
        self.0.aspect_ratio
    }
    #[inline]
    pub fn overscan_margin(&self) -> f32 {
        self.0.overscan_margin
    }
    #[inline]
    pub fn pal60(&self) -> bool {
        self.0.pal60
    }
    #[inline]
    pub fn set_width(mut self, width: i32) {
        self.0.width = width;
    }
    #[inline]
    pub fn set_height(mut self, height: i32) {
        self.0.height = height;
    }
    #[inline]
    pub fn set_interlaced(mut self, interlaced: Interlace) {
        self.0.interlaced = interlaced as _;
    }
    #[inline]
    pub fn set_aspect_ratio(mut self, aspect_ratio: f32) {
        self.0.aspect_ratio = aspect_ratio;
    }
    #[inline]
    pub fn set_overscan_margin(mut self, overscan_margin: f32) {
        self.0.overscan_margin = overscan_margin;
    }
    #[inline]
    pub fn set_pal60(mut self, pal60: bool) {
        self.0.pal60 = pal60;
    }
}

impl From<Resolution> for resolution_t {
    #[inline]
    fn from(value: Resolution) -> Self {
        let Self {
            width,
            height,
            interlaced,
            aspect_ratio,
            overscan_margin,
            pal60,
        } = value.0;
        Self {
            width,
            height,
            interlaced,
            aspect_ratio,
            overscan_margin,
            pal60,
        }
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Display(());

static mut DISPLAY_INIT: bool = false;

#[doc = "Initialize the display to a particular resolution and bit depth\n\n Initialize video system.  This sets up a double, triple, or multiple\n buffered drawing surface which can be blitted or rendered to using\n software or hardware.\n\n @param[in] res\n            The requested resolution. Use either one of the pre-defined\n            resolution (such as #RESOLUTION_320x240) or define a custom one.\n @param[in] bit\n            The requested bit depth (#DEPTH_16_BPP or #DEPTH_32_BPP)\n @param[in] num_buffers\n            Number of buffers, usually 2 or 3, but can be more. Triple buffering\n            is recommended in case the application cannot hold a steady full framerate,\n            so that slowdowns don't impact too much.\n @param[in] gamma\n            The requested gamma setting\n @param[in] filters\n            The requested display filtering options, see #filter_options_t"]
#[inline]
pub fn init(
    res: Resolution,
    bit: BitDepth,
    num_buffers: u32,
    gamma: Gamma,
    filters: Filters,
) -> Display {
    unsafe {
        assert_eq!((&raw mut DISPLAY_INIT).read_volatile(), false);
        (&raw mut DISPLAY_INIT).write_volatile(true);
        display_init(res.into(), bit as _, num_buffers, gamma as _, filters as _);
    }
    Display(())
}

impl Drop for Display {
    #[doc = "Close the display\n\n Close a display and free buffer memory associated with it."]
    #[inline]
    fn drop(&mut self) {
        unsafe {
            display_close();
            (&raw mut DISPLAY_INIT).write_volatile(false);
        }
    }
}

impl Display {
    #[doc = "Get a display buffer for rendering\n\n Grab a surface that is safe for drawing, spin-waiting until one is\n available.\n\n When you are done drawing on the buffer, use #display_show to schedule\n the buffer to be displayed on the screen during next vblank.\n\n It is possible to get more than a display buffer at the same time, for\n instance to begin working on a new frame while the previous one is still\n being rendered in parallel through RDP. It is important to notice that\n surfaces will always be shown on the screen in the order they were gotten,\n irrespective of the order #display_show is called.\n\n @return A valid surface to render to."]
    #[must_use]
    #[inline]
    pub fn color_buffer(&mut self) -> DisplayBuffer<'_> {
        unsafe { DisplayBuffer(&mut *(display_get() as *mut Surface)) }
    }

    #[doc = "Try getting a display surface\n\n This is similar to #display_get, but it does not block if no\n display is available and return NULL instead.\n\n @return A valid surface to render to or NULL if none is available."]
    #[must_use]
    #[inline]
    pub fn try_color_buffer(&mut self) -> Option<DisplayBuffer<'_>> {
        unsafe {
            core::ptr::NonNull::new(display_try_get() as *mut Surface)
                .map(|mut s| DisplayBuffer(s.as_mut()))
        }
    }

    #[doc = "Return a memory surface that can be used as Z-buffer for the current\n        resolution\n\n This function lazily allocates and returns a surface that can be used\n as Z-buffer for the current resolution. The surface is automatically freed\n when the display is closed.\n\n @return surface_t    The Z-buffer surface"]
    #[must_use]
    #[inline]
    pub fn z_buffer(&mut self) -> DisplayBuffer<'_> {
        unsafe { DisplayBuffer(&mut *(display_get_zbuf() as *mut Surface)) }
    }

    #[doc = "Get a display buffer and a Z buffer for rendering"]
    #[must_use]
    #[inline]
    pub fn buffers(&mut self) -> (DisplayBuffer<'_>, DisplayBuffer<'_>) {
        unsafe {
            let color = DisplayBuffer(&mut *(display_get() as *mut Surface));
            let z = DisplayBuffer(&mut *(display_get_zbuf() as *mut Surface));
            (color, z)
        }
    }

    #[doc = "Try getting a display buffer and a Z buffer for rendering"]
    #[must_use]
    #[inline]
    pub fn try_buffers(&mut self) -> Option<(DisplayBuffer<'_>, DisplayBuffer<'_>)> {
        unsafe {
            core::ptr::NonNull::new(display_try_get() as *mut Surface).map(|mut s| {
                let color = DisplayBuffer(s.as_mut());
                let z = DisplayBuffer(&mut *(display_get_zbuf() as *mut Surface));
                (color, z)
            })
        }
    }

    #[doc = "Get the currently configured width of the display in pixels"]
    #[inline]
    pub fn width(&self) -> u32 {
        unsafe { display_get_width() }
    }

    #[doc = "Get the currently configured height of the display in pixels"]
    #[inline]
    pub fn height(&self) -> u32 {
        unsafe { display_get_height() }
    }

    #[doc = "Get the currently configured bitdepth of the display (in bytes per pixels)"]
    #[inline]
    pub fn bitdepth(&self) -> u32 {
        unsafe { display_get_bitdepth() }
    }

    #[doc = "Get the currently configured number of buffers"]
    #[inline]
    pub fn num_buffers(&self) -> u32 {
        unsafe { display_get_num_buffers() }
    }

    #[doc = "Get the current refresh rate of the video output in Hz\n\n The refresh rate is normally 50 for PAL and 60 for NTSC, but this function\n returns the hardware-accurate number which is close to those but not quite\n exact. Moreover, this will also account for advanced VI configurations\n affecting the refresh rate, like PAL60.\n\n @return float        Refresh rate in Hz (frames per second)"]
    #[inline]
    pub fn refresh_rate(&self) -> f32 {
        unsafe { display_get_refresh_rate() }
    }

    #[doc = "Get the current number of frames per second being rendered\n\n @return float Frames per second"]
    #[inline]
    pub fn fps(&self) -> f32 {
        unsafe { display_get_fps() }
    }

    #[doc = "Returns the \"delta time\", that is the time it took to the last frame\n        to be prepared and rendered.\n\n This function is useful for time-based animations and physics, as it allows\n to calculate the time elapsed between frames. Call this function once per\n frame to get the time elapsed since the last frame.\n\n @note Do not call this function more than once per frame. If needed, cache\n       the result in a variable and use it multiple times.\n\n @return float        Time elapsed since the last complete frame (in seconds)"]
    #[inline]
    pub fn delta_time(&self) -> f32 {
        unsafe { display_get_delta_time() }
    }

    #[doc = "Configure a limit for the frames per second\n\n This function allows to set a limit for the frames per second to render.\n The limit is enforced by the display module, which will slow down calls\n to display_get() if need to respect the limit.\n\n Passing 0 as argument will disable the limit.\n\n @param fps           The maximum number of frames per second to render (fractionals allowed)"]
    #[inline]
    pub fn set_fps_limit(&mut self, fps: f32) {
        unsafe { display_set_fps_limit(fps) }
    }

    #[doc = "Returns a surface that points to the framebuffer currently being shown on screen."]
    #[must_use]
    #[inline]
    pub fn current_framebuffer(&self) -> Surface<'_> {
        unsafe { Surface(display_get_current_framebuffer(), core::marker::PhantomData) }
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct DisplayBuffer<'s>(&'s mut Surface<'s>);

impl<'s> DisplayBuffer<'s> {
    #[doc = "Display a buffer on the screen\n\n Display a surface to the screen on the next vblank.\n\n Notice that this function does not accept any arbitrary surface, but only\n those returned by #display_get, which are owned by the display module.\n\n @param[in] surf\n            A surface to show (previously retrieved using #display_get)"]
    #[inline]
    pub fn show(self) {
        unsafe { display_show(&mut self.0.0) }
    }
}

wrapper! { DisplayBuffer<'s> => Surface<'s> { self => self.0 } }
