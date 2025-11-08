use url::Url;

#[derive(Debug, Default, PartialEq)]
pub struct Company {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub address: Option<String>,
    pub website: Option<Url>,
    pub phone: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_default_instance() {
        let expected = Company {
            id: None,
            name: None,
            address: None,
            website: None,
            phone: None,
        };
        let result: Company = Default::default();

        assert!(expected == result);
    }
}
