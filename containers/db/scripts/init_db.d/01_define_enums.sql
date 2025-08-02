-- Create enums
CREATE TYPE public.login_type AS ENUM ('local', 'oauth');
CREATE TYPE public.provider_type AS ENUM ('apple', 'meta', 'google', 'x');
