DROP TABLE notes;
CREATE TABLE notes(
    id			INTEGER PRIMARY KEY,
    date        	DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    title 	   	TEXT NOT NULL,
    note		TEXT NOT NULL,
    company_id	INTEGER,
    contact_id	INTEGER,
    job_posting_id	INTEGER,
   interview_id	INTEGER
);
