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
    fn save(&mut self, mut conn: Connection) {
        /*
            If there is not ID, we create an entry, get the ID saved and add it back to this instance.

            If there is an ID, we updating the entry in the database.

        Table Notes:
        table_name: company
        Fields:
        - id
        - name
        - address
        - website
        - phone

        Insert example:
        INSERT INTO company (name, address, website, phone) VALUES (name, address, website phone)

        Update example:
        UPDATE company SET name="", address="", website="", phone="" where id = id;
         */

        match self.id {
            Some(id) => {
                // TODO: Log the result which has usize representing how many rows were affected
                let _ = conn
                    .execute(
                        "UPDATE company SET name=?2, address=?3, website=?4, phone=?5 where id =?1",
                        (
                            &id,
                            &self.name.as_ref().unwrap(),
                            &self.address.as_ref().unwrap(),
                            &self.website.as_ref().unwrap().as_str(),
                            &self.phone.as_ref().unwrap(),
                        ),
                    )
                    .unwrap();
            }
            None => {
                // TODO: Log the result which has usize representing how many rows were affected
                let _ = conn.execute(
                "INSERT INTO company (name, address, website, phone) VALUES (?1, ?2, ?3, ?4)",
                (
                    &self.name.as_ref().unwrap(),
                    &self.address.as_ref().unwrap(),
                    &self.website.as_ref().unwrap().as_str(),
                    &self.phone.as_ref().unwrap(),
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

    fn run_migrations() {
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
    fn test_run_migrations() {
        run_migrations();
    }
}
