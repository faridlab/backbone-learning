-- Down: drop learning.skills table
DROP TABLE IF EXISTS learning.skills CASCADE;
DROP FUNCTION IF EXISTS learning.skills_audit_timestamp() CASCADE;
