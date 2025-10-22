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

// build and start server
docker compose exec app cargo build
docker compose exec app cargo run

// test the endpoint inside the container
docker compose exec app curl 127.0.0.1:8000/rustaceans

// test non-existing endpoint with header to see better response
docker compose exec app curl localhost:8000/rustaceansasdf -H 'Accept: application/json'

// if we add new stuff on the docker-compose.yml we need to update the container
docker compose up -d

// enter into the container
docker compose exec app bash

// make a post request
curl 127.0.0.1:8000/rustaceans -H 'Content-type:  application/json' -d '{"name": "John Doe", "email":"johndoe@gmail.com"}'

// list recent post request
curl 127.0.0.1:8000/rustaceans/1

// update request
// first H header is for the request and the second is for
// the format response
curl 127.0.0.1:8000/rustaceans/1 -X PUT
-H 'Content-type: application/json'
-H 'Accept: application/json'
-d '{"name": "John Doe2", "email":"johndoe@gmail.com"}'

// delete request
curl 127.0.0.1:8000/rustaceans/1 -X DELETE

// run our cargo test
docker compose exec app cargo test

// Roles migrations
docker compose exec app diesel migration generate create_users
docker compose exec app dielse migration generate create_roles
docker compose exec app dielse migration generate create_user_roles

docker compose exec app diesel migration run
docker compose exec app diesel migration revert

// now that we create a cli and server binary we need to run our backend in a different way
docker compose exec app cargo run --bin server

// we can set a default binary to rustaceans adding `default-run` to our manifest key (Cargo.toml)

// create a user using the cli (make sure to run migrations before execute the command)
docker compose exec app cargo run --bin cli users create david pass123456 admin
