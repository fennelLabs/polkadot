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
//! Autogenerated weights for `pallet_fast_unstake`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-27, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_fast_unstake
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_fast_unstake`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_fast_unstake::WeightInfo for WeightInfo<T> {
	/// Storage: FastUnstake ErasToCheckPerBlock (r:1 w:0)
	/// Proof: FastUnstake ErasToCheckPerBlock (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking ValidatorCount (r:1 w:0)
	/// Proof: Staking ValidatorCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: FastUnstake Head (r:1 w:1)
	/// Proof: FastUnstake Head (max_values: Some(1), max_size: Some(886), added: 1381, mode: MaxEncodedLen)
	/// Storage: FastUnstake CounterForQueue (r:1 w:0)
	/// Proof: FastUnstake CounterForQueue (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase CurrentPhase (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Staking CurrentEra (r:1 w:0)
	/// Proof: Staking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking SlashingSpans (r:16 w:0)
	/// Proof Skipped: Staking SlashingSpans (max_values: None, max_size: None, mode: Measured)
	/// Storage: Staking Bonded (r:16 w:16)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Validators (r:16 w:0)
	/// Proof: Staking Validators (max_values: None, max_size: Some(45), added: 2520, mode: MaxEncodedLen)
	/// Storage: Staking Nominators (r:16 w:0)
	/// Proof: Staking Nominators (max_values: None, max_size: Some(558), added: 3033, mode: MaxEncodedLen)
	/// Storage: System Account (r:16 w:16)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:16 w:16)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:0 w:16)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: Staking Payee (r:0 w:16)
	/// Proof: Staking Payee (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// The range of component `b` is `[1, 16]`.
	fn on_idle_unstake(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1127 + b * (391 ±0)`
		//  Estimated: `6126 + b * (17736 ±0)`
		// Minimum execution time: 78_437 nanoseconds.
		Weight::from_ref_time(41_049_087)
			.saturating_add(Weight::from_proof_size(6126))
			// Standard Error: 70_691
			.saturating_add(Weight::from_ref_time(42_011_037).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().reads((6_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((5_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_proof_size(17736).saturating_mul(b.into()))
	}
	/// Storage: FastUnstake ErasToCheckPerBlock (r:1 w:0)
	/// Proof: FastUnstake ErasToCheckPerBlock (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking ValidatorCount (r:1 w:0)
	/// Proof: Staking ValidatorCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: FastUnstake Head (r:1 w:1)
	/// Proof: FastUnstake Head (max_values: Some(1), max_size: Some(886), added: 1381, mode: MaxEncodedLen)
	/// Storage: FastUnstake CounterForQueue (r:1 w:0)
	/// Proof: FastUnstake CounterForQueue (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase CurrentPhase (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Staking CurrentEra (r:1 w:0)
	/// Proof: Staking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking ErasStakers (r:257 w:0)
	/// Proof Skipped: Staking ErasStakers (max_values: None, max_size: None, mode: Measured)
	/// The range of component `v` is `[1, 256]`.
	/// The range of component `b` is `[1, 16]`.
	fn on_idle_check(v: u32, b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1481 + v * (19543 ±0) + b * (48 ±0)`
		//  Estimated: `9009 + v * (41563 ±0) + b * (104 ±0)`
		// Minimum execution time: 697_307 nanoseconds.
		Weight::from_ref_time(698_793_000)
			.saturating_add(Weight::from_proof_size(9009))
			// Standard Error: 6_876_720
			.saturating_add(Weight::from_ref_time(229_912_763).saturating_mul(v.into()))
			// Standard Error: 110_350_707
			.saturating_add(Weight::from_ref_time(3_429_229_662).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_proof_size(41563).saturating_mul(v.into()))
			.saturating_add(Weight::from_proof_size(104).saturating_mul(b.into()))
	}
	/// Storage: FastUnstake ErasToCheckPerBlock (r:1 w:0)
	/// Proof: FastUnstake ErasToCheckPerBlock (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:1)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: FastUnstake Queue (r:1 w:1)
	/// Proof: FastUnstake Queue (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: FastUnstake Head (r:1 w:0)
	/// Proof: FastUnstake Head (max_values: Some(1), max_size: Some(886), added: 1381, mode: MaxEncodedLen)
	/// Storage: Staking Bonded (r:1 w:0)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Validators (r:1 w:0)
	/// Proof: Staking Validators (max_values: None, max_size: Some(45), added: 2520, mode: MaxEncodedLen)
	/// Storage: Staking Nominators (r:1 w:1)
	/// Proof: Staking Nominators (max_values: None, max_size: Some(558), added: 3033, mode: MaxEncodedLen)
	/// Storage: Staking CounterForNominators (r:1 w:1)
	/// Proof: Staking CounterForNominators (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: VoterList ListNodes (r:2 w:2)
	/// Proof: VoterList ListNodes (max_values: None, max_size: Some(154), added: 2629, mode: MaxEncodedLen)
	/// Storage: VoterList ListBags (r:1 w:1)
	/// Proof: VoterList ListBags (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	/// Storage: VoterList CounterForListNodes (r:1 w:1)
	/// Proof: VoterList CounterForListNodes (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking CurrentEra (r:1 w:0)
	/// Proof: Staking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: FastUnstake CounterForQueue (r:1 w:1)
	/// Proof: FastUnstake CounterForQueue (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn register_fast_unstake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2117`
		//  Estimated: `29662`
		// Minimum execution time: 118_076 nanoseconds.
		Weight::from_ref_time(119_160_000)
			.saturating_add(Weight::from_proof_size(29662))
			.saturating_add(T::DbWeight::get().reads(15))
			.saturating_add(T::DbWeight::get().writes(10))
	}
	/// Storage: FastUnstake ErasToCheckPerBlock (r:1 w:0)
	/// Proof: FastUnstake ErasToCheckPerBlock (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:0)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: FastUnstake Queue (r:1 w:1)
	/// Proof: FastUnstake Queue (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: FastUnstake Head (r:1 w:0)
	/// Proof: FastUnstake Head (max_values: Some(1), max_size: Some(886), added: 1381, mode: MaxEncodedLen)
	/// Storage: FastUnstake CounterForQueue (r:1 w:1)
	/// Proof: FastUnstake CounterForQueue (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn deregister() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1220`
		//  Estimated: `8476`
		// Minimum execution time: 41_749 nanoseconds.
		Weight::from_ref_time(42_129_000)
			.saturating_add(Weight::from_proof_size(8476))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FastUnstake ErasToCheckPerBlock (r:0 w:1)
	/// Proof: FastUnstake ErasToCheckPerBlock (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn control() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_839 nanoseconds.
		Weight::from_ref_time(2_959_000)
			.saturating_add(Weight::from_proof_size(0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
