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
//! Autogenerated weights for `runtime_parachains::ump`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-23, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm4`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --pallet=runtime_parachains::ump
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/runtime_parachains_ump.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_parachains::ump`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains::ump::WeightInfo for WeightInfo<T> {
	/// The range of component `s` is `[0, 51200]`.
	fn process_upward_message(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_556 nanoseconds.
		Weight::from_ref_time(5_656_000)
			.saturating_add(Weight::from_proof_size(0))
			// Standard Error: 8
			.saturating_add(Weight::from_ref_time(1_697).saturating_mul(s.into()))
	}
	/// Storage: Ump NeedsDispatch (r:1 w:1)
	/// Proof Skipped: Ump NeedsDispatch (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	/// Proof Skipped: Ump NextDispatchRoundStartWith (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Ump RelayDispatchQueues (r:0 w:1)
	/// Proof Skipped: Ump RelayDispatchQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: Ump RelayDispatchQueueSize (r:0 w:1)
	/// Proof Skipped: Ump RelayDispatchQueueSize (max_values: None, max_size: None, mode: Measured)
	fn clean_ump_after_outgoing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `272`
		//  Estimated: `2078`
		// Minimum execution time: 8_932 nanoseconds.
		Weight::from_ref_time(9_271_000)
			.saturating_add(Weight::from_proof_size(2078))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Ump Overweight (r:1 w:1)
	/// Proof Skipped: Ump Overweight (max_values: None, max_size: None, mode: Measured)
	/// Storage: Ump CounterForOverweight (r:1 w:1)
	/// Proof: Ump CounterForOverweight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn service_overweight() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `257`
		//  Estimated: `3231`
		// Minimum execution time: 21_961 nanoseconds.
		Weight::from_ref_time(22_329_000)
			.saturating_add(Weight::from_proof_size(3231))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
