-- Your SQL goes here
CREATE TABLE snippets (
  id INTEGER NOT NULL PRIMARY KEY AUTO_INCREMENT,
  title VARCHAR
(100) NOT NULL,
  content TEXT NOT NULL,
  created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  expires DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER `date_trigger` BEFORE INSERT ON `snippets`
FOR EACH ROW SET
    NEW.expires = TIMESTAMPADD(DAY, 365, NEW.created);

CREATE INDEX idx_snippets_created ON snippets(created); 