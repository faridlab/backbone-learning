-- Down: drop learning.employee_competencies table
DROP TABLE IF EXISTS learning.employee_competencies CASCADE;
DROP FUNCTION IF EXISTS learning.employee_competencies_audit_timestamp() CASCADE;
