use chrono::{DateTime, Utc};
use url::Url;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Interview {
    pub id: Option<i64>,
    pub url: Option<Url>,
    pub date: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub interview_type: Option<i64>, // Need to change the schema/model to add "_id"
    pub company_id: Option<i64>,
    pub recruiter_id: Option<i64>,
    pub contact_id: Option<i64>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_default_instance() {
        let expected = Interview {
            id: None,
            url: None,
            date: None,
            description: None,
            interview_type: None,
            company_id: None,
            recruiter_id: None,
            contact_id: None,
        };

        let result: Interview = Default::default();

        assert!(result == expected);
    }
}
