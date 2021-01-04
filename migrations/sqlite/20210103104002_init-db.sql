CREATE TABLE projects (
  id                TEXT PRIMARY KEY NOT NULL,
  version           INT DEFAULT 0 NOT NULL,
  name              TEXT NOT NULL,
  body              TEXT NOT NULL,

  created_at        TIMESTAMP NOT NULL
                        DEFAULT current_timestamp,
  updated_at        TIMESTAMP NOT NULL
                        DEFAULT current_timestamp
);

CREATE TABLE documents (
  id                TEXT PRIMARY KEY NOT NULL,
  doctype           TEXT NOT NULL,
  version           INTEGER DEFAULT 0 NOT NULL,
  name              TEXT NOT NULL,
  body              TEXT NOT NULL,

  created_at        TIMESTAMP NOT NULL
                        DEFAULT current_timestamp,
  updated_at        TIMESTAMP NOT NULL
                        DEFAULT current_timestamp,

  project_id        TEXT NOT NULL,

  FOREIGN KEY (project_id)
  REFERENCES projects (id)
    ON DELETE CASCADE
    ON UPDATE NO ACTION
);

CREATE TABLE translations (
  id                TEXT PRIMARY KEY NOT NULL,
  version           INTEGER DEFAULT 0 NOT NULL,
  name              TEXT NOT NULL,
  body              TEXT NOT NULL,

  created_at        TIMESTAMP NOT NULL
                        DEFAULT current_timestamp,
  updated_at        TIMESTAMP NOT NULL
                        DEFAULT current_timestamp,

  document_id       TEXT NOT NULL,

  FOREIGN KEY (document_id)
  REFERENCES documents (id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION
);

CREATE TABLE changes (
  id                INTEGER PRIMARY KEY AUTOINCREMENT,
  document_version  TEXT NOT NULL,
  forward           TEXT NOT NULL,
  reverse           TEXT NOT NULL,

  created_at        TIMESTAMP NOT NULL
                        DEFAULT current_timestamp,
  updated_at        TIMESTAMP NOT NULL
                        DEFAULT current_timestamp,

  document_id       TEXT NOT NULL,

  FOREIGN KEY (document_id)
  REFERENCES documents (id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION
);

CREATE TRIGGER project_updated_at
  AFTER UPDATE
  ON projects FOR EACH ROW
  BEGIN
    UPDATE projects SET updated_at = current_timestamp
      WHERE id = old.id;
  END;

CREATE TRIGGER document_updated_at
  AFTER UPDATE
  ON documents FOR EACH ROW
  BEGIN
    UPDATE documents SET updated_at = current_timestamp
      WHERE id = old.id;
  END;

CREATE TRIGGER translation_updated_at
  AFTER UPDATE
  ON translations FOR EACH ROW
  BEGIN
    UPDATE translations SET updated_at = current_timestamp
      WHERE id = old.id;
  END;

CREATE TRIGGER change_updated_at
  AFTER UPDATE
  ON changes FOR EACH ROW
  BEGIN
    UPDATE changes SET updated_at = current_timestamp
      WHERE id = old.id;
  END;

