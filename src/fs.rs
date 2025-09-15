use core::{
    ffi::CStr,
    num::niche_types::{I32NotAllOnes, U32NotAllOnes},
};

use crate::{sys::dragonfs::*, ucstr::UCStr};

#[doc = "DragonFS Return values"]
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Error {
    #[doc = "Input parameters invalid"]
    InvalidInput = DFS_EBADINPUT as _,
    #[doc = "File does not exist"]
    NotFound = DFS_ENOFILE as _,
    #[doc = "Bad filesystem"]
    InvalidFileSystem = DFS_EBADFS as _,
    #[doc = "Too many open files"]
    OutOfMemory = DFS_ENFILE as _,
    #[doc = "Invalid file handle"]
    InvalidHandle = DFS_EBADHANDLE as _,
    #[doc = "Unknown error"]
    Unknown = i32::MIN as _,
}

#[repr(transparent)]
#[derive(Debug)]
pub struct DfsFile(pub(crate) U32NotAllOnes);

impl DfsFile {
    #[doc = "Open a file given a path\n\n Check if we have any free file handles, and if we do, try\n to open the file specified.  Supports absolute and relative\n paths\n\n @param[in] path\n            Path of the file to open\n\n @return A valid file handle to reference the file by or a negative error on failure."]
    #[inline]
    pub fn open(filename: &CStr) -> crate::io::Result<Self> {
        use crate::sys::fcntl;
        unsafe {
            crate::io::Error::catch_negative(fcntl::open(filename.as_ptr(), fcntl::O_RDONLY as _))
                .map(|fd| Self(U32NotAllOnes::new_unchecked(fd as u32)))
        }
    }
    #[doc = "Seek to an offset in the file\n\n @param[in] handle\n            A valid file handle as returned from `dfs_open`.\n @param[in] offset\n            A byte offset from the origin to seek from.\n @param[in] origin\n            An offset to seek from.  Either `SEEK_SET`, `SEEK_CUR` or `SEEK_END`.\n\n @return DFS_ESUCCESS on success or a negative value on error."]
    #[inline]
    pub fn seek(&mut self, pos: embedded_io::SeekFrom) -> Result<()> {
        use crate::sys::unistd;
        let (pos, seek) = match pos {
            embedded_io::SeekFrom::Start(pos) => (<_>::try_from(pos), unistd::SEEK_SET),
            embedded_io::SeekFrom::End(pos) => (<_>::try_from(pos), unistd::SEEK_END),
            embedded_io::SeekFrom::Current(pos) => (<_>::try_from(pos), unistd::SEEK_CUR),
        };
        let pos = pos.map_err(|_| Error::InvalidInput)?;
        Error::catch(unsafe { dfs_seek(self.0.as_inner(), pos, seek as _) })
    }
    #[doc = "Return the current offset into a file\n\n @param[in] handle\n            A valid file handle as returned from `dfs_open`.\n\n @return The current byte offset into a file or a negative error on failure."]
    #[inline]
    pub fn tell(&self) -> u32 {
        unsafe { dfs_tell(self.0.as_inner()) as _ }
    }
    #[doc = "Return whether the end of file has been reached\n\n @param[in] handle\n            A valid file handle as returned from `dfs_open`.\n\n @return 1 if the end of file is reached, 0 if not, and a negative value on error."]
    #[inline]
    pub fn is_eof(&self) -> bool {
        unsafe { dfs_eof(self.0.as_inner()) != 0 }
    }
    #[doc = "Return the file size of an open file\n\n @param[in] handle\n            A valid file handle as returned from `dfs_open`.\n\n @return The file size in bytes or a negative value on failure."]
    #[inline]
    pub fn size(&self) -> u32 {
        unsafe { dfs_size(self.0.as_inner()) as _ }
    }
}

impl Drop for DfsFile {
    #[doc = "Close an already open file handle.\n\n @param[in] handle\n            A valid file handle as returned from `dfs_open`.\n\n @return DFS_ESUCCESS on success or a negative value on error."]
    fn drop(&mut self) {
        unsafe { dfs_close(self.0.as_inner()) };
    }
}

impl embedded_io::ErrorType for DfsFile {
    type Error = Error;
}

impl embedded_io::Read for DfsFile {
    #[doc = "Read data from a file\n\n Note that no caching is performed: if you need to read small amounts\n (eg: one byte at a time), consider using standard C API instead (fopen())\n which performs internal buffering to avoid too much overhead.\n\n @param[out] buf\n             Buffer to read into\n @param[in]  size\n             Size of each element to read\n @param[in]  count\n             Number of elements to read\n @param[in]  handle\n             A valid file handle as returned from `dfs_open`.\n\n @return The actual number of bytes read or a negative value on failure."]
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let len = buf.len().try_into().map_err(|_| Error::InvalidInput)?;
        Error::catch_negative(unsafe { dfs_read(buf.as_mut_ptr() as _, 1, len, self.0.as_inner()) })
            .map(|i| i as usize)
    }
}

