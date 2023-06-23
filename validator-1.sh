 #!/bin/bash
./target/release/polkadot key insert --base-path /tmp/relay/bob --chain ./chainspec.json --key-type babe --scheme sr25519 --suri "$(gsutil cat gs://whiteflag-0-admin/fennel-validator-key.sh)"
./target/release/polkadot key insert --base-path /tmp/relay/bob --chain ./chainspec.json --key-type gran --scheme ed25519 --suri "$(gsutil cat gs://whiteflag-0-admin/fennel-validator-key.sh)"
./target/release/polkadot --validator --base-path /tmp/relay/bob --chain ./chainspec.json --port 30334  --rpc-port 9945 --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWLj8FgbwDtLQbA5ozWqqx6vM1n2rmjwQMczSiDveyWa5G --rpc-methods Unsafe --name FnlDot1 --rpc-external --prometheus-external --rpc-cors all -ldebug
