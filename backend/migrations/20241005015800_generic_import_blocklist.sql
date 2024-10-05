CREATE TABLE level_blocklist (
  "game" TEXT NOT NULL,
  "level_id" TEXT NOT NULL,
  "reason" TEXT
);
CREATE UNIQUE INDEX level_blocklist_pkey ON level_blocklist ("game", "level_id");
