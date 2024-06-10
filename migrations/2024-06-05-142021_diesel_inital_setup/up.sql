-- Your SQL goes here
CREATE TABLE "channel"(
	"id" UUID NOT NULL PRIMARY KEY,
	"name" TEXT NOT NULL,
	"description" TEXT NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL,
	"updated_at" TIMESTAMPTZ NOT NULL
);

CREATE TABLE "message"(
	"id" UUID NOT NULL PRIMARY KEY,
	"content" TEXT NOT NULL,
	"channel_id" UUID NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL,
	"updated_at" TIMESTAMPTZ NOT NULL,
	FOREIGN KEY ("channel_id") REFERENCES "channel"("id")
);

