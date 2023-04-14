// Copyright (C) Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for `pallet_xcm_benchmarks::generic`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-14, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --pallet=pallet_xcm_benchmarks::generic
// --chain=kusama-dev
// --header=./file_header.txt
// --template=./xcm/pallet-xcm-benchmarks/template.hbs
// --output=./runtime/kusama/src/weights/xcm/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for `pallet_xcm_benchmarks::generic`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo<T> {
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Proof Skipped: XcmPallet SupportedVersion (max_values: None, max_size: None, mode: Measured)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Proof Skipped: XcmPallet VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Proof Skipped: XcmPallet SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	// Proof Skipped: Dmp DownwardMessageQueueHeads (max_values: None, max_size: None, mode: Measured)
	pub fn report_holding() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `211`
		//  Estimated: `14420`
		// Minimum execution time: 29_572_000 picoseconds.
		Weight::from_parts(30_281_000, 14420)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	pub fn buy_execution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_046_000 picoseconds.
		Weight::from_parts(3_116_000, 0)
	}
	// Storage: XcmPallet Queries (r:1 w:0)
	// Proof Skipped: XcmPallet Queries (max_values: None, max_size: None, mode: Measured)
	pub fn query_response() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `169`
		//  Estimated: `3634`
		// Minimum execution time: 11_221_000 picoseconds.
		Weight::from_parts(11_588_000, 3634)
			.saturating_add(T::DbWeight::get().reads(1))
	}
	pub fn transact() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 13_158_000 picoseconds.
		Weight::from_parts(13_453_000, 0)
	}
	pub fn refund_surplus() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_334_000 picoseconds.
		Weight::from_parts(3_394_000, 0)
	}
	pub fn set_error_handler() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_984_000 picoseconds.
		Weight::from_parts(3_043_000, 0)
	}
	pub fn set_appendix() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_000_000 picoseconds.
		Weight::from_parts(3_104_000, 0)
	}
	pub fn clear_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_991_000 picoseconds.
		Weight::from_parts(3_069_000, 0)
	}
	pub fn descend_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_862_000 picoseconds.
		Weight::from_parts(3_932_000, 0)
	}
	pub fn clear_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_983_000 picoseconds.
		Weight::from_parts(3_076_000, 0)
	}
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Proof Skipped: XcmPallet SupportedVersion (max_values: None, max_size: None, mode: Measured)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Proof Skipped: XcmPallet VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Proof Skipped: XcmPallet SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	// Proof Skipped: Dmp DownwardMessageQueueHeads (max_values: None, max_size: None, mode: Measured)
	pub fn report_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `211`
		//  Estimated: `14420`
		// Minimum execution time: 24_598_000 picoseconds.
		Weight::from_parts(24_938_000, 14420)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: XcmPallet AssetTraps (r:1 w:1)
	// Proof Skipped: XcmPallet AssetTraps (max_values: None, max_size: None, mode: Measured)
	pub fn claim_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `226`
		//  Estimated: `3691`
		// Minimum execution time: 15_675_000 picoseconds.
		Weight::from_parts(15_871_000, 3691)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	pub fn trap() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_013_000 picoseconds.
		Weight::from_parts(3_059_000, 0)
	}
	// Storage: XcmPallet VersionNotifyTargets (r:1 w:1)
	// Proof Skipped: XcmPallet VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Proof Skipped: XcmPallet SupportedVersion (max_values: None, max_size: None, mode: Measured)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Proof Skipped: XcmPallet VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Proof Skipped: XcmPallet SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	// Proof Skipped: Dmp DownwardMessageQueueHeads (max_values: None, max_size: None, mode: Measured)
	pub fn subscribe_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `211`
		//  Estimated: `18096`
		// Minimum execution time: 30_853_000 picoseconds.
		Weight::from_parts(31_232_000, 18096)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: XcmPallet VersionNotifyTargets (r:0 w:1)
	// Proof Skipped: XcmPallet VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	pub fn unsubscribe_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_228_000 picoseconds.
		Weight::from_parts(5_317_000, 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	pub fn burn_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_721_000 picoseconds.
		Weight::from_parts(4_861_000, 0)
	}
	pub fn expect_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_137_000 picoseconds.
		Weight::from_parts(3_193_000, 0)
	}
	pub fn expect_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_083_000 picoseconds.
		Weight::from_parts(3_148_000, 0)
	}
	pub fn expect_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_968_000 picoseconds.
		Weight::from_parts(3_059_000, 0)
	}
	pub fn expect_transact_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_170_000 picoseconds.
		Weight::from_parts(3_277_000, 0)
	}
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Proof Skipped: XcmPallet SupportedVersion (max_values: None, max_size: None, mode: Measured)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Proof Skipped: XcmPallet VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Proof Skipped: XcmPallet SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	// Proof Skipped: Dmp DownwardMessageQueueHeads (max_values: None, max_size: None, mode: Measured)
	pub fn query_pallet() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `211`
		//  Estimated: `14420`
		// Minimum execution time: 31_659_000 picoseconds.
		Weight::from_parts(32_008_000, 14420)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	pub fn expect_pallet() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_469_000 picoseconds.
		Weight::from_parts(8_650_000, 0)
	}
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Proof Skipped: XcmPallet SupportedVersion (max_values: None, max_size: None, mode: Measured)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Proof Skipped: XcmPallet VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Proof Skipped: XcmPallet SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	// Proof Skipped: Dmp DownwardMessageQueueHeads (max_values: None, max_size: None, mode: Measured)
	pub fn report_transact_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `211`
		//  Estimated: `14420`
		// Minimum execution time: 24_819_000 picoseconds.
		Weight::from_parts(25_278_000, 14420)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	pub fn clear_transact_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_033_000 picoseconds.
		Weight::from_parts(3_093_000, 0)
	}
	pub fn set_topic() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_996_000 picoseconds.
		Weight::from_parts(3_047_000, 0)
	}
	pub fn clear_topic() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_951_000 picoseconds.
		Weight::from_parts(3_054_000, 0)
	}
	pub fn set_fees_mode() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_040_000 picoseconds.
		Weight::from_parts(3_135_000, 0)
	}
	pub fn unpaid_execution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_232_000 picoseconds.
		Weight::from_parts(3_327_000, 0)
	}
}
