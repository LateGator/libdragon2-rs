use core::{marker::PhantomData, ptr::NonNull};

use crate::{rsp::UCode, sys::rspq::*};

#[repr(transparent)]
#[derive(Debug)]
pub struct Block(pub(crate) NonNull<rspq_block_t>);

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BlockRef<'b>(pub(crate) NonNull<rspq_block_t>, PhantomData<&'b Block>);

#[repr(transparent)]
#[derive(Debug)]
pub struct Syncpoint(pub(crate) rspq_syncpoint_t);

#[repr(transparent)]
#[derive(Debug)]
pub struct RspQ(pub(crate) ());

#[doc = "Initialize the RSPQ library.\n\n This should be called by the initialization functions of the higher-level\n libraries using the RSP command queue. It can be safely called multiple\n times without side effects.\n\n It is not required by applications to call this explicitly in the main\n function."]
#[inline]
pub fn init() -> RspQ {
    unsafe {
        rspq_init();
    }
    RspQ(())
}

impl RspQ {
    #[doc = "Register a rspq overlay into the RSP queue engine.\n\n This function registers a rspq overlay into the queue engine.\n An overlay is a RSP ucode that has been written to be compatible with the\n queue engine (see rsp_queue.inc for instructions) and is thus able to\n execute commands that are enqueued in the queue. An overlay doesn't have\n a single entry point: it exposes multiple functions bound to different\n commands, that will be called by the queue engine when the commands are enqueued.\n\n The function returns the overlay ID, which is the ID to use to enqueue\n commands for this overlay. The overlay ID must be passed to `rspq_write`\n when adding new commands. rspq allows up to 16 overlays to be registered\n simultaneously, as the overlay ID occupies the top 4 bits of each command.\n The lower 4 bits specify the command ID, so in theory each overlay could\n offer a maximum of 16 commands. To overcome this limitation, this function\n will reserve multiple consecutive IDs in case an overlay with more than 16\n commands is registered. These additional IDs are silently occupied and\n never need to be specified explicitly when queueing commands.\n\n For example if an overlay with 32 commands were registered, this function\n could return ID 0x60, and ID 0x70 would implicitly be reserved as well.\n To queue the twenty first command of this overlay, you would write\n `rspq_write(ovl_id, 0x14, ...)`, where `ovl_id` is the value that was returned\n by this function.\n\n @param      overlay_ucode  The overlay to register\n\n @return     The overlay ID that has been assigned to the overlay. Note that\n             this value will be preshifted by 28 (eg: 0x60000000 for ID 6),\n             as this is the expected format used by `rspq_write`."]
    #[inline]
    pub unsafe fn overlay_register(&self, ucode: &UCode) -> u32 {
        unsafe { rspq_overlay_register(&ucode.0 as *const _ as _) }
    }
    #[doc = "Register an overlay into the RSP queue engine assigning a static ID to it\n\n This function works similar to `rspq_overlay_register`, except it will\n attempt to assign the specified ID to the overlay instead of automatically\n choosing one. Note that if the ID (or a consecutive IDs) is already used\n by another overlay, this function will assert, so careful usage is advised.\n\n Assigning a static ID can mostly be useful for debugging purposes.\n\n @param      overlay_ucode  The ucode to register\n @param      overlay_id     The ID to register the overlay with. This ID\n                            must be preshifted by 28 (eg: 0x40000000).\n\n @see `rspq_overlay_register`"]
    #[inline]
    pub unsafe fn overlay_register_static(&self, ucode: &UCode, overlay_id: u32) {
        unsafe { rspq_overlay_register_static(&ucode.0 as *const _ as _, overlay_id) }
    }
    #[doc = "Unregister a ucode overlay from the RSP queue engine.\n\n This function removes an overlay that has previously been registered\n with `rspq_overlay_register` or `rspq_overlay_register_static` from the\n queue engine. After calling this function, the specified overlay ID\n (and consecutive IDs in case the overlay has more than 16 commands)\n is no longer valid and must not be used to write new commands into the queue.\n\n Note that when new overlays are registered, the queue engine may recycle\n IDs from previously unregistered overlays.\n\n @param      overlay_id  The ID of the ucode (as returned by\n                         `rspq_overlay_register`) to unregister."]
    #[inline]
    pub unsafe fn overlay_unregister(&self, overlay_id: u32) {
        unsafe { rspq_overlay_unregister(overlay_id) }
    }
    #[doc = "Return a pointer to the overlay state (in RDRAM)\n\n Overlays can define a section of DMEM as persistent state. This area will be\n preserved across overlay switching, by reading back into RDRAM the DMEM\n contents when the overlay is switched away.\n\n This function returns a pointer to the state area in RDRAM (not DMEM). It is\n meant to modify the state on the CPU side while the overlay is not loaded.\n The layout of the state and its size should be known to the caller.\n\n To avoid race conditions between overlay state access by CPU and RSP, this\n function first calls `rspq_wait` to force a full sync and make sure the RSP is\n idle. As such, it should be treated as a debugging function.\n\n @param      overlay_ucode  The ucode overlay for which the state pointer will be returned.\n\n @return     Pointer to the overlay state (in RDRAM). The pointer is returned in\n             the cached segment, so make sure to handle cache coherency appropriately."]
    #[inline]
    pub unsafe fn overlay_state<'u, T>(&self, ucode: &'u mut UCode) -> &'u mut T {
        unsafe { &mut *(rspq_overlay_get_state(&ucode.0 as *const _ as _) as *mut _) }
    }
    #[doc = "Write a new command into the RSP queue.\n\nThis macro is the main entry point to add a command to the RSP queue. It can\nbe used as a variadic argument function, in which the first argument is\nthe overlay ID, the second argument is the command index within the overlay,\nand the other arguments are the command arguments (additional 32-bit words).\n\n@code{.c}\n// This example adds to the queue a command called CMD_SPRITE with \n// index 0xA, with its arguments, for a total of three words. The overlay\n// was previously registered via `rspq_overlay_register`.\n\n`define` CMD_SPRITE  0xA\n\nrspq_write(gfx_overlay_id, CMD_SPRITE,\nsprite_num, \n(x0 << 16) | y0,\n(x1 << 16) | y1);\n@endcode\n\nAs explained in the top-level documentation, the overlay ID (4 bits) and the\ncommand index (4 bits) are composed to form the command ID, and are stored\nin the most significant byte of the first word. So, the first command argument\nword, if provided, must have the upper MSB empty, to leave space\nfor the command ID itself.\n\nFor instance, assuming the overlay ID for an overlay called `gfx` is 1, \n`rspq_write(gfx_id, 0x2, 0x00FF2233)` is a correct call, which\nwrites `0x12FF2233` into the RSP queue; it will invoke command `2` in\noverlay with ID 0x1, and the first word contains other 24 bits of arguments\nthat will be parsed by the RSP code. Instead, `rspq_write(gfx_id, 0x2, 0x11FF2233)`\nis an invalid call because the MSB of the first word is non-zero, and thus\nthe highest byte `0x11` would clash with the overlay ID and command index.\n\n`rspq_write(gfx_id, 0x2)` is a valid call, and equivalent to\n`rspq_write(gfx_id, 0x2, 0x0)`. It will write `0x12000000` in the queue.\n\nNotice that after a call to `rspq_write`, the command might or might not\nget executed by the RSP, depending on timing. If you want to make sure that\nthe command will be executed, use `rspq_flush`. You can call `rspq_flush`\nafter you have finished writing a batch of related commands. See `rspq_flush`\ndocumentation for more information.\n\n`rspq_write` allows to write a full command with a single call, which is\nnormally the easiest way to do it; it supports up to 16 argument words.\nIn case it is needed to assemble larger commands, see `rspq_write_begin`\nfor an alternative API.\n\n@note Each command can be up to `RSPQ_MAX_SHORT_COMMAND_SIZE` 32-bit words.\n\n@param      ovl_id    The overlay ID of the command to enqueue. Notice that\nthis must be a value preshifted by 28, as returned\nby `rspq_overlay_register`.\n@param      cmd_id    Index of the command to call, within the overlay.\n@param      ...       Optional arguments for the command\n\n@see `rspq_overlay_register`\n@see `rspq_flush`\n@see `rspq_write_begin`"]
    #[inline]
    pub unsafe fn write<const N: usize>(&mut self, ovl_id: u32, cmd_id: u32, args: [u32; N]) {
        const {
            assert!(
                N <= RSPQ_MAX_SHORT_COMMAND_SIZE as usize,
                "too many arguments to RspQ::write, please use RspQ::write_begin instead"
            );
        }
        unsafe {
            let ptr = (&raw mut rspq_cur_pointer).read_volatile();
            for (i, arg) in args.iter().copied().enumerate() {
                ptr.add(i + 1).write_volatile(arg);
            }
            ptr.write_volatile(ovl_id + (cmd_id << 24));
            (&raw mut rspq_cur_pointer).write_volatile(ptr.add(1 + args.len()));
            if ptr > (&raw mut rspq_cur_sentinel).read_volatile() {
                rspq_next_buffer();
            }
        }
    }
    #[doc = "Begin writing a new command into the RSP queue.\n\n This command initiates a sequence to enqueue a new command into the RSP\n queue. Call this command passing the overlay ID and command ID of the command\n to create. Then, call `rspq_write_arg` once per each argument word that\n composes the command. Finally, call `rspq_write_end` to finalize and enqueue\n the command.\n\n A sequence made by `rspq_write_begin`, `rspq_write_arg`, `rspq_write_end` is\n functionally equivalent to a call to `rspq_write`, but it allows to\n create bigger commands, and might better fit some situations where arguments\n are calculated on the fly. Performance-wise, the code generated by\n `rspq_write_begin` + `rspq_write_arg` + `rspq_write_end` should be very similar\n to a single call to `rspq_write`, though just a bit slower. It is advisable\n to use `rspq_write` whenever possible.\n\n Make sure to read the documentation of `rspq_write` as well for further\n details.\n\n @param      ovl_id    The overlay ID of the command to enqueue. Notice that\n                       this must be a value preshifted by 28, as returned\n                       by `rspq_overlay_register`.\n @param      cmd_id    Index of the command to call, within the overlay.\n @param      size      The size of the commands in 32-bit words\n @returns              A write cursor, that must be passed to `rspq_write_arg`\n                       and `rspq_write_end`\n\n @see `rspq_write_arg`\n @see `rspq_write_end`\n @see `rspq_write`"]
    #[inline]
    pub unsafe fn write_begin(&mut self, ovl_id: u32, cmd_id: u32, size: u32) -> Write<'_> {
        debug_assert!(
            size <= RSPQ_MAX_COMMAND_SIZE,
            "The maximum command size is {RSPQ_MAX_COMMAND_SIZE}!"
        );

        unsafe {
            let cur = (&raw mut rspq_cur_pointer).read_volatile();
            if rspq_cur_pointer
                > (&raw mut rspq_cur_sentinel)
                    .read_volatile()
                    .sub(size as usize)
            {
                rspq_next_buffer();
            }
            (&raw mut rspq_cur_pointer).write_volatile(cur.add(size as usize));
            Write {
                _rspq: PhantomData,
                first_word: ovl_id + (cmd_id << 24),
                pointer: cur.add(1),
                first: cur,
                is_first: true,
            }
        }
    }
    #[doc = "Make sure that RSP starts executing up to the last written command.\n\n RSP processes the command queue asynchronously as it is being written.\n If it catches up with the CPU, it halts itself and waits for the CPU to\n notify that more commands are available. On the contrary, if the RSP lags\n behind it might keep executing commands as they are written without ever\n sleeping. So in general, at any given moment the RSP could be crunching\n commands or sleeping waiting to be notified that more commands are available.\n\n This means that writing a command via `rspq_write` is not enough to make sure\n it is executed; depending on timing and batching performed\n by RSP, it might either be executed automatically or not. `rspq_flush` makes\n sure that the RSP will see it and execute it.\n\n This function does not block: it just make sure that the RSP will run the\n full command queue written until now. If you need to actively wait until the\n last written command has been executed, use `rspq_wait`.\n\n It is suggested to call rspq_flush every time a new \"batch\" of commands\n has been written. In general, it is not a problem to call it often because\n it is very very fast (takes only ~20 cycles). For instance, it can be called\n after every rspq_write without many worries, but if you know that you are\n going to write a number of subsequent commands in straight line code, you\n can postpone the call to `rspq_flush` after the whole sequence has been written.\n\n @code{.c}\n \t\t// This example shows some code configuring the lights for a scene.\n \t\t// The command in this sample is called CMD_SET_LIGHT and requires\n \t\t// a light index and the RGB colors for the list to update.\n      uint32_t gfx_overlay_id;\n\n \t\t`define` CMD_SET_LIGHT  0x7\n\n \t\tfor (int i=0; i<MAX_LIGHTS; i++) {\n \t\t\trspq_write(gfx_overlay_id, CMD_SET_LIGHT, i,\n \t\t\t    (lights[i].r << 16) | (lights[i].g << 8) | lights[i].b);\n \t\t}\n\n \t\t// After enqueuing multiple commands, it is sufficient\n \t\t// to call rspq_flush once to make sure the RSP runs them (in case\n \t\t// it was idling).\n \t\trspq_flush();\n @endcode\n\n @note This is an experimental API. In the future, it might become\n       a no-op, and flushing could happen automatically at every `rspq_write`.\n       We are keeping it separate from `rspq_write` while experimenting more\n       with the RSPQ API.\n\n @note This function is a no-op if it is called while a block is being recorded\n       (see `rspq_block_begin` / `rspq_block_end`). This means calling this function\n       in a block recording context will not guarantee the execution of commands\n       that were queued prior to starting the block.\n"]
    #[inline]
    pub fn flush(&self) {
        unsafe { rspq_flush() }
    }
    #[doc = "Wait until all commands in the queue have been executed by RSP.\n\n This function blocks until all commands present in the queue have\n been executed by the RSP and the RSP is idle. If the queue contained also\n RDP commands, it also waits for those commands to finish drawing.\n\n This function exists mostly for debugging purposes. Calling this function\n is not necessary, as the CPU can continue adding commands to the queue\n while the RSP is running them. If you need to synchronize between RSP and CPU\n (eg: to access data that was processed by RSP) prefer using `rspq_syncpoint_new` /\n `rspq_syncpoint_wait` which allows for more granular synchronization."]
    #[inline]
    pub fn wait(&self) {
        unsafe { rspq_wait() }
    }
    #[doc = "Create a syncpoint in the queue.\n\n This function creates a new \"syncpoint\" referencing the current position\n in the queue. It is possible to later check when the syncpoint\n is reached by the RSP via `rspq_syncpoint_check` and `rspq_syncpoint_wait`.\n\n @return     ID of the just-created syncpoint.\n\n @note It is not possible to create a syncpoint within a block because it\n       is meant to be a one-time event. Otherwise the same syncpoint would\n       potentially be triggered multiple times, which is not supported.\n\n @note It is not possible to create a syncpoint from the high-priority queue\n       due to the implementation requiring syncpoints to be triggered\n       in the same order they have been created.\n\n @see `rspq_syncpoint_t`\n @see `rspq_syncpoint_new_cb`"]
    #[must_use]
    #[inline]
    pub fn syncpoint(&self) -> Syncpoint {
        Syncpoint(unsafe { rspq_syncpoint_new() })
    }
    #[doc = "Create a syncpoint in the queue that triggers a callback on the CPU.\n\n This function is similar to `rspq_syncpoint_new`: it creates a new \"syncpoint\"\n that references the current position in the queue. When the RSP reaches\n the syncpoint, it notifies the CPU, that will invoke the provided callback\n function.\n\n The callback function will be called *outside* of the interrupt context, so\n that it is safe for instance to call into most the standard library.\n\n The callback function is guaranteed to be called after the RSP has reached\n the syncpoint, but there is no guarantee on \"how much\" after. In general\n the callbacks will be treated as \"lower priority\" by rspq, so they will\n be called in best effort.\n\n @param func          Callback function to call when the syncpoint is reached\n @param arg           Argument to pass to the callback function\n @return rspq_syncpoint_t     ID of the just-created syncpoint.\n\n @see `rspq_syncpoint_t`\n @see `rspq_syncpoint_new`"]
    #[inline]
    pub fn syncpoint_cb<T: crate::n64::InterruptArg>(&mut self, func: T::Fn, data: T) -> Syncpoint {
        Syncpoint(unsafe { rspq_syncpoint_new_cb(Some(T::cast_fn(func)), data.into_ptr()) })
    }
    #[doc = "Enqueue a callback to be called by the CPU\n\n This function enqueues a callback that will be called by the CPU when\n the RSP has finished all commands put in the queue until now.\n\n An example of a use case for this function is to free resources such as\n rspq blocks that are no longer needed, but that you want to make sure that\n are not referenced anymore by the RSP.\n\n See also `rdpq_call_deferred` that, in addition to waiting for RSP, it also\n waits for RDP to process all pending commands before calling the callback.\n\n @note DO NOT CALL RSPQ FUNCTIONS INSIDE THE CALLBACK (including enqueueing\n       new rspq commands). This might cause a deadlock or corruption, and it\n       is not supported.\n\n @param func      Callback function\n @param arg       Argument to pass to the callback\n\n @see `rdpq_call_deferred`"]
    #[inline]
    pub fn call_deferred<T: crate::n64::InterruptArg>(&mut self, func: T::Fn, data: T) {
        self.syncpoint_cb(func, data);
        self.flush();
    }
    #[doc = "Begin creating a new block.\n\n This function begins writing a command block (see `rspq_block_t`).\n While a block is being written, all calls to `rspq_write`\n will record the commands into the block, without actually scheduling them for\n execution. Use `rspq_block_end` to close the block and get a reference to it.\n\n Only one block at a time can be created. Calling `rspq_block_begin`\n twice (without any intervening `rspq_block_end`) will cause an assert.\n\n During block creation, the RSP will keep running as usual and\n execute commands that have been already added to the queue.\n\n @note Calls to `rspq_flush` are ignored during block creation, as the RSP\n       is not going to execute the block commands anyway."]
    #[must_use]
    #[inline]
    pub fn block_begin(&mut self) -> BlockBuilder<'_> {
        unsafe {
            assert!(crate::sys::rdpq::rspq_block.is_null());
            rspq_block_begin();
            BlockBuilder(PhantomData)
        }
    }
    #[doc = "Add to the RSP queue a command that runs a block.\n\n This function runs a block that was previously created via `rspq_block_begin`\n and `rspq_block_end`. It schedules a special command in the queue\n that will run the block, so that execution of the block will happen in\n order relative to other commands in the queue.\n\n Blocks can call other blocks. For instance, if a block A has been fully\n created, it is possible to call `rspq_block_run(A)` at any point during the\n creation of a second block B; this means that B will contain the special\n command that will call A.\n\n @param block The block that must be run\n\n @note The maximum depth of nested block calls is 8."]
    #[inline]
    pub fn block_run<'b>(&self, block: impl Into<BlockRef<'b>>) {
        unsafe { rspq_block_run(block.into().0.as_ptr()) }
    }
    #[doc = "Start building a high-priority queue.\n\n This function enters a special mode in which a high-priority queue is\n activated and can be filled with commands. After this function has been\n called, all commands will be put in the high-priority queue, until\n `rspq_highpri_end` is called.\n\n The RSP will start processing the high-priority queue almost instantly\n (as soon as the current command is done), pausing the normal queue. This will\n also happen while the high-priority queue is being built, to achieve the\n lowest possible latency. When the RSP finishes processing the high priority\n queue (after `rspq_highpri_end` closes it), it resumes processing the normal\n queue from the exact point that was left.\n\n The goal of the high-priority queue is to either schedule latency-sensitive\n commands like audio processing, or to schedule immediate RSP calculations\n that should be performed right away, just like they were preempting what\n the RSP is currently doing.\n\n It is possible to create multiple high-priority queues by calling\n `rspq_highpri_begin` / `rspq_highpri_end` multiple times with short\n delays in-between. The RSP will process them in order. Notice that\n there is a overhead in doing so, so it might be advisable to keep\n the high-priority mode active for a longer period if possible. On the\n other hand, a shorter high-priority queue allows for the RSP to\n switch back to processing the normal queue before the next one\n is created.\n\n @note It is not possible to create a block while the high-priority queue is\n       active. Arrange for constructing blocks beforehand.\n\n @note It is currently not possible to call a block from the\n       high-priority queue. (FIXME: to be implemented)\n"]
    #[inline]
    pub fn highpri_begin(&mut self) -> Highpri<'_> {
        unsafe { rspq_highpri_begin() }
        Highpri(PhantomData)
    }
    #[doc = "Wait for the RSP to finish processing all high-priority queues.\n\n This function will spin-lock waiting for the RSP to finish processing\n all high-priority queues. It is meant for debugging purposes or for situations\n in which the high-priority queue is known to be very short and fast to run.\n Also note that it is not possible to create syncpoints in the high-priority queue."]
    #[inline]
    pub fn highpri_sync(&self) {
        unsafe { rspq_highpri_sync() }
    }
    #[doc = "Enqueue a no-op command in the queue.\n\n This function enqueues a command that does nothing. This is mostly\n useful for debugging purposes."]
    #[inline]
    pub fn noop(&self) {
        unsafe { rspq_noop() }
    }
    #[doc = "Enqueue a command to do a DMA transfer from DMEM to RDRAM\n\n @param      rdram_addr  The RDRAM address (destination, must be aligned to 8)\n @param[in]  dmem_addr   The DMEM address (source, must be aligned to 8)\n @param[in]  len         Number of bytes to transfer (must be multiple of 8)\n @param[in]  is_async    If true, the RSP does not wait for DMA completion\n                         and processes the next command as the DMA is in progress.\n                         If false, the RSP waits until the transfer is finished\n                         before processing the next command.\n\n @note The argument is_async refers to the RSP only. From the CPU standpoint,\n       this function is always asynchronous as it just adds a command\n       to the queue."]
    #[inline]
    pub unsafe fn dma_to_rdram<T>(&self, rdram: &mut [T], dmem_addr: u32, is_async: bool) {
        unsafe {
            rspq_dma_to_rdram(
                rdram.as_ptr() as _,
                dmem_addr,
                (rdram.len() * size_of::<T>()) as _,
                is_async,
            );
        }
    }
    #[doc = "Enqueue a command to do a DMA transfer from RDRAM to DMEM\n\n @param[in]  dmem_addr   The DMEM address (destination, must be aligned to 8)\n @param      rdram_addr  The RDRAM address (source, must be aligned to 8)\n @param[in]  len         Number of bytes to transfer (must be multiple of 8)\n @param[in]  is_async    If true, the RSP does not wait for DMA completion\n                         and processes the next command as the DMA is in progress.\n                         If false, the RSP waits until the transfer is finished\n                         before processing the next command.\n\n @note The argument is_async refers to the RSP only. From the CPU standpoint,\n       this function is always asynchronous as it just adds a command\n       to the queue."]
    #[inline]
    pub unsafe fn dma_to_dmem<T>(&self, dmem_addr: u32, rdram: &[T], is_async: bool) {
        unsafe {
            rspq_dma_to_dmem(
                dmem_addr,
                rdram.as_ptr() as _,
                (rdram.len() * size_of::<T>()) as _,
                is_async,
            );
        }
    }
}

