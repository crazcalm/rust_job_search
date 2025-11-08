use chrono::{DateTime, Utc};

#[derive(Debug, Default, PartialEq)]
pub struct Note {
    pub id: Option<u32>,
    pub date: DateTime<Utc>,
    pub table_name: String,
    pub table_entry_id: u32,
    pub title: String,
    pub note: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_default_instance() {
        let expected = Note {
            id: None,
            table_name: String::new(),
            table_entry_id: 0,
            title: String::new(),
            note: String::new(),
            ..Default::default()
        };

        let result: Note = Default::default();

        assert!(result == expected);
    }
}
