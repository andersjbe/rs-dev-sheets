-- Your SQL goes here
CREATE TABLE users (
  id VARCHAR NOT NULL PRIMARY KEY,
  username VARCHAR NOT NULL UNIQUE,
  email VARCHAR NOT NULL UNIQUE,
  pw_hash VARCHAR NOT NULL,
  profile_image_url VARCHAR DEFAULT 'https://upload.wikimedia.org/wikipedia/commons/thumb/2/2c/Default_pfp.svg/2048px-Default_pfp.svg.png' NOT NULL,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
)