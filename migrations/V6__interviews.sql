CREATE TABLE interviews(
    id                  INTEGER PRIMARY KEY,
    url                	TEXT NOT NULL DEFAULT "",
    date        	DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    description         TEXT NOT NULL DEFAULT "",
    interview_type      	INTEGER NOT NULL,
    company_id           INTEGER,
    recruiter_id         INTEGER,
    contact_id           INTEGER,
    FOREIGN KEY(company_id)		   REFERENCES company(id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY(recruiter_id)    	   REFERENCES recruiters(id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY(contact_id)     		   REFERENCES contacts(id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY(interview_type)  	   REFERENCES interview_type(id)
);
