use core::{ffi::CStr, marker::PhantomData, ptr::NonNull};

use crate::sys::mixer::*;

#[repr(transparent)]
#[derive(Debug)]
pub struct Mixer<'s>(PhantomData<&'s mut crate::audio::Audio>);

static_wrapper! { Mixer<'s> => crate::audio::Audio { crate::audio::Audio(()) } }

#[doc = "Initialize the mixer\n\n The mixer must be initialized after the audio subsystem (audio_init).\n The number of channels specified is the maximum number of channels\n used by the application. Specifying a higher number means using\n more memory as the mixer will allocate one sample buffer per channel,\n but it does not affect performance (which correlates to the\n actual number of simultaneously playing channels).\n\n @param[in]    num_channels   Number of channels to initialize."]
#[inline]
pub fn init(_audio: &mut crate::audio::Audio, channels: u32) -> Mixer<'_> {
    unsafe {
        mixer_init(channels as _);
        Mixer(PhantomData)
    }
}

impl<'s> Drop for Mixer<'s> {
    #[doc = "Deinitialize the mixer."]
    #[inline]
    fn drop(&mut self) {
        unsafe {
            mixer_close();
        }
    }
}

#[doc = "A waveform that can be played back through the mixer.\n\n waveform_t represents a waveform that can be played back by the mixer.\n A waveform_t does not hold the actual samples because most real-world use\n cases do not keep all samples in memory, but rather load them and/or\n decompress them in real-time while the playback is happening.\n So waveform_t instead should be thought of as the generator of a\n waveform.\n\n To create a waveform, use one of waveform implementations such as wav64.\n Waveform implementations are in charge of generating the samples by actually\n implementing an audio format like VADPCM or MPEG-2.\n\n Waveforms can produce samples as 8-bit or 16-bit. Samples must always be\n signed. Stereo waveforms (interleaved samples) are supported: when used\n with `mixer_ch_play`, they will use automatically two channels (the specified\n one and the following)."]
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Waveform(pub(crate) waveform_t);

impl Waveform {
    #[inline]
    pub const unsafe fn from_raw(wave: waveform_t) -> Self {
        Self(wave)
    }
    #[doc = "Name of the waveform (for debugging purposes)"]
    #[inline]
    pub const fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0.name) }
    }
    #[doc = "Width of a sample of this waveform, in bits.\n\n Supported values are 8 or 16. Notice that samples must always be signed."]
    #[inline]
    pub const fn bits(&self) -> u8 {
        self.0.bits
    }
    #[doc = "Number of interleaved audio channels in this waveforms.\n\n Supported values are 1 and 2 (mono and stereo waveforms). Notice that\n a stereo waveform will use two consecutive mixer channels to be played back."]
    #[inline]
    pub const fn channels(&self) -> u8 {
        self.0.channels
    }
    #[doc = "Desired playback frequency (in samples per second, aka Hz)."]
    #[inline]
    pub const fn frequency(&self) -> f32 {
        self.0.frequency
    }
    #[doc = "Length of the waveform, in number of samples.\n\n If the length is not known, this value should be set to `WAVEFORM_UNKNOWN_LEN`."]
    #[inline]
    pub const fn len(&self) -> u32 {
        self.0.len as _
    }
    #[doc = "Length of the loop of the waveform (from the end).\n\n This value describes how many samples of the tail of the waveform needs\n to be played in a loop. For instance, if len==1200 and loop_len=500, the\n waveform will be played once, and then the last 700 samples will be\n repeated in loop."]
    #[inline]
    pub const fn loop_len(&self) -> u32 {
        self.0.loop_len as _
    }
    #[inline]
    pub const fn as_raw(&self) -> *mut waveform_t {
        &self.0 as *const _ as _
    }
}

