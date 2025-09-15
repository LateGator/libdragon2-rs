use core::sync::atomic::{AtomicPtr, Ordering};

use crate::sys::audio::*;

#[repr(transparent)]
#[derive(Debug)]
pub struct Audio(pub(crate) ());

static mut AUDIO_INIT: bool = false;

#[doc = "Initialize the audio subsystem\n\n This function will set up the AI to play at a given frequency and\n allocate a number of back buffers to write data to.\n\n @note Before re-initializing the audio subsystem to a new playback\n       frequency, remember to call [`Audio::drop`].\n\n @param[in] frequency\n            The frequency in Hz to play back samples at\n @param[in] numbuffers\n            The number of buffers to allocate internally"]
#[inline]
pub fn init(frequency: u32, numbuffers: u32) -> Audio {
    unsafe {
        assert_eq!((&raw mut AUDIO_INIT).read_volatile(), false);
        (&raw mut AUDIO_INIT).write_volatile(true);
        audio_init(frequency as _, numbuffers as _);
        Audio(())
    }
}

impl Drop for Audio {
    #[doc = "Close the audio subsystem\n\n This function closes the audio system and cleans up any internal\n memory allocated by [`init`]."]
    #[inline]
    fn drop(&mut self) {
        unsafe {
            audio_close();
            (&raw mut AUDIO_INIT).write_volatile(false);
        }
    }
}

impl Audio {
    #[doc = "Install a audio callback to fill the audio buffer when required.\n\n This function allows to implement a pull-based audio system. It registers\n a callback which will be invoked under interrupt whenever the AI is ready\n to have more samples enqueued. The callback can fill the provided audio\n data with samples that will be enqueued for DMA to AI.\n\n @param[in] fill_buffer_callback   Callback to fill an empty audio buffer"]
    #[inline]
    pub fn set_buffer_callback(func: Option<fn(&mut [i16])>) {
        static AUDIO_CALLBACK: AtomicPtr<()> = AtomicPtr::new(core::ptr::null_mut());
        unsafe extern "C" fn audio_callback_trampoline(
            buffer: *mut ::core::ffi::c_short,
            numsamples: usize,
        ) {
            let ptr = AUDIO_CALLBACK.load(Ordering::Relaxed);
            if ptr.is_null() {
                return;
            }
            unsafe {
                let buffer = core::slice::from_raw_parts_mut(buffer, numsamples);
                let func: fn(&mut [i16]) = core::mem::transmute(ptr);
                func(buffer);
            }
        }
        let (func, trampoline) = match func {
            Some(func) => (func as usize, Some(audio_callback_trampoline as _)),
            None => (0, None),
        };
        unsafe {
            audio_set_buffer_callback(trampoline);
            AUDIO_CALLBACK.store(func as *mut (), Ordering::Relaxed);
        }
    }
    #[doc = "Pause or resume audio playback\n\n Should only be used when a `fill_buffer_callback` has been set\n in [`init`].\n Silence will be generated while playback is paused."]
    #[inline]
    pub fn pause(&self, pause: bool) {
        unsafe { audio_pause(pause) }
    }
    #[doc = "Return whether there is an empty buffer to write to\n\n This function will check to see if there are any buffers that are not full to\n write data to.  If all buffers are full, wait until the AI has played back\n the next buffer in its queue and try writing again."]
    #[inline]
    pub fn can_write(&self) -> bool {
        unsafe { audio_can_write() != 0 }
    }
    #[doc = "Write a chunk of silence\n\n This function will write silence to be played back by the audio system.\n It writes exactly [`Self::bufer_length`] stereo samples.\n\n @note This function will block until there is room to write an audio sample.\n       If you do not want to block, check to see if there is room by calling\n       [`Self::can_write`]."]
    #[inline]
    pub fn write_silence(&mut self) {
        unsafe { audio_write_silence() }
    }
    #[doc = "Return actual frequency of audio playback\n\n @return Frequency in Hz of the audio playback"]
    #[inline]
    pub fn frequency(&self) -> u32 {
        unsafe { audio_get_frequency() as _ }
    }
    #[doc = "Get the number of stereo samples that fit into an allocated buffer\n\n @note To get the number of bytes to allocate, multiply the return by\n       2 * sizeof( short )\n\n @return The number of stereo samples in an allocated buffer"]
    #[inline]
    pub fn buffer_length(&self) -> u32 {
        unsafe { audio_get_buffer_length() as _ }
    }
    #[doc = "Start writing to the first free internal buffer.\n\n This function is similar to [`Self::push`] but instead of taking samples\n and copying them to an internal buffer, it returns the pointer to the\n internal buffer. This allows generating the samples directly in the buffer\n that will be sent via DMA to AI, without any subsequent memory copy.\n\n The buffer should be filled with stereo interleaved samples, and\n exactly [`Self::buffer_length`] samples should be written.\n\n After you have written the samples, call audio_write_end() to notify\n the library that the buffer is ready to be sent to AI.\n\n @note This function will block until there is room to write an audio sample.\n       If you do not want to block, check to see if there is room by calling\n       [`Self::can_write`].\n\n @return  Pointer to the internal memory buffer where to write samples."]
    #[inline]
    pub fn write_begin(&mut self) -> Buffer<'_> {
        Buffer(unsafe {
            core::slice::from_raw_parts_mut(audio_write_begin(), self.buffer_length() as usize)
        })
    }
    #[doc = "Push a chunk of audio data (high-level function)\n\n This function is an easy-to-use, higher level alternative to all\n the audio_write* functions. It pushes audio samples into output\n hiding the complexity required to match the fixed-size audio buffers.\n\n The function accepts a @p buffer of stereo interleaved audio samples;\n @p nsamples is the number of samples in the buffer. The function will\n push the samples into output as much as possible.\n\n If @p blocking is true, it will stop and wait until all samples have\n been pushed into output. If @p blocking is false, it will stop as soon\n as there are no more free buffers to push samples into, and will return\n the number of pushed samples. It is up to the caller to then take care\n of this and later try to call audio_push again with the remaining samples.\n\n @note You CANNOT mixmatch this function with the other audio_write* functions,\n       and viceversa. If you decide to use audio_push, use it exclusively to\n       push the audio.\n\n @param buffer        Buffer containing stereo samples to be played\n @param nsamples      Number of stereo samples in the buffer\n @param blocking      If true, wait until all samples have been pushed\n @return int          Number of samples pushed into output"]
    #[inline]
    pub fn push(&mut self, buffer: &[i16], blocking: bool) -> usize {
        unsafe { audio_push(buffer.as_ptr(), buffer.len() as _, blocking) as _ }
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Buffer<'s>(&'s mut [i16]);

impl<'s> crate::Undroppable for &mut Buffer<'s> {
    const ERROR: &'static str = "Finish the audio write with Buffer::end";
}
impl<'s> Drop for Buffer<'s> {
    #[inline]
    fn drop(&mut self) {
        let _ = crate::DropBomb::new(self);
    }
}

impl<'s> Buffer<'s> {
    #[doc = "Complete writing to an internal buffer.\n\n This function is meant to be used in pair with audio_write_begin().\n Call this once you have generated the samples, so that the audio\n system knows the buffer has been filled and can be played back.\n"]
    #[inline]
    pub fn end(self) {
        let _ = core::mem::ManuallyDrop::new(self);
        unsafe { audio_write_end() }
    }
}

wrapper! { Buffer<'s> => [i16] { self => self.0 } }
