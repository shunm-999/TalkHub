-- Your SQL goes here
CREATE TABLE "message"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"content" TEXT NOT NULL,
	"channel_id" INT4 NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL,
	"updated_at" TIMESTAMPTZ NOT NULL,
	FOREIGN KEY ("channel_id") REFERENCES "channel"("id")
);

CREATE TABLE "channel"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"name" TEXT NOT NULL,
	"description" TEXT NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL,
	"updated_at" TIMESTAMPTZ NOT NULL
);

