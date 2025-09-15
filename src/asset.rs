use core::{ffi::CStr, ptr::NonNull};

use alloc_::boxed::Box;

use crate::sys::asset::*;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Compression {
    Level2,
    Level3,
}

#[doc = "Enable a non-default compression level\n\nThis function must be called if any asset that will be loaded use a non-default compression level. The default compression level is 1, for which no initialization is required.\n\nCurrently, level 2 and 3 requires initialization. If you have any assets compressed with level 2 or 3, you must call this function before loading them."]
#[inline]
pub fn init_compression(level: Compression) {
    match level {
        Compression::Level2 => unsafe { __asset_init_compression_lvl2() },
        Compression::Level3 => unsafe { __asset_init_compression_lvl3() },
    }
}

#[doc = "Load an asset file (possibly uncompressing it)\n\n This function loads a file from a file system (eg: from ROM or SD).\n If the file was compressed using the mkasset tool, it will be\n automatically uncompressed.\n\n @param fn        Filename to load (including filesystem prefix, eg: \"rom:/foo.dat\")\n @param sz        If not NULL, this will be filed with the uncompressed size of the loaded file\n @return void*    Pointer to the loaded file (must be freed with free() when done)"]
#[inline]
pub fn load(filename: &CStr) -> Box<[u8]> {
    unsafe {
        let mut size = core::mem::MaybeUninit::uninit();
        let asset = asset_load(filename.as_ptr(), size.as_mut_ptr());
        assert!(!asset.is_null());
        Box::from_raw(core::slice::from_raw_parts_mut(
            asset as _,
            size.assume_init() as usize,
        ))
    }
}

pub trait AssetExt: crate::sealed::Sealed {
    #[doc = "Load an asset file (possibly uncompressing it)\n\n This function loads an asset embedded within a larger file. It requires in\n input an open file pointer, seeked to the beginning of the asset, and the\n size of the asset itself. If the asset is compressed, it is transparently\n decompressed.\n\n After this function returns, for technical reasons, the position of the\n provided file pointer becomes undefined. If you need to use it again, make\n sure to seek it.\n\n A memory buffer to hold the uncompressed asset is automatically allocated\n and returned. It must be freed using free() when the buffer is not required\n anymore. The memory is guaranteed to be aligned by at least `ASSET_ALIGNMENT_MIN`.\n\n @param f         pre-seeked file pointer, pointing to a valid asset header (or\n                  actual data if uncompressed)\n @param sz        size of input data (compressed or not). It will be filled\n                  the uncompressed asset size, which is equal to the input value if the\n                  asset is not compressed.\n @return void*    Allocated buffer filled with the uncompressed asset content"]
    fn load_asset(&mut self) -> Box<[u8]>;
    #[doc = "Load an asset file (possibly uncompressing it)\n\n This is the lowest-level asset loading function, that is\n needed only for advanced use cases. In general, prefer using\n any of other variants if possible, as the other APIs are\n harder to misuse.\n\n This function loads an asset potentially embedded within a\n larger, opened file. It requires an open file pointer, seeked\n to the beginning of the asset, and the size of the asset itself.\n If the asset is compressed, it is transparently decompressed.\n\n After this function returns, for technical reasons, the position\n of the provided file pointer becomes undefined. If you need to\n use it again, make sure to seek it.\n\n The memory buffer to hold the uncompressed asset must be provided as\n input, together with its size. If the provided buffer is too small (or\n it is NULL), the function does not load the asset and returns false,\n change buf_size to contain the minimum required size for the buffer.\n Notice that the minimum buffer size might be slightly larger than\n the uncompressed asset size, because some extra space might be required\n to perform in-place decompression. The minimum buffer size can either\n be calculated a build time (the assetcomp library exposes a function to\n do so), or queried at runtime by simply calling this function with a NULL\n input buffer.\n\n @param f         pre-seeked file pointer, pointing to a valid asset header\n                  (or actual data if uncompressed)\n @param sz        [in/out]: size of input data (compressed or not). It will\n                  be filled the uncompressed asset size, which is equal to\n                  the input value if the asset is not compressed.\n @param buf       Pointer to the buffer where data must be loaded into.\n                  If the buffer pointer is NULL, or it is too small,\n                  asset_loadf_into will fail.\n @param buf_size  [in/out]: Size of the provided input buffer. Changed to\n                  minimum required size, if it was too small.\n\n @return true The function has succeeded and the asset was loaded\n @return false The function has failed because the provided buffer was too small.\n               In this case, *buf_size is changed to contain the minimum size\n               that is required to load this asset."]
    fn load_asset_into(&mut self, buf: &mut [u8]) -> Result<u32, u32>;
}

