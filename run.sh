set -e

echo "Buduję aplikację";

cd src_app
yarn
yarn run flow
./build.sh
cd ..

cd src_api

echo "Odpalam testy"
cargo test

echo "Buduję api";
cargo build

echo "Uruchamiam server ...";

RUST_BACKTRACE=1 ./target/debug/online_disk ../../online_disk_data ../src_app/dist 127.0.0.1:7777
