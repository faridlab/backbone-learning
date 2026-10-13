-- Down: drop the declared foreign keys for learning module
ALTER TABLE IF EXISTS learning.employee_skills DROP CONSTRAINT IF EXISTS fk_employee_skills_skill_id;
ALTER TABLE IF EXISTS learning.employee_competencies DROP CONSTRAINT IF EXISTS fk_employee_competencies_competency_id;
ALTER TABLE IF EXISTS learning.course_enrollments DROP CONSTRAINT IF EXISTS fk_course_enrollments_course_id;
