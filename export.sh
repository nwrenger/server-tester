#/bin/bash
cargo build -r
cp target/release/server-test ./
zip -r exp/server_test_lin.zip server-test static/
rm server-test