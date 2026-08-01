-- Down: drop learning.courses table
DROP TABLE IF EXISTS learning.courses CASCADE;
DROP FUNCTION IF EXISTS learning.courses_audit_timestamp() CASCADE;
