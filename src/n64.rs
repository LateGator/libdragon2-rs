use core::{alloc::Layout, ptr::NonNull};

use crate::sys::n64sys;

#[doc = "A physical address on the MIPS bus.\n\n Physical addresses are 32-bit wide, and are used to address the memory\n space of the MIPS R4300 CPU. The MIPS R4300 CPU has a 32-bit address bus,\n and can address up to 4 GiB of memory.\n\n Physical addresses are just numbers, they cannot be used as pointers (dereferenced).\n To access them, you must first convert them virtual addresses using the\n `VirtualCachedAddr` or `VirtualUncachedAddr` macros.\n\n In general, libdragon will try to use `phys_addr_t` whenever a physical\n address is expected or returned, and C pointers for virtual addresses.\n Unfortunately, not all codebase can be changed to follow this convention\n for backward compatibility reasons."]
type PhysAddr = u32;

#[derive(Debug, Copy, Clone)]
pub struct HeapStats {
    pub total: u32,
    pub used: u32,
}

#[doc = "Reset types"]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Reset {
    #[doc = "Cold reset (power on)"]
    Cold = n64sys::reset_type_t_RESET_COLD as _,
    #[doc = "Warm reset (reset button)"]
    Warm = n64sys::reset_type_t_RESET_WARM as _,
}

#[doc = "Type of TV video output"]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum TV {
    #[doc = "Video output is PAL"]
    PAL = n64sys::tv_type_t_TV_PAL,
    #[doc = "Video output is NTSC"]
    NTSC = n64sys::tv_type_t_TV_NTSC,
    #[doc = "Video output is M-PAL"]
    MPAL = n64sys::tv_type_t_TV_MPAL,
}

#[doc = "Frequency of the RCP"]
#[inline]
pub fn rcp_frequency() -> u32 {
    match unsafe { n64sys::__boot_consoletype != 0 } {
        true => 96000000,
        false => 62500000,
    }
}

#[doc = "Frequency of the MIPS R4300 CPU"]
#[inline]
pub fn cpu_frequency() -> u32 {
    match unsafe { n64sys::__boot_consoletype != 0 } {
        true => 144000000,
        false => 93750000,
    }
}

#[doc = "Return the physical memory address for a given virtual address (pointer)"]
#[inline]
pub fn physical_addr<T>(addr: NonNull<T>) -> PhysAddr {
    (addr.as_ptr() as u32) & !0xE0000000
}

#[doc = "Create a virtual addresses in a cached segment to access a physical address\n\nThis macro creates a virtual address that can be used to access a physical\naddress in the cached segment of the memory. The cached segment is the\nsegment of memory that is cached by the CPU, and is the default segment\nfor all memory accesses.\n\nThe virtual address created by this macro can be used as a pointer in C\nto access the physical address."]
#[inline]
pub const fn virtual_cached_addr<T>(addr: PhysAddr) -> NonNull<T> {
    unsafe { NonNull::new_unchecked((addr | 0x80000000) as i32 as isize as _) }
}

#[doc = "Create a virtual addresses in an uncached segment to access a physical address\n\nThis macro creates a virtual address that can be used to access a physical\naddress in the uncached segment of the memory. The uncached segment is the\nsegment of memory that is not cached by the CPU, and is used for memory\nthat is accessed by hardware devices, like the RCP.\n\nThe virtual address created by this macro can be used as a pointer in C\nto access the physical address."]
#[inline]
pub const fn virtual_uncached_addr<T>(addr: PhysAddr) -> NonNull<T> {
    unsafe { NonNull::new_unchecked((addr | 0xA0000000) as i32 as isize as _) }
}

#[doc = "Return the uncached memory address for a given virtual address"]
#[inline]
pub fn uncached_addr<T>(addr: NonNull<T>) -> NonNull<T> {
    addr.map_addr(|p| p | 0x2000_0000)
}

