// This file is part of Darwinia.
//
// Copyright (C) 2018-2023 Darwinia Network
// SPDX-License-Identifier: GPL-3.0
//
// Darwinia is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Darwinia is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Darwinia. If not, see <https://www.gnu.org/licenses/>.

// darwinia
#[allow(unused_imports)]
use crate::*;
// substrate
#[allow(unused_imports)]
use frame_support::{migration, storage::unhashed};

pub struct CustomOnRuntimeUpgrade;
impl frame_support::traits::OnRuntimeUpgrade for CustomOnRuntimeUpgrade {
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::DispatchError> {
		Ok(Vec::new())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
		Ok(())
	}

	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		migrate()
	}
}

fn migrate() -> frame_support::weights::Weight {
	// substrate
	use pallet_balances::Locks;

	[
		("0xd891ce6a97b4f01a8b9b36d0298aa3631fe2eef5", b"phrelect"),
		("0xabcf7060a68f62624f7569ada9d78b5a5db0782a", b"phrelect"),
		("0x88a39b052d477cfde47600a7c9950a441ce61cb4", b"phrelect"),
		("0x9f33a4809aa708d7a399fedba514e0a0d15efa85", b"phrelect"),
		("0x0a1287977578f888bdc1c7627781af1cc000e6ab", b"phrelect"),
		("0xe59261f6d4088bcd69985a3d369ff14cc54ef1e5", b"phrelect"),
		("0x7ae2a0914db8bfbdad538b0eac3fa473a0e07843", b"democrac"),
		("0xacfa39b864e42d1bd3792783a571d2958af0bf1f", b"democrac"),
		("0x3e25247cff03f99a7d83b28f207112234fee73a6", b"phrelect"),
		("0xb2960e11b253c107f973cd778bbe1520e35e8602", b"phrelect"),
		("0x4ed7ae57608cf4f60753cde4f49cf821c293ed2a", b"democrac"),
		("0x5b7544b3f6abd9e03fba494796b1ee6f9543e2e4", b"phrelect"),
		("0x44cda595218ddb3810fb66c2e982f50ea00255ee", b"phrelect"),
	]
	.iter()
	.for_each(|(acct, lid)| {
		if let Ok(acct) = array_bytes::hex_n_into::<_, AccountId, 20>(acct) {
			<Locks<Runtime>>::mutate(acct, |ls| {
				ls.retain(|l| &l.id != *lid);
			});
		}
	});

	// frame_support::weights::Weight::zero()
	<Runtime as frame_system::Config>::DbWeight::get().reads_writes(0, 26)
}
