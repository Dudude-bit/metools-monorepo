DEFINE TABLE tokens SCHEMAFULL;

DEFINE FIELD created_at ON tokens VALUE time::now() READONLY;
DEFINE FIELD token ON tokens TYPE uuid;
DEFINE FIELD user ON tokens TYPE record<users>;
