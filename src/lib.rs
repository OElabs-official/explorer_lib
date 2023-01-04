use 
{
    std::
    {
        error::Error,
        path::{PathBuf,Path,},
        fs,fmt,
    },
};
mod core;
mod base_funcs;
pub mod apis;

pub const EXT_PHOTOS : &'static[&'static str]  = &["jpg","bmp","heif","avif","tiff","png","jp2","webp","heic","gif","pnm","dds","tga","exr","ico"];
pub const EXT_VIDEOS : &'static[&'static str]  = &["avi","mkv","mp4","wmv","ts","rmvb"];
pub const EXT_AUDIOS : &'static[&'static str]  = &["mp3","aac","ape","flac"];
pub const EXT_DOCUMENTS : &'static[&'static str]  = &["xls","doc","ppt","odt","pdf"];
pub const EXT_COMPRESSED : &'static[&'static str]  = &["7z","rar","zip","tar","gz","xz"];
