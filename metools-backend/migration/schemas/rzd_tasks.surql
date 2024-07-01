DEFINE TABLE rzd_tasks SCHEMAFULL;

DEFINE FIELD created_at ON rzd_tasks VALUE time::now() READONLY;
DEFINE FIELD type ON rzd_tasks TYPE string;
DEFINE FIELD data ON rzd_tasks TYPE object;
DEFINE FIELD user ON rzd_tasks TYPE record<users>;
