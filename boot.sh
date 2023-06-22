 #!/bin/bash
/app/target/release/polkadot key insert --base-path /tmp/relay/alice --chain /app/chainspec.json --key-type babe --scheme sr25519 --suri "coral intact chalk flash neither aunt prepare slender label tuna deposit rapid"
/app/target/release/polkadot key insert --base-path /tmp/relay/alice --chain /app/chainspec.json --key-type gran --scheme ed25519 --suri "coral intact chalk flash neither aunt prepare slender label tuna deposit rapid"
/app/target/release/polkadot --validator --base-path /tmp/relay/alice --chain /app/chainspec.json --port 30333 --rpc-port 9944 --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" --name FnlDot0 --node-key 5b9c74d6cade40cc1a1a60ed06693945c3cde94775715a01e85fe2d4289a0453 --rpc-cors all --prometheus-external --rpc-methods Unsafe --rpc-external
