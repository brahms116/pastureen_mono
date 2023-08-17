DROP TABLE IF EXISTS pastureen_user CASCADE;
CREATE TABLE pastureen_user(
  id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  fname TEXT,
  lname TEXT,
  email TEXT UNIQUE NOT NULL,
  password TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT NOW()
);


DROP TABLE IF EXISTS refresh_token CASCADE;
CREATE TABLE refresh_token(
  token TEXT UNIQUE PRIMARY KEY NOT NULL,
  user_id UUID NOT NULL REFERENCES pastureen_user(id) ON DELETE CASCADE,
  root_token TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT NOW()
);

