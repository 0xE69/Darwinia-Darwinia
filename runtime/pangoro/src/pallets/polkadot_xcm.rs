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
use crate::*;
// polkadot
use xcm::latest::prelude::*;
// substrate
use frame_support::traits::Currency;

/// Means for transacting assets on this chain.
pub type LocalAssetTransactor = xcm_builder::CurrencyAdapter<
	// Use this currency:
	Balances,
	// Use this currency when it is a fungible asset matching the given location or name:
	xcm_builder::IsConcrete<AnchoringSelfReserve>,
	// Do a simple punn to convert an AccountId32 MultiLocation into a native chain account ID:
	LocationToAccountId,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// We don't track any teleports.
	(),
>;

frame_support::parameter_types! {
	pub const RelayNetwork: NetworkId = NetworkId::Westend;
	pub RelayChainOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();
}
/// Type for specifying how a `MultiLocation` can be converted into an `AccountId`. This is used
/// when determining ownership of accounts for asset transacting and when attempting to use XCM
/// `Transact` in order to determine the dispatch Origin.
pub type LocationToAccountId = (
	// The parent (Relay-chain) origin converts to the parent `AccountId`.
	xcm_builder::ParentIsPreset<AccountId>,
	// Sibling parachain origins convert to AccountId via the `ParaId::into`.
	xcm_builder::SiblingParachainConvertsVia<polkadot_parachain::primitives::Sibling, AccountId>,
	// Straight up local `AccountId20` origins just alias directly to `AccountId`.
	xcm_builder::AccountKey20Aliases<RelayNetwork, AccountId>,
	// The rest of locations are converted via hashing it.
	darwinia_common_runtime::xcm_configs::Account20Hash<AccountId>,
);
/// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
/// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
/// biases the kind of local `Origin` it will become.
pub type XcmOriginToTransactDispatchOrigin = (
	// Sovereign account converter; this attempts to derive an `AccountId` from the origin location
	// using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
	// foreign chains who want to have a local sovereign account on this chain which they control.
	xcm_builder::SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
	// Native converter for Relay-chain (Parent) location; will converts to a `Relay` origin when
	// recognized.
	xcm_builder::RelayChainAsNative<RelayChainOrigin, RuntimeOrigin>,
	// Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
	// recognized.
	xcm_builder::SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
	// Native signed account converter; this just converts an `AccountKey20` origin into a normal
	// `RuntimeOrigin::Signed` origin of the same 20-byte value.
	xcm_builder::SignedAccountKey20AsNative<RelayNetwork, RuntimeOrigin>,
	// Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
	pallet_xcm::XcmPassthrough<RuntimeOrigin>,
);

pub type Barrier = xcm_builder::TrailingSetTopicAsId<
	xcm_builder::DenyThenTry<
		xcm_builder::DenyReserveTransferToRelayChain,
		(
			xcm_builder::TakeWeightCredit,
			xcm_builder::WithComputedOrigin<
				(
					// If the message is one that immediately attemps to pay for execution, then
					// allow it.
					xcm_builder::AllowTopLevelPaidExecutionFrom<frame_support::traits::Everything>,
					// Parent, its pluralities (i.e. governance bodies), and the Fellows plurality
					// get free execution.
					xcm_builder::AllowUnpaidExecutionFrom<
						darwinia_common_runtime::xcm_configs::ParentOrParentsExecutivePlurality,
					>,
					// Subscriptions for version tracking are OK.
					xcm_builder::AllowSubscriptionsFrom<
						darwinia_common_runtime::xcm_configs::ParentOrSiblings,
					>,
				),
				UniversalLocation,
				ConstU32<8>,
			>,
			// Expected responses are OK.
			xcm_builder::AllowKnownQueryResponses<PolkadotXcm>,
		),
	>,
>;

frame_support::parameter_types! {
	pub const MaxAssetsIntoHolding: u32 = 64;
	pub const MaxInstructions: u32 = 100;
	pub AnchoringSelfReserve: MultiLocation = MultiLocation::new(
		0,
		X1(PalletInstance(<Balances as frame_support::traits::PalletInfoAccess>::index() as u8))
	);
	pub UniversalLocation: InteriorMultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
	/// The amount of weight an XCM operation takes. This is a safe overestimate.
	pub BaseXcmWeight: frame_support::weights::Weight = frame_support::weights::Weight::from_parts(1_000_000_000, 1024);
	/// A temporary weight value for each XCM instruction.
	/// NOTE: This should be removed after we account for PoV weights.
	pub const TempFixedXcmWeight: frame_support::weights::Weight = frame_support::weights::Weight::from_parts(1_000_000_000, 0);
}

pub struct ToTreasury;
impl xcm_builder::TakeRevenue for ToTreasury {
	fn take_revenue(revenue: MultiAsset) {
		if let MultiAsset { id: Concrete(_location), fun: Fungible(amount) } = revenue {
			let treasury_account = Treasury::account_id();
			let _ = Balances::deposit_creating(&treasury_account, amount);

			log::trace!(
				target: "xcm::weight",
				"LocalAssetTrader::to_treasury amount: {amount:?}, treasury: {treasury_account:?}"
			);
		}
	}
}

