use core::{ffi::CStr, ptr::NonNull};

use crate::{mixer::Waveform, sys::wav64::*};

#[doc = "WAV64 structure\n\n This structure is initialized by [`open`] to refer to an opened WAV64\n file. It is meant to be played back through the audio mixer, implementing\n the `waveform_t` interface. As such, samples are not preloaded in memory\n but rather loaded on request when needed for playback, streaming directly\n from ROM. See `waveform_t` for more details.\n\n Use `wav64_play` to playback. For more advanced usage, call directly the\n mixer functions, accessing the `wave` structure field."]
#[repr(C)]
#[derive(Debug)]
pub struct Wav64 {
    wave: Waveform,
    st: *mut wav64_state_t,
}

wrapper! { Wav64 => Waveform { self => self.wave } }

#[doc = "WAV64 structure\n\n This structure is initialized by [`open`] to refer to an opened WAV64\n file. It is meant to be played back through the audio mixer, implementing\n the `waveform_t` interface. As such, samples are not preloaded in memory\n but rather loaded on request when needed for playback, streaming directly\n from ROM. See `waveform_t` for more details.\n\n Use `wav64_play` to playback. For more advanced usage, call directly the\n mixer functions, accessing the `wave` structure field."]
#[repr(transparent)]
#[derive(Debug)]
pub struct BoxWav64(pub(crate) NonNull<Wav64>);

ptr_wrapper! { BoxWav64 => Wav64 { self => self.0 } }

#[doc = "WAV64 streaming mode"]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum StreamingMode {
    #[doc = "Full streaming\n\n This is the default mode for streaming. All samples of the files are\n streamed from the file (typically, from ROM but could also be SD) on-demand.\n\n This uses the least amount of memory but requires data transfers from\n the storage device during playback."]
    Full = wav64_streaming_mode_t_WAV64_STREAMING_FULL as _,
    #[doc = "Preload and decompress the whole file\n\n This mode preloads the whole file into memory and decompresses it in full\n at the beginning. This is useful for small files that can fit in memory,\n and for which you do not want to pay for the streaming overhead."]
    None = wav64_streaming_mode_t_WAV64_STREAMING_NONE as _,
}

#[doc = "WAV64 loading parameters (to be passed to [`Wav64::load`])"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LoadParms(pub(crate) wav64_loadparms_t);

#[doc = "Enable a non-default compression level\n\nThis function must be called if any wav64 that will be loaded use\na non-default compression level. The default compression level is 1 (VADPCM)\nfor which no initialization is required. Level 0 (uncompressed) also\nrequires no initialization.\n\nCurrently, only level 3 requires initialization (level 2 does not exist yet).\nIf you have any wav64 compressed with level 3, you must call this function\nbefore opening them.\n\n@code{.c}\nwav64_init_compression(3);\n\nwav64_open(&jingle, \"rom:/jingle.wav64\");\n@endcode\n\n@param level     Compression level to initialize\n\n@see `wav64_open`"]
#[inline]
pub fn init_compression(level: crate::asset::Compression) {
    match level {
        crate::asset::Compression::Level3 => unsafe { __wav64_init_compression_lvl3() },
        _ => panic!("Unsupported compression level"),
    }
}

#[doc = "Open a WAV64 file for playback.\n\n This function opens the file, parses the header, and initializes for\n playing back through the audio mixer.\n\n @param   wav         Pointer to wav64_t structure\n @param   fn          Filename of the wav64 (with filesystem prefix). Currently,\n                      only files on DFS (\"rom:/\") are supported."]
#[inline]
pub fn open(fname: &CStr) -> Wav64 {
    unsafe {
        let mut wav = core::mem::MaybeUninit::uninit();
        wav64_open(wav.as_mut_ptr(), fname.as_ptr());
        core::mem::transmute(wav.assume_init())
    }
}
#[doc = "Load a WAV64 file for playback.\n\n This function opens the file, parses the header, and initializes for\n playing back through the audio mixer.\n\n You can use the `wav64_loadparms_t` structure to specify additional parameters\n for the loading process, like the maximum number of simultaneous playbacks\n or the streaming mode.\n\n @param   fn          Filename of the wav64 (with filesystem prefix).\n @param   parms       Optional loading parameters (or NULL for defaults)."]
#[inline]
pub fn load(fname: &CStr, parms: Option<&LoadParms>) -> BoxWav64 {
    unsafe {
        BoxWav64(
            NonNull::new(wav64_load(
                fname.as_ptr(),
                parms
                    .map(|p| &p.0 as *const _ as _)
                    .unwrap_or_else(core::ptr::null_mut),
            ) as _)
            .unwrap(),
        )
    }
}

impl Drop for Wav64 {
    #[doc = "Close a WAV64 file.\n\n This function closes the file and frees any resources associated with it.\n If the file is currently playing, playback will be stopped.\n\n @param wav \t\t\tPointer to wav64_t structure"]
    #[inline]
    fn drop(&mut self) {
        unsafe { wav64_close(self.as_raw()) }
    }
}

impl Drop for BoxWav64 {
    #[doc = "Close a WAV64 file.\n\n This function closes the file and frees any resources associated with it.\n If the file is currently playing, playback will be stopped.\n\n @param wav \t\t\tPointer to wav64_t structure"]
    #[inline]
    fn drop(&mut self) {
        unsafe { wav64_close(self.as_raw()) }
    }
}

impl Wav64 {
    #[doc = "Configure a WAV64 file for looping playback."]
    #[inline]
    pub fn set_loop(&mut self, loop_: bool) {
        unsafe { wav64_set_loop(self.as_raw(), loop_) }
    }
    #[doc = "Start playing a WAV64 file.\n\n This is just a simple wrapper that calls `mixer_ch_play` on the WAV64's\n waveform (wav64_t::wave). For advanced usages, please call directly the\n mixer functions.\n\n It is possible to start the same waveform on multiple independent channels.\n Playback will automatically stop when the waveform is finished, unless it\n is looping. To stop playing a wav64 file before it is normally finished,\n call `mixer_ch_stop` on the channel used for playback.\n\n @param   wav         Pointer to wav64_t structure\n @param   ch          Channel of the mixer to use for playback."]
    #[inline]
    pub fn play(&mut self, _mixer: &mut crate::mixer::Mixer, ch: u32) {
        unsafe { wav64_play(self.as_raw(), ch as _) }
    }
    #[doc = "Get the (possibly compressed) bitrate of the WAV64 file.\n\n @param wav \t\t\tPointer to wav64_t structure\n @return int \t\t\tBitrate in bits per second"]
    #[inline]
    pub fn bitrate(&self) -> u32 {
        unsafe { wav64_get_bitrate(self.as_raw()) as _ }
    }
    #[inline]
    pub const fn as_raw(&self) -> *mut wav64_s {
        self as *const _ as _
    }
}

impl LoadParms {
    #[inline]
    pub const fn new() -> Self {
        Self(wav64_loadparms_s {
            streaming_mode: StreamingMode::Full as _,
        })
    }
    #[doc = "Streaming mode for the wav64\n\n See [`StreamingMode`] for details."]
    #[inline]
    pub const fn streaming_mode(mut self, streaming_mode: StreamingMode) -> Self {
        self.0.streaming_mode = streaming_mode as _;
        self
    }
}

impl Default for LoadParms {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
