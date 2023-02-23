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
//! Autogenerated weights for `pallet_elections_phragmen`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-22, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_elections_phragmen
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/rococo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_elections_phragmen`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_elections_phragmen::WeightInfo for WeightInfo<T> {
	/// Storage: PhragmenElection Candidates (r:1 w:0)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Members (r:1 w:0)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:0)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Voting (r:1 w:1)
	/// Proof Skipped: PhragmenElection Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `v` is `[1, 16]`.
	fn vote_equal(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `427 + v * (80 ±0)`
		//  Estimated: `9438 + v * (320 ±0)`
		// Minimum execution time: 24_582 nanoseconds.
		Weight::from_ref_time(25_601_833)
			.saturating_add(Weight::from_proof_size(9438))
			// Standard Error: 2_778
			.saturating_add(Weight::from_ref_time(98_028).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_proof_size(320).saturating_mul(v.into()))
	}
	/// Storage: PhragmenElection Candidates (r:1 w:0)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Members (r:1 w:0)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:0)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Voting (r:1 w:1)
	/// Proof Skipped: PhragmenElection Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `v` is `[2, 16]`.
	fn vote_more(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `395 + v * (80 ±0)`
		//  Estimated: `9310 + v * (320 ±0)`
		// Minimum execution time: 33_162 nanoseconds.
		Weight::from_ref_time(34_408_894)
			.saturating_add(Weight::from_proof_size(9310))
			// Standard Error: 3_459
			.saturating_add(Weight::from_ref_time(107_049).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_proof_size(320).saturating_mul(v.into()))
	}
	/// Storage: PhragmenElection Candidates (r:1 w:0)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Members (r:1 w:0)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:0)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Voting (r:1 w:1)
	/// Proof Skipped: PhragmenElection Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `v` is `[2, 16]`.
	fn vote_less(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `427 + v * (80 ±0)`
		//  Estimated: `9438 + v * (320 ±0)`
		// Minimum execution time: 33_075 nanoseconds.
		Weight::from_ref_time(34_558_830)
			.saturating_add(Weight::from_proof_size(9438))
			// Standard Error: 7_844
			.saturating_add(Weight::from_ref_time(93_110).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_proof_size(320).saturating_mul(v.into()))
	}
	/// Storage: PhragmenElection Voting (r:1 w:1)
	/// Proof Skipped: PhragmenElection Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	fn remove_voter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `917`
		//  Estimated: `7166`
		// Minimum execution time: 29_431 nanoseconds.
		Weight::from_ref_time(29_743_000)
			.saturating_add(Weight::from_proof_size(7166))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: PhragmenElection Candidates (r:1 w:1)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Members (r:1 w:0)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:0)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 1000]`.
	fn submit_candidacy(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2805 + c * (48 ±0)`
		//  Estimated: `9894 + c * (144 ±0)`
		// Minimum execution time: 27_507 nanoseconds.
		Weight::from_ref_time(20_235_171)
			.saturating_add(Weight::from_proof_size(9894))
			// Standard Error: 877
			.saturating_add(Weight::from_ref_time(87_558).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_proof_size(144).saturating_mul(c.into()))
	}
	/// Storage: PhragmenElection Candidates (r:1 w:1)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 1000]`.
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `314 + c * (48 ±0)`
		//  Estimated: `796 + c * (48 ±0)`
		// Minimum execution time: 22_940 nanoseconds.
		Weight::from_ref_time(15_989_682)
			.saturating_add(Weight::from_proof_size(796))
			// Standard Error: 876
			.saturating_add(Weight::from_ref_time(57_772).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_proof_size(48).saturating_mul(c.into()))
	}
	/// Storage: PhragmenElection Members (r:1 w:1)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:1)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:1 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	fn renounce_candidacy_members() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3079`
		//  Estimated: `17375`
		// Minimum execution time: 40_172 nanoseconds.
		Weight::from_ref_time(40_681_000)
			.saturating_add(Weight::from_proof_size(17375))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: PhragmenElection RunnersUp (r:1 w:1)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	fn renounce_candidacy_runners_up() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1742`
		//  Estimated: `2237`
		// Minimum execution time: 25_522 nanoseconds.
		Weight::from_ref_time(26_206_000)
			.saturating_add(Weight::from_proof_size(2237))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Benchmark Override (r:0 w:0)
	/// Proof Skipped: Benchmark Override (max_values: None, max_size: None, mode: Measured)
	fn remove_member_without_replacement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_000_000_000 nanoseconds.
		Weight::from_ref_time(2_000_000_000_000)
			.saturating_add(Weight::from_proof_size(0))
	}
	/// Storage: PhragmenElection Members (r:1 w:1)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: PhragmenElection RunnersUp (r:1 w:1)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:1 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	fn remove_member_with_replacement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3079`
		//  Estimated: `19978`
		// Minimum execution time: 54_309 nanoseconds.
		Weight::from_ref_time(55_097_000)
			.saturating_add(Weight::from_proof_size(19978))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: PhragmenElection Voting (r:10001 w:10000)
	/// Proof Skipped: PhragmenElection Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: PhragmenElection Members (r:1 w:0)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:0)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Candidates (r:1 w:0)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:10000 w:10000)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: System Account (r:10000 w:10000)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `v` is `[5000, 10000]`.
	/// The range of component `d` is `[0, 5000]`.
	fn clean_defunct_voters(v: u32, _d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `35961 + v * (872 ±0)`
		//  Estimated: `148908 + v * (12340 ±0)`
		// Minimum execution time: 296_803_791 nanoseconds.
		Weight::from_ref_time(298_351_085_000)
			.saturating_add(Weight::from_proof_size(148908))
			// Standard Error: 257_135
			.saturating_add(Weight::from_ref_time(37_138_623).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(v.into())))
			.saturating_add(Weight::from_proof_size(12340).saturating_mul(v.into()))
	}
	/// Storage: PhragmenElection Candidates (r:1 w:1)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Members (r:1 w:1)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:1)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Voting (r:10001 w:0)
	/// Proof Skipped: PhragmenElection Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:962 w:962)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: PhragmenElection ElectionRounds (r:1 w:1)
	/// Proof Skipped: PhragmenElection ElectionRounds (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:0 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 1000]`.
	/// The range of component `v` is `[1, 10000]`.
	/// The range of component `e` is `[10000, 160000]`.
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + v * (639 ±0) + e * (28 ±0)`
		//  Estimated: `5334639 + v * (5714 ±4) + e * (123 ±0) + c * (2560 ±0)`
		// Minimum execution time: 31_537_372 nanoseconds.
		Weight::from_ref_time(31_663_201_000)
			.saturating_add(Weight::from_proof_size(5334639))
			// Standard Error: 540_619
			.saturating_add(Weight::from_ref_time(44_981_688).saturating_mul(v.into()))
			// Standard Error: 34_693
			.saturating_add(Weight::from_ref_time(2_335_777).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(265))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes(6))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_proof_size(5714).saturating_mul(v.into()))
			.saturating_add(Weight::from_proof_size(123).saturating_mul(e.into()))
			.saturating_add(Weight::from_proof_size(2560).saturating_mul(c.into()))
	}
}
