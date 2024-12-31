// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>


//! Autogenerated weights for pallet_services_payment
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 43.0.0
//! DATE: 2024-12-31, STEPS: `16`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `tomasz-XPS-15-9520`, CPU: `12th Gen Intel(R) Core(TM) i7-12700H`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/tanssi-node
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_services_payment
// --extrinsic
// *
// --chain=dev
// --steps
// 16
// --repeat
// 1
// --template=benchmarking/frame-weight-pallet-template.hbs
// --json-file
// raw.json
// --output
// tmp/pallet_services_payment.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_services_payment.
pub trait WeightInfo {
	fn purchase_credits() -> Weight;
	fn set_block_production_credits() -> Weight;
	fn set_given_free_credits() -> Weight;
	fn set_refund_address() -> Weight;
	fn set_max_core_price() -> Weight;
	fn on_container_authors_noted(n: u32, ) -> Weight;
	fn on_collators_assigned() -> Weight;
	fn set_max_tip() -> Weight;
}

/// Weights for pallet_services_payment using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn purchase_credits() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `155`
		//  Estimated: `6196`
		// Minimum execution time: 45_102_000 picoseconds.
		Weight::from_parts(45_102_000, 6196)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ServicesPayment::BlockProductionCredits` (r:0 w:1)
	/// Proof: `ServicesPayment::BlockProductionCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	fn set_block_production_credits() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_864_000 picoseconds.
		Weight::from_parts(6_864_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ServicesPayment::GivenFreeCredits` (r:0 w:1)
	/// Proof: `ServicesPayment::GivenFreeCredits` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	fn set_given_free_credits() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_076_000 picoseconds.
		Weight::from_parts(5_076_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Registrar::ParaManager` (r:1 w:0)
	/// Proof: `Registrar::ParaManager` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::RefundAddress` (r:0 w:1)
	/// Proof: `ServicesPayment::RefundAddress` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn set_refund_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `220`
		//  Estimated: `3517`
		// Minimum execution time: 12_430_000 picoseconds.
		Weight::from_parts(12_430_000, 3517)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Registrar::ParaManager` (r:1 w:0)
	/// Proof: `Registrar::ParaManager` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::MaxCorePrice` (r:0 w:1)
	/// Proof: `ServicesPayment::MaxCorePrice` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	fn set_max_core_price() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `220`
		//  Estimated: `3517`
		// Minimum execution time: 12_141_000 picoseconds.
		Weight::from_parts(12_141_000, 3517)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ServicesPayment::BlockProductionCredits` (r:50 w:0)
	/// Proof: `ServicesPayment::BlockProductionCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:50 w:50)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 50]`.
	fn on_container_authors_noted(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `300 + n * (132 ±0)`
		//  Estimated: `990 + n * (2603 ±0)`
		// Minimum execution time: 20_800_000 picoseconds.
		Weight::from_parts(8_063_256, 990)
			// Standard Error: 289_417
			.saturating_add(Weight::from_parts(14_388_705, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2603).saturating_mul(n.into()))
	}
	/// Storage: `ServicesPayment::CollatorAssignmentCredits` (r:1 w:0)
	/// Proof: `ServicesPayment::CollatorAssignmentCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::MaxTip` (r:1 w:0)
	/// Proof: `ServicesPayment::MaxTip` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	fn on_collators_assigned() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `315`
		//  Estimated: `3593`
		// Minimum execution time: 36_282_000 picoseconds.
		Weight::from_parts(36_282_000, 3593)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ServicesPayment::MaxTip` (r:0 w:1)
	/// Proof: `ServicesPayment::MaxTip` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	fn set_max_tip() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_832_000 picoseconds.
		Weight::from_parts(5_832_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn purchase_credits() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `155`
		//  Estimated: `6196`
		// Minimum execution time: 45_102_000 picoseconds.
		Weight::from_parts(45_102_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `ServicesPayment::BlockProductionCredits` (r:0 w:1)
	/// Proof: `ServicesPayment::BlockProductionCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	fn set_block_production_credits() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_864_000 picoseconds.
		Weight::from_parts(6_864_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `ServicesPayment::GivenFreeCredits` (r:0 w:1)
	/// Proof: `ServicesPayment::GivenFreeCredits` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	fn set_given_free_credits() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_076_000 picoseconds.
		Weight::from_parts(5_076_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Registrar::ParaManager` (r:1 w:0)
	/// Proof: `Registrar::ParaManager` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::RefundAddress` (r:0 w:1)
	/// Proof: `ServicesPayment::RefundAddress` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn set_refund_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `220`
		//  Estimated: `3517`
		// Minimum execution time: 12_430_000 picoseconds.
		Weight::from_parts(12_430_000, 3517)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Registrar::ParaManager` (r:1 w:0)
	/// Proof: `Registrar::ParaManager` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::MaxCorePrice` (r:0 w:1)
	/// Proof: `ServicesPayment::MaxCorePrice` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	fn set_max_core_price() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `220`
		//  Estimated: `3517`
		// Minimum execution time: 12_141_000 picoseconds.
		Weight::from_parts(12_141_000, 3517)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `ServicesPayment::BlockProductionCredits` (r:50 w:0)
	/// Proof: `ServicesPayment::BlockProductionCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:50 w:50)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 50]`.
	fn on_container_authors_noted(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `300 + n * (132 ±0)`
		//  Estimated: `990 + n * (2603 ±0)`
		// Minimum execution time: 20_800_000 picoseconds.
		Weight::from_parts(8_063_256, 990)
			// Standard Error: 289_417
			.saturating_add(Weight::from_parts(14_388_705, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(n.into())))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2603).saturating_mul(n.into()))
	}
	/// Storage: `ServicesPayment::CollatorAssignmentCredits` (r:1 w:0)
	/// Proof: `ServicesPayment::CollatorAssignmentCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::MaxTip` (r:1 w:0)
	/// Proof: `ServicesPayment::MaxTip` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	fn on_collators_assigned() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `315`
		//  Estimated: `3593`
		// Minimum execution time: 36_282_000 picoseconds.
		Weight::from_parts(36_282_000, 3593)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `ServicesPayment::MaxTip` (r:0 w:1)
	/// Proof: `ServicesPayment::MaxTip` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	fn set_max_tip() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_832_000 picoseconds.
		Weight::from_parts(5_832_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
