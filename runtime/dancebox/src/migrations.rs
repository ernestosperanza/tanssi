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

//! # Migrations
//!
//! This module acts as a registry where each migration is defined. Each migration should implement
//! the "Migration" trait declared in the pallet-migrations crate.

use frame_support::{storage::types::StorageValue, weights::Weight};

use pallet_invulnerables::WeightInfo;
use pallet_migrations::{GetMigrations, Migration};
use sp_runtime::BoundedVec;
use sp_std::{marker::PhantomData, prelude::*};

use crate::{Invulnerables, Runtime, RuntimeOrigin, LOG_TARGET};

pub struct CollatorSelectionStorageValuePrefix;
impl frame_support::traits::StorageInstance for CollatorSelectionStorageValuePrefix {
    const STORAGE_PREFIX: &'static str = "Invulnerables";
    fn pallet_prefix() -> &'static str {
        "CollatorSelection"
    }
}
pub type CollatorSelectionInvulnerablesValue<T> = StorageValue<
    CollatorSelectionStorageValuePrefix,
    BoundedVec<
        <T as frame_system::Config>::AccountId,
        <T as pallet_invulnerables::Config>::MaxInvulnerables,
    >,
>;

pub struct MigrateInvulnerables<T>(pub PhantomData<T>);
impl<T> Migration for MigrateInvulnerables<T>
where
    T: pallet_invulnerables::Config,
{
    fn friendly_name(&self) -> &str {
        "TM_MigrateInvulnerables"
    }

    fn migrate(&self, _available_weight: Weight) -> Weight {
        log::info!(target: LOG_TARGET, "migrate");

        let invulnerables = CollatorSelectionInvulnerablesValue::<Runtime>::take()
            .expect("Failed to get invulnerables from CollatorSelection pallet storage.");
        let invulnerables_len = invulnerables.len();
        Invulnerables::set_invulnerables(RuntimeOrigin::root(), invulnerables.to_vec())
            .expect("Failed to set invulnerables");
        <T as pallet_invulnerables::Config>::WeightInfo::set_invulnerables(invulnerables_len as u32)
    }

    /// Run a standard pre-runtime test. This works the same way as in a normal runtime upgrade.
    #[cfg(feature = "try-runtime")]
    fn pre_upgrade(&self) -> Result<Vec<u8>, sp_runtime::DispatchError> {
        log::info!(target: LOG_TARGET, "pre_upgrade");

        use parity_scale_codec::Encode;

        let number_of_invulnerables = CollatorSelectionInvulnerablesValue::<Runtime>::get()
            .expect("Failed to get invulnerables from CollatorSelection pallet storage.")
            .to_vec()
            .len();
        Ok((number_of_invulnerables as u32).encode())
    }

    /// Run a standard post-runtime test. This works the same way as in a normal runtime upgrade.
    #[cfg(feature = "try-runtime")]
    fn post_upgrade(
        &self,
        number_of_invulnerables: Vec<u8>,
    ) -> Result<(), sp_runtime::DispatchError> {
        log::info!(target: LOG_TARGET, "post_upgrade");
        use parity_scale_codec::Decode;

        let stored_invulnerables = Invulnerables::invulnerables().to_vec();
        let mut sorted_invulnerables = stored_invulnerables.clone();
        sorted_invulnerables.sort();
        assert_eq!(
            stored_invulnerables, sorted_invulnerables,
            "after migration, the stored invulnerables should be sorted"
        );

        let number_of_invulnerables: u32 = Decode::decode(&mut number_of_invulnerables.as_slice())
            .expect("the state parameter should be something that was generated by pre_upgrade");
        let stored_invulnerables_len = stored_invulnerables.len() as u32;
        assert_eq!(
            number_of_invulnerables, stored_invulnerables_len,
            "after migration, there should be the same number of invulnerables"
        );

        Ok(())
    }
}

pub struct DanceboxMigrations<Runtime>(PhantomData<Runtime>);

impl<Runtime> GetMigrations for DanceboxMigrations<Runtime>
where
    Runtime: pallet_invulnerables::Config,
{
    fn get_migrations() -> Vec<Box<dyn Migration>> {
        let migrate_invulnerables = MigrateInvulnerables::<Runtime>(Default::default());

        vec![Box::new(migrate_invulnerables)]
    }
}
