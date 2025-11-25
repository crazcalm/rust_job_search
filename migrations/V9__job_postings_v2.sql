-- Recreate job_postings table with no foreign_keys and no recuiter_id field
DROP TABLE job_posting;
CREATE TABLE job_posting(
    id                  INTEGER PRIMARY KEY,
    url                	TEXT NOT NULL DEFAULT "",
    date_applied        DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    description         TEXT NOT NULL DEFAULT "",
    interviewed         INTEGER NOT NULL DEFAULT "",
    company_id           INTEGER,
    recruiter_id         INTEGER,
    contact_id           INTEGER
);