impl<'s> Mixer<'s> {
    #[doc = "Set master volume.\n\n This is a global attenuation factor (range [0..1]) that will be applied\n to all channels and simplify implementing a global volume control.\n\n @param[in]    vol            Master volume (range [0..1])"]
    pub fn set_vol(&mut self, vol: f32) {
        unsafe { mixer_set_vol(vol) }
    }
    #[doc = "Set channel volume (as left/right).\n\n Configure channel volume for the specified channel, specifying two values:\n one for the left output and one for the right output.\n\n The volume is an attenuation (no amplification is performed).\n Valid volume range in [0..1], where 0 is silence and 1 is original\n channel sample volume (no attenuation performed).\n\n Notice that it's perfectly valid to set left/right volumes even if the\n channel itself will play a mono waveforms, as it allows to balance a mono\n sample between the two final output channels.\n\n @param[in]   ch              Channel index\n @param[in]   lvol            Left volume (range [0..1])\n @param[in]   rvol            Right volume (range [0..1])"]
    pub fn ch_set_vol(&mut self, ch: u32, lvol: f32, rvol: f32) {
        unsafe { mixer_ch_set_vol(ch as _, lvol, rvol) }
    }
    #[doc = "Set channel volume (as volume and panning).\n\n Configure the left and right channel volumes for the specified channel,\n Using a central volume value and a panning value to specify left/right\n balance.\n\n Valid volume range in [0..1], where 0 is silence and 1 is maximum\n volume (no attenuation).\n\n Valid panning range is [0..1] where 0 is 100% left, and 1 is 100% right.\n\n Notice that panning 0.5 balance the sound but causes an attenuation of 50%.\n\n @param[in]   ch              Channel index\n @param[in]   vol             Central volume (range [0..1])\n @param[in]   pan             Panning (range [0..1], center is 0.5)"]
    pub fn ch_set_vol_pan(&mut self, ch: u32, vol: f32, pan: f32) {
        unsafe { mixer_ch_set_vol_pan(ch as _, vol, pan) }
    }
    #[doc = "Set channel volume with Dolby Pro Logic II encoding.\n\n Configure the volumes of the specified channel according to the Dolby Pro\n Logic II matrix encoding. This allows to encode samples with a virtual surround\n system, that can be decoded with a Dolby 5.1 compatible equipment.\n\n The function accepts the volumes configured for the 5 channels: front left,\n front right, center, surround left, surround right. These values can be\n calculated from a 3D scene\n\n @param[in]   ch              Channel index\n @param[in]   fl              Front left volume (range [0..1])\n @param[in]   fr              Front right volume (range [0..1])\n @param[in]   c               Central volume (range [0..1])\n @param[in]   sl              Surround left volume (range [0..1])\n @param[in]   sr              Surround right volume (range [0..1])"]
    pub fn ch_set_vol_dolby(&mut self, ch: u32, fl: f32, fr: f32, c: f32, sl: f32, sr: f32) {
        unsafe { mixer_ch_set_vol_dolby(ch as _, fl, fr, c, sl, sr) }
    }
    #[doc = "Start playing the specified waveform on the specified channel.\n\n This function immediately begins playing the waveform, interrupting any\n other waveform that might have been reproduced on this channel.\n\n Waveform settings are applied to the mixer channel; for instance, the\n frequency of the channel is modified to adapt to the frequency requested\n for correct playback of the waveform.\n\n If the waveform is marked as stereo (channels == 2), the mixer will need\n two channels to play it back. \"ch\" will be used for the left samples,\n while \"ch+1\" will be used for the right samples. After this, it is\n forbidden to call mixer functions on \"ch+1\" until the stereo\n waveform is stopped.\n\n If the same waveform (same pointer) was already being played or was the\n last one that was played on this channel, the channel sample buffer\n is retained, so that any cached samples might be reused.\n\n @param[in]   ch              Channel index\n @param[in]   wave            Waveform to playback\n\n @see `mixer_ch_stop`"]
    pub fn ch_play(&mut self, ch: u32, wave: &'s mut Waveform) {
        unsafe { mixer_ch_play(ch as _, &mut wave.0) }
    }
    #[doc = "Change the frequency for the specified channel.\n\n By default, the frequency is the one required by the waveform associated\n to the channel, but this function allows to override.\n\n This function must be called after `mixer_ch_play`, as otherwise the\n frequency is reset to the default of the waveform.\n\n @param[in]   ch              Channel index\n @param[in]   frequency       Playback frequency (in Hz / samples per second)"]
    pub fn ch_set_freq(&mut self, ch: u32, frequency: f32) {
        unsafe { mixer_ch_set_freq(ch as _, frequency) }
    }
    #[doc = "Change the current playback position within a waveform.\n\n This function can be useful to seek to a specific point of the waveform.\n The position must be specified in number of samples (not bytes). Fractional\n values account for accurate resampling position.\n\n This function must be called after `mixer_ch_play`, as otherwise the\n position is reset to the beginning of the waveform.\n\n @param[in]   ch              Channel index\n @param[in]   pos             Playback position (in number of samples)"]
    pub fn ch_set_pos(&mut self, ch: u32, pos: f64) {
        unsafe { mixer_ch_set_pos(ch as _, pos) }
    }
    #[doc = " Read the current playback position of the waveform in the channel.\n\n The position is returned as number of samples. Fractional values account\n for accurate resampling position.\n\n @param[in]   ch              Channel index\n @return                      Playback position (in number of samples)"]
    pub fn ch_pos(&self, ch: u32) -> f64 {
        unsafe { mixer_ch_get_pos(ch as _) }
    }
    #[doc = "Stop playing samples on the specified channel."]
    pub fn ch_stop(&mut self, ch: u32) {
        unsafe { mixer_ch_stop(ch as _) }
    }
    #[doc = "Return true if the channel is currently playing samples."]
    pub fn ch_is_playing(&mut self, ch: u32) -> bool {
        unsafe { mixer_ch_playing(ch as _) }
    }
    #[doc = "Return the waveform being played on this channel, or NULL if none."]
    pub unsafe fn ch_playing_waveform(&self, ch: u32) -> Option<&Waveform> {
        unsafe { NonNull::new(mixer_ch_playing_waveform(ch as _) as _).map(|p| p.as_ref()) }
    }
    #[doc = "Configure the limits of a channel with respect to sample bit size, and\nfrequency.\n\n This is an advanced function that should be used with caution, only in\n situations in which it is paramount to control the memory usage of the mixer.\n\n By default, each channel in the mixer is capable of doing 16-bit playback\n with a frequency up to the mixer output sample rate (eg: 44100hz). This means\n that the mixer will allocate sample buffers required for this kind of\n capability.\n\n If it is known that certain channels will use only 8-bit waveforms and/or\n a lower frequency, it is possible to call this function to inform the mixer\n of these limits. This will cause the mixer to reallocate the samplebuffers\n lowering its memory usage (note: multiple calls to this function for different\n channels will of course be batched to cause only one reallocation).\n\n Note also that this function can be used to increase the maximum frequency\n over the mixer sample rate, in case this is required. This works correctly\n but since it causes downsampling, it is generally a waste of memory bandwidth\n and processing power.\n\n \"max_buf_sz\" can be used to limit the maximum buffer size that will be\n allocated for this channel (in bytes). This is a hard cap, applied on top\n of the optimal buffer size that will be calculated by \"max_bits\" and\n \"max_frequency\", and can be used in situations where there are very strong\n memory constraints that must be respected. Use 0 if you don't want to impose\n a limit.\n\n @param[in]   ch              Channel index\n @param[in]   max_bits        Maximum number of bits per sample (or 0 to reset\n                              this to default, which is currently 16).\n @param[in]   max_frequency   Maximum playback frequency for this channel\n                              in Hz / samples per seconds (or 0 to reset\n                              this to default, which is the output sample\n                              rate as specified in `audio_init`).\n @param[in]   max_buf_sz      Maximum buffer size in bytes (or 0 to reset\n                              this default, which is calculated using the\n                              other limits, the playback output rate, and\n                              the number of audio buffers specified in\n                              `audio_init`)."]
    pub fn ch_set_limits(ch: u32, max_bits: u32, max_frequency: f32, max_buf_sz: u32) {
        unsafe { mixer_ch_set_limits(ch as _, max_bits as _, max_frequency, max_buf_sz as _) }
    }
    #[doc = "Throttle the mixer by specifying the maximum number of samples\nit can generate.\n\n This is an advanced function that should only be called to achieve perfect\n sync between a possibly slowing-down video and audio.\n\n Normally, once the mixer is initiated and assuming mixer_poll is called\n frequently enough, the audio will playback uninterrupted, irrespective of\n any slow down in the main loop. This is the expected behavior for background\n music for instance, but it does not work for video players or cut-scenes in\n which the music must be perfectly synchronized with the video. If the video\n happens to slowdown, the music will desynchronize.\n\n mixer_throttle sets a budget of samples that the mixer is allowed to\n generate. Every time the function is called, the specified number of samples\n is added to the budget. Every time the mixer playbacks the channel, the\n budget is decreased. If the budget reaches zero, the mixer will automatically\n pause playback until the budget is increased again, possibly creating\n audio cracks.\n\n To achieve perfect sync, call `mixer_throttle` every time a video frame\n was generated, and pass the maximum number of samples that the mixer is\n allowed to produce. Typically, you will want to pass the audio samplerate\n divided by the video framerate, which corresponds to the number of\n audio samples per video frame.\n\n @param[in]   num_samples     Number of new samples that the mixer is allowed\n                              to produce for this channel. This will be added\n                              to whatever allowance was left.\n\n @see `mixer_unthrottle`"]
    pub fn throttle(&mut self, num_samples: f32) {
        unsafe { mixer_throttle(num_samples) }
    }
    #[doc = "Unthrottle the mixer\n\n Switch back the mixer to the default unthrottled status, after some calls to\n `mixer_throttle`.\n\n After calling `mixer_unthrottle`, the mixer will no longer be limited and\n will produce all the samples requested via `mixer_poll`.\n\n @see `mixer_throttle`"]
    pub fn unthrottle(&mut self) {
        unsafe { mixer_unthrottle() }
    }
    #[doc = "Start writing to the first free internal buffer.\n\n See [`crate::audio::Audio::push`]>"]
    #[inline]
    pub fn write_begin(&mut self) -> Buffer<'_> {
        Buffer((**self).write_begin())
    }
    #[doc = "Run the mixer to produce output samples.\n\n This function will fetch the required samples from all the channels and\n mix them together according to each channel's settings. The output will\n be written into the specified buffer (out). nsamples is the number of\n samples that should be produced.\n\n A common pattern would be to call `audio_write_begin` to obtain an audio\n buffer's pointer, and pass it to mixer_poll.\n\n mixer_poll performs mixing using RSP. If RSP is busy, mixer_poll will\n spin-wait until the RSP is free, to perform audio processing.\n\n Since the N64 AI can only be fed with an even number of samples, mixer_poll\n does not accept odd numbers.\n\n This function will respect throttling, if configured via `mixer_throttle`.\n In this case, it may produce less samples than requested, depending on\n the current allowance. The rest of the output buffer will be zeroed.\n\n @param[in]   out             Output buffer were samples will be written.\n @param[in]   nsamples        Number of stereo samples to generate."]
    pub fn poll(&mut self, buf: &mut [i16]) {
        unsafe { mixer_poll(buf.as_mut_ptr(), buf.len() as _) }
    }
    #[doc = "Request the mixer to try and write audio samples to be played,\n if possible.\n\n This function is a user helper for asking the mixer and audio subsystems\n to play audio during a game frame. You should call this function many times\n during one frame (eg. during the render step or after processing each game\n object) as many times as necessary. Not polling the audio subsystem often\n enough will result in audio stutter."]
    pub fn try_play(&mut self) {
        unsafe { mixer_try_play() }
    }
    #[doc = "Register a time-based event into the mixer.\n\n Register a new event into the mixer. \"delay\" is the number of samples to\n wait before calling the event callback. \"cb\" is the event callback. \"ctx\"\n is an opaque pointer that will be passed to the callback when invoked.\n\n @param[in]   delay           Number of samples to wait before invoking\n                              the event.\n @param[in]   cb              Event callback to invoke\n @param[in]   ctx             Context opaque pointer to pass to the callback"]
    pub fn add_event<T: crate::n64::InterruptArg>(
        &mut self,
        delay: i64,
        func: fn(T) -> i32,
        data: T,
    ) {
        unsafe { mixer_add_event(delay, Some(core::mem::transmute(func)), data.into_ptr()) }
    }
    #[doc = "Deregister a time-based event from the mixer.\n\n Deregister an event from the mixer. \"cb\" is the event callback, and \"ctx\"\n is the opaque context pointer. Notice that an event can also deregister\n itself by returning 0 when called.\n\n @param[in]    cb             Callback that was registered via `mixer_add_event`\n @param[in]    ctx            Opaque pointer that was registered with the callback."]
    pub fn remove_event<T: crate::n64::InterruptArg>(&mut self, func: fn(T) -> i32, data: T) {
        unsafe { mixer_remove_event(Some(core::mem::transmute(func)), data.into_ptr()) }
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Buffer<'s>(crate::audio::Buffer<'s>);

