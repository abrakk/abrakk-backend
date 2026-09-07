-- Create custom enum types used across the schema

CREATE TYPE difficulty_level AS ENUM ('beginner', 'intermediate', 'advanced');

CREATE TYPE subject_area AS ENUM (
    'mathematics',
    'literacy',
    'science',
    'arts',
    'social_studies',
    'languages',
    'physical_education',
    'other'
);

CREATE TYPE user_role AS ENUM ('contributor', 'educator', 'admin');
