-- Your SQL goes here

CREATE TABLE uploads (
  id VARCHAR NOT NULL PRIMARY KEY,
  filename VARCHAR NOT NULL,
  timestamp BIGINT NOT NULL,
  size INTEGER NOT NULL,
  hash VARCHAR NOT NULL
)
