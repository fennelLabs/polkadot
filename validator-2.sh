 #!/bin/bash
./target/release/polkadot key insert --base-path /tmp/relay/charlie --chain ./chainspec.json --key-type babe --scheme sr25519 --suri "$(gsutil cat gs://whiteflag-0-admin/fennel-validator-2-key.sh)"
./target/release/polkadot key insert --base-path /tmp/relay/charlie --chain ./chainspec.json --key-type imon --scheme sr25519 --suri "$(gsutil cat gs://whiteflag-0-admin/fennel-validator-2-key.sh)"
./target/release/polkadot key insert --base-path /tmp/relay/charlie --chain ./chainspec.json --key-type para --scheme sr25519 --suri "$(gsutil cat gs://whiteflag-0-admin/fennel-validator-2-key.sh)"
./target/release/polkadot key insert --base-path /tmp/relay/charlie --chain ./chainspec.json --key-type asgn --scheme sr25519 --suri "$(gsutil cat gs://whiteflag-0-admin/fennel-validator-2-key.sh)"
./target/release/polkadot key insert --base-path /tmp/relay/charlie --chain ./chainspec.json --key-type audi --scheme sr25519 --suri "$(gsutil cat gs://whiteflag-0-admin/fennel-validator-2-key.sh)"
./target/release/polkadot key insert --base-path /tmp/relay/charlie --chain ./chainspec.json --key-type beef --scheme ecdsa --suri "$(gsutil cat gs://whiteflag-0-admin/fennel-validator-2-key.sh)"
./target/release/polkadot key insert --base-path /tmp/relay/charlie --chain ./chainspec.json --key-type gran --scheme ed25519 --suri "$(gsutil cat gs://whiteflag-0-admin/fennel-validator-2-key.sh)"
./target/release/polkadot --charlie --validator --base-path /tmp/relay/charlie --chain ./chainspec.json --port 30335 --rpc-port 9946 --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWLj8FgbwDtLQbA5ozWqqx6vM1n2rmjwQMczSiDveyWa5G --rpc-methods Unsafe --name FnlDot2 --rpc-external --prometheus-external --rpc-cors all
