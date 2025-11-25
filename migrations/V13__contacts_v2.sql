DROP TABLE contacts;
CREATE TABLE contacts(
    id                  INTEGER PRIMARY KEY,
    name          	TEXT NOT NULL DEFAULT "",
    email               TEXT NOT NULL DEFAULT "",
    phone               TEXT NOT NULL DEFAULT "",
    description         TEXT NOT NULL DEFAULT "",
    company_id           INTEGER,
    recruiter		 INTEGER,
    interviewer		 INTEGER,
    other			INTEGER
);
