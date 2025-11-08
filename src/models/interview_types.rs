#[derive(Debug, Default, PartialEq)]
pub struct InterviewType {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_default_instance() {
        let expected = InterviewType {
            id: None,
            name: None,
            description: None,
        };

        let result: InterviewType = Default::default();

        assert!(result == expected);
    }
}