impl<'s> Buffer<'s> {
    #[doc = "Run the mixer to produce output samples.\n\n This function will fetch the required samples from all the channels and\n mix them together according to each channel's settings. The output will\n be written into the specified buffer (out). nsamples is the number of\n samples that should be produced.\n\n A common pattern would be to call `audio_write_begin` to obtain an audio\n buffer's pointer, and pass it to mixer_poll.\n\n mixer_poll performs mixing using RSP. If RSP is busy, mixer_poll will\n spin-wait until the RSP is free, to perform audio processing.\n\n Since the N64 AI can only be fed with an even number of samples, mixer_poll\n does not accept odd numbers.\n\n This function will respect throttling, if configured via `mixer_throttle`.\n In this case, it may produce less samples than requested, depending on\n the current allowance. The rest of the output buffer will be zeroed.\n\n @param[in]   out             Output buffer were samples will be written.\n @param[in]   nsamples        Number of stereo samples to generate."]
    #[inline]
    pub fn poll(&mut self) {
        self.poll_sub(0, self.len())
    }
    #[inline]
    pub fn poll_sub(&mut self, offset: usize, len: usize) {
        unsafe { mixer_poll(self[offset..offset + len].as_mut_ptr(), len as _) }
    }
    #[doc = "Complete writing to an internal buffer.\n\n This function is meant to be used in pair with audio_write_begin().\n Call this once you have generated the samples, so that the audio\n system knows the buffer has been filled and can be played back.\n"]
    #[inline]
    pub fn end(self) {
        self.0.end();
    }
}

wrapper! { Buffer<'s> => [i16] { self => self.0 } }
