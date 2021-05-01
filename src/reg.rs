//! da registry parser

use crate::{Api, Platform, types, xml};
use roxmltree::{Document, Node};
use std::{collections::HashMap, convert::TryFrom};

fn s(s: &str) -> String {
    s.into()
}

#[derive(Debug)]
pub enum Error {
    InvalidAttribute(String, String, String),
    MissingAttribute(String, String),
    UnknownAttribute(String, String),
    UnknownElement(String, Option<String>),
    Xml(roxmltree::Error),
}

pub struct Registry {

}

pub enum EnumVariant {
    /// `GLint`. Not actually standardized, but like two constants use negative values.
    Int32(i32),
    /// `GLenum`/`GLuint`, the default usually and also when `type="u"` is specified.
    Uint32(u32),
    /// `GLuint64`, when `type="ull"` is specified.
    Uint64(u64),
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
                    let value = child.attribute("value").unwrap();
                    let (literal, radix) = match value {
                        x if x.starts_with("0x") => (&value[2..], 16),
                        x if x.len() > 1 && x.starts_with('0') => (&value[1..], 8),
                        _ => (value, 10),
                    };
                    let variant = match child.attribute("type") {
                        None if literal.starts_with('-') => i32::from_str_radix(literal, radix).map(EnumVariant::Int32),
                        Some("ull") => u64::from_str_radix(literal, radix).map(EnumVariant::Uint64),
                        Some("u") | None => u32::from_str_radix(literal, radix).map(EnumVariant::Uint32),
                        Some(unknown) => return Err(Error::InvalidAttribute(s("enum"), s("type"), unknown.into())),
                    }.map_err(|_| Error::InvalidAttribute(s("enum"), s("value"), value.into()))?;

                    out.insert(name.into(), variant);
                } else {
                    // todo!("redefinition of {}", name);
                }
            },
            "unused" => (),
            unknown => return Err(Error::UnknownElement(unknown.into(), Some(s("enums")))),
        }
    }

    Ok(())
}
