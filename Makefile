APP = sqlx-bug

db: db_create db_migrate db_populate

db_clean:
	rm -f ${APP}.db ${APP}.db-shm ${APP}.db-wal

db_create: db_clean
	sqlx database create

db_migrate:
	sqlx migrate run

db_populate:
	cat data.sql | sqlite3 sqlx-bug.db
