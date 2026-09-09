-- Hand-authored (user-owned). Not regenerated.
--
-- Best-effort restore sketch for the tenancy strip (ADR-0029). This is a breaking module
-- release against dev-stage databases: the down re-adds the company_id column as nullable
-- with its plain index, but restores NO data, NO RLS policy, and NO company-leading
-- index/columns — rows written after the strip (or after the decorator re-keyed them)
-- carry org_unit_id only. The composing service's tenancy decorator remains the live
-- fence; treat this down as a schema-shape sketch for archaeology, not a usable rollback.

ALTER TABLE learning.competencies           ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE learning.courses                ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE learning.course_enrollments     ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE learning.employee_competencies  ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE learning.employee_skills        ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE learning.skills                 ADD COLUMN IF NOT EXISTS company_id uuid;

CREATE INDEX IF NOT EXISTS idx_competencies_company_id          ON learning.competencies (company_id);
CREATE INDEX IF NOT EXISTS idx_courses_company_id               ON learning.courses (company_id);
CREATE INDEX IF NOT EXISTS idx_course_enrollments_company_id    ON learning.course_enrollments (company_id);
CREATE INDEX IF NOT EXISTS idx_employee_competencies_company_id ON learning.employee_competencies (company_id);
CREATE INDEX IF NOT EXISTS idx_employee_skills_company_id       ON learning.employee_skills (company_id);
CREATE INDEX IF NOT EXISTS idx_skills_company_id                ON learning.skills (company_id);
