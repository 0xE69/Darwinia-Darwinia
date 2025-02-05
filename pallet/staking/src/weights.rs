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

//! Autogenerated weights for darwinia_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-11-17, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("pangolin-dev"), DB CACHE: 1024

// Executed Command:
// target/release/darwinia
// benchmark
// pallet
// --header
// .maintain/license-header
// --template
// .maintain/pallet-weight-template.hbs
// --heap-pages
// 4096
// --chain
// pangolin-dev
// --output
// pallet/staking/src/weights.rs
// --extrinsic
// *
// --pallet
// darwinia-staking

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(missing_docs)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for darwinia_staking.
pub trait WeightInfo {
	fn stake(x: u32, ) -> Weight;
	fn unstake(x: u32, ) -> Weight;
	fn restake(x: u32, ) -> Weight;
	fn claim() -> Weight;
	fn collect() -> Weight;
	fn nominate() -> Weight;
	fn chill() -> Weight;
	fn payout() -> Weight;
	fn set_collator_count() -> Weight;
}

/// Weights for darwinia_staking using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `DarwiniaStaking::Ledgers` (r:1 w:1)
	/// Proof: `DarwiniaStaking::Ledgers` (`max_values`: None, `max_size`: Some(1833), added: 4308, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::RingPool` (r:1 w:1)
	/// Proof: `DarwiniaStaking::RingPool` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(166), added: 2641, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::KtonPool` (r:1 w:1)
	/// Proof: `DarwiniaStaking::KtonPool` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Deposit::Deposits` (r:1 w:1)
	/// Proof: `Deposit::Deposits` (`max_values`: None, `max_size`: Some(26150), added: 28625, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[0, 1023]`.
	fn stake(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1977`
		//  Estimated: `29615`
		// Minimum execution time: 65_025 nanoseconds.
		Weight::from_parts(119_080_833, 0)
			.saturating_add(Weight::from_parts(29615, 0))
			// Standard Error: 1_240
			.saturating_add(Weight::from_parts(6_811, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: `DarwiniaStaking::Ledgers` (r:1 w:1)
	/// Proof: `DarwiniaStaking::Ledgers` (`max_values`: None, `max_size`: Some(1833), added: 4308, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::RingPool` (r:1 w:1)
	/// Proof: `DarwiniaStaking::RingPool` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::KtonPool` (r:1 w:1)
	/// Proof: `DarwiniaStaking::KtonPool` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Deposit::Deposits` (r:1 w:0)
	/// Proof: `Deposit::Deposits` (`max_values`: None, `max_size`: Some(26150), added: 28625, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[0, 1023]`.
	fn unstake(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1589`
		//  Estimated: `29615`
		// Minimum execution time: 11_158 nanoseconds.
		Weight::from_parts(51_460_547, 0)
			.saturating_add(Weight::from_parts(29615, 0))
			// Standard Error: 896
			.saturating_add(Weight::from_parts(3_363, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `DarwiniaStaking::Ledgers` (r:1 w:1)
	/// Proof: `DarwiniaStaking::Ledgers` (`max_values`: None, `max_size`: Some(1833), added: 4308, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::RingPool` (r:1 w:1)
	/// Proof: `DarwiniaStaking::RingPool` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::KtonPool` (r:1 w:1)
	/// Proof: `DarwiniaStaking::KtonPool` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Deposit::Deposits` (r:1 w:0)
	/// Proof: `Deposit::Deposits` (`max_values`: None, `max_size`: Some(26150), added: 28625, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[0, 1023]`.
	fn restake(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1693`
		//  Estimated: `29615`
		// Minimum execution time: 10_061 nanoseconds.
		Weight::from_parts(43_375_019, 0)
			.saturating_add(Weight::from_parts(29615, 0))
			// Standard Error: 750
			.saturating_add(Weight::from_parts(3_582, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `DarwiniaStaking::Ledgers` (r:1 w:1)
	/// Proof: `DarwiniaStaking::Ledgers` (`max_values`: None, `max_size`: Some(1833), added: 4308, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(166), added: 2641, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `Deposit::Deposits` (r:1 w:1)
	/// Proof: `Deposit::Deposits` (`max_values`: None, `max_size`: Some(26150), added: 28625, mode: `MaxEncodedLen`)
	fn claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2215`
		//  Estimated: `29615`
		// Minimum execution time: 87_073 nanoseconds.
		Weight::from_parts(89_352_000, 0)
			.saturating_add(Weight::from_parts(29615, 0))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `DarwiniaStaking::Collators` (r:1 w:1)
	/// Proof: `DarwiniaStaking::Collators` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	fn collect() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `410`
		//  Estimated: `3497`
		// Minimum execution time: 7_784 nanoseconds.
		Weight::from_parts(8_159_000, 0)
			.saturating_add(Weight::from_parts(3497, 0))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `DarwiniaStaking::Ledgers` (r:1 w:0)
	/// Proof: `DarwiniaStaking::Ledgers` (`max_values`: None, `max_size`: Some(1833), added: 4308, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::Collators` (r:1 w:0)
	/// Proof: `DarwiniaStaking::Collators` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::Nominators` (r:1 w:1)
	/// Proof: `DarwiniaStaking::Nominators` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	fn nominate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `578`
		//  Estimated: `5298`
		// Minimum execution time: 9_357 nanoseconds.
		Weight::from_parts(9_716_000, 0)
			.saturating_add(Weight::from_parts(5298, 0))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `DarwiniaStaking::Nominators` (r:0 w:1)
	/// Proof: `DarwiniaStaking::Nominators` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::Collators` (r:0 w:1)
	/// Proof: `DarwiniaStaking::Collators` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	fn chill() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_204 nanoseconds.
		Weight::from_parts(3_458_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `DarwiniaStaking::ExposureCacheStates` (r:1 w:0)
	/// Proof: `DarwiniaStaking::ExposureCacheStates` (`max_values`: Some(1), `max_size`: Some(3), added: 498, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::ExposureCache2` (r:1 w:0)
	/// Proof: `DarwiniaStaking::ExposureCache2` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `DarwiniaStaking::PendingRewards` (r:1 w:1)
	/// Proof: `DarwiniaStaking::PendingRewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn payout() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1449`
		//  Estimated: `4914`
		// Minimum execution time: 129_233 nanoseconds.
		Weight::from_parts(131_538_000, 0)
			.saturating_add(Weight::from_parts(4914, 0))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `DarwiniaStaking::CollatorCount` (r:0 w:1)
	/// Proof: `DarwiniaStaking::CollatorCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_collator_count() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_745 nanoseconds.
		Weight::from_parts(1_829_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `DarwiniaStaking::Ledgers` (r:1 w:1)
	/// Proof: `DarwiniaStaking::Ledgers` (`max_values`: None, `max_size`: Some(1833), added: 4308, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::RingPool` (r:1 w:1)
	/// Proof: `DarwiniaStaking::RingPool` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(166), added: 2641, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::KtonPool` (r:1 w:1)
	/// Proof: `DarwiniaStaking::KtonPool` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Deposit::Deposits` (r:1 w:1)
	/// Proof: `Deposit::Deposits` (`max_values`: None, `max_size`: Some(26150), added: 28625, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[0, 1023]`.
	fn stake(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1977`
		//  Estimated: `29615`
		// Minimum execution time: 65_025 nanoseconds.
		Weight::from_parts(119_080_833, 0)
			.saturating_add(Weight::from_parts(29615, 0))
			// Standard Error: 1_240
			.saturating_add(Weight::from_parts(6_811, 0).saturating_mul(x.into()))
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}
	/// Storage: `DarwiniaStaking::Ledgers` (r:1 w:1)
	/// Proof: `DarwiniaStaking::Ledgers` (`max_values`: None, `max_size`: Some(1833), added: 4308, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::RingPool` (r:1 w:1)
	/// Proof: `DarwiniaStaking::RingPool` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::KtonPool` (r:1 w:1)
	/// Proof: `DarwiniaStaking::KtonPool` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Deposit::Deposits` (r:1 w:0)
	/// Proof: `Deposit::Deposits` (`max_values`: None, `max_size`: Some(26150), added: 28625, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[0, 1023]`.
	fn unstake(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1589`
		//  Estimated: `29615`
		// Minimum execution time: 11_158 nanoseconds.
		Weight::from_parts(51_460_547, 0)
			.saturating_add(Weight::from_parts(29615, 0))
			// Standard Error: 896
			.saturating_add(Weight::from_parts(3_363, 0).saturating_mul(x.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `DarwiniaStaking::Ledgers` (r:1 w:1)
	/// Proof: `DarwiniaStaking::Ledgers` (`max_values`: None, `max_size`: Some(1833), added: 4308, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::RingPool` (r:1 w:1)
	/// Proof: `DarwiniaStaking::RingPool` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::KtonPool` (r:1 w:1)
	/// Proof: `DarwiniaStaking::KtonPool` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Deposit::Deposits` (r:1 w:0)
	/// Proof: `Deposit::Deposits` (`max_values`: None, `max_size`: Some(26150), added: 28625, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[0, 1023]`.
	fn restake(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1693`
		//  Estimated: `29615`
		// Minimum execution time: 10_061 nanoseconds.
		Weight::from_parts(43_375_019, 0)
			.saturating_add(Weight::from_parts(29615, 0))
			// Standard Error: 750
			.saturating_add(Weight::from_parts(3_582, 0).saturating_mul(x.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `DarwiniaStaking::Ledgers` (r:1 w:1)
	/// Proof: `DarwiniaStaking::Ledgers` (`max_values`: None, `max_size`: Some(1833), added: 4308, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(166), added: 2641, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `Deposit::Deposits` (r:1 w:1)
	/// Proof: `Deposit::Deposits` (`max_values`: None, `max_size`: Some(26150), added: 28625, mode: `MaxEncodedLen`)
	fn claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2215`
		//  Estimated: `29615`
		// Minimum execution time: 87_073 nanoseconds.
		Weight::from_parts(89_352_000, 0)
			.saturating_add(Weight::from_parts(29615, 0))
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: `DarwiniaStaking::Collators` (r:1 w:1)
	/// Proof: `DarwiniaStaking::Collators` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	fn collect() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `410`
		//  Estimated: `3497`
		// Minimum execution time: 7_784 nanoseconds.
		Weight::from_parts(8_159_000, 0)
			.saturating_add(Weight::from_parts(3497, 0))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `DarwiniaStaking::Ledgers` (r:1 w:0)
	/// Proof: `DarwiniaStaking::Ledgers` (`max_values`: None, `max_size`: Some(1833), added: 4308, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::Collators` (r:1 w:0)
	/// Proof: `DarwiniaStaking::Collators` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::Nominators` (r:1 w:1)
	/// Proof: `DarwiniaStaking::Nominators` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	fn nominate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `578`
		//  Estimated: `5298`
		// Minimum execution time: 9_357 nanoseconds.
		Weight::from_parts(9_716_000, 0)
			.saturating_add(Weight::from_parts(5298, 0))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `DarwiniaStaking::Nominators` (r:0 w:1)
	/// Proof: `DarwiniaStaking::Nominators` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::Collators` (r:0 w:1)
	/// Proof: `DarwiniaStaking::Collators` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	fn chill() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_204 nanoseconds.
		Weight::from_parts(3_458_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `DarwiniaStaking::ExposureCacheStates` (r:1 w:0)
	/// Proof: `DarwiniaStaking::ExposureCacheStates` (`max_values`: Some(1), `max_size`: Some(3), added: 498, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::ExposureCache2` (r:1 w:0)
	/// Proof: `DarwiniaStaking::ExposureCache2` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `DarwiniaStaking::PendingRewards` (r:1 w:1)
	/// Proof: `DarwiniaStaking::PendingRewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn payout() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1449`
		//  Estimated: `4914`
		// Minimum execution time: 129_233 nanoseconds.
		Weight::from_parts(131_538_000, 0)
			.saturating_add(Weight::from_parts(4914, 0))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `DarwiniaStaking::CollatorCount` (r:0 w:1)
	/// Proof: `DarwiniaStaking::CollatorCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_collator_count() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_745 nanoseconds.
		Weight::from_parts(1_829_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
