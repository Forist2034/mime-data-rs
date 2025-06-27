#![no_std]

#[derive(Debug)]
#[non_exhaustive]
pub struct MimeData {
    pub mime: &'static str,
    pub extensions: &'static [&'static str],
    pub compressible: Option<bool>,
}

mod data {
    include!(concat!(env!("OUT_DIR"), "/data.rs"));
}
mod ty_map {
    include!(concat!(env!("OUT_DIR"), "/ty_map.rs"));
}
mod ext_map {
    include!(concat!(env!("OUT_DIR"), "/ext_map.rs"));
}

pub fn lookup_mime_info(m: &'static str) -> Option<&'static MimeData> {
    ty_map::TY_MAP.get(m).map(|m| *m)
}
pub fn lookup_extension_info(ext: &'static str) -> Option<&'static [&'static MimeData]> {
    ext_map::EXT_MAP.get(ext).map(|m| *m)
}
