CREATE TABLE notes(
    id			INTEGER PRIMARY KEY,
    date        	DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    table_name		TEXT NOT NULL,
    table_entry_id 	INTEGER NOT NULL,
    title 	   	TEXT NOT NULL,
    note		TEXT NOT NULL
);
