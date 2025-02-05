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
use crate::{mock::*, tests::*};
// frontier
use fp_evm::FeeCalculator;
// substrate
use frame_support::pallet_prelude::Weight;
use sp_core::U256;
use sp_runtime::transaction_validity::{InvalidTransaction, TransactionValidityError};

pub fn legacy_erc20_creation_unsigned_transaction() -> LegacyUnsignedTransaction {
	LegacyUnsignedTransaction {
		nonce: U256::zero(),
		gas_price: U256::from(1),
		gas_limit: U256::from(1_000_000),
		action: ethereum::TransactionAction::Create,
		value: U256::zero(),
		input: array_bytes::hex2bytes_unchecked(ERC20_CONTRACT_BYTECODE),
	}
}

#[test]
fn test_dispatch_legacy_transaction_works() {
	let alice = address_build(1);
	let relayer = address_build(2);

	ExtBuilder::default()
		.with_balances(vec![
			(alice.address, 1_000_000_000_000),
			(relayer.address, 1_000_000_000_000),
		])
		.build()
		.execute_with(|| {
			let mock_message_id = [0; 4];
			let t = legacy_erc20_creation_unsigned_transaction().sign(&alice.private_key);
			let call = RuntimeCall::MessageTransact(crate::Call::message_transact {
				transaction: Box::new(t),
			});
			let message = prepare_message(call);

			let result = Dispatch::dispatch(
				SOURCE_CHAIN_ID,
				TARGET_CHAIN_ID,
				&relayer.address,
				mock_message_id,
				Ok(message),
				|_, _| Ok(()),
			);
			assert!(result.dispatch_result);
			System::assert_has_event(RuntimeEvent::Dispatch(
				pallet_bridge_dispatch::Event::MessageDispatched(
					SOURCE_CHAIN_ID,
					mock_message_id,
					Ok(()),
				),
			));
		});
}

#[test]
fn test_dispatch_legacy_transaction_weight_mismatch() {
	let alice = address_build(1);
	let relayer = address_build(2);

	ExtBuilder::default()
		.with_balances(vec![
			(alice.address, 1_000_000_000_000),
			(relayer.address, 1_000_000_000_000),
		])
		.build()
		.execute_with(|| {
			let mock_message_id = [0; 4];
			let mut unsigned_tx = legacy_erc20_creation_unsigned_transaction();
			unsigned_tx.gas_limit = U256::from(62500001);
			let t = unsigned_tx.sign(&alice.private_key);
			let call = RuntimeCall::MessageTransact(crate::Call::message_transact {
				transaction: Box::new(t),
			});
			let message = prepare_message(call);

			let result = Dispatch::dispatch(
				SOURCE_CHAIN_ID,
				TARGET_CHAIN_ID,
				&relayer.address,
				mock_message_id,
				Ok(message),
				|_, _| Ok(()),
			);

			assert!(!result.dispatch_result);
			System::assert_has_event(RuntimeEvent::Dispatch(
				pallet_bridge_dispatch::Event::MessageWeightMismatch(
					SOURCE_CHAIN_ID,
					mock_message_id,
					Weight::from_parts(1249875606000, 0),
					Weight::from_parts(1000000000000, 0),
				),
			));
		});
}

#[test]
fn test_dispatch_legacy_transaction_with_autoset_nonce() {
	let alice = address_build(1);
	let relayer = address_build(2);

	ExtBuilder::default()
		.with_balances(vec![
			(alice.address, 1_000_000_000_000),
			(relayer.address, 1_000_000_000_000),
		])
		.build()
		.execute_with(|| {
			let mock_message_id = [0; 4];
			let mut unsigned_tx = legacy_erc20_creation_unsigned_transaction();
			unsigned_tx.nonce = U256::MAX;
			let t = unsigned_tx.sign(&alice.private_key);
			let call = RuntimeCall::MessageTransact(crate::Call::message_transact {
				transaction: Box::new(t),
			});
			let message = prepare_message(call);

			let result = Dispatch::dispatch(
				SOURCE_CHAIN_ID,
				TARGET_CHAIN_ID,
				&relayer.address,
				mock_message_id,
				Ok(message),
				|_, _| Ok(()),
			);

			assert!(result.dispatch_result);
		});
}

#[test]
fn test_dispatch_legacy_transaction_with_autoset_gas_price() {
	let alice = address_build(1);
	let relayer = address_build(2);

	ExtBuilder::default()
		.with_balances(vec![
			(alice.address, 1_000_000_000_000),
			(relayer.address, 1_000_000_000_000),
		])
		.build()
		.execute_with(|| {
			let mock_message_id = [0; 4];
			let mut unsigned_tx = legacy_erc20_creation_unsigned_transaction();
			unsigned_tx.gas_price =
				<Runtime as pallet_evm::Config>::FeeCalculator::min_gas_price().0 - 1;
			let t = unsigned_tx.sign(&alice.private_key);
			let call = RuntimeCall::MessageTransact(crate::Call::message_transact {
				transaction: Box::new(t),
			});
			let message = prepare_message(call);

			let result = Dispatch::dispatch(
				SOURCE_CHAIN_ID,
				TARGET_CHAIN_ID,
				&relayer.address,
				mock_message_id,
				Ok(message),
				|_, _| Ok(()),
			);

			assert!(result.dispatch_result);
		});
}

#[test]
fn test_dispatch_legacy_transaction_with_insufficient_relayer_balance() {
	let alice = address_build(1);
	let relayer1 = address_build(2);
	let relayer2 = address_build(3);

	ExtBuilder::default()
		.with_balances(vec![
			(alice.address, 1_000_000_000_000),
			(relayer1.address, 1_000),
			(relayer2.address, 1_000_000_000_000),
		])
		.build()
		.execute_with(|| {
			let mock_message_id = [0; 4];
			let unsigned_tx = legacy_erc20_creation_unsigned_transaction();
			let t = unsigned_tx.sign(&alice.private_key);
			let call = RuntimeCall::MessageTransact(crate::Call::message_transact {
				transaction: Box::new(t),
			});
			let message = prepare_message(call);

			// Failed in pre-dispatch balance check
			let before_dispatch =
				pallet_evm::Pallet::<Runtime>::account_basic(&relayer1.address).0.balance;
			let result = Dispatch::dispatch(
				SOURCE_CHAIN_ID,
				TARGET_CHAIN_ID,
				&relayer1.address,
				mock_message_id,
				Ok(message.clone()),
				|_, _| Ok(()),
			);
			assert!(!result.dispatch_result);
			System::assert_has_event(RuntimeEvent::Dispatch(
				pallet_bridge_dispatch::Event::MessageCallValidateFailed(
					SOURCE_CHAIN_ID,
					mock_message_id,
					TransactionValidityError::Invalid(InvalidTransaction::Payment),
				),
			));
			let after_dispatch =
				pallet_evm::Pallet::<Runtime>::account_basic(&relayer1.address).0.balance;
			assert_eq!(before_dispatch, after_dispatch);

			let before_dispatch =
				pallet_evm::Pallet::<Runtime>::account_basic(&relayer2.address).0.balance;
			let result = Dispatch::dispatch(
				SOURCE_CHAIN_ID,
				TARGET_CHAIN_ID,
				&relayer2.address,
				mock_message_id,
				Ok(message),
				|_, _| Ok(()),
			);
			assert!(result.dispatch_result);
			let after_dispatch =
				pallet_evm::Pallet::<Runtime>::account_basic(&relayer2.address).0.balance;
			assert!(before_dispatch > after_dispatch);
		});
}
