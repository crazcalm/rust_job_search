use rusqlite::Connection;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Recruiter {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub description: Option<String>,
    pub company_id: Option<i64>,
}

impl Recruiter {
    fn set_id(&mut self, id: i64) {
        self.id = Some(id);
    }

    fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    fn set_email(&mut self, email: String) {
        self.email = Some(email);
    }

    fn set_phone(&mut self, phone: String) {
        self.phone = Some(phone);
    }

    fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    fn set_company_id(&mut self, company_id: i64) {
        self.company_id = Some(company_id);
    }

    fn update(&mut self, other: &Self) {
        if other.id.is_some() {
            self.set_id(other.id.unwrap());
        }

        if other.name.is_some() {
            self.set_name(other.name.clone().unwrap());
        }

        if other.description.is_some() {
            self.set_description(other.description.clone().unwrap());
        }

        if other.phone.is_some() {
            self.set_phone(other.phone.clone().unwrap());
        }

        if other.email.is_some() {
            self.set_email(other.email.clone().unwrap());
        }

        if other.company_id.is_some() {
            self.set_company_id(other.company_id.clone().unwrap());
        }
    }
}

impl Recruiter {
    fn get(conn: &Connection, id: i64) -> Result<Self, rusqlite::Error> {
        conn.query_row(
            "SELECT id, name, email, phone, description, company_id FROM recruiters WHERE id = ?1",
            [&id],
            |row| {
                Ok(Self {
                    id: row.get(0).ok(),
                    name: row.get(1).ok(),
                    email: row.get(2).ok(),
                    phone: row.get(3).ok(),
                    description: row.get(4).ok(),
                    company_id: row.get(5).ok(),
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
                        "UPDATE recruiters SET name=?2, email=?3, phone=?4, description=?5, company_id=?6 where id =?1",
                        (
                            &id,
                            &self.name.as_ref(),
                            &self.email.as_ref(),
                            &self.phone.as_ref(),
                            &self.description.as_ref(),
			    &self.company_id.as_ref(),
                        ),
                    )
                    .unwrap();
            }
            None => {
                // TODO: Log the result which has usize representing how many rows were affected
                let _ = conn.execute(
                "INSERT INTO recruiters (name, email, phone, description, company_id) VALUES (?1, ?2, ?3, ?4, ?5)",
                (
                    &self.name.as_ref(),
                    &self.email.as_ref(),
                    &self.phone.as_ref(),
                    &self.description.as_ref(),
		    &self.company_id.as_ref(),
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
    use refinery::{embed_migrations, Migration};
    use rusqlite::Connection;
    use url::Url;

    use crate::models::Company;

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
    fn test_update_method() {
        let mut result = Recruiter {
            id: Some(32),
            name: Some("test_name".to_string()),
            email: Some("email".to_string()),
            phone: Some("phone".to_string()),
            description: Some("description".to_string()),
            company_id: Some(4),
        };

        let original = result.clone();
        let update_none = Default::default();
        result.update(&update_none);

        assert!(result == original);

        let update_all = Recruiter {
            id: Some(23),
            name: Some("test_name2".to_string()),
            email: Some("email2".to_string()),
            phone: Some("phone2".to_string()),
            description: Some("description2".to_string()),
            company_id: Some(6),
        };

        result.update(&update_all);
        assert!(result == update_all);
    }

    #[test]
    fn test_setter_methods() {
        let expected = Recruiter {
            id: Some(23),
            name: Some("test_name".to_string()),
            email: Some("email".to_string()),
            phone: Some("phone".to_string()),
            description: Some("description".to_string()),
            company_id: Some(4),
        };

        let mut result: Recruiter = Default::default();
        result.set_id(expected.id.clone().unwrap());
        result.set_name(expected.name.clone().unwrap());
        result.set_phone(expected.phone.clone().unwrap());
        result.set_description(expected.description.clone().unwrap());
        result.set_email(expected.email.clone().unwrap());
        result.set_company_id(expected.company_id.clone().unwrap());

        assert!(result == expected);
    }

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

    #[test]
    fn test_get_method() {
        let conn = run_migrations();

        let mut company = Company {
            id: None,
            name: Some("test_name".to_string()),
            address: Some("test address".to_string()),
            phone: Some("test phone".to_string()),
            website: Some(Url::parse("http://test-website.com").unwrap()),
        };
        company.save(&conn);

        let mut recruiter = Recruiter {
            id: None,
            email: Some("test email".to_string()),
            name: Some("test_name".to_string()),
            description: Some("test description".to_string()),
            phone: Some("test phone".to_string()),
            company_id: Some(company.id.unwrap()),
        };
        recruiter.save(&conn);

        let recruiter_from_db = Recruiter::get(&conn, recruiter.id.unwrap()).unwrap();

        assert!(recruiter == recruiter_from_db);
    }

    #[test]
    fn test_save_method() {
        let conn = run_migrations();

        let mut recruiter = Recruiter {
            id: None,
            name: Some("test_name".to_string()),
            email: Some("test_email".to_string()),
            phone: Some("test phone".to_string()),
            description: Some("test description".to_string()),
            company_id: None,
        };
        recruiter.save(&conn);

        let db_data = conn
            .query_row(
                "SELECT id, name, email, phone, description, company_id FROM recruiters WHERE id = ?1",
                [&recruiter.id],
                |row| {
                    Ok(Recruiter {
                        id: row.get(0).ok(),
                        name: row.get(1).ok(),
			email: row.get(2).ok(),
                        phone: row.get(3).ok(),
                       description: row.get(4).ok(),
		       company_id: row.get(5).ok(),
                    })
                },
            )
            .unwrap();

        // Testing "create" aspect of save method.
        assert!(recruiter == db_data);

        recruiter.set_name("New Name".to_string());
        recruiter.set_email("New Email".to_string());
        recruiter.set_description("New Description".to_string());
        recruiter.set_phone("New Phone".to_string());

        recruiter.save(&conn);

        let db_data_change = conn
            .query_row(
                "SELECT id, name, email, phone, description, company_id FROM recruiters WHERE id = ?1",
                [&recruiter.id],
                |row| {
                    Ok(Recruiter {
                        id: row.get(0).ok(),
                        name: row.get(1).ok(),
			email: row.get(2).ok(),
                        phone: row.get(3).ok(),
                       description: row.get(4).ok(),
		       company_id: row.get(5).ok(),
                    })
                },
            )
            .unwrap();

        // Testing "update" aspect of save method.
        assert!(recruiter == db_data_change);
    }
}
