CREATE TABLE todo (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    comment TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT 'f'
)