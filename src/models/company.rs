use url::Url;

#[derive(Debug, Default, PartialEq)]
pub struct Company {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub address: Option<String>,
    pub website: Option<Url>,
    pub phone: Option<String>,
}

#[cfg(test)]
mod test {
    use refinery::{embed_migrations, Migration};
    use rusqlite::Connection;

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
    fn test_run_migrations() {
        run_migrations();
    }
}
