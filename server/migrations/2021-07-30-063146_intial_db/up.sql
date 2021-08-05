CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "citext";

CREATE TABLE "users"
(
    "id"            UUID PRIMARY KEY            DEFAULT uuid_generate_v1mc(),
    "username"      VARCHAR(25)                 NOT NULL UNIQUE,
    "firstname"     VARCHAR(255)                NOT NULL,
    "lastname"      VARCHAR(255)                NOT NULL,
    "password"      VARCHAR(255)                NOT NULL,
    "email"         VARCHAR(255)                NOT NULL UNIQUE,
    "profile_pic"   VARCHAR(255)                        ,
    "is_admin"      BOOLEAN                     NOT NULL DEFAULT false,
    "facebook_id"   VARCHAR(255)                NOT NULL,
    "discord_token" UUID                        NOT NULL DEFAULT uuid_generate_v4()
);

CREATE TABLE "posts"
(
    "id"            UUID PRIMARY KEY            DEFAULT uuid_generate_v1mc(),
    "is_approved"   BOOLEAN                     NOT NULL DEFAULT false,
    "title"         VARCHAR(255)                NOT NULL,
    "thumbnail"     VARCHAR(255)                NOT NULL,
    "body"          TEXT                        NOT NULL,
    "creation_date" TIMESTAMP                   NOT NULL DEFAULT current_timestamp,
    "approval_date" TIMESTAMP,
    "post_author"     UUID,
    CONSTRAINT fk0_author
        FOREIGN KEY (post_author)
            REFERENCES users(id)
            ON DELETE SET NULL
);

CREATE TABLE "comments"
(
    "id"            UUID PRIMARY KEY            DEFAULT uuid_generate_v1mc(),
    "post_id"       UUID                        NOT NULL,
    "author_id"     UUID                        NOT NULL,
    "body"          VARCHAR(500)                NOT NULL,
    "upvotes"       INTEGER                     NOT NULL DEFAULT 0,

    CONSTRAINT fk0_post
        FOREIGN KEY (post_id)
            REFERENCES posts(id)
            ON DELETE SET NULL,
    CONSTRAINT fk1_author
        FOREIGN KEY (author_id)
            REFERENCES users(id)
            ON DELETE SET NULL
)