impl Syncpoint {
    #[doc = "Check whether a syncpoint was reached by RSP or not.\n\n This function checks whether a syncpoint was reached. It never blocks.\n If you need to wait for a syncpoint to be reached, use `rspq_syncpoint_wait`\n instead of polling this function.\n\n @param[in]  sync_id  ID of the syncpoint to check\n\n @return true if the RSP has reached the syncpoint, false otherwise\n\n @see `rspq_syncpoint_t`"]
    #[inline]
    pub fn check(&self) -> bool {
        unsafe { rspq_syncpoint_check(self.0) }
    }
    #[doc = "Wait until a syncpoint is reached by RSP.\n\n This function blocks waiting for the RSP to reach the specified syncpoint.\n If the syncpoint was already called at the moment of call, the function\n exits immediately.\n\n @param[in]  sync_id  ID of the syncpoint to wait for\n\n @see `rspq_syncpoint_t`"]
    #[inline]
    pub fn wait(self) {
        unsafe { rspq_syncpoint_wait(self.0) }
    }
}

#[derive(Debug)]
pub struct Write<'s> {
    _rspq: PhantomData<&'s mut RspQ>,
    first_word: u32,
    pointer: *mut u32,
    first: *mut u32,
    is_first: bool,
}

impl<'s> crate::Undroppable for &mut Write<'s> {
    const ERROR: &'static str = "Finish the RSPQ write Write::end";
}

