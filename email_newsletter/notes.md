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

curl -i -X POST -d 'email=thomas_mann5@hotmail.com&name=Tom' http://127.0.0.1:8000/subscriptions
