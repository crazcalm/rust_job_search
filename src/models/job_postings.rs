use chrono::{DateTime, Utc};
use url::Url;

#[derive(Debug, Default, PartialEq)]
pub struct JobPosting {
    pub id: Option<u32>,
    pub url: Option<Url>,
    pub date_applied: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub interviewed: Option<bool>,
    pub company_id: Option<u32>,
    pub recruiter_id: Option<u32>,
    pub contact_id: Option<u32>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_default_instance() {
        let expected = JobPosting {
            id: None,
            description: None,
            interviewed: None,
            company_id: None,
            recruiter_id: None,
            contact_id: None,
            ..Default::default()
        };
        let result: JobPosting = Default::default();

        assert!(result == expected);
    }
}
