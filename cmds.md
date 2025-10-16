// check fi diesel orm can communicate with our postgres container
docker compose exec app diesel setup

// list all the diesel cmds
docker compose exec app diesel
docker compose exec app diesel migration

// list all the diesel migration we have
docker compose exec app diesel migration list

// create two migration for two tables: rustaceans, create
// it's recommended to keep separeted migrations to be
// more flexible to run in prod if some of them are heavy file
docker compose exec app diesel migration generate create_rustaceans
docker compose exec app diesel migration generate create_crates

// after write the down and up sql queries we run the migration
docker compose exec app diesel migration run

// if we need to revert the migration
docker compose exec app diesel migration revert
