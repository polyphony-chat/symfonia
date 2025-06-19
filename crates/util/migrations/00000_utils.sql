-- # tzone table
-- https://postgres.cz/wiki/PostgreSQL_SQL_Tricks_III#:~:text=Domain%20for%20time-,zone,-David%20E.%20Wheleer
CREATE OR REPLACE FUNCTION is_timezone(tz TEXT) RETURNS BOOLEAN as $$ BEGIN PERFORM now() AT TIME ZONE tz;
RETURN TRUE;
EXCEPTION
WHEN invalid_parameter_value THEN RETURN FALSE;
END;
$$ language plpgsql STABLE;
CREATE DOMAIN timezone AS TEXT CHECK (is_timezone(value));
CREATE TABLE IF NOT EXISTS tzone (
    tzone_name text PRIMARY KEY constraint chk_is_timzeone CHECK (is_timezone(tzone_name))
);
INSERT INTO tzone (tzone_name)
SELECT name
FROM pg_timezone_names;
-- # entity type enum
CREATE TYPE entity_type AS ENUM ('actor', 'guild', 'channel', 'message');