impl<'s> Drop for Write<'s> {
    #[inline]
    fn drop(&mut self) {
        let _ = crate::DropBomb::new(self);
    }
}

impl<'s> Write<'s> {
    #[doc = "Add one argument to the command being enqueued.\n\n This function adds one more argument to the command currently being\n enqueued. This function must be called after `rspq_write_begin`; it should\n be called multiple times (one per argument word), and then `rspq_write_end`\n should be called to terminate enqueuing the command.\n\n See also `rspq_write` for a more straightforward API for command enqueuing.\n\n @param       w       The write cursor (returned by `rspq_write_begin`)\n @param       value   New 32-bit argument word to add to the command.\n\n @note The first argument must have its MSB set to 0, to leave space for\n       the command ID. See `rspq_write` documentation for a more complete\n       explanation.\n\n @see `rspq_write_begin`\n @see `rspq_write_end`\n @see `rspq_write`"]
    #[inline]
    pub fn arg(&mut self, value: u32) {
        if self.is_first {
            self.first_word |= value;
            self.is_first = false;
        } else {
            unsafe {
                self.pointer.write_volatile(value);
                self.pointer = self.pointer.add(1);
            }
        }
    }
    #[doc = "Finish enqueuing a command into the queue.\n\n This function should be called to terminate a sequence for command\n enqueuing, after `rspq_write_begin` and (multiple) calls to `rspq_write_arg`.\n\n After calling this command, the write cursor cannot be used anymore.\n\n @param       w       The write cursor (returned by `rspq_write_begin`)\n\n @see `rspq_write_begin`\n @see `rspq_write_arg`\n @see `rspq_write`"]
    #[inline]
    pub fn end(self) {
        let write = core::mem::ManuallyDrop::new(self);
        unsafe {
            write.first.write_volatile(write.first_word);
        }
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Highpri<'s>(PhantomData<&'s mut RspQ>);

static_wrapper! { Highpri<'s> => RspQ { RspQ(()) } }

impl<'s> crate::Undroppable for &mut Highpri<'s> {
    const ERROR: &'static str = "Finish the high priority block with Highpri::end";
}

impl<'s> Drop for Highpri<'s> {
    #[inline]
    fn drop(&mut self) {
        let _ = crate::DropBomb::new(self);
    }
}

impl<'s> Highpri<'s> {
    #[doc = "Finish building the high-priority queue and close it.\n\n This function terminates and closes the high-priority queue. After this\n command is called, all following commands will be added to the normal queue.\n\n Notice that the RSP does not wait for this function to be called: it will\n start running the high-priority queue as soon as possible, even while it is\n being built."]
    #[inline]
    pub fn end(self) {
        let _ = core::mem::ManuallyDrop::new(self);
        unsafe { rspq_highpri_end() }
    }
}

impl Block {
    #[inline]
    pub const fn as_raw(&self) -> *mut rspq_block_t {
        self.0.as_ptr()
    }
    #[inline]
    pub const fn as_block_ref<'b>(&'b self) -> BlockRef<'b> {
        BlockRef(self.0, PhantomData)
    }
}

