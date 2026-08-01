-- Down: drop learning.employee_skills table
DROP TABLE IF EXISTS learning.employee_skills CASCADE;
DROP FUNCTION IF EXISTS learning.employee_skills_audit_timestamp() CASCADE;
