-- Hand-authored (user-owned). Not regenerated.
--
-- Strip every company-fence artifact from the learning tables (ADR-0029): the module is
-- tenant-agnostic; org scoping is installed by the COMPOSING service's tenancy decorator,
-- never by the module. Dropped here, per table: the company-leading indexes, the
-- <table>_company_isolation RLS policy, and the company_id column itself.
--
-- Ordering guard (the decorator must run FIRST on any database with data): the module
-- never moves tenancy data. A table is safe to strip when EITHER
--   a) it carries org_unit_id with no NULLs — the decorator backfilled it from company_id —
--      or b) it is empty (a fresh database: the earlier chain files created it empty).
-- Otherwise the strip RAISEs, naming the decorator step, rather than dropping a column
-- that still holds the only tenancy key. The file is re-runnable (every drop is IF EXISTS
-- and the tracker has no checksums), so a failed run retries cleanly after the decorator
-- lands.
--
-- RLS enable/force flags are deliberately NOT touched: the decorator owns those now.

DO $$
DECLARE
    t text;
    has_org boolean;
    org_nulls bigint;
    total bigint;
    offenders text := '';
BEGIN
    FOREACH t IN ARRAY ARRAY['competencies', 'courses', 'course_enrollments', 'employee_competencies', 'employee_skills', 'skills']
    LOOP
        IF to_regclass(format('learning.%I', t)) IS NULL THEN
            CONTINUE; -- chain not fully applied on this database; nothing to strip
        END IF;

        SELECT EXISTS (
                   SELECT 1 FROM information_schema.columns
                   WHERE table_schema = 'learning' AND table_name = t AND column_name = 'org_unit_id'
               )
        INTO has_org;

        EXECUTE format('SELECT count(*) FROM learning.%I', t) INTO total;

        IF has_org THEN
            EXECUTE format(
                'SELECT count(*) FROM learning.%I WHERE org_unit_id IS NULL', t)
            INTO org_nulls;
        ELSE
            org_nulls := total; -- no org column: every row's only tenancy key is company_id
        END IF;

        IF has_org AND org_nulls = 0 THEN
            CONTINUE; -- decorator backfilled: safe
        END IF;
        IF total = 0 THEN
            CONTINUE; -- empty table (fresh database): safe
        END IF;
        offenders := offenders || format(' learning.%s (%s rows, %s rows not covered by org_unit_id);', t, total, org_nulls);
    END LOOP;

    IF offenders <> '' THEN
        RAISE EXCEPTION 'refusing to strip company_id — these tables are not yet covered by the tenancy decorator:%. Apply the composing service''s tenancy decorator (it backfills org_unit_id from company_id) and re-run; it is the only step that moves tenancy data.', offenders;
    END IF;
END $$;

-- ── competencies ───────────────────────────────────────────────────────────────
DROP INDEX IF EXISTS learning.idx_competencies_company_id_name;
DROP POLICY IF EXISTS competencies_company_isolation ON learning.competencies;
ALTER TABLE learning.competencies DROP COLUMN IF EXISTS company_id;

-- ── courses ────────────────────────────────────────────────────────────────────
DROP INDEX IF EXISTS learning.idx_courses_company_id_name;
DROP POLICY IF EXISTS courses_company_isolation ON learning.courses;
ALTER TABLE learning.courses DROP COLUMN IF EXISTS company_id;

-- ── course_enrollments ─────────────────────────────────────────────────────────
DROP INDEX IF EXISTS learning.idx_course_enrollments_company_id_employee_id;
DROP POLICY IF EXISTS course_enrollments_company_isolation ON learning.course_enrollments;
ALTER TABLE learning.course_enrollments DROP COLUMN IF EXISTS company_id;

-- ── employee_competencies ──────────────────────────────────────────────────────
DROP INDEX IF EXISTS learning.idx_employee_competencies_company_id_employee_id;
DROP POLICY IF EXISTS employee_competencies_company_isolation ON learning.employee_competencies;
ALTER TABLE learning.employee_competencies DROP COLUMN IF EXISTS company_id;

-- ── employee_skills ────────────────────────────────────────────────────────────
DROP POLICY IF EXISTS employee_skills_company_isolation ON learning.employee_skills;
ALTER TABLE learning.employee_skills DROP COLUMN IF EXISTS company_id;

-- ── skills ─────────────────────────────────────────────────────────────────────
DROP INDEX IF EXISTS learning.idx_skills_company_id_name;
DROP POLICY IF EXISTS skills_company_isolation ON learning.skills;
ALTER TABLE learning.skills DROP COLUMN IF EXISTS company_id;

-- No tenant-free indexes are restored: the pre-strip uniques (course / competency /
-- skill names) and lookup composites all led with company_id, so their tenant-free
-- equivalents are POSTURE, not domain invariants — the composing service's tenancy
-- decorator installs the per-unit forms, and none of them are declared in this module
-- (see schema/models — "No tenancy indexes").
