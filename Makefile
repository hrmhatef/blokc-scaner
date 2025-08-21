build:
	cargo build

format:
	cargo fmt

migrate.status:
	sea-orm-cli migrate status -u "sqlite://indexer.sqlite?mode=rwc" -d "migration/"

migrate.apply:
	sea-orm-cli migrate up -u "sqlite://indexer.sqlite?mode=rwc" -d "migration/"

migrate.refresh:
	sea-orm-cli migrate refresh -u "sqlite://indexer.sqlite?mode=rwc" -d "migration/"

orm.generate:
	sea-orm-cli generate entity -u "sqlite://indexer.sqlite?mode=rwc" -o "libs/orm/src/entities"

run-server:
	cargo run --bin server

run-indexer:
	cargo run --bin indexer
