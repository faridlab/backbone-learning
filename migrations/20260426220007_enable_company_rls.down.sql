-- Down: remove the company RLS fence for learning module

-- Reverse the company RLS fence for learning.competencies
DROP POLICY IF EXISTS competencies_company_isolation ON learning.competencies;
ALTER TABLE learning.competencies NO FORCE ROW LEVEL SECURITY;
ALTER TABLE learning.competencies DISABLE ROW LEVEL SECURITY;

-- Reverse the company RLS fence for learning.courses
DROP POLICY IF EXISTS courses_company_isolation ON learning.courses;
ALTER TABLE learning.courses NO FORCE ROW LEVEL SECURITY;
ALTER TABLE learning.courses DISABLE ROW LEVEL SECURITY;

-- Reverse the company RLS fence for learning.course_enrollments
DROP POLICY IF EXISTS course_enrollments_company_isolation ON learning.course_enrollments;
ALTER TABLE learning.course_enrollments NO FORCE ROW LEVEL SECURITY;
ALTER TABLE learning.course_enrollments DISABLE ROW LEVEL SECURITY;

-- Reverse the company RLS fence for learning.employee_competencies
DROP POLICY IF EXISTS employee_competencies_company_isolation ON learning.employee_competencies;
ALTER TABLE learning.employee_competencies NO FORCE ROW LEVEL SECURITY;
ALTER TABLE learning.employee_competencies DISABLE ROW LEVEL SECURITY;

-- Reverse the company RLS fence for learning.employee_skills
DROP POLICY IF EXISTS employee_skills_company_isolation ON learning.employee_skills;
ALTER TABLE learning.employee_skills NO FORCE ROW LEVEL SECURITY;
ALTER TABLE learning.employee_skills DISABLE ROW LEVEL SECURITY;

-- Reverse the company RLS fence for learning.skills
DROP POLICY IF EXISTS skills_company_isolation ON learning.skills;
ALTER TABLE learning.skills NO FORCE ROW LEVEL SECURITY;
ALTER TABLE learning.skills DISABLE ROW LEVEL SECURITY;

