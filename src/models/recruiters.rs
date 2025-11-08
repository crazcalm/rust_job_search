#[derive(Debug, Default, PartialEq)]
pub struct Recruiter {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub description: Option<String>,
    pub company_id: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_default_instance() {
        let expected = Recruiter {
            id: None,
            name: None,
            email: None,
            phone: None,
            description: None,
            company_id: None,
        };

        let result: Recruiter = Default::default();

        assert!(result == expected);
    }
}