impl embedded_io::Seek for DfsFile {
    #[inline]
    fn seek(&mut self, pos: embedded_io::SeekFrom) -> Result<u64> {
        self.seek(pos)?;
        self.stream_position()
    }
    #[inline]
    fn rewind(&mut self) -> Result<()> {
        Error::catch(unsafe { dfs_seek(self.0.as_inner(), 0, crate::sys::unistd::SEEK_SET as _) })
    }
    #[inline]
    fn stream_position(&mut self) -> Result<u64> {
        Ok(self.tell() as _)
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct File(pub(crate) I32NotAllOnes);

impl File {
    #[inline]
    pub fn try_open(filename: &CStr) -> crate::io::Result<Self> {
        use crate::sys::fcntl;
        unsafe {
            crate::io::Error::catch_negative(fcntl::open(filename.as_ptr(), fcntl::O_RDONLY as _))
                .map(|fd| Self(I32NotAllOnes::new_unchecked(fd)))
        }
    }
    #[inline]
    pub fn open(filename: &CStr) -> Self {
        use embedded_io::{Error, ErrorKind};
        match Self::try_open(filename) {
            Ok(file) => file,
            Err(error) => {
                if cfg!(feature = "debug") {
                    if error.kind() == ErrorKind::InvalidInput {
                        let filename = filename.to_bytes();
                        if !filename
                            .iter()
                            .position(|&f| f == b':')
                            .map(|i| !filename[i..].starts_with(b":/"))
                            .unwrap_or(false)
                        {
                            panic!(
                                "File not found: {filename:?}\nDid you forget the filesystem prefix? (e.g. \"rom:/\")"
                            );
                        }
                        if filename.starts_with(b"rom:/") {
                            panic!(
                                "File not found: {filename:?}\nDid you forget to call fs::init(), or did it return an error?"
                            );
                        }
                    }
                    panic!("error opening file: {filename:?}: {error}");
                } else {
                    panic!();
                }
            }
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { crate::sys::unistd::close(self.0.as_inner()) };
    }
}

impl embedded_io::ErrorType for File {
    type Error = crate::io::Error;
}

impl embedded_io::Read for File {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> crate::io::Result<usize> {
        crate::io::Error::catch_negative(unsafe {
            crate::sys::unistd::read(self.0.as_inner(), buf.as_mut_ptr() as _, buf.len())
        })
        .map(|i| i as usize)
    }
}

impl embedded_io::Seek for File {
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
            unistd::lseek(self.0.as_inner(), pos, seek as _)
        })
        .map(|i| i as u64)
    }
}

#[doc = "Directory entry type"]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum FileType {
    File = crate::sys::dir::DT_REG as _,
    Dir = crate::sys::dir::DT_DIR as _,
}

#[doc = "Directory entry structure"]
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Dir(pub(crate) crate::sys::dir::dir_t);

impl Dir {
    #[doc = "The name of the directory entry (relative to the directory path)"]
    #[inline]
    pub const fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0.d_name.as_ptr()) }
    }
    #[doc = "The type of the directory entry.  See `DT_REG` and `DT_DIR`."]
    #[inline]
    pub const fn file_type(&self) -> FileType {
        unsafe { core::mem::transmute(self.0.d_type) }
    }
    #[doc = "Size of the file.\n\n This value is well defined for files. For directories, the value\n is filesystem-dependent.\n\n If negative, the filesystem does not report the size during directory\n walking."]
    #[inline]
    pub const fn size(&self) -> Option<u64> {
        if self.0.d_size >= 0 {
            Some(self.0.d_size as u64)
        } else {
            None
        }
    }
}

#[doc = "Supported return values for directory walking"]
#[repr(i32)]
pub enum Walk {
    #[doc = "Abort walking and exit immediately"]
    Abort = crate::sys::dir::DIR_WALK_ABORT as _,
    #[doc = "Error walking the directory (errno will be set)"]
    Error = crate::sys::dir::DIR_WALK_ERROR as _,
    #[doc = "Continue walking"]
    Continue = crate::sys::dir::DIR_WALK_CONTINUE as _,
    #[doc = "Do not recurse within the current directory"]
    SkipDir = crate::sys::dir::DIR_WALK_SKIPDIR as _,
    #[doc = "Stop walking the current directory, return up one level"]
    Up = crate::sys::dir::DIR_WALK_GOUP as _,
}