#[doc = "Return the cached memory address for a given virtual address"]
#[inline]
pub unsafe fn cached_addr<T>(addr: NonNull<T>) -> NonNull<T> {
    addr.map_addr(|p| unsafe { core::num::NonZero::new_unchecked(p.get() & !0x2000_0000) })
}

#[doc = "Returns the 32-bit hardware tick counter\n\nThis macro returns the current value of the hardware tick counter,\npresent in the CPU coprocessor 0. The counter increments at half of the\nprocessor clock speed (see `ticks_per_second`), and overflows every\n91.625 seconds.\n\nIt is fine to use this hardware counter for measuring small time intervals,\nas long as `TICKS_DISTANCE` or `TICKS_BEFORE` are used to compare different\ncounter reads, as those macros correctly handle overflows.\n\nMost users might find more convenient to use `get_ticks`, a similar function\nthat returns a 64-bit counter with the same frequency that never overflows.\n\n@see `TICKS_BEFORE`"]
#[inline]
pub fn ticks_read() -> u32 {
    let value;
    unsafe { core::arch::asm!(".set     noat\n\tmfc0     {}, $9", out(reg) value) };
    value
}

#[doc = "Number of updates to the count register per second"]
#[inline]
pub fn ticks_per_second() -> u32 {
    cpu_frequency() / 2
}

#[doc = "Calculate the time passed between two ticks\n\nIf `from` is before `to`, the distance in time is positive, otherwise it is negative."]
#[inline]
pub fn ticks_distance(from: u32, to: u32) -> i32 {
    to.wrapping_sub(from) as i32
}

#[doc = "Return how much time has passed since the instant `t0`."]
#[inline]
pub fn ticks_since(t0: u32) -> i32 {
    ticks_distance(t0, ticks_read())
}

#[doc = "Returns true if `t1` is before `t2`.\n\nThis is similar to `t1 < t2`, but it correctly handles timer overflows\nwhich are very frequent. Notice that the hardware counter overflows every\n~91 seconds, so it's not possible to compare times that are more than\n~45 seconds apart.\n\nUse `get_ticks` to get a 64-bit counter that never overflows.\n"]
#[inline]
pub fn ticks_before(t1: u32, t2: u32) -> bool {
    ticks_distance(t1, t2) > 0
}

#[doc = "Returns equivalent count ticks for the given milliseconds."]
#[inline]
pub fn ticks_from_ms(val: u32) -> u32 {
    val * (ticks_per_second() / 1000)
}

#[doc = "Returns equivalent count ticks for the given microseconds."]
#[inline]
pub fn ticks_from_us(val: u32) -> u32 {
    val * (8 * ticks_per_second() / 1000000) / 8
}

#[doc = "Returns equivalent count ticks for the given microseconds."]
#[inline]
pub fn ticks_to_us(val: u32) -> u32 {
    val * 8 / (8 * ticks_per_second() / 1000000)
}

#[doc = "Returns equivalent count ticks for the given microseconds."]
#[inline]
pub fn ticks_to_ms(val: u32) -> u32 {
    val / (ticks_per_second() / 1000)
}

#[doc = "Return true if we are running on a iQue player"]
#[inline]
pub fn is_bbplayer() -> bool {
    unsafe { n64sys::__boot_consoletype != 0 }
}
#[doc = "Is system NTSC/PAL/MPAL\n\n Checks enum hard-coded in PIF BootROM to indicate the tv type of the system.\n\n @return enum value indicating PAL, NTSC or MPAL"]
#[inline]
pub fn tv_type() -> TV {
    unsafe { core::mem::transmute(n64sys::__boot_tvtype) }
}
#[doc = "Get reset type\n\n This function returns the reset type, that can be used to differentiate\n a cold boot from a warm boot (that is, after pressing the reset button).\n\n For instance, a game might want to skip mandatory intros (eg: logos)\n on a warm boot."]
#[inline]
pub fn reset_type() -> Reset {
    unsafe { core::mem::transmute(n64sys::sys_reset_type()) }
}
#[doc = "Get amount of available memory.\n\n @return amount of total available memory in bytes."]
#[inline]
pub fn memory_size() -> u32 {
    unsafe { n64sys::get_memory_size() as u32 }
}
#[doc = "Is expansion pak in use.\n\n Checks whether the maximum available memory has been expanded to 8 MiB.\n If your application needs to the use of the expansion pak, you should provide\n an error message to the user if it is not present. Libdragon offers a\n function to do this, `assert_memory_expanded`, which will emit an error\n\n @return true if expansion pak detected, false otherwise.\n\n @note On iQue, this function returns true only if the game has been assigned\n       exactly 8 MiB of RAM."]
#[inline]
pub fn is_memory_expanded() -> bool {
    unsafe { n64sys::is_memory_expanded() }
}