impl AssetExt for crate::fs::File {
    fn load_asset(&mut self) -> Box<[u8]> {
        let mut size = core::mem::MaybeUninit::uninit();
        let asset = unsafe { asset_loadfd(self.0.as_inner(), size.as_mut_ptr()) };
        assert!(!asset.is_null());
        unsafe {
            Box::from_raw(core::slice::from_raw_parts_mut(
                asset as _,
                size.assume_init() as usize,
            ))
        }
    }
    fn load_asset_into(&mut self, buf: &mut [u8]) -> Result<u32, u32> {
        let mut size = core::mem::MaybeUninit::uninit();
        let mut buf_size = buf.len().min(i32::MAX as usize) as i32;
        let res = unsafe {
            asset_loadfd_into(
                self.0.as_inner(),
                size.as_mut_ptr(),
                buf.as_mut_ptr() as _,
                &mut buf_size,
            )
        };
        if !res {
            Err(buf_size as u32)
        } else {
            Ok(unsafe { size.assume_init() as u32 })
        }
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Asset(pub(crate) NonNull<crate::sys::stdio::FILE>);

impl Asset {
    #[doc = "Open an asset file for reading (with transparent decompression)\n\n This function opens a file from a file system (eg: from ROM or SD).\n If the file was compressed using the mkasset tool, it will be\n automatically uncompressed as it is being read.\n\n Note that since the file might be compressed, the returned\n FILE* cannot be arbitrarily seeked backward, as that would be impossible\n to do efficiently on a compressed file. Seeking forward is supported and is\n simulated by reading (decompressing) and discarding data. You can rewind\n the file to the start though, (by using either fseek or rewind).\n\n This behavior of the returned file is enforced also for non compressed\n assets, so that the code is ready to switch to compressed assets if\n required. If you need random access to an uncompressed file, simply use\n the standard fopen() function.\n\n @param fn        Filename to load (including filesystem prefix, eg: \"rom:/foo.dat\")\n @param sz        If not NULL, this will be filed with the uncompressed size of the loaded file\n @return FILE*    FILE pointer to use with standard C functions (fread, fclose)"]
    pub fn open(filename: &CStr) -> Self {
        let file = crate::fs::File::open(filename);
        let file = unsafe { asset_fdopen(file.0.as_inner(), core::ptr::null_mut()) };
        let Some(file) = NonNull::new(file) else {
            panic!("Asset loading failed: {filename:?}");
        };
        Self(file)
    }
    pub fn open_with_size(filename: &CStr) -> (Self, u32) {
        let file = crate::fs::File::open(filename);
        let mut size = core::mem::MaybeUninit::uninit();
        let file = unsafe { asset_fdopen(file.0.as_inner(), size.as_mut_ptr()) };
        let Some(file) = NonNull::new(file) else {
            panic!("Asset loading failed: {filename:?}");
        };
        (Self(file), unsafe { size.assume_init() as u32 })
    }
}

impl Drop for Asset {
    fn drop(&mut self) {
        unsafe { crate::sys::stdio::fclose(self.0.as_ptr()) };
    }
}

impl embedded_io::ErrorType for Asset {
    type Error = crate::io::Error;
}

impl embedded_io::Read for Asset {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> crate::io::Result<usize> {
        let len = buf
            .len()
            .try_into()
            .map_err(|_| crate::io::Error::new(embedded_io::ErrorKind::InvalidInput))?;
        let ret =
            unsafe { crate::sys::stdio::fread(buf.as_mut_ptr() as _, 1, len, self.0.as_ptr()) };
        if ret == 0 {
            crate::io::Error::catch(unsafe { crate::sys::stdio::ferror(self.0.as_ptr()) })?;
        }
        Ok(ret as usize)
    }
}

impl embedded_io::Seek for Asset {
    #[inline]
    fn seek(&mut self, pos: embedded_io::SeekFrom) -> crate::io::Result<u64> {
        use crate::sys::unistd;
        let (pos, seek) = match pos {
            embedded_io::SeekFrom::Start(pos) => (<_>::try_from(pos), unistd::SEEK_SET),
            embedded_io::SeekFrom::End(pos) => (<_>::try_from(pos), unistd::SEEK_END),
            embedded_io::SeekFrom::Current(pos) => (<_>::try_from(pos), unistd::SEEK_CUR),
        };
        let pos = pos.map_err(|_| crate::io::Error::new(embedded_io::ErrorKind::InvalidInput))?;
        crate::io::Error::catch_negative(unsafe {
            crate::sys::stdio::fseek(self.0.as_ptr(), pos, seek as _)
        })
        .map(|i| i as u64)
    }
    #[inline]
    fn stream_position(&mut self) -> crate::io::Result<u64> {
        crate::io::Error::catch_negative(unsafe { crate::sys::stdio::ftell(self.0.as_ptr()) })
            .map(|i| i as u64)
    }
}