pub type Result<T> = core::result::Result<T, Error>;

#[doc = "Initialize the filesystem.\n\n This function will initialize the filesystem to read from cartridge space.  This function will\n also register DragonFS with newlib so that standard POSIX/C file operations\n work with DragonFS, using the `rom:/` prefix\". It will search the DFS image by itself, using the rompak TOC (see\n rompak_internal.h). Most users should use this function.\n\n @return DFS_ESUCCESS on success or a negative error otherwise."]
#[inline]
pub fn init() -> Result<()> {
    init_at(DFS_DEFAULT_LOCATION)
}

#[doc = "Initialize the filesystem at an offset.\n\n Given a base offset where the filesystem should be found, this function will\n initialize the filesystem to read from cartridge space.  This function will\n also register DragonFS with newlib so that standard POSIX/C file operations\n work with DragonFS, using the \"rom:/\" prefix\".\n\n This should be used if the ROM cannot be built with a rompak TOC for some reason. The function needs to know where the DFS image is located within the cartridge\n space. A virtual address should be passed. This is normally 0xB0000000 + the offset\n used when building your ROM + the size of the header file used (typically 0x1000).\n\n @param[in] base_fs_loc\n            Virtual address in cartridge space at which to find the filesystem.\n\n @return DFS_ESUCCESS on success or a negative error otherwise."]
#[inline]
pub fn init_at(base_fs_loc: u32) -> Result<()> {
    Error::catch(unsafe { dfs_init(base_fs_loc) })
}

#[doc = "Return the physical address of a file (in ROM space)\n\n This function should be used for highly-specialized, high-performance\n use cases. Using dfs_open / dfs_read is generally acceptable\n performance-wise, and is easier to use rather than managing\n direct access to PI space.\n\n Direct access to ROM data must go through io_read or dma_read. Do not\n dereference directly as the console might hang if the PI is busy.\n\n @param[in] path\n            Name of the file\n\n @return A pointer to the physical address of the file body, or 0\n         if the file was not found.\n\n @see [`rom_size`]"]
#[inline]
pub fn rom_addr(path: &CStr) -> u32 {
    unsafe { dfs_rom_addr(path.as_ptr()) }
}

#[doc = "Return the size of a file (in ROM data)\n\n Returns the size of a file without opening it. Can be used in conjunction\n with dfs_rom_addr to perform DMA without without calling dfs_open.\n\n @param[in] path\n            Name of the file\n\n @return The size of a file in ROM, or DFS_ENOFILE if the file was not found.\n\n @see `dfs_rom_addr`\n"]
#[inline]
pub fn rom_size(path: &CStr) -> u32 {
    unsafe { dfs_rom_size(path.as_ptr()) as _ }
}

unsafe extern "C" fn walk_trampoline<F: FnMut(&CStr, &Dir) -> Walk>(
    fn_: *const ::core::ffi::c_char,
    dir: *mut crate::sys::dir::dir_t,
    data: *mut ::core::ffi::c_void,
) -> i32 {
    unsafe {
        let func = &mut *(data as *mut F);
        let ret = func(CStr::from_ptr(fn_), core::mem::transmute(&mut *dir));
        if let Walk::Error = ret {
            *crate::sys::stdio::__errno() = 1;
        }
        ret as _
    }
}

#[doc = "Walk a directory tree\n\n This function walks a directory tree, calling the callback for each file and directory found.\n\n The callback is of type `dir_walk_callback_t`, and its return value determines the behavior\n of the walk. In fact, the callback can request to abort the walk (`DIR_WALK_ABORT`), skip the\n current directory (`DIR_WALK_SKIPDIR`), or stop walking the current directory and return up one\n level (`DIR_WALK_GOUP`). See `dir_walk_callback_t` for more information.\n\n @param path      The path to the directory to walk\n @param cb        The callback function to call for each file and directory\n @param data      User data to pass to the callback\n @return 0        on success\n @return -1       on error (errno will be set)\n @return -2       if abort was requested by the callback via DIR_WALK_ABORT"]
#[inline]
pub fn walk<F: FnMut(&CStr, &Dir) -> Walk>(path: &CStr, mut func: F) -> Result<bool> {
    unsafe {
        Error::catch_dir(crate::sys::dir::dir_walk(
            path.as_ptr(),
            Some(walk_trampoline::<F>),
            (&mut func) as *mut F as _,
        ))
    }
}

