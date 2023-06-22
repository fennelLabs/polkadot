 #!/bin/bash
/app/target/release/polkadot key insert --base-path /tmp/relay/charlie --chain /app/chainspec.json --key-type babe --scheme sr25519 --suri "hip top kick mule title lawn add magic donkey galaxy invite blouse"
/app/target/release/polkadot key insert --base-path /tmp/relay/charlie --chain /app/chainspec.json --key-type gran --scheme ed25519 --suri "hip top kick mule title lawn add magic donkey galaxy invite blouse"
/app/target/release/polkadot --validator --base-path /tmp/relay/charlie --chain /app/chainspec.json --port 30335 --rpc-port 9946 --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWFhL6eiQU24RM5p35FcZNSAE3UccKiE88yweD4Z2PD746 --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" --rpc-methods Unsafe --name FnlDot2 --rpc-external --prometheus-external --rpc-cors all
