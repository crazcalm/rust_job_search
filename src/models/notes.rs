use chrono::{DateTime, Utc};

#[derive(Debug, Default, PartialEq)]
pub struct Note {
    pub id: Option<i64>,
    pub date: DateTime<Utc>,
    pub title: String,
    pub note: String,
    pub company_id: Option<i64>,
    pub contact_id: Option<i64>,
    pub job_posting_id: Option<i64>,
    pub interview_id: Option<i64>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_default_instance() {
        let expected = Note {
            id: None,
            title: String::new(),
            note: String::new(),
            ..Default::default()
        };

        let result: Note = Default::default();

        assert!(result == expected);
    }
}
