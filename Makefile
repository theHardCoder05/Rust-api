DB_URL=postgresql://root:secret@localhost:5432/rustdb?sslmode=disable

network:
	docker network create rust-network

postgres:
	docker run --name postgres --network rust-network -p 5432:5432 -e POSTGRES_USER=root -e POSTGRES_PASSWORD=secret -d postgres:14-alpine

mysql:
	docker run --name mysql8 -p 3306:3306  -e MYSQL_ROOT_PASSWORD=secret -d mysql:8

createdb:
	docker exec -it postgres createdb --username=root --owner=root rustdb

dropdb:
	docker exec -it postgres dropdb rustdb

migrateup:
	sqlx migrate run



migratedown:
	sqlx migrate revert



db_docs:
	dbdocs build doc/db.dbml

db_schema:
	dbml2sql --postgres -o doc/schema.sql doc/db.dbml

sqlc:
	sqlc generate

test:
	cargo test

server:
	cargo run

mock:
	mockgen -package mockdb -destination db/mock/store.go github.com/techschool/simplebank/db/sqlc Store




.PHONY: network postgres createdb dropdb migrateup migratedown migrateup1 migratedown1 db_docs db_schema sqlc test server mock 
