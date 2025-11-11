use chrono::{DateTime, Utc};
use rusqlite::Connection;
use url::Url;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct JobPosting {
    pub id: Option<i64>,
    pub url: Option<Url>,
    pub date_applied: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub interviewed: Option<bool>,
    pub company_id: Option<i64>,
    pub recruiter_id: Option<i64>,
    pub contact_id: Option<i64>,
}

impl JobPosting {
    pub fn set_contact_id(&mut self, contact_id: i64) {
        self.contact_id = Some(contact_id);
    }

    pub fn set_recruiter_id(&mut self, recruiter_id: i64) {
        self.recruiter_id = Some(recruiter_id);
    }

    pub fn set_company_id(&mut self, company_id: i64) {
        self.company_id = Some(company_id);
    }

    pub fn set_interviewed(&mut self, interviewed: bool) {
        self.interviewed = Some(interviewed)
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn set_date_applied(&mut self, date_applied: DateTime<Utc>) {
        self.date_applied = Some(date_applied);
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
        if other.date_applied.is_some() {
            self.set_date_applied(other.date_applied.clone().unwrap());
        }
        if other.description.is_some() {
            self.set_description(other.description.clone().unwrap());
        }
        if other.interviewed.is_some() {
            self.set_interviewed(other.interviewed.clone().unwrap());
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

impl JobPosting {
    fn get(conn: &Connection, id: i64) -> Result<Self, rusqlite::Error> {
        conn.query_row(
            "SELECT id, url, date_applied, description, interviewed, company_id, recruiter_id, contact_id FROM job_posting WHERE id = ?1",
            [&id],
            |row| {
                Ok(Self {
                    id: row.get(0).ok(),
                    url: row.get(1).ok(),
                    date_applied: row.get(2).ok(),
                    description: row.get(3).ok(),
                    interviewed: row.get(4).ok(),
                    company_id: row.get(5).ok(),
		    recruiter_id: row.get(6).ok(),
		    contact_id: row.get(7).ok(),
                })
            },
        )
    }
    fn save(&mut self, conn: &Connection) {
        /*
        pub id: Option<i64>,
            pub url: Option<Url>,
            pub date_applied: Option<DateTime<Utc>>,
            pub description: Option<String>,
            pub interviewed: Option<bool>,
            pub company_id: Option<i64>,
            pub recruiter_id: Option<i64>,
            pub contact_id: Option<i64>,
             */

        match self.id {
            Some(id) => {
                // TODO: Log the result which has usize representing how many rows were affected
                let _ = conn
                    .execute(
                        "UPDATE job_posting SET url=?2, date_applied=?3, description=?4, interviewed=?5, company_id=?6, recruiter_id=?7, contact_id=?8 where id =?1",
                        (
                            &id,
                            &self.url.as_ref(),
                            &self.date_applied.as_ref(),
                            &self.description.as_ref(),
                            &self.interviewed.as_ref(),
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
                "INSERT INTO job_posting (url, date_applied, description, interviewed, company_id, recruiter_id, contact_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                (
                    &self.url.as_ref(),
                    &self.date_applied.as_ref(),
                    &self.description.as_ref(),
                    &self.interviewed.as_ref(),
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
    use chrono::{DateTime, Utc};
    use url::Url;

    #[test]
    fn test_update_method() {
        let mut result = JobPosting {
            id: Some(23),
            url: Some(Url::parse("http://test.com").unwrap()),
            date_applied: Some(Utc::now()),
            interviewed: Some(true),
            description: Some("description".to_string()),
            company_id: Some(4),
            recruiter_id: Some(5),
            contact_id: Some(6),
        };

        let original = result.clone();
        let update_none: JobPosting = Default::default();
        result.update(&update_none);

        assert!(result == original);

        let update_all = JobPosting {
            id: Some(24),
            url: Some(Url::parse("http://test2.com").unwrap()),
            date_applied: Some(Utc::now()),
            interviewed: Some(false),
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
        let expected = JobPosting {
            id: Some(23),
            url: Some(Url::parse("http://test.com").unwrap()),
            date_applied: Some(Utc::now()),
            interviewed: Some(true),
            description: Some("description".to_string()),
            company_id: Some(4),
            recruiter_id: Some(5),
            contact_id: Some(6),
        };

        let mut result: JobPosting = Default::default();
        result.set_id(expected.id.clone().unwrap());
        result.set_url(expected.url.clone().unwrap());
        result.set_date_applied(expected.date_applied.clone().unwrap());
        result.set_description(expected.description.clone().unwrap());
        result.set_interviewed(expected.interviewed.clone().unwrap());
        result.set_company_id(expected.company_id.clone().unwrap());
        result.set_recruiter_id(expected.recruiter_id.clone().unwrap());
        result.set_contact_id(expected.contact_id.clone().unwrap());

        assert!(result == expected);
    }

    #[test]
    fn test_create_default_instance() {
        let expected = JobPosting {
            id: None,
            url: None,
            description: None,
            interviewed: None,
            company_id: None,
            recruiter_id: None,
            contact_id: None,
            date_applied: None,
        };
        let result: JobPosting = Default::default();

        assert!(result == expected);
    }
}
