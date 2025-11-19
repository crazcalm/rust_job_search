use rusqlite::Connection;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct InterviewType {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl InterviewType {
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description)
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn set_id(&mut self, id: i64) {
        self.id = Some(id);
    }

    pub fn update(&mut self, other: &Self) {
        if other.id.is_some() {
            self.set_id(other.id.unwrap());
        }
        if other.name.is_some() {
            self.set_name(other.name.clone().unwrap());
        }
        if other.description.is_some() {
            self.set_description(other.description.clone().unwrap());
        }
    }
}

impl InterviewType {
    fn get(conn: &Connection, id: i64) -> Result<Self, rusqlite::Error> {
        conn.query_row(
            "SELECT id, name, description FROM interview_type WHERE id = ?1",
            [&id],
            |row| {
                Ok(Self {
                    id: row.get(0).ok(),
                    name: row.get(1).ok(),
                    description: row.get(2).ok(),
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
                        "UPDATE interview_type SET name=?2, description=?3 where id =?1",
                        (&id, &self.name.as_ref(), &self.description.as_ref()),
                    )
                    .unwrap();
            }
            None => {
                // TODO: Log the result which has usize representing how many rows were affected
                let _ = conn
                    .execute(
                        "INSERT INTO interview_type (name, description) VALUES (?1, ?2)",
                        (&self.name.as_ref(), &self.description.as_ref()),
                    )
                    .unwrap();

                self.set_id(conn.last_insert_rowid());
            }
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use refinery::{Migration, embed_migrations};
    use rusqlite::Connection;

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
    fn test_get_method() {
        let conn = run_migrations();

        let mut interview_type = InterviewType {
            id: None,
            name: Some("test name".to_string()),
            description: Some("description".to_string()),
        };
        interview_type.save(&conn);

        let interview_type_from_db = InterviewType::get(&conn, interview_type.id.unwrap()).unwrap();

        assert!(interview_type == interview_type_from_db);
    }

    #[test]
    fn test_save_method() {
        let conn = run_migrations();

        let mut interview_type = InterviewType {
            id: None,
            name: Some("test name".to_string()),
            description: Some("description".to_string()),
        };
        interview_type.save(&conn);

        let db_data = conn
            .query_row(
                "SELECT id, name, description FROM interview_type WHERE id = ?1",
                [&interview_type.id],
                |row| {
                    Ok(InterviewType {
                        id: row.get(0).ok(),
                        name: row.get(1).ok(),
                        description: row.get(2).ok(),
                    })
                },
            )
            .unwrap();

        // Testing "create" aspect of save method.
        assert!(interview_type == db_data);

        interview_type.set_name("Updated name".to_string());
        interview_type.set_description("New Description".to_string());

        interview_type.save(&conn);

        let db_data_change = conn
            .query_row(
                "SELECT id, name, description FROM interview_type WHERE id = ?1",
                [&interview_type.id],
                |row| {
                    Ok(InterviewType {
                        id: row.get(0).ok(),
                        name: row.get(1).ok(),
                        description: row.get(2).ok(),
                    })
                },
            )
            .unwrap();

        // Testing "update" aspect of save method.
        assert!(interview_type == db_data_change);
    }

    #[test]
    fn test_update() {
        let mut base = InterviewType {
            id: Some(23),
            name: Some("test_name".to_string()),
            description: Some("test description".to_string()),
        };

        let update_all = InterviewType {
            id: Some(12),
            name: Some("test_name_2".to_string()),
            description: Some("test description".to_string()),
        };

        let update_none: InterviewType = Default::default();

        let base_clone = base.clone();
        base.update(&update_none);
        assert!(base == base_clone);

        base.update(&update_all);
        assert!(base == update_all);
    }

    #[test]
    fn test_setter_methods() {
        let expected = InterviewType {
            id: Some(23),
            name: Some("test_name".to_string()),
            description: Some("test description".to_string()),
        };

        let mut interview_type: InterviewType = Default::default();
        // Note: The id does not have to be cloned because it is not passed by reference like strings are.
        interview_type.set_id(expected.id.unwrap());
        interview_type.set_name(expected.name.clone().unwrap());
        interview_type.set_description(expected.description.clone().unwrap());

        assert!(interview_type == expected);
    }

    #[test]
    fn test_create_default_instance() {
        let expected = InterviewType {
            id: None,
            name: None,
            description: None,
        };

        let result: InterviewType = Default::default();

        assert!(result == expected);
    }
}
