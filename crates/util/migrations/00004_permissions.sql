CREATE TYPE permission_override_value AS ENUM (
    'allow', 'unchanged', 'disallow'
);
CREATE TYPE permission_override_type AS ENUM (
    'channel.view',
    'channel.write',
    'channel.modify'
);
CREATE TYPE permission_override AS (
    type permission_override_type,
    value permission_override_value
);
CREATE TYPE permission_overrides AS (
    entity text,
    overrides permission_override []
);
