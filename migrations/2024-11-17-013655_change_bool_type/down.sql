-- This file should undo anything in `up.sql`

ALTER TABLE players
ALTER COLUMN is_active TYPE BOOLEAN;