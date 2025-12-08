CREATE TABLE levels_smm1 (
  "id" TEXT PRIMARY KEY NOT NULL,
  "year" BIGINT NOT NULL,

  "title" TEXT NOT NULL,
  "uploaded_at" DATE NOT NULL,

  "attempts" BIGINT NOT NULL,
  "footprints" BIGINT NOT NULL,
  "likes" BIGINT NOT NULL
);
CREATE INDEX levels_smm1_year_idx ON levels_smm1 ("year");

CREATE TABLE levels_smm2 (
  "id" TEXT PRIMARY KEY NOT NULL,
  "year" BIGINT NOT NULL,

  "title" TEXT NOT NULL,
  "description" TEXT,
  "uploaded_at" TIMESTAMP WITH TIME ZONE NOT NULL,
  "clearcheck_ms" BIGINT NOT NULL,

  "attempts" BIGINT NOT NULL,
  "footprints" BIGINT NOT NULL,
  "likes" BIGINT NOT NULL,
  "boos" BIGINT NOT NULL,
  "comments" BIGINT NOT NULL,

  "clear_condition" TEXT,
  "style" TEXT NOT NULL,
  "theme" TEXT NOT NULL,
  "tags" TEXT[] NOT NULL
);
CREATE INDEX levels_smm2_year_idx ON levels_smm2 ("year");
CREATE INDEX levels_smm2_attempts_idx ON levels_smm2 ("attempts");
CREATE INDEX levels_smm2_style_idx ON levels_smm2 ("style");
CREATE INDEX levels_smm2_theme_idx ON levels_smm2 ("theme");

CREATE TABLE known_cleared (
  "game" TEXT NOT NULL,
  "level_id" TEXT NOT NULL
);
CREATE UNIQUE INDEX known_cleared_pkey ON known_cleared ("game", "level_id");

CREATE TABLE level_reservations (
  "player" UUID PRIMARY KEY NOT NULL,
  "game" TEXT NOT NULL,
  "level_id" TEXT NOT NULL,
  "expire_at" TIMESTAMP WITH TIME ZONE
);
CREATE INDEX level_reservations_game_level_id_idx ON level_reservations ("game", "level_id");
CREATE INDEX level_reservations_expire_at_idx ON level_reservations ("expire_at");
