CREATE TABLE blogs (
    id bigserial NOT NULL,
    title character varying NOT NULL,
    content character varying NOT NULL,
    -- created_time timestamp with time zone NOT NULL,
    CONSTRAINT blogs_pkey PRIMARY KEY (id)
);