#[doc = "Read the number of ticks since system startup\n\n The frequency of this counter is `ticks_per_second`. The counter will\n never overflow, being a 64-bit number.\n\n @return The number of ticks since system startup"]
#[inline]
pub fn ticks() -> u64 {
    unsafe { n64sys::get_ticks() }
}
#[doc = "Read the number of microseconds since system startup\n\n This is similar to `get_ticks`, but converts the result in integer\n microseconds for convenience.\n\n @return The number of microseconds since system startup"]
#[inline]
pub fn ticks_us() -> u64 {
    unsafe { n64sys::get_ticks_us() }
}
#[doc = "Read the number of millisecounds since system startup\n\n This is similar to `get_ticks`, but converts the result in integer\n milliseconds for convenience.\n\n @return The number of millisecounds since system startup"]
#[inline]
pub fn ticks_ms() -> u64 {
    unsafe { n64sys::get_ticks_ms() }
}
#[doc = "Spin wait until the number of ticks have elapsed\n\n @param[in] wait\n            Number of ticks to wait\n            Maximum accepted value is 0xFFFFFFFF ticks"]
#[inline]
pub fn wait_ticks(wait: u32) {
    unsafe {
        n64sys::wait_ticks(wait);
    }
}
#[doc = "Spin wait until the number of milliseconds have elapsed\n\n @param[in] wait_ms\n            Number of milliseconds to wait\n            Maximum accepted value is 91625 ms"]
#[inline]
pub fn wait_ms(wait_ms: u32) {
    unsafe { n64sys::wait_ms(wait_ms) }
}
#[doc = "Force a complete halt of all processors\n\n @note It should occur whenever a reset has been triggered\n and its past its RESET_TIME_LENGTH grace time period.\n This function will shut down the RSP and the CPU, blank the VI.\n Eventually the RDP will flush and complete its work as well.\n The system will recover after a reset or power cycle.\n"]
#[inline]
pub fn die() -> ! {
    unsafe { n64sys::die() }
}
#[doc = "Force a data cache invalidate over a memory region\n\n Use this to force the N64 to update cache from RDRAM.\n\n The cache is made by cachelines of 16 bytes. If a memory region is invalidated\n and the memory region is not fully aligned to cachelines, a larger area\n than that requested will be invalidated; depending on the arrangement of\n the data segments and/or heap, this might make data previously\n written by the CPU in regular memory locations to be unexpectedly discarded,\n causing bugs.\n\n For this reason, this function must only be called with an address aligned\n to 16 bytes, and with a length which is an exact multiple of 16 bytes; it\n will assert otherwise.\n\n As an alternative, consider using `data_cache_hit_writeback_invalidate`,\n that first writebacks the affected cachelines to RDRAM, guaranteeing integrity\n of memory areas that share cachelines with the region that must be invalidated.\n\n @param[in] addr\n            Pointer to memory in question\n @param[in] length\n            Length in bytes of the data pointed at by addr"]
#[inline]
pub fn data_cache_hit_invalidate<T>(data: &[T]) {
    unsafe {
        n64sys::data_cache_hit_invalidate(data.as_ptr() as _, (data.len() * size_of::<T>()) as _)
    }
}
#[doc = "Force a data cache writeback over a memory region\n\n Use this to force cached memory to be written to RDRAM.\n\n @param[in] addr\n            Pointer to memory in question\n @param[in] length\n            Length in bytes of the data pointed at by addr"]
#[inline]
pub fn data_cache_hit_writeback<T>(data: &[T]) {
    unsafe {
        n64sys::data_cache_hit_writeback(data.as_ptr() as _, (data.len() * size_of::<T>()) as _)
    }
}
#[doc = "Force a data cache writeback invalidate over a memory region\n\n Use this to force cached memory to be written to RDRAM\n and then invalidate the corresponding cache lines.\n\n @param[in] addr\n            Pointer to memory in question\n @param[in] length\n            Length in bytes of the data pointed at by addr"]
#[inline]
pub fn data_cache_hit_writeback_invalidate<T>(data: &[T]) {
    unsafe {
        n64sys::data_cache_hit_writeback_invalidate(
            data.as_ptr() as _,
            (data.len() * size_of::<T>()) as _,
        )
    }
}
#[doc = "Force a data cache index writeback invalidate over a memory region\n\n @param[in] addr\n            Pointer to memory in question\n @param[in] length\n            Length in bytes of the data pointed at by addr"]
#[inline]
pub fn data_cache_index_writeback_invalidate<T>(data: &[T]) {
    unsafe {
        n64sys::data_cache_index_writeback_invalidate(
            data.as_ptr() as _,
            (data.len() * size_of::<T>()) as _,
        )
    }
}
#[doc = "Force a data cache writeback invalidate over whole memory\n\n Also see `data_cache_hit_writeback_invalidate`\n"]
#[inline]
pub fn data_cache_writeback_invalidate_all() {
    unsafe { n64sys::data_cache_writeback_invalidate_all() }
}
#[doc = "Force an instruction cache writeback over a memory region\n\n Use this to force cached memory to be written to RDRAM.\n\n @param[in] addr\n            Pointer to memory in question\n @param[in] length\n            Length in bytes of the data pointed at by addr"]
#[inline]
pub fn inst_cache_hit_writeback<T>(data: &[T]) {
    unsafe {
        n64sys::inst_cache_hit_writeback(data.as_ptr() as _, (data.len() * size_of::<T>()) as _)
    }
}
#[doc = "Force an instruction cache invalidate over whole memory\n\n Also see `inst_cache_hit_invalidate`\n"]
#[inline]
pub fn inst_cache_invalidate_all() {
    unsafe { n64sys::inst_cache_invalidate_all() }
}
#[doc = "Assert that the expansion pak is present.\n\n This function will emit an error screen if the expansion pak is not present,\n and will halt the system. It should be called in main() to ensure that the\n expansion pak is present before proceeding with the rest of\n the application. This enforces a good pattern to make the application fails\n early with a proper error message (rather than a crash) if the expansion pak\n is not present.\n\n If you want to provide your own graphical error screen, use\n `is_memory_expanded` instead to check if the expansion pak is present,\n and then show your own error screen if it is not present."]
#[inline]
pub fn assert_memory_expanded() {
    unsafe { n64sys::assert_memory_expanded() }
}
#[doc = "Return information about memory usage of the heap"]
#[inline]
pub fn heap_stats() -> HeapStats {
    let mut stats = core::mem::MaybeUninit::uninit();
    unsafe {
        n64sys::sys_get_heap_stats(stats.as_mut_ptr());
        let stats = stats.assume_init();
        HeapStats {
            total: stats.total as u32,
            used: stats.used as u32,
        }
    }
}
#[doc = "Allocate a buffer that will be accessed as uncached memory, specifying alignment\n\n This function is similar to `malloc_uncached`, but allows to force a higher\n alignment to the buffer (just like memalign does). See `malloc_uncached`\n for reference.\n\n @param[in]  align The alignment of the buffer in bytes (eg: 64)\n @param[in]  size  The size of the buffer to allocate\n\n @return a pointer to the start of the buffer (in the uncached segment)\n\n @see `malloc_uncached`"]
#[inline]
pub unsafe fn alloc_uncached(layout: Layout) -> *mut u8 {
    unsafe { n64sys::malloc_uncached_aligned(layout.align() as _, layout.size()) as _ }
}
#[doc = "Free an uncached memory buffer\n\n This function frees a memory buffer previously allocated via `malloc_uncached`.\n\n @param[in]  buf  The buffer to free\n\n @see `malloc_uncached`"]
#[inline]
pub fn dealloc_uncached(buf: *mut u8) {
    unsafe { n64sys::free_uncached(buf as _) }
}

