cargo clean
cargo check
cargo run

cargo watch -x check
cargo test

cargo clippy
cargo clippy -- -D warnings

cargo fmt
cargo fmt -- --check

sqlx --help


docker ps -a
docker start container_id
SKIP_DOCKER=true ./scripts/init_db.sh

curl -i -X POST -d 'email=thomas_mann6@hotmail.com&name=Tom' http://127.0.0.1:8000/subscriptions

docker build --tag emailnewsletter --file Dockerfile .
docker run -p 8000:8000 emailnewsletter
curl -v http://127.0.0.1:8000/health_check
curl --request POST \
--data 'name=le%20guin&email=ursula_le_guin%40gmail.com' \
127.0.0.1:8000/subscriptions --verbose


cargo sqlx prepare --workspace

