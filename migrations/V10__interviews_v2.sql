-- recreate interviews table with no foriegn keys and remove recruiter_id field
DROP TABLE interviews;
CREATE TABLE interviews(
    id                  INTEGER PRIMARY KEY,
    url                	TEXT NOT NULL DEFAULT "",
    date        	DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    description         TEXT NOT NULL DEFAULT "",
    interview_type      	INTEGER NOT NULL,
    company_id           INTEGER,
   contact_id           INTEGER
);
