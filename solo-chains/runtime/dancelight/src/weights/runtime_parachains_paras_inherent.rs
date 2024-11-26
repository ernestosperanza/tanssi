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


//! Autogenerated weights for runtime_parachains::paras_inherent
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2024-09-25, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-1`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("starlight-dev"), DB CACHE: 1024

// Executed Command:
// target/release/tanssi-relay
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// runtime_parachains::paras_inherent
// --extrinsic
// *
// --chain=starlight-dev
// --steps
// 50
// --repeat
// 20
// --template=benchmarking/frame-weight-runtime-template.hbs
// --json-file
// raw.json
// --output
// tmp/starlight_weights/runtime_parachains::paras_inherent.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for runtime_parachains::paras_inherent using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains::paras_inherent::WeightInfo for SubstrateWeight<T> {
	/// Storage: `ParaInherent::Included` (r:1 w:1)
	/// Proof: `ParaInherent::Included` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::ParentHash` (r:1 w:0)
	/// Proof: `System::ParentHash` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `ParasShared::AllowedRelayParents` (r:1 w:1)
	/// Proof: `ParasShared::AllowedRelayParents` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::CurrentSessionIndex` (r:1 w:0)
	/// Proof: `ParasShared::CurrentSessionIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::AvailabilityCores` (r:1 w:1)
	/// Proof: `ParaScheduler::AvailabilityCores` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::ActiveValidatorKeys` (r:1 w:0)
	/// Proof: `ParasShared::ActiveValidatorKeys` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Babe::AuthorVrfRandomness` (r:1 w:0)
	/// Proof: `Babe::AuthorVrfRandomness` (`max_values`: Some(1), `max_size`: Some(33), added: 528, mode: `MaxEncodedLen`)
	/// Storage: `ParaSessionInfo::Sessions` (r:1 w:0)
	/// Proof: `ParaSessionInfo::Sessions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Disputes` (r:1 w:1)
	/// Proof: `ParasDisputes::Disputes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::BackersOnDisputes` (r:1 w:1)
	/// Proof: `ParasDisputes::BackersOnDisputes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Included` (r:1 w:1)
	/// Proof: `ParasDisputes::Included` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParaInherent::OnChainVotes` (r:1 w:1)
	/// Proof: `ParaInherent::OnChainVotes` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Frozen` (r:1 w:0)
	/// Proof: `ParasDisputes::Frozen` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaInclusion::V1` (r:2 w:1)
	/// Proof: `ParaInclusion::V1` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DeliveryFeeFactor` (r:1 w:1)
	/// Proof: `Dmp::DeliveryFeeFactor` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpChannelDigests` (r:1 w:1)
	/// Proof: `Hrmp::HrmpChannelDigests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::FutureCodeUpgrades` (r:1 w:0)
	/// Proof: `Paras::FutureCodeUpgrades` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::Paras` (r:1 w:0)
	/// Proof: `Registrar::Paras` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::SessionStartBlock` (r:1 w:0)
	/// Proof: `ParaScheduler::SessionStartBlock` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::ValidatorGroups` (r:1 w:0)
	/// Proof: `ParaScheduler::ValidatorGroups` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::ClaimQueue` (r:1 w:1)
	/// Proof: `ParaScheduler::ClaimQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TanssiCollatorAssignment::CollatorContainerChain` (r:1 w:0)
	/// Proof: `TanssiCollatorAssignment::CollatorContainerChain` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `OnDemandAssignmentProvider::QueueStatus` (r:1 w:0)
	/// Proof: `OnDemandAssignmentProvider::QueueStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `OnDemandAssignmentProvider::AffinityEntries` (r:1 w:0)
	/// Proof: `OnDemandAssignmentProvider::AffinityEntries` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `OnDemandAssignmentProvider::FreeEntries` (r:1 w:0)
	/// Proof: `OnDemandAssignmentProvider::FreeEntries` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::ActiveValidatorIndices` (r:1 w:0)
	/// Proof: `ParasShared::ActiveValidatorIndices` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::DisabledValidators` (r:1 w:0)
	/// Proof: `Session::DisabledValidators` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpWatermarks` (r:0 w:1)
	/// Proof: `Hrmp::HrmpWatermarks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::Heads` (r:0 w:1)
	/// Proof: `Paras::Heads` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::UpgradeGoAheadSignal` (r:0 w:1)
	/// Proof: `Paras::UpgradeGoAheadSignal` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::MostRecentContext` (r:0 w:1)
	/// Proof: `Paras::MostRecentContext` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `v` is `[10, 200]`.
	fn enter_variable_disputes(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `67813`
		//  Estimated: `73753 + v * (23 ±0)`
		// Minimum execution time: 984_560_000 picoseconds.
		Weight::from_parts(602_393_192, 73753)
			// Standard Error: 21_493
			.saturating_add(Weight::from_parts(41_840_102, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(28_u64))
			.saturating_add(T::DbWeight::get().writes(14_u64))
			.saturating_add(Weight::from_parts(0, 23).saturating_mul(v.into()))
	}
	/// Storage: `ParaInherent::Included` (r:1 w:1)
	/// Proof: `ParaInherent::Included` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::ParentHash` (r:1 w:0)
	/// Proof: `System::ParentHash` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `ParasShared::AllowedRelayParents` (r:1 w:1)
	/// Proof: `ParasShared::AllowedRelayParents` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::CurrentSessionIndex` (r:1 w:0)
	/// Proof: `ParasShared::CurrentSessionIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::AvailabilityCores` (r:1 w:1)
	/// Proof: `ParaScheduler::AvailabilityCores` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::ActiveValidatorKeys` (r:1 w:0)
	/// Proof: `ParasShared::ActiveValidatorKeys` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Babe::AuthorVrfRandomness` (r:1 w:0)
	/// Proof: `Babe::AuthorVrfRandomness` (`max_values`: Some(1), `max_size`: Some(33), added: 528, mode: `MaxEncodedLen`)
	/// Storage: `ParaInherent::OnChainVotes` (r:1 w:1)
	/// Proof: `ParaInherent::OnChainVotes` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Frozen` (r:1 w:0)
	/// Proof: `ParasDisputes::Frozen` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaInclusion::V1` (r:2 w:1)
	/// Proof: `ParaInclusion::V1` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::SessionStartBlock` (r:1 w:0)
	/// Proof: `ParaScheduler::SessionStartBlock` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::ValidatorGroups` (r:1 w:0)
	/// Proof: `ParaScheduler::ValidatorGroups` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::ClaimQueue` (r:1 w:1)
	/// Proof: `ParaScheduler::ClaimQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TanssiCollatorAssignment::CollatorContainerChain` (r:1 w:0)
	/// Proof: `TanssiCollatorAssignment::CollatorContainerChain` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `OnDemandAssignmentProvider::QueueStatus` (r:1 w:0)
	/// Proof: `OnDemandAssignmentProvider::QueueStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `OnDemandAssignmentProvider::AffinityEntries` (r:1 w:0)
	/// Proof: `OnDemandAssignmentProvider::AffinityEntries` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `OnDemandAssignmentProvider::FreeEntries` (r:1 w:0)
	/// Proof: `OnDemandAssignmentProvider::FreeEntries` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::ActiveValidatorIndices` (r:1 w:0)
	/// Proof: `ParasShared::ActiveValidatorIndices` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::DisabledValidators` (r:1 w:0)
	/// Proof: `Session::DisabledValidators` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn enter_bitfields() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42369`
		//  Estimated: `48309`
		// Minimum execution time: 389_412_000 picoseconds.
		Weight::from_parts(405_977_000, 48309)
			.saturating_add(T::DbWeight::get().reads(20_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `ParaInherent::Included` (r:1 w:1)
	/// Proof: `ParaInherent::Included` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::ParentHash` (r:1 w:0)
	/// Proof: `System::ParentHash` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `ParasShared::AllowedRelayParents` (r:1 w:1)
	/// Proof: `ParasShared::AllowedRelayParents` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::CurrentSessionIndex` (r:1 w:0)
	/// Proof: `ParasShared::CurrentSessionIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::AvailabilityCores` (r:1 w:1)
	/// Proof: `ParaScheduler::AvailabilityCores` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::ActiveValidatorKeys` (r:1 w:0)
	/// Proof: `ParasShared::ActiveValidatorKeys` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Babe::AuthorVrfRandomness` (r:1 w:0)
	/// Proof: `Babe::AuthorVrfRandomness` (`max_values`: Some(1), `max_size`: Some(33), added: 528, mode: `MaxEncodedLen`)
	/// Storage: `ParaInherent::OnChainVotes` (r:1 w:1)
	/// Proof: `ParaInherent::OnChainVotes` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Frozen` (r:1 w:0)
	/// Proof: `ParasDisputes::Frozen` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaInclusion::V1` (r:2 w:1)
	/// Proof: `ParaInclusion::V1` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DeliveryFeeFactor` (r:1 w:1)
	/// Proof: `Dmp::DeliveryFeeFactor` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpChannelDigests` (r:1 w:1)
	/// Proof: `Hrmp::HrmpChannelDigests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::FutureCodeUpgrades` (r:1 w:0)
	/// Proof: `Paras::FutureCodeUpgrades` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::Paras` (r:1 w:0)
	/// Proof: `Registrar::Paras` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Disputes` (r:1 w:0)
	/// Proof: `ParasDisputes::Disputes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::SessionStartBlock` (r:1 w:0)
	/// Proof: `ParaScheduler::SessionStartBlock` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::ValidatorGroups` (r:1 w:0)
	/// Proof: `ParaScheduler::ValidatorGroups` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::ClaimQueue` (r:1 w:1)
	/// Proof: `ParaScheduler::ClaimQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TanssiCollatorAssignment::CollatorContainerChain` (r:1 w:0)
	/// Proof: `TanssiCollatorAssignment::CollatorContainerChain` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `OnDemandAssignmentProvider::QueueStatus` (r:1 w:0)
	/// Proof: `OnDemandAssignmentProvider::QueueStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `OnDemandAssignmentProvider::AffinityEntries` (r:1 w:0)
	/// Proof: `OnDemandAssignmentProvider::AffinityEntries` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `OnDemandAssignmentProvider::FreeEntries` (r:1 w:0)
	/// Proof: `OnDemandAssignmentProvider::FreeEntries` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::CurrentCodeHash` (r:1 w:0)
	/// Proof: `Paras::CurrentCodeHash` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::ParaLifecycles` (r:1 w:0)
	/// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `MessageQueue::BookStateFor` (r:1 w:0)
	/// Proof: `MessageQueue::BookStateFor` (`max_values`: None, `max_size`: Some(55), added: 2530, mode: `MaxEncodedLen`)
	/// Storage: `ParasShared::ActiveValidatorIndices` (r:1 w:0)
	/// Proof: `ParasShared::ActiveValidatorIndices` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::DisabledValidators` (r:1 w:0)
	/// Proof: `Session::DisabledValidators` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Included` (r:0 w:1)
	/// Proof: `ParasDisputes::Included` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpWatermarks` (r:0 w:1)
	/// Proof: `Hrmp::HrmpWatermarks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::Heads` (r:0 w:1)
	/// Proof: `Paras::Heads` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::UpgradeGoAheadSignal` (r:0 w:1)
	/// Proof: `Paras::UpgradeGoAheadSignal` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::MostRecentContext` (r:0 w:1)
	/// Proof: `Paras::MostRecentContext` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `v` is `[101, 200]`.
	fn enter_backed_candidates_variable(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42858`
		//  Estimated: `48798`
		// Minimum execution time: 1_451_071_000 picoseconds.
		Weight::from_parts(1_489_393_656, 48798)
			// Standard Error: 19_086
			.saturating_add(Weight::from_parts(42_484, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(29_u64))
			.saturating_add(T::DbWeight::get().writes(14_u64))
	}
	/// Storage: `ParaInherent::Included` (r:1 w:1)
	/// Proof: `ParaInherent::Included` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::ParentHash` (r:1 w:0)
	/// Proof: `System::ParentHash` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `ParasShared::AllowedRelayParents` (r:1 w:1)
	/// Proof: `ParasShared::AllowedRelayParents` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::CurrentSessionIndex` (r:1 w:0)
	/// Proof: `ParasShared::CurrentSessionIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::AvailabilityCores` (r:1 w:1)
	/// Proof: `ParaScheduler::AvailabilityCores` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::ActiveValidatorKeys` (r:1 w:0)
	/// Proof: `ParasShared::ActiveValidatorKeys` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Babe::AuthorVrfRandomness` (r:1 w:0)
	/// Proof: `Babe::AuthorVrfRandomness` (`max_values`: Some(1), `max_size`: Some(33), added: 528, mode: `MaxEncodedLen`)
	/// Storage: `ParaInherent::OnChainVotes` (r:1 w:1)
	/// Proof: `ParaInherent::OnChainVotes` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Frozen` (r:1 w:0)
	/// Proof: `ParasDisputes::Frozen` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaInclusion::V1` (r:2 w:1)
	/// Proof: `ParaInclusion::V1` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DeliveryFeeFactor` (r:1 w:1)
	/// Proof: `Dmp::DeliveryFeeFactor` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpChannelDigests` (r:1 w:1)
	/// Proof: `Hrmp::HrmpChannelDigests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::FutureCodeUpgrades` (r:1 w:0)
	/// Proof: `Paras::FutureCodeUpgrades` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::Paras` (r:1 w:0)
	/// Proof: `Registrar::Paras` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Disputes` (r:1 w:0)
	/// Proof: `ParasDisputes::Disputes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::SessionStartBlock` (r:1 w:0)
	/// Proof: `ParaScheduler::SessionStartBlock` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::ValidatorGroups` (r:1 w:0)
	/// Proof: `ParaScheduler::ValidatorGroups` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::ClaimQueue` (r:1 w:1)
	/// Proof: `ParaScheduler::ClaimQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TanssiCollatorAssignment::CollatorContainerChain` (r:1 w:0)
	/// Proof: `TanssiCollatorAssignment::CollatorContainerChain` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `OnDemandAssignmentProvider::QueueStatus` (r:1 w:0)
	/// Proof: `OnDemandAssignmentProvider::QueueStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `OnDemandAssignmentProvider::AffinityEntries` (r:1 w:0)
	/// Proof: `OnDemandAssignmentProvider::AffinityEntries` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `OnDemandAssignmentProvider::FreeEntries` (r:1 w:0)
	/// Proof: `OnDemandAssignmentProvider::FreeEntries` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::CurrentCodeHash` (r:1 w:0)
	/// Proof: `Paras::CurrentCodeHash` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::FutureCodeHash` (r:1 w:0)
	/// Proof: `Paras::FutureCodeHash` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::UpgradeRestrictionSignal` (r:1 w:0)
	/// Proof: `Paras::UpgradeRestrictionSignal` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::ParaLifecycles` (r:1 w:0)
	/// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `MessageQueue::BookStateFor` (r:1 w:0)
	/// Proof: `MessageQueue::BookStateFor` (`max_values`: None, `max_size`: Some(55), added: 2530, mode: `MaxEncodedLen`)
	/// Storage: `ParasShared::ActiveValidatorIndices` (r:1 w:0)
	/// Proof: `ParasShared::ActiveValidatorIndices` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::DisabledValidators` (r:1 w:0)
	/// Proof: `Session::DisabledValidators` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Included` (r:0 w:1)
	/// Proof: `ParasDisputes::Included` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpWatermarks` (r:0 w:1)
	/// Proof: `Hrmp::HrmpWatermarks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::Heads` (r:0 w:1)
	/// Proof: `Paras::Heads` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::UpgradeGoAheadSignal` (r:0 w:1)
	/// Proof: `Paras::UpgradeGoAheadSignal` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::MostRecentContext` (r:0 w:1)
	/// Proof: `Paras::MostRecentContext` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn enter_backed_candidate_code_upgrade() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42871`
		//  Estimated: `48811`
		// Minimum execution time: 37_166_821_000 picoseconds.
		Weight::from_parts(37_859_964_000, 48811)
			.saturating_add(T::DbWeight::get().reads(31_u64))
			.saturating_add(T::DbWeight::get().writes(14_u64))
	}

	fn enter_empty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `8967`
		//  Estimated: `12432`
		// Minimum execution time: 144_751_000 picoseconds.
		Weight::from_parts(153_966_000, 0)
			.saturating_add(Weight::from_parts(0, 12432))
			.saturating_add(T::DbWeight::get().reads(15))
			.saturating_add(T::DbWeight::get().writes(5))
	}
}