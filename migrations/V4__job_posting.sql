CREATE TABLE job_posting(
    id                  INTEGER PRIMARY KEY,
    url                	TEXT NOT NULL DEFAULT "",
    date_applied        DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    description         TEXT NOT NULL DEFAULT "",
    interviewed         INTEGER NOT NULL DEFAULT "",
    company_id           INTEGER,
    recruiter_id         INTEGER,
    contact_id           INTEGER,
    FOREIGN KEY(company_id)      REFERENCES company(id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY(recruiter_id)    REFERENCES recruiters(id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY(contact_id)      REFERENCES contacts(id) ON DELETE CASCADE ON UPDATE CASCADE
);
