set -e

echo "Buduję aplikację";

cd src_app
yarn
./build.sh
cd ..

cd src_api

echo "Buduję api";
cargo build

echo "Uruchamiam server ...";

RUST_BACKTRACE=1 ./target/debug/online_disk ../../online_disk_data ../src_app/dist
