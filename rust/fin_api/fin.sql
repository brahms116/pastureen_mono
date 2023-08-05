DROP TABLE IF EXISTS transaction_type CASCADE;
CREATE TABLE transaction_type (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    name TEXT NOT NULL
);

DROP TABLE IF EXISTS match_rule CASCADE;
CREATE TABLE match_rule (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  order_index integer UNIQUE NOT NULL,
  name TEXT NOT NULL,
  match_string TEXT NOT NULL,
  transaction_type_id UUID REFERENCES transaction_type(id)
    ON DELETE CASCADE 
    ON UPDATE CASCADE
    NOT NULL, 
  description TEXT NOT NULL DEFAULT ''
);

DROP TABLE IF EXISTS unprocessed_transaction CASCADE;
CREATE TABLE unprocessed_transaction (
  id TEXT PRIMARY KEY,
  amount_cents integer NOT NULL,
  description TEXT NOT NULL DEFAULT '',
  date INTEGER NOT NULL
);

DROP TABLE IF EXISTS transaction CASCADE;
CREATE TABLE transaction (
  id TEXT PRIMARY KEY,
  transaction_type_id UUID REFERENCES transaction_type(id)
    ON DELETE CASCADE 
    ON UPDATE CASCADE
    NOT NULL,
  amount_cents integer NOT NULL,
  description TEXT NOT NULL DEFAULT '',
  date INTEGER NOT NULL
);
