pub mod reg;
pub mod types;
pub mod xml;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Api {
    Gl,
    NotGlPleaseDontGiveMeAnIrrefutableLint,
    ApiNumberThree,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Platform {
    Windows,
    Linux,
    Mac,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