#[doc = "Check if a filename matches a pattern\n\n This function is a simplified version of fnmatch that only supports the following\n special characters:\n\n  * `?` - Matches any single character\n  * `*` - Matches any sequence of characters, except '/'. It can be used to match\n         multiple files in a single directory.\n  * `**` - Matches any sequence of characters, including '/'. It can be used to match\n         files within directory trees\n\n Example of patterns:\n\n @code\n   *.txt              - Matches all files with a .txt extension\n   **/*.txt           - Matches all files with a .txt extension in all directories\n                        under the starting directory\n   hero/**/*.sprite   - Matches all files with a .sprite extension in the\n                        hero directory and all its subdirectories\n   catalog?.dat       - Matches catalog1.dat, catalog2.dat, etc.\n   *w*/*.txt          - Matches all files with a .txt extension in directories\n                        that contain the letter 'w'\n @endcode\n\n @param pattern       The pattern to match against\n @param fullpath      The full path to match\n @return true         The filename matches the pattern\n @return false        The filename does not match the pattern\n"]
#[inline]
pub fn fnmatch(pattern: &CStr, fullpath: &CStr) -> bool {
    unsafe { crate::sys::dir::dir_fnmatch(pattern.as_ptr(), fullpath.as_ptr()) }
}

#[doc = "Glob a directory tree using a pattern\n\n This function walks a directory tree searching for files and directories that match a pattern.\n The pattern is a simplified version of fnmatch; see `dir_fnmatch` for more information\n about the supported special characters.\n\n The callback function is called for each file and directory that matches the pattern. The callback\n can then decide how to proceed using its return value (see `dir_walk_callback_t` for more information).\n\n @code{.c}\n   int mycb(const char *fn, dir_t *dir, void *data) {\n      debugf(\"Found sprite file: %s\\n\", fn);\n      return DIR_WALK_CONTINUE;\n   }\n\n   // Search for all files with a .sprite extension in all directories under rom:/\n   dir_glob(\"**/*.sprite\", \"rom:/\", mycb, NULL);\n @endcode\n\n @note the glob pattern is matched against pathnames **relative** to the starting directory.\n       For example, if you start the search at \"rom:/sprites\", the pattern `hero/*.sprite`\n       will match all files with a .sprite extension in the \"rom:/sprites/hero\" directory.\n\n @param pattern       The pattern to match against (see `dir_fnmatch`)\n @param path          The path to the directory to start the search\n @param cb            The callback function to call for each file and directory\n @param data          User data to pass to the callback\n @return 0 on success,\n @return -1 on error (errno will be set)\n @return -2 if abort was requested by the callback via DIR_WALK_ABORT\n"]
#[inline]
pub fn glob<F: FnMut(&CStr, &Dir) -> Walk>(
    pattern: &CStr,
    path: &CStr,
    mut func: F,
) -> Result<bool> {
    unsafe {
        Error::catch_dir(crate::sys::dir::dir_glob(
            pattern.as_ptr(),
            path.as_ptr(),
            Some(walk_trampoline::<F>),
            (&mut func) as *mut F as _,
        ))
    }
}

impl Error {
    #[doc = "Convert DFS error code into an error string"]
    #[inline]
    pub fn name(self) -> &'static UCStr {
        unsafe { UCStr::from_ptr(dfs_strerror(self as _)) }
    }
    #[inline]
    pub(crate) fn catch(ret: i32) -> Result<()> {
        if ret != DFS_ESUCCESS as _ {
            if ret < DFS_EBADHANDLE || ret > 0 {
                Err(Error::Unknown)
            } else {
                Err(unsafe { core::mem::transmute(ret) })
            }
        } else {
            Ok(())
        }
    }
    #[inline]
    pub(crate) fn catch_negative(ret: i32) -> Result<i32> {
        if ret != DFS_ESUCCESS as _ {
            if ret < DFS_EBADHANDLE {
                Err(Error::Unknown)
            } else {
                Err(unsafe { core::mem::transmute(ret) })
            }
        } else {
            Ok(ret)
        }
    }
    #[inline]
    pub(crate) fn catch_dir(ret: i32) -> Result<bool> {
        match ret {
            0 => Ok(true),
            -1 => Self::catch(-unsafe { *crate::sys::stdio::__errno() }).map(|_| true),
            _ => Ok(false),
        }
    }
}

impl embedded_io::Error for Error {
    fn kind(&self) -> embedded_io::ErrorKind {
        use embedded_io::ErrorKind::*;
        match self {
            Self::InvalidInput => Interrupted,
            Self::NotFound => NotFound,
            Self::InvalidFileSystem => AddrNotAvailable,
            Self::OutOfMemory => OutOfMemory,
            Self::InvalidHandle => InvalidData,
            Self::Unknown => Other,
        }
    }
}

impl core::fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.name().fmt(f)
    }
}
