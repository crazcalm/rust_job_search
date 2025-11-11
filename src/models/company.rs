use rusqlite::Connection;
use url::Url;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Company {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub address: Option<String>,
    pub website: Option<Url>,
    pub phone: Option<String>,
}

impl Company {
    fn set_id(&mut self, id: i64) {
        // Note: Changed from u32 to i64 because the "last_inserted_row_id" returns i64
        self.id = Some(id);
    }

    fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    fn set_address(&mut self, address: String) {
        self.address = Some(address);
    }

    fn set_website(&mut self, website: Url) {
        self.website = Some(website);
    }

    fn set_phone(&mut self, phone: String) {
        self.phone = Some(phone);
    }

    fn update(&mut self, other: &Self) {
        if other.id.is_some() {
            self.set_id(other.id.unwrap());
        }

        if other.name.is_some() {
            self.set_name(other.name.clone().unwrap());
        }

        if other.address.is_some() {
            self.set_address(other.address.clone().unwrap());
        }

        if other.phone.is_some() {
            self.set_phone(other.phone.clone().unwrap());
        }

        if other.website.is_some() {
            self.set_website(other.website.clone().unwrap());
        }
    }
}

impl Company {
    pub fn get(conn: &Connection, id: i64) -> Result<Self, rusqlite::Error> {
        conn.query_row(
            "SELECT id, name, address, website, phone FROM company WHERE id = ?1",
            [&id],
            |row| {
                let website_option: Option<String> = row.get(3).ok();
                let website_row = match website_option {
                    Some(website) => Some(Url::parse(website.as_str()).unwrap()),
                    None => None,
                };

                Ok(Company {
                    id: row.get(0).ok(),
                    name: row.get(1).ok(),
                    address: row.get(2).ok(),
                    website: website_row,
                    phone: row.get(4).ok(),
                })
            },
        )
    }
    pub fn save(&mut self, conn: &Connection) {
        match self.id {
            Some(id) => {
                // TODO: Log the result which has usize representing how many rows were affected
                let _ = conn
                    .execute(
                        "UPDATE company SET name=?2, address=?3, website=?4, phone=?5 where id =?1",
                        (
                            &id,
                            &self.name.as_ref(),
                            &self.address.as_ref(),
                            &self.website.as_ref(),
                            &self.phone.as_ref(),
                        ),
                    )
                    .unwrap();
            }
            None => {
                // TODO: Log the result which has usize representing how many rows were affected
                let _ = conn.execute(
                "INSERT INTO company (name, address, website, phone) VALUES (?1, ?2, ?3, ?4)",
                (
                    &self.name.as_ref(),
                    &self.address.as_ref(),
                    &self.website.as_ref(),
                    &self.phone.as_ref(),
                ),
		).unwrap();

                self.set_id(conn.last_insert_rowid());
            }
        };
    }
}

#[cfg(test)]
mod test {
    use refinery::{embed_migrations, Migration};
    use rusqlite::Connection;
    use url::Url;

    use super::*;

    /*
    use log::info;
    use refinery::Migration;
    use rusqlite::Connection;

    refinery::embed_migrations!("migrations");

    fn main() {
        let mut conn = Connection::open_in_memory().unwrap();

        // or run all migrations in one go
        migrations::runner().run(&mut conn).unwrap();

    }
        */

    fn run_migrations() -> Connection {
        refinery::embed_migrations!("migrations");
        let mut conn = Connection::open_in_memory().unwrap();
        migrations::runner().run(&mut conn).unwrap();

        /*
                You can verify that the migrations are running with
                `cargo test -- --no-capture`

            The first table stuff get printed to stdout.
        */
        let _ = conn.query_row("select * from sqlite_master;", [], |row| {
            println!("{row:?}");
            Ok(())
        });

        conn
    }

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

    #[test]
    fn test_update() {
        let mut base = Company {
            id: Some(23),
            name: Some("test_name".to_string()),
            address: Some("test address".to_string()),
            phone: Some("test phone".to_string()),
            website: Some(Url::parse("http://test-website.com").unwrap()),
        };

        let update_all = Company {
            id: Some(12),
            name: Some("test_name_2".to_string()),
            address: Some("test address".to_string()),
            phone: Some("test phone".to_string()),
            website: Some(Url::parse("http://test-website_2.com").unwrap()),
        };

        let update_none = Company {
            id: None,
            name: None,
            address: None,
            phone: None,
            website: None,
        };

        let base_clone = base.clone();
        base.update(&update_none);
        assert!(base == base_clone);

        base.update(&update_all);
        assert!(base == update_all);
    }

    #[test]
    fn test_setter_methods() {
        let expected = Company {
            id: Some(23),
            name: Some("test_name".to_string()),
            address: Some("test address".to_string()),
            phone: Some("test phone".to_string()),
            website: Some(Url::parse("http://test-website.com").unwrap()),
        };

        let mut company: Company = Default::default();
        // Note: The id does not have to be cloned because it is not passed by reference like strings are.
        company.set_id(expected.id.unwrap());
        company.set_name(expected.name.clone().unwrap());
        company.set_address(expected.address.clone().unwrap());
        company.set_phone(expected.phone.clone().unwrap());
        company.set_website(expected.website.clone().unwrap());

        assert!(company == expected);
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

        let company_from_db = Company::get(&conn, company.id.unwrap()).unwrap();

        assert!(company == company_from_db);
    }

    #[test]
    fn test_save_method() {
        let conn = run_migrations();

        let mut company = Company {
            id: None,
            name: Some("test_name".to_string()),
            address: Some("test address".to_string()),
            phone: Some("test phone".to_string()),
            website: Some(Url::parse("http://test-website.com").unwrap()),
        };
        company.save(&conn);

        let db_data = conn
            .query_row(
                "SELECT id, name, address, website, phone FROM company WHERE id = ?1",
                [&company.id],
                |row| {
                    let website_option: Option<String> = row.get(3).ok();
                    let website_row = match website_option {
                        Some(website) => Some(Url::parse(website.as_str()).unwrap()),
                        None => None,
                    };

                    Ok(Company {
                        id: row.get(0).ok(),
                        name: row.get(1).ok(),
                        address: row.get(2).ok(),
                        website: website_row,
                        phone: row.get(4).ok(),
                    })
                },
            )
            .unwrap();

        // Testing "create" aspect of save method.
        assert!(company == db_data);

        company.set_name("New Name".to_string());
        company.set_address("New Address".to_string());
        company.set_website(Url::parse("http://new_website.com").unwrap());
        company.set_phone("New Phone".to_string());

        company.save(&conn);

        let db_data_change = conn
            .query_row(
                "SELECT id, name, address, website, phone FROM company WHERE id = ?1",
                [&company.id],
                |row| {
                    let website_option: Option<String> = row.get(3).ok();
                    let website_row = match website_option {
                        Some(website) => Some(Url::parse(website.as_str()).unwrap()),
                        None => None,
                    };

                    Ok(Company {
                        id: row.get(0).ok(),
                        name: row.get(1).ok(),
                        address: row.get(2).ok(),
                        website: website_row,
                        phone: row.get(4).ok(),
                    })
                },
            )
            .unwrap();

        // Testing "update" aspect of save method.
        assert!(company == db_data_change);
    }
}
