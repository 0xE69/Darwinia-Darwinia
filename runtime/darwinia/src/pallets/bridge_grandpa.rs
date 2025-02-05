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

pub use pallet_bridge_grandpa::Instance1 as WithKusamaGrandpa;

// darwinia
use crate::*;

pub type KusamaHeadersToKeep = ConstU32<500>;

impl pallet_bridge_grandpa::Config<WithKusamaGrandpa> for Runtime {
	type BridgedChain = bp_crab::DarwiniaLike;
	type HeadersToKeep = KusamaHeadersToKeep;
	type MaxBridgedAuthorities = ConstU32<4_096>;
	// Kusama chain currently has 1000 validators.
	// Double the default value `65536` here.
	type MaxBridgedHeaderSize = ConstU32<131_072>;
	type MaxRequests = ConstU32<50>;
	type WeightInfo = weights::pallet_bridge_grandpa::WeightInfo<Self>;
}
