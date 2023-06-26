 #!/bin/bash
./target/release/polkadot key insert --base-path /tmp/relay/alice --chain ./chainspec.json --key-type babe --scheme sr25519 --suri "$(gsutil cat gs://whiteflag-0-admin/fennel-boot-key.sh)"
./target/release/polkadot key insert --base-path /tmp/relay/alice --chain ./chainspec.json --key-type imon --scheme sr25519 --suri "$(gsutil cat gs://whiteflag-0-admin/fennel-boot-key.sh)"
./target/release/polkadot key insert --base-path /tmp/relay/alice --chain ./chainspec.json --key-type para --scheme sr25519 --suri "$(gsutil cat gs://whiteflag-0-admin/fennel-boot-key.sh)"
./target/release/polkadot key insert --base-path /tmp/relay/alice --chain ./chainspec.json --key-type asgn --scheme sr25519 --suri "$(gsutil cat gs://whiteflag-0-admin/fennel-boot-key.sh)"
./target/release/polkadot key insert --base-path /tmp/relay/alice --chain ./chainspec.json --key-type audi --scheme sr25519 --suri "$(gsutil cat gs://whiteflag-0-admin/fennel-boot-key.sh)"
./target/release/polkadot key insert --base-path /tmp/relay/alice --chain ./chainspec.json --key-type beef --scheme ecdsa --suri "$(gsutil cat gs://whiteflag-0-admin/fennel-boot-key.sh)"
./target/release/polkadot key insert --base-path /tmp/relay/alice --chain ./chainspec.json --key-type gran --scheme ed25519 --suri "$(gsutil cat gs://whiteflag-0-admin/fennel-boot-key.sh)"
./target/release/polkadot --alice --validator --base-path /tmp/relay/alice --chain ./chainspec.json --port 30333 --rpc-port 9944 --name FnlDot0 --node-key $(gsutil cat gs://whiteflag-0-admin/fennel-node-key.sh) --rpc-cors all --prometheus-external --rpc-methods Unsafe --rpc-external
