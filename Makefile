build:
	cargo build

format:
	cargo fmt

migrate.status:
	sea-orm-cli migrate status -u "sqlite://indexer.sqlite?mode=rwc" -d "./src/orm/migration/"

migrate.apply:
	sea-orm-cli migrate up -u "sqlite://indexer.sqlite?mode=rwc" -d "./src/orm/migration/"

migrate.refresh:
	sea-orm-cli migrate refresh -u "sqlite://indexer.sqlite?mode=rwc" -d "./src/orm/migration/"

orm.generate:
	sea-orm-cli generate entity -u "sqlite://indexer.sqlite?mode=rwc" -o "src/orm/entities"
