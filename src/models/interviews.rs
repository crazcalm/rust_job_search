use chrono::{DateTime, Utc};
use rusqlite::Connection;
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

impl Interview {
    pub fn set_contact_id(&mut self, contact_id: i64) {
        self.contact_id = Some(contact_id);
    }

    pub fn set_recruiter_id(&mut self, recruiter_id: i64) {
        self.recruiter_id = Some(recruiter_id);
    }

    pub fn set_company_id(&mut self, company_id: i64) {
        self.company_id = Some(company_id);
    }

    pub fn set_interview_type(&mut self, interview_type: i64) {
        self.interview_type = Some(interview_type)
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn set_date(&mut self, date: DateTime<Utc>) {
        self.date = Some(date);
    }

    pub fn set_id(&mut self, id: i64) {
        self.id = Some(id);
    }

    pub fn set_url(&mut self, url: Url) {
        self.url = Some(url);
    }

    pub fn update(&mut self, other: &Self) {
        if other.id.is_some() {
            self.set_id(other.id.clone().unwrap());
        }

        if other.url.is_some() {
            self.set_url(other.url.clone().unwrap());
        }
        if other.date.is_some() {
            self.set_date(other.date.clone().unwrap());
        }
        if other.description.is_some() {
            self.set_description(other.description.clone().unwrap());
        }
        if other.interview_type.is_some() {
            self.set_interview_type(other.interview_type.clone().unwrap());
        }
        if other.company_id.is_some() {
            self.set_company_id(other.company_id.clone().unwrap());
        }
        if other.recruiter_id.is_some() {
            self.set_recruiter_id(other.recruiter_id.clone().unwrap());
        }
        if other.contact_id.is_some() {
            self.set_contact_id(other.contact_id.clone().unwrap());
        }
    }
}

impl Interview {
    fn get(conn: &Connection, id: i64) -> Result<Self, rusqlite::Error> {
        conn.query_row(
            "SELECT id, url, date, description, interview_type, company_id, recruiter_id, contact_id FROM interviews WHERE id = ?1",
            [&id],
            |row| {
                Ok(Self {
                    id: row.get(0).ok(),
                    url: row.get(1).ok(),
                    date: row.get(2).ok(),
                    description: row.get(3).ok(),
                    interview_type: row.get(4).ok(),
                    company_id: row.get(5).ok(),
		    recruiter_id: row.get(6).ok(),
		    contact_id: row.get(7).ok(),
                })
            },
        )
    }
    fn save(&mut self, conn: &Connection) {
        match self.id {
            Some(id) => {
                // TODO: Log the result which has usize representing how many rows were affected
                let _ = conn
                    .execute(
                        "UPDATE interviews SET url=?2, date=?3, description=?4, interview_type=?5, company_id=?6, recruiter_id=?7, contact_id=?8 where id =?1",
                        (
                            &id,
                            &self.url.as_ref(),
                            &self.date.as_ref(),
                            &self.description.as_ref(),
                            &self.interview_type.as_ref(),
			    &self.company_id.as_ref(),
			    &self.recruiter_id.as_ref(),
			    &self.contact_id.as_ref(),
                        ),
                    )
                    .unwrap();
            }
            None => {
                // TODO: Log the result which has usize representing how many rows were affected
                let _ = conn.execute(
                "INSERT INTO interviews (url, date, description, interview_type, company_id, recruiter_id, contact_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                (
                    &self.url.as_ref(),
                    &self.date.as_ref(),
                    &self.description.as_ref(),
                    &self.interview_type.as_ref(),
		    &self.company_id.as_ref(),
		    &self.recruiter_id.as_ref(),
		    &self.contact_id.as_ref(),
                ),
		).unwrap();

                self.set_id(conn.last_insert_rowid());
            }
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Utc;
    use rusqlite::Connection;
    use url::Url;

    fn run_migrations() -> Connection {
        refinery::embed_migrations!("migrations");
        let mut conn = Connection::open_in_memory().unwrap();
        migrations::runner().run(&mut conn).unwrap();

        /*
                You can verify that the migrations are running with
                `cargo test -- --no-capture`

            The first table stuff get printed to stdout.

        TODO: figure out how I want to go about saving stuff...
             */
        let _ = conn.query_row("select * from sqlite_master;", [], |row| {
            println!("{row:?}");
            Ok(())
        });

        conn
    }

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

    #[test]
    fn test_get_method() {
        let conn = run_migrations();

        let mut interview = Interview {
            id: None,
            url: Some(Url::parse("http://test.com").unwrap()),
            date: Some(Utc::now()),
            interview_type: Some(1),
            description: Some("description".to_string()),
            company_id: None,
            recruiter_id: None,
            contact_id: None,
        };
        interview.save(&conn);

        let interview_from_db = Interview::get(&conn, interview.id.unwrap()).unwrap();

        assert!(interview == interview_from_db);
    }

    #[test]
    fn test_save_method() {
        let conn = run_migrations();

        let mut interview = Interview {
            id: None,
            url: Some(Url::parse("http://test.com").unwrap()),
            date: Some(Utc::now()),
            interview_type: Some(1),
            description: Some("description".to_string()),
            company_id: None,
            recruiter_id: None,
            contact_id: None,
        };
        interview.save(&conn);

        let db_data = conn.query_row(
            "SELECT id, url, date, description, interview_type, company_id, recruiter_id, contact_id FROM interviews WHERE id = ?1",
            [&interview.id],
            |row| {
                Ok(Interview{
                    id: row.get(0).ok(),
                    url: row.get(1).ok(),
                    date: row.get(2).ok(),
                    description: row.get(3).ok(),
                    interview_type: row.get(4).ok(),
                    company_id: row.get(5).ok(),
		    recruiter_id: row.get(6).ok(),
		    contact_id: row.get(7).ok(),
                })
            },
        ).unwrap();

        // Testing "create" aspect of save method.
        assert!(interview == db_data);

        interview.set_url(Url::parse("http://url.com").unwrap());
        interview.set_description("New Description".to_string());
        interview.set_date(Utc::now());
        interview.set_interview_type(2);

        interview.save(&conn);

        let db_data_change = conn.query_row(
            "SELECT id, url, date, description, interview_type, company_id, recruiter_id, contact_id FROM interviews WHERE id = ?1",
            [&interview.id],
            |row| {
                Ok(Interview {
                    id: row.get(0).ok(),
                    url: row.get(1).ok(),
                    date: row.get(2).ok(),
                    description: row.get(3).ok(),
                    interview_type: row.get(4).ok(),
                    company_id: row.get(5).ok(),
		    recruiter_id: row.get(6).ok(),
		    contact_id: row.get(7).ok(),
                })
            },
        ).unwrap();

        // Testing "update" aspect of save method.
        assert!(interview == db_data_change);
    }

    #[test]
    fn test_update_method() {
        let mut result = Interview {
            id: Some(23),
            url: Some(Url::parse("http://test.com").unwrap()),
            date: Some(Utc::now()),
            interview_type: Some(1),
            description: Some("description".to_string()),
            company_id: Some(4),
            recruiter_id: Some(5),
            contact_id: Some(6),
        };

        let original = result.clone();
        let update_none: Interview = Default::default();
        result.update(&update_none);

        assert!(result == original);

        let update_all = Interview {
            id: Some(24),
            url: Some(Url::parse("http://test2.com").unwrap()),
            date: Some(Utc::now()),
            interview_type: Some(2),
            description: Some("description2".to_string()),
            company_id: Some(41),
            recruiter_id: Some(51),
            contact_id: Some(61),
        };

        result.update(&update_all);
        assert!(result == update_all);
    }

    #[test]
    fn test_setter_methods() {
        let expected = Interview {
            id: Some(23),
            url: Some(Url::parse("http://test.com").unwrap()),
            date: Some(Utc::now()),
            interview_type: Some(1),
            description: Some("description".to_string()),
            company_id: Some(4),
            recruiter_id: Some(5),
            contact_id: Some(6),
        };

        let mut result: Interview = Default::default();
        result.set_id(expected.id.clone().unwrap());
        result.set_url(expected.url.clone().unwrap());
        result.set_date(expected.date.clone().unwrap());
        result.set_description(expected.description.clone().unwrap());
        result.set_interview_type(expected.interview_type.clone().unwrap());
        result.set_company_id(expected.company_id.clone().unwrap());
        result.set_recruiter_id(expected.recruiter_id.clone().unwrap());
        result.set_contact_id(expected.contact_id.clone().unwrap());

        assert!(result == expected);
    }
}
