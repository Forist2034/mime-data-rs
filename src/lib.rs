#![no_std]

/* wrap data in struct allows change type layout (e.g. use string table index)
    later.
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mime(&'static str);
impl Mime {
    pub fn as_str(self) -> &'static str {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Extension(&'static str);
impl Extension {
    pub fn as_str(self) -> &'static str {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Extensions(&'static [Extension]);
impl Extensions {
    pub fn as_slice(&self) -> &'static [Extension] {
        self.0
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct MimeData {
    pub mime: Mime,
    pub extensions: Extensions,
    pub compressible: Option<bool>,
}

#[derive(Debug)]
pub struct ExtensionInfo(&'static [&'static MimeData]);
impl ExtensionInfo {
    pub fn mime_data(&self) -> &'static [&'static MimeData] {
        self.0
    }
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
    ty_map::TY_MAP.get(m).copied()
}
pub fn lookup_extension_info(ext: &'static str) -> Option<ExtensionInfo> {
    ext_map::EXT_MAP.get(ext).map(|m| ExtensionInfo(m))
}
