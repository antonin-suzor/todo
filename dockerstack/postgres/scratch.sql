DROP TABLE IF EXISTS "todo_element_data" CASCADE;
DROP TABLE IF EXISTS "account" CASCADE;

CREATE TABLE IF NOT EXISTS "account"
(
--     "id"   SERIAL PRIMARY KEY, -- commented out because we want to use static user ids for now
    "id"   INTEGER PRIMARY KEY,
    "name" TEXT NOT NULL
);

INSERT INTO "account" (id, name)
VALUES (1, 'user ABC'),
       (2, 'user DEF');

CREATE TABLE IF NOT EXISTS "todo_element_data"
(
    "id"          SERIAL PRIMARY KEY,
    "title"       TEXT    NOT NULL,
    "done"        BOOLEAN NOT NULL DEFAULT FALSE,
    "description" TEXT    NOT NULL DEFAULT '',
    "account_id"  INTEGER NOT NULL REFERENCES "account" (id) ON DELETE CASCADE
);

INSERT INTO "todo_element_data" (title, done, description, account_id)
VALUES ('TODO element for user 1', FALSE, 'Element description', 1),
       ('DONE element for user 1', TRUE, 'Element description', 1),
       ('TODO element for user 2', FALSE, 'Element description', 2),
       ('DONE element for user 2', TRUE, 'Element description', 2);

