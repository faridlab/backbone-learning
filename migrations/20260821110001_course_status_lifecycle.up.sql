-- Migration: replace the course bookability boolean with a status enum
--
-- Tree-wide convention: one `status` enum field per lifecycle, no boolean
-- impostors (docs/refactoring-schema in the serpa workspace). The boolean
-- migrates only rows deviating from its own column default. The enum type is
-- created unqualified so it lands beside the module's other enum types in
-- public, where the sqlx type_name resolves.

DO $$ BEGIN CREATE TYPE course_status AS ENUM ('active', 'inactive'); EXCEPTION WHEN duplicate_object THEN NULL; END $$;

ALTER TABLE learning.courses ADD COLUMN status course_status NOT NULL DEFAULT 'active';
UPDATE learning.courses SET status = 'inactive' WHERE NOT is_active;
ALTER TABLE learning.courses DROP COLUMN is_active;
