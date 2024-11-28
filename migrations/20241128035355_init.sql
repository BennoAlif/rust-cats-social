-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS cats (
    id SERIAL PRIMARY KEY,
    name VARCHAR(30) NOT NULL,
    race VARCHAR(30) NOT NULL CHECK (race IN ('Persian', 'Maine Coon', 'Siamese', 'Ragdoll', 'Bengal', 'Sphynx', 'British Shorthair', 'Abyssinian', 'Scottish Fold', 'Birman')),
    sex VARCHAR(6) NOT NULL CHECK (sex IN ('male', 'female')),
    age_in_month INT NOT NULL CHECK (age_in_month BETWEEN 1 AND 120082),
    description VARCHAR(200) NOT NULL,
    img_urls TEXT[] NOT NULL CHECK (array_length(img_urls, 1) >= 1),
    user_id INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE NO ACTION ON UPDATE NO ACTION
);

CREATE TYPE match_status AS ENUM ('pending', 'approved', 'rejected');

CREATE TABLE cat_matches (
    id SERIAL PRIMARY KEY,
    user_cat_id INT NOT NULL,
    match_cat_id INT NOT NULL,
    status match_status DEFAULT 'pending',
    message VARCHAR(200),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_cat_id) REFERENCES cats(id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (match_cat_id) REFERENCES cats(id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE INDEX idx_users_email ON users (email);
CREATE INDEX idx_users_id ON users (id);
CREATE INDEX idx_cats_id ON cats (id);
CREATE INDEX idx_cats_user_id ON cats (user_id);
CREATE INDEX idx_cat_matches_id ON cat_matches (id);
CREATE INDEX idx_cat_matches_user_cat_id ON cat_matches (user_cat_id);
CREATE INDEX idx_cat_matches_match_cat_id ON cat_matches (match_cat_id);