impl Drop for Block {
    #[doc = "Free a block that is not needed any more.\n\n After calling this function, the block is invalid and must not be called\n anymore. Notice that a block that was recently run via `rspq_block_run`\n might still be referenced in the RSP queue, and in that case it is invalid\n to free it before the RSP has processed it.\n\n In this case, you must free it once you are absolutely sure that the RSP\n has processed it (eg: at the end of a frame), or use `rspq_call_deferred`\n or `rdpq_call_deferred`, that handle the synchronization for you.\n\n @param  block  The block\n\n @note If the block was being called by other blocks, these other blocks\n       become invalid and will make the RSP crash if called. Make sure\n       that freeing a block is only done when no other blocks reference it."]
    #[inline]
    fn drop(&mut self) {
        unsafe { rspq_block_free(self.as_raw()) }
    }
}

impl<'b> BlockRef<'b> {
    #[inline]
    pub const unsafe fn from_raw(block: *mut rspq_block_t) -> Option<Self> {
        match NonNull::new(block) {
            Some(block) => Some(Self(block, PhantomData)),
            None => None,
        }
    }
}

impl<'b> From<&'b Block> for BlockRef<'b> {
    #[inline]
    fn from(value: &'b Block) -> Self {
        value.as_block_ref()
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct BlockBuilder<'s>(PhantomData<&'s mut RspQ>);

impl<'s> crate::Undroppable for &mut BlockBuilder<'s> {
    const ERROR: &'static str = "Finish the block with BlockBuilder::end";
}

impl<'s> Drop for BlockBuilder<'s> {
    #[inline]
    fn drop(&mut self) {
        let _ = crate::DropBomb::new(self);
    }
}

impl<'s> BlockBuilder<'s> {
    #[doc = "Finish creating a block.\n\n This function completes a block and returns a reference to it (see `rspq_block_t`).\n After this function is called, all subsequent `rspq_write`\n will resume working as usual: they will add commands to the queue\n for immediate RSP execution.\n\n To run the created block, use `rspq_block_run`.\n\n @return A reference to the just created block\n\n @see rspq_block_begin\n @see rspq_block_run"]
    pub fn end(self) -> Block {
        let _ = core::mem::ManuallyDrop::new(self);
        Block(NonNull::new(unsafe { rspq_block_end() }).unwrap())
    }
}

static_wrapper! { BlockBuilder<'s> => RspQ { RspQ(()) } }
