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

//! Prepare worker.

polkadot_node_core_pvf_common::decl_worker_main!(
    "prepare-worker",
    polkadot_node_core_pvf_prepare_worker::worker_entrypoint,
    tanssi_relay_cli::NODE_VERSION,
    env!("SUBSTRATE_CLI_COMMIT_HASH"),
);
