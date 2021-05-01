pub mod xml;

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
