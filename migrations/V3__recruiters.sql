CREATE TABLE recruiters(
    id                  INTEGER PRIMARY KEY,
    name          	TEXT NOT NULL DEFAULT "",
    email               TEXT NOT NULL DEFAULT "",
    phone               TEXT NOT NULL DEFAULT "",
    description         TEXT NOT NULL DEFAULT "",
    company_id           INTEGER,
    FOREIGN KEY (company_id) REFERENCES company(id) ON DELETE CASCADE ON UPDATE CASCADE
);
