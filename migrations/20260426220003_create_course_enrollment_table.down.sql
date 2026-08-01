-- Down: drop learning.course_enrollments table
DROP TABLE IF EXISTS learning.course_enrollments CASCADE;
DROP FUNCTION IF EXISTS learning.course_enrollments_audit_timestamp() CASCADE;
