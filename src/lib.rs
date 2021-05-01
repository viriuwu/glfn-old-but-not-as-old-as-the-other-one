pub mod reg;
pub mod types;
pub mod xml;

use std::convert::TryFrom;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Api {
    Gl,
    Gles2,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Platform {
    Windows,
    Linux,
    Mac,
}

impl TryFrom<&str> for Api {
    type Error = ();
    fn try_from(x: &str) -> Result<Self, Self::Error> {
        match x {
            "gl" => Ok(Api::Gl),
            "gles2" => Ok(Api::Gles2),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
