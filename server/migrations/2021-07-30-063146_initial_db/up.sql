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
    "discord_token" UUID                        NOT NULL DEFAULT uuid_generate_v4(),
    "is_discord_token_used" BOOLEAN             NOT NULL DEFAULT false
);

CREATE TABLE "posts"
(
    "id"            UUID PRIMARY KEY            DEFAULT uuid_generate_v1mc(),
    "is_approved"   BOOLEAN                     NOT NULL DEFAULT false,
    "title"         VARCHAR(255)                NOT NULL,
    "thumbnail"     VARCHAR(255),
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
    "author_id"     UUID,
    "body"          VARCHAR(8000)               NOT NULL,

    CONSTRAINT fk0_post
        FOREIGN KEY (post_id)
            REFERENCES posts(id)
            ON DELETE CASCADE,
    CONSTRAINT fk1_author
        FOREIGN KEY (author_id)
            REFERENCES users(id)
            ON DELETE SET NULL
);

CREATE TABLE "upvotes"
(
    "id"            UUID PRIMARY KEY            DEFAULT uuid_generate_v1mc(),
    "post_id"       UUID                        NOT NULL,
    "user_id"       UUID                        NOT NULL,

    CONSTRAINT fk1_post
        FOREIGN KEY (post_id)
            REFERENCES posts(id)
            ON DELETE CASCADE,

    CONSTRAINT fk2_author
        FOREIGN KEY (user_id)
            REFERENCES users(id)
            ON DELETE CASCADE
);

CREATE TABLE "downvotes"
(
    "id"            UUID PRIMARY KEY            DEFAULT uuid_generate_v1mc(),
    "post_id"       UUID                        NOT NULL,
    "user_id"       UUID                        NOT NULL,

    CONSTRAINT fk2_post
        FOREIGN KEY (post_id)
            REFERENCES posts(id)
            ON DELETE CASCADE,

    CONSTRAINT fk3_author
        FOREIGN KEY (user_id)
            REFERENCES users(id)
            ON DELETE CASCADE
);