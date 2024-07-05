DEFINE TABLE verify_tokens SCHEMAFULL;

DEFINE FIELD created_at ON verify_tokens VALUE time::now() READONLY;
DEFINE FIELD valid_until ON verify_tokens TYPE datetime;
DEFINE FIELD token ON verify_tokens TYPE uuid;
DEFINE FIELD user ON verify_tokens TYPE record<users>;