pub unsafe trait InterruptArg: Sized + Send + Sync + 'static {
    type Fn;
    unsafe fn cast_fn(func: Self::Fn) -> unsafe extern "C" fn(*mut core::ffi::c_void);
    unsafe fn from_ptr(ptr: *mut core::ffi::c_void) -> Self;
    unsafe fn into_ptr(self) -> *mut core::ffi::c_void;
}

unsafe impl InterruptArg for usize {
    type Fn = fn(usize);
    #[inline]
    unsafe fn cast_fn(func: Self::Fn) -> unsafe extern "C" fn(*mut core::ffi::c_void) {
        unsafe { core::mem::transmute(func) }
    }
    #[inline]
    unsafe fn from_ptr(ptr: *mut core::ffi::c_void) -> Self {
        ptr as _
    }
    #[inline]
    unsafe fn into_ptr(self) -> *mut core::ffi::c_void {
        self as _
    }
}
unsafe impl InterruptArg for isize {
    type Fn = fn(isize);
    #[inline]
    unsafe fn cast_fn(func: Self::Fn) -> unsafe extern "C" fn(*mut core::ffi::c_void) {
        unsafe { core::mem::transmute(func) }
    }
    #[inline]
    unsafe fn from_ptr(ptr: *mut core::ffi::c_void) -> Self {
        ptr as usize as _
    }
    #[inline]
    unsafe fn into_ptr(self) -> *mut core::ffi::c_void {
        self as usize as _
    }
}
unsafe impl InterruptArg for () {
    type Fn = fn();
    #[inline]
    unsafe fn cast_fn(func: Self::Fn) -> unsafe extern "C" fn(*mut core::ffi::c_void) {
        unsafe { core::mem::transmute(func) }
    }
    #[inline]
    unsafe fn from_ptr(_ptr: *mut core::ffi::c_void) -> Self {
        ()
    }
    #[inline]
    unsafe fn into_ptr(self) -> *mut core::ffi::c_void {
        core::ptr::null_mut()
    }
}
unsafe impl<T: Send + Sync + 'static> InterruptArg for &'static T {
    type Fn = fn(&'static T);
    #[inline]
    unsafe fn cast_fn(func: Self::Fn) -> unsafe extern "C" fn(*mut core::ffi::c_void) {
        unsafe { core::mem::transmute(func) }
    }
    #[inline]
    unsafe fn from_ptr(ptr: *mut core::ffi::c_void) -> Self {
        unsafe { core::mem::transmute(ptr) }
    }
    #[inline]
    unsafe fn into_ptr(self) -> *mut core::ffi::c_void {
        unsafe { core::mem::transmute(self) }
    }
}
unsafe impl<T: Send + Sync + 'static> InterruptArg for &'static mut T {
    type Fn = fn(&'static mut T);
    #[inline]
    unsafe fn cast_fn(func: Self::Fn) -> unsafe extern "C" fn(*mut core::ffi::c_void) {
        unsafe { core::mem::transmute(func) }
    }
    #[inline]
    unsafe fn from_ptr(ptr: *mut core::ffi::c_void) -> Self {
        unsafe { core::mem::transmute(ptr) }
    }
    #[inline]
    unsafe fn into_ptr(self) -> *mut core::ffi::c_void {
        unsafe { core::mem::transmute(self) }
    }
}