pub struct XcmCallDispatcher;
impl xcm_executor::traits::CallDispatcher<RuntimeCall> for XcmCallDispatcher {
	fn dispatch(
		call: RuntimeCall,
		origin: RuntimeOrigin,
	) -> Result<
		sp_runtime::traits::PostDispatchInfoOf<RuntimeCall>,
		sp_runtime::DispatchErrorWithPostInfo<sp_runtime::traits::PostDispatchInfoOf<RuntimeCall>>,
	> {
		if let Ok(raw_origin) =
			TryInto::<frame_system::RawOrigin<AccountId>>::try_into(origin.clone().caller)
		{
			if let (
				RuntimeCall::EthereumXcm(pallet_ethereum_xcm::Call::transact { .. }),
				frame_system::RawOrigin::Signed(account_id),
			) = (call.clone(), raw_origin)
			{
				return RuntimeCall::dispatch(
					call,
					pallet_ethereum_xcm::Origin::XcmEthereumTransaction(account_id.into()).into(),
				);
			}
		}

		RuntimeCall::dispatch(call, origin)
	}
}

pub struct XcmExecutorConfig;
impl xcm_executor::Config for XcmExecutorConfig {
	type Aliasers = frame_support::traits::Nothing;
	type AssetClaims = PolkadotXcm;
	type AssetExchanger = ();
	type AssetLocker = ();
	// How to withdraw and deposit an asset.
	type AssetTransactor = LocalAssetTransactor;
	type AssetTrap = PolkadotXcm;
	type Barrier = Barrier;
	type CallDispatcher = XcmCallDispatcher;
	type FeeManager = ();
	type IsReserve = xcm_builder::NativeAsset;
	type IsTeleporter = ();
	type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
	type MessageExporter = ();
	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	type PalletInstancesInfo = AllPalletsWithSystem;
	type ResponseHandler = PolkadotXcm;
	type RuntimeCall = RuntimeCall;
	type SafeCallFilter = frame_support::traits::Everything;
	type SubscriptionService = PolkadotXcm;
	type Trader = xcm_configs::LocalAssetTrader<
		frame_support::weights::ConstantMultiplier<
			Balance,
			darwinia_common_runtime::xcm_configs::XcmBaseWeightFee,
		>,
		AnchoringSelfReserve,
		AccountId,
		Balances,
		DealWithFees<Runtime>,
		ToTreasury,
	>;
	type UniversalAliases = frame_support::traits::Nothing;
	// Teleporting is disabled.
	type UniversalLocation = UniversalLocation;
	type Weigher = xcm_builder::FixedWeightBounds<TempFixedXcmWeight, RuntimeCall, MaxInstructions>;
	type XcmSender = XcmRouter;
}

/// No local origins on this chain are allowed to dispatch XCM sends/executions.
pub type LocalOriginToLocation =
	xcm_primitives::SignedToAccountId20<RuntimeOrigin, AccountId, RelayNetwork>;
/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = xcm_builder::WithUniqueTopic<(
	// Two routers - use UMP to communicate with the relay chain:
	cumulus_primitives_utility::ParentAsUmp<ParachainSystem, PolkadotXcm, ()>,
	// ..and XCMP to communicate with the sibling chains.
	XcmpQueue,
)>;

#[cfg(feature = "runtime-benchmarks")]
frame_support::parameter_types! {
	pub ReachableDest: Option<MultiLocation> = Some(Parent.into());
}

impl pallet_xcm::Config for Runtime {
	type AdminOrigin = Root;
	// ^ Override for AdvertisedXcmVersion default
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
	type Currency = Balances;
	type CurrencyMatcher = ();
	type ExecuteXcmOrigin = xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type MaxLockers = ConstU32<8>;
	type MaxRemoteLockConsumers = ();
	#[cfg(feature = "runtime-benchmarks")]
	type ReachableDest = ReachableDest;
	type RemoteLockConsumerIdentifier = ();
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type SendXcmOrigin = xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type SovereignAccountOf = LocationToAccountId;
	type TrustedLockers = ();
	type UniversalLocation = UniversalLocation;
	type Weigher = xcm_builder::FixedWeightBounds<BaseXcmWeight, RuntimeCall, MaxInstructions>;
	type WeightInfo = pallet_xcm::TestWeightInfo;
	type XcmExecuteFilter = frame_support::traits::Everything;
	type XcmExecutor = xcm_executor::XcmExecutor<XcmExecutorConfig>;
	type XcmReserveTransferFilter = frame_support::traits::Everything;
	type XcmRouter = XcmRouter;
	type XcmTeleportFilter = frame_support::traits::Nothing;

	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
}

impl cumulus_pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = xcm_executor::XcmExecutor<XcmExecutorConfig>;
}
