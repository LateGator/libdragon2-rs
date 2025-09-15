use core::ffi::CStr;

use crate::sys::ym64::*;

#[doc = "Player of a .YM64 file.\n\n This structure holds the state a player of a YM64 module. It can be\n initialized using [`YM64Player::open`], and played with [`YM64Player::play`].\n\n See the rest of this module for more functions."]
#[repr(transparent)]
#[derive(Debug)]
pub struct YM64Player(pub(crate) ym64player_t);

#[doc = "Structure containing information about a YM song"]
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct SongInfo(pub(crate) ym64player_songinfo_t);

impl YM64Player {
    #[doc = "Open a YM64 file for playback\n\n @param[in]  \tplayer \t\tYM64 player to initialize\n @param[in] \tfn \t\t\tFilename of the XM64 (with filesystem prefix, e.g. `rom://`)."]
    #[inline]
    pub fn open(fname: &CStr) -> Self {
        let mut ym = core::mem::MaybeUninit::uninit();
        unsafe {
            ym64player_open(ym.as_mut_ptr(), fname.as_ptr(), core::ptr::null_mut());
            Self(ym.assume_init())
        }
    }
    #[doc = "Open a YM64 file for playback\n\n @param[in]  \tplayer \t\tYM64 player to initialize\n @param[in] \tfn \t\t\tFilename of the XM64 (with filesystem prefix, e.g. `rom://`).\n @param[out] \tinfo \t\tSstructure to fill with information on the song)"]
    #[inline]
    pub fn open_with_info(fname: &CStr) -> (Self, SongInfo) {
        let mut ym = core::mem::MaybeUninit::uninit();
        let mut info = core::mem::MaybeUninit::uninit();
        unsafe {
            ym64player_open(ym.as_mut_ptr(), fname.as_ptr(), info.as_mut_ptr());
            (Self(ym.assume_init()), SongInfo(info.assume_init()))
        }
    }
    #[doc = "Return the number of channels used in the mixer for playback.\n\n Depending on the AY emulator compile-time settings, this could be either\n 1 or 2 (mono or stereo). Notice that the YM64 currently mixes itself the\n 3 internal channels of the AY8910 chip, so only a final output stream\n is passed to the mixer.\n\n @param[in]  \tplayer \t\tYM64 player\n @return  \t\t\t\tNumber of mixer channels."]
    pub fn num_channels(&self) -> u32 {
        unsafe { ym64player_num_channels(&self.0 as *const _ as *mut _) as u32 }
    }
    #[doc = "Start playback of a YM file.\n\n @param[in]\tplayer \t\tYM64 player\n @param[in] \tfirst_ch\tFirst mixer channel to use for playback"]
    pub fn play(&mut self, first_ch: u32) {
        unsafe { ym64player_play(&mut self.0, first_ch as _) }
    }
    #[doc = "Read the total duration the YM module.\n\n The function returns the total duration of the YM module, in ticks (internal\n YM position) or seconds. You can pass NULL to information that you are not\n interested in receiving.\n\n @param[in]\tplayer \t\tYM64 player\n @param[out] \tlen \t\tTotal duration in ticks\n @param[out] \tsecs \t\tTotal duration in seconds"]
    pub fn duration(&mut self) -> (u32, f32) {
        let mut len = core::mem::MaybeUninit::uninit();
        let mut secs = core::mem::MaybeUninit::uninit();
        unsafe {
            ym64player_duration(&mut self.0, len.as_mut_ptr(), secs.as_mut_ptr());
            (len.assume_init() as _, secs.assume_init())
        }
    }
    #[doc = "Read the current position of the YM module.\n\n The function returns the current position expressed in ticks (internal\n YM position), and also expressed as number of seconds. You can pass NULL\n to information that you are not interested in receiving.\n\n @param[in]\tplayer \t\tYM64 player\n @param[out] \tpos \t\tCurrent position in ticks\n @param[out] \tsecs \t\tCurrent position in seconds"]
    pub fn tell(&mut self) -> (u32, f32) {
        let mut pos = core::mem::MaybeUninit::uninit();
        let mut secs = core::mem::MaybeUninit::uninit();
        unsafe {
            ym64player_tell(&mut self.0, pos.as_mut_ptr(), secs.as_mut_ptr());
            (pos.assume_init() as _, secs.assume_init())
        }
    }
    #[doc = "Seek to a specific position in the YM module.\n\n The function seeks to a new absolute position expressed in ticks (internal\n YM position). Notice that it's not possible to seek in a YM64 file that has\n been compressed. audioconv64 compresses YM files by default.\n\n @param[in]\tplayer \t\tYM64 player\n @param[out] \tpos \t\tAbsolute position in ticks\n @return                  True if it was possible to seek, false if\n                          the file is compressed."]
    pub fn seek(&mut self, pos: u32) -> bool {
        unsafe { ym64player_seek(&mut self.0, pos as _) }
    }
    #[doc = "Stop YM playback.\n\n The YM module will keep the current position. Use [`Self::play`] to continue\n playback."]
    pub fn stop(&mut self) {
        unsafe { ym64player_stop(&mut self.0) }
    }
}

impl Drop for YM64Player {
    #[doc = "Close and deallocate the YM64 player."]
    #[inline]
    fn drop(&mut self) {
        unsafe { ym64player_close(&mut self.0) }
    }
}

impl SongInfo {
    #[doc = "Name of the song"]
    pub fn name(&self) -> &[u8; 128] {
        unsafe { core::mem::transmute(&self.0.name) }
    }
    #[doc = "Author of the song"]
    pub fn author(&self) -> &[u8; 128] {
        unsafe { core::mem::transmute(&self.0.author) }
    }
    #[doc = "Comment of the song"]
    pub fn comment(&self) -> &[u8; 128] {
        unsafe { core::mem::transmute(&self.0.comment) }
    }
}
