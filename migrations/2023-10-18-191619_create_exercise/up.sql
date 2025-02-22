-- Your SQL goes here
CREATE TABLE exercise (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  deleted BOOLEAN NOT NULL DEFAULT FALSE,
  lastudpdatedate VARCHAR 
)