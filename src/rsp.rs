use crate::sys::rsp::*;

#[doc = "RSP ucode definition.\n\n This small structure holds the text/data pointers to a RSP ucode program\n in RDRAM. It also contains the name (for the debugging purposes) and\n the initial PC (usually 0).\n\n If you're using libdragon's build system (n64.mk), use DEFINE_RSP_UCODE()\n to initialize one of these."]
#[repr(transparent)]
#[derive(Debug)]
pub struct UCode(#[doc(hidden)] pub rsp_ucode_t);

impl UCode {
    #[doc(hidden)]
    pub const EMPTY: Self = UCode(rsp_ucode_t {
        code: core::ptr::null_mut(),
        code_end: core::ptr::null_mut(),
        data: core::ptr::null_mut(),
        data_end: core::ptr::null_mut(),
        meta: core::ptr::null_mut(),
        meta_end: core::ptr::null_mut(),
        name: core::ptr::null_mut(),
        start_pc: 0,
        crash_handler: None,
        assert_handler: None,
    });
}

#[macro_export]
macro_rules! ucode {
    ($ident:ident $(, $($args:tt)* $(,)?)?) => { $crate::paste! {
        const {
            unsafe extern "C" {
                static [<$ident _text_start>]: ::core::primitive::u8;
                static [<$ident _data_start>]: ::core::primitive::u8;
                static [<$ident _meta_start>]: ::core::primitive::u8;
                static [<$ident _text_end>]: ();
                static [<$ident _data_end>]: ();
                static [<$ident _meta_end>]: ();
            }
            unsafe {
                $crate::rsp::UCode(rsp_ucode_t {
                    code: &[<$ident _text_start>] as *const _ as _,
                    code_end: &[<$ident _text_end>] as *const _ as _,
                    data: &[<$ident _data_start>] as *const _ as _,
                    data_end: &[<$ident _data_end>] as *const _ as _,
                    meta: &[<$ident _meta_start>] as *const _ as _,
                    meta_end: &[<$ident _meta_end>] as *const _ as _,
                    name: $crate::cstr!($ident).as_ptr()
                    $(, $($args)*)?
                    , ..$crate::rsp::UCode::EMPTY.0
                })
            }
        }
    } };
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Rsp(());

#[doc = "Initialize the RSP subsytem."]
#[inline]
pub fn init() -> Rsp {
    unsafe {
        rsp_init();
        get()
    }
}

#[inline]
pub unsafe fn get() -> Rsp {
    Rsp(())
}

impl Rsp {
    #[doc = "Load a RSP ucode.\n\n This function allows to load a RSP ucode into the RSP internal memory.\n The function executes the transfer right away, so it is responsibility\n of the caller making sure that it's a good time to do it.\n\n The function internally keeps a pointer to the last loaded ucode. If the\n ucode passed is the same, it does nothing. This makes it easier to write\n code that optimistically switches between different ucodes, but without\n forcing transfers every time.\n\n @param[in]     ucode       Ucode to load into RSP"]
    #[inline]
    pub unsafe fn load(ucode: &UCode) {
        unsafe { rsp_load(&ucode.0 as *const _ as _) }
    }
    #[doc = "Run RSP ucode.\n\n This function starts running the RSP, and wait until the ucode is finished."]
    #[inline]
    pub fn run(&self) {
        unsafe { rsp_run() }
    }
    #[doc = "Run RSP async.\n\n This function starts running the RSP in background. Use [`wait`] to\n synchronize later."]
    #[inline]
    pub fn run_async(&self) {
        unsafe { __rsp_run_async(SP_WSTATUS_SET_INTR_BREAK) }
    }
    #[doc = "Wait until RSP has finished processing.\n\n This function will wait until the RSP is halted. It contains a fixed\n timeout of 500 ms, after which `rsp_crash` is invoked to abort the program."]
    #[inline]
    pub fn wait(&self) {
        unsafe { rsp_wait() }
    }
    #[doc = "Do a DMA transfer to load a piece of code into RSP IMEM.\n\n This is a lower-level function that actually executes a DMA transfer\n from RDRAM to IMEM. Prefer using `rsp_load` instead.\n\n @note in order for this function to be interoperable with `rsp_load`, it\n will reset the last loaded ucode cache.\n\n @param[in]     code          Pointer to buffer in RDRAM containing code.\n                              Must be aligned to 8 bytes.\n @param[in]     size          Size of the code to load. Must be a multiple of 8.\n @param[in]     imem_offset   Byte offset in IMEM where to load the code.\n                              Must be a multiple of 8."]
    #[inline]
    pub unsafe fn load_code(&self, code: &[u32], imem_offset: u32) {
        unsafe {
            rsp_load_code(
                code.as_ptr() as _,
                (code.len() * size_of::<u32>()) as _,
                imem_offset,
            )
        }
    }
    #[doc = "Do a DMA transfer to load a piece of data into RSP DMEM.\n\n This is a lower-level function that actually executes a DMA transfer\n from RDRAM to DMEM. Prefer using rsp_load instead.\n\n @param[in]     data          Pointer to buffer in RDRAM containing data.\n                              Must be aligned to 8 bytes.\n @param[in]     size          Size of the data to load. Must be a multiple of 8.\n @param[in]     dmem_offset   Offset in DMEM where to load the code.\n                              Must be a multiple of 8."]
    #[inline]
    pub unsafe fn load_data<T>(&self, data: &[T], dmem_offset: u32) {
        unsafe {
            rsp_load_data(
                data.as_ptr() as _,
                (data.len() * size_of::<T>()) as _,
                dmem_offset,
            )
        }
    }
    #[doc = "Do a DMA transfer to load a piece of code from RSP IMEM to RDRAM.\n\n This is a lower-level function that actually executes a DMA transfer\n from IMEM to RDRAM.\n\n @param[in]     code          Pointer to buffer in RDRAM where to write code.\n                              Must be aligned to 8 bytes.\n @param[in]     size          Size of the code to load. Must be a multiple of 8.\n @param[in]     imem_offset   Byte offset in IMEM where where the code will\n                              be loaded from. Must be a multiple of 8."]
    #[inline]
    pub unsafe fn read_code(&self, code: &mut [u32], imem_offset: u32) {
        unsafe {
            rsp_read_code(
                code.as_mut_ptr() as _,
                (code.len() * size_of::<u32>()) as _,
                imem_offset,
            )
        }
    }
    #[doc = "Do a DMA transfer to load a piece of data from RSP DMEM to RDRAM.\n\n This is a lower-level function that actually executes a DMA transfer\n from DMEM to RDRAM.\n\n @param[in]     data          Pointer to buffer in RDRAM where to write data.\n                              Must be aligned to 8 bytes.\n @param[in]     size          Size of the data to load. Must be a multiple of 8.\n @param[in]     dmem_offset   Byte offset in IMEM where where the data will\n                              be loaded from. Must be a multiple of 8."]
    #[inline]
    pub unsafe fn read_data<T>(&self, data: &mut [T], dmem_offset: u32) {
        unsafe {
            rsp_read_code(
                data.as_mut_ptr() as _,
                (data.len() * size_of::<T>()) as _,
                dmem_offset,
            )
        }
    }
}
