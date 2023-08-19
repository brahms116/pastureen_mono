CREATE TABLE pastureen_user(
  email TEXT UNIQUE NOT NULL PRIMARY KEY,
  fname TEXT NOT NULL,
  lname TEXT NOT NULL,
  password TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT NOW()
);


CREATE TABLE refresh_token(
  token TEXT UNIQUE PRIMARY KEY NOT NULL,
  user_email TEXT NOT NULL REFERENCES pastureen_user(email) ON DELETE CASCADE,
  root_token TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT NOW()
);

