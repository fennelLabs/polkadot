// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `runtime_common::crowdloan`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm6`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --pallet=runtime_common::crowdloan
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/rococo/src/weights/runtime_common_crowdloan.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_common::crowdloan`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_common::crowdloan::WeightInfo for WeightInfo<T> {
	/// Storage: Crowdloan Funds (r:1 w:1)
	/// Proof Skipped: Crowdloan Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: Registrar Paras (r:1 w:1)
	/// Proof Skipped: Registrar Paras (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras ParaLifecycles (r:1 w:0)
	/// Proof Skipped: Paras ParaLifecycles (max_values: None, max_size: None, mode: Measured)
	/// Storage: Crowdloan NextFundIndex (r:1 w:1)
	/// Proof Skipped: Crowdloan NextFundIndex (max_values: Some(1), max_size: None, mode: Measured)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `418`
		//  Estimated: `9592`
		// Minimum execution time: 39_138 nanoseconds.
		Weight::from_parts(41_066_000, 0)
			.saturating_add(Weight::from_parts(0, 9592))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Crowdloan Funds (r:1 w:1)
	/// Proof Skipped: Crowdloan Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: Slots Leases (r:1 w:0)
	/// Proof Skipped: Slots Leases (max_values: None, max_size: None, mode: Measured)
	/// Storage: Auctions AuctionInfo (r:1 w:0)
	/// Proof: Auctions AuctionInfo (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:1)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Crowdloan EndingsCount (r:1 w:0)
	/// Proof Skipped: Crowdloan EndingsCount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Crowdloan NewRaise (r:1 w:1)
	/// Proof Skipped: Crowdloan NewRaise (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: unknown `0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291` (r:1 w:1)
	/// Proof Skipped: unknown `0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291` (r:1 w:1)
	fn contribute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `474`
		//  Estimated: `14402`
		// Minimum execution time: 113_847 nanoseconds.
		Weight::from_parts(115_656_000, 0)
			.saturating_add(Weight::from_parts(0, 14402))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Crowdloan Funds (r:1 w:1)
	/// Proof Skipped: Crowdloan Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:1)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: unknown `0xc85982571aa615c788ef9b2c16f54f25773fd439e8ee1ed2aa3ae43d48e880f0` (r:1 w:1)
	/// Proof Skipped: unknown `0xc85982571aa615c788ef9b2c16f54f25773fd439e8ee1ed2aa3ae43d48e880f0` (r:1 w:1)
	fn withdraw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `785`
		//  Estimated: `12237`
		// Minimum execution time: 54_332 nanoseconds.
		Weight::from_parts(55_814_000, 0)
			.saturating_add(Weight::from_parts(0, 12237))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Skipped Metadata (r:0 w:0)
	/// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
	/// The range of component `k` is `[0, 1000]`.
	fn refund(k: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `190 + k * (221 ±0)`
		//  Estimated: `195 + k * (221 ±0)`
		// Minimum execution time: 40_419 nanoseconds.
		Weight::from_parts(42_695_000, 0)
			.saturating_add(Weight::from_parts(0, 195))
			// Standard Error: 13_706
			.saturating_add(Weight::from_parts(23_364_182, 0).saturating_mul(k.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(k.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(k.into())))
			.saturating_add(Weight::from_parts(0, 221).saturating_mul(k.into()))
	}
	/// Storage: Crowdloan Funds (r:1 w:1)
	/// Proof Skipped: Crowdloan Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn dissolve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `439`
		//  Estimated: `5517`
		// Minimum execution time: 30_943 nanoseconds.
		Weight::from_parts(32_116_000, 0)
			.saturating_add(Weight::from_parts(0, 5517))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Crowdloan Funds (r:1 w:1)
	/// Proof Skipped: Crowdloan Funds (max_values: None, max_size: None, mode: Measured)
	fn edit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `267`
		//  Estimated: `2742`
		// Minimum execution time: 17_805 nanoseconds.
		Weight::from_parts(18_346_000, 0)
			.saturating_add(Weight::from_parts(0, 2742))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Crowdloan Funds (r:1 w:0)
	/// Proof Skipped: Crowdloan Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: unknown `0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291` (r:1 w:1)
	/// Proof Skipped: unknown `0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291` (r:1 w:1)
	fn add_memo() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `444`
		//  Estimated: `5838`
		// Minimum execution time: 25_274 nanoseconds.
		Weight::from_parts(25_881_000, 0)
			.saturating_add(Weight::from_parts(0, 5838))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Crowdloan Funds (r:1 w:0)
	/// Proof Skipped: Crowdloan Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: Crowdloan NewRaise (r:1 w:1)
	/// Proof Skipped: Crowdloan NewRaise (max_values: Some(1), max_size: None, mode: Measured)
	fn poke() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `271`
		//  Estimated: `3512`
		// Minimum execution time: 17_388 nanoseconds.
		Weight::from_parts(18_053_000, 0)
			.saturating_add(Weight::from_parts(0, 3512))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Auctions AuctionInfo (r:1 w:0)
	/// Proof: Auctions AuctionInfo (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Crowdloan EndingsCount (r:1 w:1)
	/// Proof Skipped: Crowdloan EndingsCount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Crowdloan NewRaise (r:1 w:1)
	/// Proof Skipped: Crowdloan NewRaise (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Crowdloan Funds (r:100 w:0)
	/// Proof Skipped: Crowdloan Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: Auctions AuctionCounter (r:1 w:0)
	/// Proof: Auctions AuctionCounter (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Paras ParaLifecycles (r:100 w:0)
	/// Proof Skipped: Paras ParaLifecycles (max_values: None, max_size: None, mode: Measured)
	/// Storage: Slots Leases (r:100 w:0)
	/// Proof Skipped: Slots Leases (max_values: None, max_size: None, mode: Measured)
	/// Storage: Auctions Winning (r:1 w:1)
	/// Proof: Auctions Winning (max_values: None, max_size: Some(1920), added: 4395, mode: MaxEncodedLen)
	/// Storage: Auctions ReservedAmounts (r:100 w:100)
	/// Proof: Auctions ReservedAmounts (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
	/// Storage: System Account (r:100 w:100)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `n` is `[2, 100]`.
	fn on_initialize(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `224 + n * (420 ±0)`
		//  Estimated: `7477 + n * (14663 ±0)`
		// Minimum execution time: 113_445 nanoseconds.
		Weight::from_parts(114_171_000, 0)
			.saturating_add(Weight::from_parts(0, 7477))
			// Standard Error: 48_008
			.saturating_add(Weight::from_parts(48_852_541, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().reads((5_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 14663).saturating_mul(n.into()))
	}
}
