-- Down: drop learning.competencies table
DROP TABLE IF EXISTS learning.competencies CASCADE;
DROP FUNCTION IF EXISTS learning.competencies_audit_timestamp() CASCADE;
