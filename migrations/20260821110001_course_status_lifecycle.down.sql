-- Down for course_status_lifecycle: restore the bookability boolean.

ALTER TABLE learning.courses ADD COLUMN is_active BOOLEAN NOT NULL DEFAULT true;
UPDATE learning.courses SET is_active = false WHERE status = 'inactive';
ALTER TABLE learning.courses DROP COLUMN status;
DROP TYPE IF EXISTS course_status;
