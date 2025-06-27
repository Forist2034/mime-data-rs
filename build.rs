use std::{collections::HashMap, fmt::Write, path::PathBuf};

#[derive(serde::Deserialize)]
struct Data<'a> {
    #[serde(default)]
    #[serde(borrow)]
    extensions: Vec<&'a str>,
    #[serde(default)]
    compressible: Option<bool>,
}
struct VecMap<'a>(Vec<(&'a str, Data<'a>)>);
impl<'a> serde::Deserialize<'a> for VecMap<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        struct Visitor;
        impl<'a> serde::de::Visitor<'a> for Visitor {
            type Value = VecMap<'a>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("map")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'a>,
            {
                let mut ret = match map.size_hint() {
                    Some(s) => Vec::with_capacity(s),
                    None => Vec::new(),
                };
                while let Some(kv) = map.next_entry()? {
                    ret.push(kv);
                }
                Ok(VecMap(ret))
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}

struct RefArray<'a>(&'a [&'a str]);
impl<'a> std::fmt::Debug for RefArray<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('&')?;
        std::fmt::Debug::fmt(self.0, f)
    }
}
#[derive(Debug)]
struct MimeData<'a> {
    mime: &'a str,
    extensions: RefArray<'a>,
    compressible: Option<bool>,
}

macro_rules! data_var_name {
    () => {
        "DATA"
    };
}
const DATA_PATH: &str = "./data/db.json";
const DATA_VAR: &str = concat!("crate::data::", data_var_name!());

fn main() {
    let data_bin = std::fs::read(DATA_PATH).unwrap();
    println!("cargo::rerun-if-changed={DATA_PATH}");
    let data: VecMap = serde_json::from_slice(&data_bin).unwrap();

    let data_array: Vec<_> = data
        .0
        .iter()
        .map(|(mime, info)| MimeData {
            mime,
            extensions: RefArray(&info.extensions),
            compressible: info.compressible,
        })
        .collect();
    let mut ty_map_gen = phf_codegen::Map::new();
    let mut ext_map: HashMap<&str, Vec<usize>> = std::collections::HashMap::new();

    for (idx, (mime, info)) in data.0.iter().enumerate() {
        ty_map_gen.entry(*mime, format!("&{DATA_VAR}[{idx}]"));
        for ext in info.extensions.iter() {
            ext_map.entry(ext).or_default().push(idx);
        }
    }

    let mut ext_map_gen = phf_codegen::Map::new();
    for (ext, idx) in ext_map {
        let mut val_str = String::new();
        val_str.push_str("&[");
        for i in idx {
            write!(val_str, "&{DATA_VAR}[{i}],").unwrap();
        }
        val_str.push(']');
        ext_map_gen.entry(ext, val_str);
    }

    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::write(
        out_dir.join("data.rs"),
        format!(
            "{};\n\npub(crate) static {}: [crate::MimeData; {}] = {data_array:#?};",
            "use crate::MimeData",
            data_var_name!(),
            data_array.len()
        ),
    )
    .unwrap();
    std::fs::write(
        out_dir.join("ty_map.rs"),
        format!(
            "{} = {};",
            concat!(
                "pub(crate) static TY_MAP: ",
                "phf::Map<&str, &crate::MimeData>",
            ),
            ty_map_gen.build()
        ),
    )
    .unwrap();
    std::fs::write(
        out_dir.join("ext_map.rs"),
        format!(
            "{} = {};",
            concat!(
                "pub(crate) static EXT_MAP: ",
                "phf::Map<&str, &[&crate::MimeData]>",
            ),
            ext_map_gen.build()
        ),
    )
    .unwrap();
}
