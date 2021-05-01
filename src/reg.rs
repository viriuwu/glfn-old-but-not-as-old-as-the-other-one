//! da registry parser

use crate::{Api, Platform, types, xml};
use roxmltree::{Document, Node};
use std::{borrow::Cow::{self, Borrowed as B}, collections::HashMap, convert::TryFrom};

fn s(s: &str) -> String {
    s.into()
}

#[derive(Debug)]
pub enum Error {
    Duplicate(String),
    InvalidAttribute(String, String, String),
    MissingAttribute(String, String),
    UnknownAttribute(String, String),
    UnknownElement(String, Option<String>),
    Xml(roxmltree::Error),
}

pub struct Registry {

}

pub struct EnumVariant {
    ty: Cow<'static, str>,
    value: EnumValue,
}

pub enum EnumValue {
    Signed(i64),
    Unsigned(u64),
}

impl Registry {
    pub fn new(api: Api, platform: Platform) -> Result<Self, Error> {
        let (query_types, xml) = match api {
            Api::Gl => (types::gl::get, xml::GL),
            _ => todo!(),
        };

        let document = Document::parse(xml).map_err(Error::Xml)?;
        let root = document.root_element();
        assert_eq!(root.tag_name().name(), "registry");

        let types = query_types(platform);
        let mut enums = HashMap::new();

        for child in root.children().filter(Node::is_element) {
            match child.tag_name().name() {
                "comment" | "types" => (),
                "enums" => parse_enum(&child, api, &mut enums)?,
                "commands" => eprintln!("commands"),
                "feature" => eprintln!("feature"),
                "extensions" => eprintln!("extensions"),
                unknown => return Err(Error::UnknownElement(unknown.into(), Some(s("registry")))),
            }
        }

        for (k, v) in &enums {
            println!("pub const {}: {} = {};", k, v.ty, match v.value {
                EnumValue::Signed(x) => x.to_string(),
                EnumValue::Unsigned(x) => format!("0x{:X}", x),
            });
        }

        todo!()
    }
}

fn parse_enum(
    el: &Node<'_, '_>,
    requested_api: Api,
    out: &mut HashMap<String, EnumVariant>,
) -> Result<(), Error> {
    for child in el.children().filter(Node::is_element) {
        match child.tag_name().name() {
            "enum" => {
                // If it's tagged as being specific to another API, skip
                if let Some(api_attr) = child.attribute("api") {
                    match Api::try_from(api_attr) {
                        Ok(api) => if api != requested_api { continue },
                        Err(()) => return Err(
                            Error::InvalidAttribute(s("enum"), s("api"), s(api_attr))
                        ),
                    }
                }

                let name = child.attribute("name")
                    .ok_or_else(|| Error::MissingAttribute(s("enum"), s("name")))?;
                if !out.contains_key(name) {
                    let value = child.attribute("value")
                        .ok_or_else(|| Error::MissingAttribute(s("enum"), s("value")))?;

                    // TODO: When it's EGL time, handle `EGL_CAST(EGLint, -1)` etc...

                    let (literal, radix) = match value {
                        x if x.starts_with("0x") => (&value[2..], 16),
                        x if x.len() > 1 && x.starts_with('0') => (&value[1..], 8),
                        _ => (value, 10),
                    };
                    let (ty, val) = match child.attribute("type") {
                        Some("ull") => (
                            B("GLuint64"),
                            u64::from_str_radix(literal, radix)
                                .map(EnumValue::Unsigned)
                        ),
                        Some("u") => (
                            B("GLuint32"),
                            u32::from_str_radix(literal, radix)
                                .map(u64::from)
                                .map(EnumValue::Unsigned),
                        ),
                        None if value.starts_with('-') => (
                            // This is probably fine?
                            // Only 1 obscure extension uses negative literals (without a type).
                            B("GLint"),
                            i32::from_str_radix(literal, radix)
                                .map(i64::from)
                                .map(EnumValue::Signed)
                        ),
                        None => (
                            B("GLenum"),
                            u32::from_str_radix(literal, radix)
                                .map(u64::from)
                                .map(EnumValue::Unsigned),
                        ),
                        Some(unknown) => return Err(
                            Error::InvalidAttribute(s("enum"), s("type"), unknown.into())
                        ),
                    };

                    out.insert(
                        name.into(),
                        EnumVariant {
                            ty,
                            value: val.map_err(|_| Error::InvalidAttribute(s("enum"), s("value"), value.into()))?,
                        },
                    );
                } else {
                    return Err(Error::Duplicate(s(name)))
                }
            },
            "unused" => (),
            unknown => return Err(Error::UnknownElement(unknown.into(), Some(s("enums")))),
        }
    }

    Ok(())
}
