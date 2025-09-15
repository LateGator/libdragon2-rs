use core::ffi::CStr;

use crate::sys::xm64::*;

#[doc = "Player of a .XM64 file.\n\n This structure holds the state a player of a XM64 module. It can be\n initialized using [`XM64Player::open`], and played with [`XM64Player::play`].\n\n See the rest of this module for more functions."]
#[repr(transparent)]
#[derive(Debug)]
pub struct XM64Player(pub(crate) xm64player_t);

impl XM64Player {
    #[doc = "Open a XM64 module file and prepare for playback.\n\n This function requires the mixer to have been already initialized\n (via mixer_init).\n\n XM64 files can carry their own embedded samples, or can use an external\n sample library. In the latter case, make sure to call [`Self::set_extsampledir`]\n to set the directory where the external samples are stored.\n\n @param player Pointer to the xm64player_t player structure to use\n @param fn     Filename of the XM64 (with filesystem prefix)."]
    #[inline]
    pub fn open(fname: &CStr) -> Self {
        let mut xm = core::mem::MaybeUninit::uninit();
        unsafe {
            xm64player_open(xm.as_mut_ptr(), fname.as_ptr());
            Self(xm.assume_init())
        }
    }
    #[doc = "Get the number of channels in the XM64 file\n\n Notice that the player needs to use one mixer channel per each XM64 channel."]
    pub fn num_channels(&self) -> u32 {
        unsafe { xm64player_num_channels(&self.0 as *const _ as *mut _) as u32 }
    }
    #[doc = "Configure a XM64 file for looping.\n\n By default, XM64 files will be played in loop. Use this function\n to disable looping.\n\n @param[in] player\n            XM64 player\n @param[in] loop\n            true to enable looping, false to disable looping."]
    pub fn set_loop(&mut self, r#loop: bool) {
        unsafe { xm64player_set_loop(&mut self.0, r#loop) }
    }
    #[doc = "Start playing the XM64 module.\n\n Notice that the player needs to use one mixer channel per each XM64 channel.\n\n @param player \tXM64 player\n @param first_ch \tIndex of the first mixer channel to use for playback."]
    pub fn play(&mut self, first_ch: u32) {
        unsafe { xm64player_play(&mut self.0, first_ch as _) }
    }
    #[doc = "Read the current position of the XM module.\n\n The function returns the current position expressed as\n pattern/row (internal XM position), and also expressed\n as number of seconds. You can pass NULL to information\n that you are not interested in receiving.\n\n @param player        XM64 player\n @param[out] patidx   Index of the XM pattern\n @param[out] row      Row within the pattern\n @param[out] secs     Total number of seconds"]
    pub fn tell(&mut self) -> (u32, u32, f32) {
        let mut patidx = core::mem::MaybeUninit::uninit();
        let mut row = core::mem::MaybeUninit::uninit();
        let mut secs = core::mem::MaybeUninit::uninit();
        unsafe {
            xm64player_tell(
                &mut self.0,
                patidx.as_mut_ptr(),
                row.as_mut_ptr(),
                secs.as_mut_ptr(),
            );
            (
                patidx.assume_init() as _,
                row.assume_init() as _,
                secs.assume_init(),
            )
        }
    }
    #[doc = "Seek to a specific position of the XM module.\n\n Seeking in XM module is \"broken by design\". What this function does\n is to move the playback cursor to the specified position, but\n it doesn't take into effect what samples / effects should be active\n at the seeking point.\n\n @param player \t\tXM64 player\n @param patidx \t\tIndex of the XM pattern to seek to\n @param row \t\t\tRow within the pattern to seek to\n @param tick \t\t\tTick within the row to seek to"]
    pub fn seek(&mut self, patidx: u32, row: u32, tick: u32) {
        unsafe { xm64player_seek(&mut self.0, patidx as _, row as _, tick as _) }
    }
    #[doc = "Change the volume of the player.\n\n This allows to tune the volume of playback. The default volume is 1.0; smaller\n values will lower the volume, higher values will amplificate (but may clip)."]
    pub fn set_vol(&mut self, volume: f32) {
        unsafe { xm64player_set_vol(&mut self.0, volume) }
    }
    #[doc = "Set a custom effect callback to allow music synchronization.\n\n This function configures a callback that will be called whenever the player\n finds an unknown / unsupported effect in any channel. These unknown effects\n can be used to add custom \"sync cues\" in the music score, and synchronize\n graphic effects or gameplay logic to them.\n\n There are many unused effect letters in XM format. For instance, a good\n choice can be effect Xxx which is used as modplug hack for MIDI support,\n but is unimplemented by standard XM players like this one.\n\n The callback will be called passing as arguments a custom context, the\n channel number, and the effect code and the effect parameter. The effect\n code is the code in extended hex format (A-F are 10-15 as in normal hex,\n but then G-Z are 16-35), while the effect parameter is one free byte that\n can be inserted in the music score."]
    pub fn set_effect_callback<T: crate::n64::InterruptArg>(
        &mut self,
        func: fn(T, u8, u8, u8),
        data: T,
    ) {
        unsafe {
            xm64player_set_effect_callback(
                &mut self.0,
                Some(core::mem::transmute(func)),
                data.into_ptr(),
            )
        }
    }
    #[doc = "Stop XM playback.\n\n The XM module will keep the current position. Use [`Self::play`] to continue\n playback."]
    pub fn stop(&mut self) {
        unsafe { xm64player_stop(&mut self.0) }
    }
}

impl Drop for XM64Player {
    #[doc = "Close and deallocate the XM64 player."]
    #[inline]
    fn drop(&mut self) {
        unsafe { xm64player_close(&mut self.0) }
    }
}

#[doc = "Configure the directory where external samples are stored.\n\n This function is used to set the directory where the external samples\n are stored. It is only used for XM64 files that use external samples.\n\n @param dir \t\tDirectory where the external samples are stored. This\n \t\t\t\t\tcan be on any filesystem, even different from XM64's one."]
pub fn set_extsampledir(dir: &CStr) {
    unsafe { xm64_set_extsampledir(dir.as_ptr()) };
}
