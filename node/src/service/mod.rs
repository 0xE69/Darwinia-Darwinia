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

//! Service and service factory implementation. Specialized wrapper over substrate service.

pub mod executors;
pub use executors::*;

pub mod frontier;

mod instant_finalize;

#[cfg(feature = "crab-native")]
pub use crab_runtime::RuntimeApi as CrabRuntimeApi;
#[cfg(feature = "darwinia-native")]
pub use darwinia_runtime::RuntimeApi as DarwiniaRuntimeApi;
#[cfg(feature = "pangolin-native")]
pub use pangolin_runtime::RuntimeApi as PangolinRuntimeApi;
#[cfg(feature = "pangoro-native")]
pub use pangoro_runtime::RuntimeApi as PangoroRuntimeApi;

// std
use std::{
	collections::BTreeMap,
	sync::{Arc, Mutex},
	time::Duration,
};
// crates.io
use futures::FutureExt;
// darwinia
use dc_primitives::*;
// substrate
use sc_client_api::Backend;
use sc_consensus::ImportQueue;
use sc_network::NetworkBlock;

/// Full client backend type.
type FullBackend = sc_service::TFullBackend<Block>;
/// Full client type.
type FullClient<RuntimeApi, Executor> =
	sc_service::TFullClient<Block, RuntimeApi, sc_executor::NativeElseWasmExecutor<Executor>>;
/// Parachain specific block import.
type ParachainBlockImport<RuntimeApi, Executor> =
	cumulus_client_consensus_common::ParachainBlockImport<
		Block,
		Arc<FullClient<RuntimeApi, Executor>>,
		FullBackend,
	>;

/// Can be called for a `Configuration` to check if it is the specific network.
pub trait IdentifyVariant {
	/// Get spec id.
	fn id(&self) -> &str;

	/// Returns if this is a configuration for the `Crab` network.
	fn is_crab(&self) -> bool {
		self.id().starts_with("crab")
	}

	/// Returns if this is a configuration for the `Darwinia` network.
	fn is_darwinia(&self) -> bool {
		self.id().starts_with("darwinia")
	}

	/// Returns if this is a configuration for the `Pangolin` network.
	fn is_pangolin(&self) -> bool {
		self.id().starts_with("pangolin")
	}

	/// Returns if this is a configuration for the `Pangoro` network.
	fn is_pangoro(&self) -> bool {
		self.id().starts_with("pangoro")
	}

	/// Returns true if this configuration is for a development network.
	fn is_dev(&self) -> bool {
		// Fulfill Polkadot.JS metadata upgrade requirements.
		self.id().ends_with("-d")
	}
}
impl IdentifyVariant for Box<dyn sc_service::ChainSpec> {
	fn id(&self) -> &str {
		sc_service::ChainSpec::id(&**self)
	}
}

/// A set of APIs that darwinia-like runtimes must implement.
pub trait RuntimeApiCollection:
	cumulus_primitives_core::CollectCollationInfo<Block>
	+ fp_rpc::ConvertTransactionRuntimeApi<Block>
	+ fp_rpc::EthereumRuntimeRPCApi<Block>
	+ moonbeam_rpc_primitives_debug::DebugRuntimeApi<Block>
	+ pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>
	+ sp_api::ApiExt<Block>
	+ sp_api::Metadata<Block>
	+ sp_block_builder::BlockBuilder<Block>
	+ sp_consensus_aura::AuraApi<Block, <<sp_consensus_aura::sr25519::AuthorityId as sp_runtime::app_crypto::AppCrypto>::Pair as sp_core::Pair>::Public>
	+ sp_offchain::OffchainWorkerApi<Block>
	+ sp_session::SessionKeys<Block>
	+ sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
	+ substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>
{
}
impl<Api> RuntimeApiCollection for Api where
	Api: cumulus_primitives_core::CollectCollationInfo<Block>
		+ fp_rpc::ConvertTransactionRuntimeApi<Block>
		+ fp_rpc::EthereumRuntimeRPCApi<Block>
		+ moonbeam_rpc_primitives_debug::DebugRuntimeApi<Block>
		+ pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>
		+ sp_api::ApiExt<Block>
		+ sp_api::Metadata<Block>
		+ sp_block_builder::BlockBuilder<Block>
		+ sp_consensus_aura::AuraApi<Block, <<sp_consensus_aura::sr25519::AuthorityId as sp_runtime::app_crypto::AppCrypto>::Pair as sp_core::Pair>::Public>
		+ sp_offchain::OffchainWorkerApi<Block>
		+ sp_session::SessionKeys<Block>
		+ sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
		+ substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>
{
}

/// Starts a `ServiceBuilder` for a full service.
///
/// Use this macro if you don't actually need the full service, but just the builder in order to
/// be able to perform chain operations.
#[allow(clippy::type_complexity)]
pub fn new_partial<RuntimeApi, Executor>(
	config: &sc_service::Configuration,
	eth_rpc_config: &crate::cli::EthRpcConfig,
) -> Result<
	sc_service::PartialComponents<
		FullClient<RuntimeApi, Executor>,
		FullBackend,
		sc_consensus::LongestChain<FullBackend, Block>,
		sc_consensus::DefaultImportQueue<Block>,
		sc_transaction_pool::FullPool<Block, FullClient<RuntimeApi, Executor>>,
		(
			fc_db::Backend<Block>,
			Option<fc_rpc_core::types::FilterPool>,
			fc_rpc_core::types::FeeHistoryCache,
			fc_rpc_core::types::FeeHistoryCacheLimit,
			ParachainBlockImport<RuntimeApi, Executor>,
			Option<sc_telemetry::Telemetry>,
			Option<sc_telemetry::TelemetryWorkerHandle>,
		),
	>,
	sc_service::Error,
>
where
	RuntimeApi: 'static
		+ Send
		+ Sync
		+ sp_api::ConstructRuntimeApi<Block, FullClient<RuntimeApi, Executor>>,
	RuntimeApi::RuntimeApi: RuntimeApiCollection,
	Executor: 'static + sc_executor::NativeExecutionDispatch,
{
	let telemetry = config
		.telemetry_endpoints
		.clone()
		.filter(|x| !x.is_empty())
		.map(|endpoints| -> Result<_, sc_telemetry::Error> {
			let worker = sc_telemetry::TelemetryWorker::new(16)?;
			let telemetry = worker.handle().new_telemetry(endpoints);
			Ok((worker, telemetry))
		})
		.transpose()?;
	let heap_pages =
		config.default_heap_pages.map_or(sc_executor::DEFAULT_HEAP_ALLOC_STRATEGY, |h| {
			sc_executor::HeapAllocStrategy::Static { extra_pages: h as _ }
		});
	let wasm_executor = sc_executor::WasmExecutor::builder()
		.with_execution_method(config.wasm_method)
		.with_max_runtime_instances(config.max_runtime_instances)
		.with_runtime_cache_size(config.runtime_cache_size)
		.with_onchain_heap_alloc_strategy(heap_pages)
		.with_offchain_heap_alloc_strategy(heap_pages)
		.build();
	let executor =
		<sc_executor::NativeElseWasmExecutor<Executor>>::new_with_wasm_executor(wasm_executor);
	let (client, backend, keystore_container, task_manager) =
		sc_service::new_full_parts::<Block, RuntimeApi, _>(
			config,
			telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
			executor,
		)?;
	let client = Arc::new(client);
	let telemetry_worker_handle = telemetry.as_ref().map(|(worker, _)| worker.handle());
	let telemetry = telemetry.map(|(worker, telemetry)| {
		task_manager.spawn_handle().spawn("telemetry", None, worker.run());
		telemetry
	});
	let transaction_pool = sc_transaction_pool::BasicPool::new_full(
		config.transaction_pool.clone(),
		config.role.is_authority().into(),
		config.prometheus_registry(),
		task_manager.spawn_essential_handle(),
		client.clone(),
	);
	let block_import = ParachainBlockImport::new(client.clone(), backend.clone());
	let import_queue = parachain_build_import_queue(
		client.clone(),
		block_import.clone(),
		config,
		telemetry.as_ref().map(|telemetry| telemetry.handle()),
		&task_manager,
	)?;
	// Frontier stuffs.
	let frontier_backend = frontier::backend(client.clone(), config, eth_rpc_config.clone())?;
	let filter_pool = Some(Arc::new(Mutex::new(BTreeMap::new())));
	let fee_history_cache = Arc::new(Mutex::new(BTreeMap::new()));
	let fee_history_cache_limit = eth_rpc_config.fee_history_limit;

	Ok(sc_service::PartialComponents {
		backend: backend.clone(),
		client,
		import_queue,
		keystore_container,
		task_manager,
		transaction_pool,
		select_chain: sc_consensus::LongestChain::new(backend),
		other: (
			frontier_backend,
			filter_pool,
			fee_history_cache,
			fee_history_cache_limit,
			block_import,
			telemetry,
			telemetry_worker_handle,
		),
	})
}

/// Start a node with the given parachain `Configuration` and relay chain `Configuration`.
///
/// This is the actual implementation that is abstract over the executor and the runtime api.
#[allow(clippy::too_many_arguments)]
#[sc_tracing::logging::prefix_logs_with("Parachain")]
async fn start_node_impl<RuntimeApi, Executor, SC>(
	parachain_config: sc_service::Configuration,
	polkadot_config: sc_service::Configuration,
	collator_options: cumulus_client_cli::CollatorOptions,
	sybil_resistance_level: cumulus_client_service::CollatorSybilResistance,
	para_id: cumulus_primitives_core::ParaId,
	start_consensus: SC,
	hwbench: Option<sc_sysinfo::HwBench>,
	eth_rpc_config: &crate::cli::EthRpcConfig,
) -> sc_service::error::Result<(sc_service::TaskManager, Arc<FullClient<RuntimeApi, Executor>>)>
where
	RuntimeApi: 'static
		+ Send
		+ Sync
		+ sp_api::ConstructRuntimeApi<Block, FullClient<RuntimeApi, Executor>>,
	RuntimeApi::RuntimeApi: RuntimeApiCollection,
	Executor: 'static + sc_executor::NativeExecutionDispatch,
	SC: FnOnce(
		Arc<FullClient<RuntimeApi, Executor>>,
		ParachainBlockImport<RuntimeApi, Executor>,
		Option<&substrate_prometheus_endpoint::Registry>,
		Option<sc_telemetry::TelemetryHandle>,
		&sc_service::TaskManager,
		Arc<dyn cumulus_relay_chain_interface::RelayChainInterface>,
		Arc<sc_transaction_pool::FullPool<Block, FullClient<RuntimeApi, Executor>>>,
		Arc<sc_network_sync::SyncingService<Block>>,
		sp_keystore::KeystorePtr,
		Duration,
		cumulus_primitives_core::ParaId,
		polkadot_primitives::CollatorPair,
		cumulus_relay_chain_interface::OverseerHandle,
		Arc<dyn Fn(Hash, Option<Vec<u8>>) + Send + Sync>,
	) -> Result<(), sc_service::Error>,
{
	let mut parachain_config = cumulus_client_service::prepare_node_config(parachain_config);
	let sc_service::PartialComponents {
		backend,
		client,
		import_queue,
		keystore_container,
		mut task_manager,
		transaction_pool,
		select_chain: _,
		other:
			(
				frontier_backend,
				filter_pool,
				fee_history_cache,
				fee_history_cache_limit,
				block_import,
				mut telemetry,
				telemetry_worker_handle,
			),
	} = new_partial::<RuntimeApi, Executor>(&parachain_config, eth_rpc_config)?;

	let (relay_chain_interface, collator_key) =
		cumulus_client_service::build_relay_chain_interface(
			polkadot_config,
			&parachain_config,
			telemetry_worker_handle,
			&mut task_manager,
			collator_options.clone(),
			hwbench.clone(),
		)
		.await
		.map_err(|e| sc_service::Error::Application(Box::new(e) as Box<_>))?;

	let validator = parachain_config.role.is_authority();
	let prometheus_registry = parachain_config.prometheus_registry().cloned();
	let import_queue_service = import_queue.service();
	let net_config = sc_network::config::FullNetworkConfiguration::new(&parachain_config.network);

	let (network, system_rpc_tx, tx_handler_controller, start_network, sync_service) =
		cumulus_client_service::build_network(cumulus_client_service::BuildNetworkParams {
			parachain_config: &parachain_config,
			net_config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			para_id,
			spawn_handle: task_manager.spawn_handle(),
			relay_chain_interface: relay_chain_interface.clone(),
			import_queue,
			sybil_resistance_level,
		})
		.await?;

	if parachain_config.offchain_worker.enabled {
		task_manager.spawn_handle().spawn(
			"offchain-workers-runner",
			"offchain-work",
			sc_offchain::OffchainWorkers::new(sc_offchain::OffchainWorkerOptions {
				runtime_api_provider: client.clone(),
				keystore: Some(keystore_container.keystore()),
				offchain_db: backend.offchain_storage(),
				transaction_pool: Some(
					sc_transaction_pool_api::OffchainTransactionPoolFactory::new(
						transaction_pool.clone(),
					),
				),
				network_provider: network.clone(),
				is_validator: parachain_config.role.is_authority(),
				enable_http_requests: false,
				custom_extensions: move |_| Vec::new(),
			})
			.run(client.clone(), task_manager.spawn_handle())
			.boxed(),
		);
	}

	let overrides = fc_storage::overrides_handle(client.clone());
	let block_data_cache = Arc::new(fc_rpc::EthBlockDataCacheTask::new(
		task_manager.spawn_handle(),
		overrides.clone(),
		eth_rpc_config.eth_log_block_cache,
		eth_rpc_config.eth_statuses_cache,
		prometheus_registry.clone(),
	));
	let pubsub_notification_sinks: fc_mapping_sync::EthereumBlockNotificationSinks<
		fc_mapping_sync::EthereumBlockNotification<Block>,
	> = Default::default();
	let pubsub_notification_sinks = Arc::new(pubsub_notification_sinks);
	// for ethereum-compatibility rpc.
	parachain_config.rpc_id_provider = Some(Box::new(fc_rpc::EthereumSubIdProvider));
	let tracing_requesters = frontier::spawn_tasks(
		&task_manager,
		client.clone(),
		backend.clone(),
		frontier_backend.clone(),
		filter_pool.clone(),
		overrides.clone(),
		fee_history_cache.clone(),
		fee_history_cache_limit,
		sync_service.clone(),
		pubsub_notification_sinks.clone(),
		eth_rpc_config.clone(),
		prometheus_registry.clone(),
	);
	let rpc_builder = {
		let client = client.clone();
		let pool = transaction_pool.clone();
		let network = network.clone();
		let filter_pool = filter_pool.clone();
		let frontier_backend = frontier_backend.clone();
		let overrides = overrides;
		let fee_history_cache = fee_history_cache.clone();
		let max_past_logs = eth_rpc_config.max_past_logs;
		let collator = parachain_config.role.is_authority();
		let eth_rpc_config = eth_rpc_config.clone();
		let sync_service = sync_service.clone();

		let slot_duration = sc_consensus_aura::slot_duration(&*client)?;
		let pending_create_inherent_data_providers = move |_, ()| async move {
			let current = sp_timestamp::InherentDataProvider::from_system_time();
			let next_slot = current.timestamp().as_millis() + slot_duration.as_millis();
			let timestamp = sp_timestamp::InherentDataProvider::new(next_slot.into());
			let slot = sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
				*timestamp,
				slot_duration,
			);
			Ok((slot, timestamp))
		};

		Box::new(move |deny_unsafe, subscription_task_executor| {
			let deps = crate::rpc::FullDeps {
				client: client.clone(),
				pool: pool.clone(),
				graph: pool.pool().clone(),
				deny_unsafe,
				is_authority: collator,
				network: network.clone(),
				sync: sync_service.clone(),
				filter_pool: filter_pool.clone(),
				frontier_backend: match frontier_backend.clone() {
					fc_db::Backend::KeyValue(bd) => Arc::new(bd),
					fc_db::Backend::Sql(bd) => Arc::new(bd),
				},
				max_past_logs,
				fee_history_cache: fee_history_cache.clone(),
				fee_history_cache_limit,
				overrides: overrides.clone(),
				block_data_cache: block_data_cache.clone(),
				forced_parent_hashes: None,
				pending_create_inherent_data_providers,
			};

			if eth_rpc_config.tracing_api.contains(&crate::cli::TracingApi::Debug)
				|| eth_rpc_config.tracing_api.contains(&crate::cli::TracingApi::Trace)
			{
				crate::rpc::create_full::<_, _, _, _, crate::rpc::DefaultEthConfig<_, _>, _>(
					deps,
					subscription_task_executor,
					pubsub_notification_sinks.clone(),
					Some(crate::rpc::TracingConfig {
						tracing_requesters: tracing_requesters.clone(),
						trace_filter_max_count: eth_rpc_config.tracing_max_count,
					}),
				)
				.map_err(Into::into)
			} else {
				crate::rpc::create_full::<_, _, _, _, crate::rpc::DefaultEthConfig<_, _>, _>(
					deps,
					subscription_task_executor,
					pubsub_notification_sinks.clone(),
					None,
				)
				.map_err(Into::into)
			}
		})
	};

	sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		rpc_builder,
		client: client.clone(),
		transaction_pool: transaction_pool.clone(),
		task_manager: &mut task_manager,
		config: parachain_config,
		keystore: keystore_container.keystore(),
		backend: backend.clone(),
		network,
		sync_service: sync_service.clone(),
		system_rpc_tx,
		tx_handler_controller,
		telemetry: telemetry.as_mut(),
	})?;

	if let Some(hwbench) = hwbench {
		sc_sysinfo::print_hwbench(&hwbench);
		// Here you can check whether the hardware meets your chains' requirements. Putting a link
		// in there and swapping out the requirements for your own are probably a good idea. The
		// requirements for a para-chain are dictated by its relay-chain.
		if !frame_benchmarking_cli::SUBSTRATE_REFERENCE_HARDWARE.check_hardware(&hwbench)
			&& validator
		{
			log::warn!(
				"⚠️  The hardware does not meet the minimal requirements for role 'Authority'."
			);
		}

		if let Some(ref mut telemetry) = telemetry {
			let telemetry_handle = telemetry.handle();
			task_manager.spawn_handle().spawn(
				"telemetry_hwbench",
				None,
				sc_sysinfo::initialize_hwbench_telemetry(telemetry_handle, hwbench),
			);
		}
	}

	let announce_block = {
		let sync_service = sync_service.clone();
		Arc::new(move |hash, data| sync_service.announce_block(hash, data))
	};
	let relay_chain_slot_duration = Duration::from_secs(6);
	let overseer_handle = relay_chain_interface
		.overseer_handle()
		.map_err(|e| sc_service::Error::Application(Box::new(e)))?;

	cumulus_client_service::start_relay_chain_tasks(
		cumulus_client_service::StartRelayChainTasksParams {
			client: client.clone(),
			announce_block: announce_block.clone(),
			para_id,
			relay_chain_interface: relay_chain_interface.clone(),
			task_manager: &mut task_manager,
			da_recovery_profile: if validator {
				cumulus_client_service::DARecoveryProfile::Collator
			} else {
				cumulus_client_service::DARecoveryProfile::FullNode
			},
			import_queue: import_queue_service,
			relay_chain_slot_duration,
			recovery_handle: Box::new(overseer_handle.clone()),
			sync_service: sync_service.clone(),
		},
	)?;

	if validator {
		start_consensus(
			client.clone(),
			block_import,
			prometheus_registry.as_ref(),
			telemetry.as_ref().map(|t| t.handle()),
			&task_manager,
			relay_chain_interface.clone(),
			transaction_pool,
			sync_service,
			keystore_container.keystore(),
			relay_chain_slot_duration,
			para_id,
			collator_key.expect("Command line arguments do not allow this. qed"),
			overseer_handle,
			announce_block,
		)?;
	}

	start_network.start_network();

	Ok((task_manager, client))
}

/// Build the import queue for the parachain runtime.
pub fn parachain_build_import_queue<RuntimeApi, Executor>(
	client: Arc<FullClient<RuntimeApi, Executor>>,
	block_import: ParachainBlockImport<RuntimeApi, Executor>,
	config: &sc_service::Configuration,
	telemetry: Option<sc_telemetry::TelemetryHandle>,
	task_manager: &sc_service::TaskManager,
) -> Result<sc_consensus::DefaultImportQueue<Block>, sc_service::Error>
where
	RuntimeApi: 'static
		+ Send
		+ Sync
		+ sp_api::ConstructRuntimeApi<Block, FullClient<RuntimeApi, Executor>>,
	RuntimeApi::RuntimeApi: RuntimeApiCollection,
	Executor: 'static + sc_executor::NativeExecutionDispatch,
{
	let slot_duration = cumulus_client_consensus_aura::slot_duration(&*client)?;

	cumulus_client_consensus_aura::import_queue::<
		sp_consensus_aura::sr25519::AuthorityPair,
		_,
		_,
		_,
		_,
		_,
	>(cumulus_client_consensus_aura::ImportQueueParams {
		block_import,
		client,
		create_inherent_data_providers: move |_, _| async move {
			let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

			let slot =
				sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
					*timestamp,
					slot_duration,
				);

			Ok((slot, timestamp))
		},
		registry: config.prometheus_registry(),
		spawner: &task_manager.spawn_essential_handle(),
		telemetry,
	})
	.map_err(Into::into)
}

/// Start a parachain node.
pub async fn start_parachain_node<RuntimeApi, Executor>(
	parachain_config: sc_service::Configuration,
	polkadot_config: sc_service::Configuration,
	collator_options: cumulus_client_cli::CollatorOptions,
	para_id: cumulus_primitives_core::ParaId,
	hwbench: Option<sc_sysinfo::HwBench>,
	eth_rpc_config: &crate::cli::EthRpcConfig,
) -> sc_service::error::Result<(sc_service::TaskManager, Arc<FullClient<RuntimeApi, Executor>>)>
where
	RuntimeApi: sp_api::ConstructRuntimeApi<Block, FullClient<RuntimeApi, Executor>>
		+ Send
		+ Sync
		+ 'static,
	RuntimeApi::RuntimeApi: RuntimeApiCollection,
	RuntimeApi::RuntimeApi:
		sp_consensus_aura::AuraApi<Block, sp_consensus_aura::sr25519::AuthorityId>,
	Executor: 'static + sc_executor::NativeExecutionDispatch,
{
	start_node_impl::<RuntimeApi, Executor, _>(
		parachain_config,
		polkadot_config,
		collator_options,
		cumulus_client_service::CollatorSybilResistance::Resistant, // Aura
		para_id,
		|client,
		 block_import,
		 prometheus_registry,
		 telemetry,
		 task_manager,
		 relay_chain_interface,
		 transaction_pool,
		 sync_oracle,
		 keystore,
		 relay_chain_slot_duration,
		 para_id,
		 collator_key,
		 overseer_handle,
		 announce_block| {
			let slot_duration = cumulus_client_consensus_aura::slot_duration(&*client)?;
			let proposer_factory = sc_basic_authorship::ProposerFactory::with_proof_recording(
				task_manager.spawn_handle(),
				client.clone(),
				transaction_pool,
				prometheus_registry,
				telemetry,
			);
			let proposer = cumulus_client_consensus_proposer::Proposer::new(proposer_factory);
			let collator_service = cumulus_client_collator::service::CollatorService::new(
				client.clone(),
				Arc::new(task_manager.spawn_handle()),
				announce_block,
				client.clone(),
			);
			let params = cumulus_client_consensus_aura::collators::basic::Params {
				create_inherent_data_providers: move |_, ()| async move { Ok(()) },
				block_import,
				para_client: client,
				relay_client: relay_chain_interface,
				sync_oracle,
				keystore,
				collator_key,
				para_id,
				overseer_handle,
				slot_duration,
				relay_chain_slot_duration,
				proposer,
				collator_service,
				// Very limited proposal time.
				authoring_duration: Duration::from_millis(500),
			};
			let fut = cumulus_client_consensus_aura::collators::basic::run::<
				Block,
				sp_consensus_aura::sr25519::AuthorityPair,
				_,
				_,
				_,
				_,
				_,
				_,
				_,
			>(params);

			task_manager.spawn_essential_handle().spawn("aura", None, fut);

			Ok(())
		},
		hwbench,
		eth_rpc_config,
	)
	.await
}

/// Start a dev node which can seal instantly.
/// !!! WARNING: DO NOT USE ELSEWHERE
pub fn start_dev_node<RuntimeApi, Executor>(
	mut config: sc_service::Configuration,
	eth_rpc_config: &crate::cli::EthRpcConfig,
) -> Result<sc_service::TaskManager, sc_service::error::Error>
where
	RuntimeApi: sp_api::ConstructRuntimeApi<Block, FullClient<RuntimeApi, Executor>>
		+ Send
		+ Sync
		+ 'static,
	RuntimeApi::RuntimeApi: RuntimeApiCollection,
	RuntimeApi::RuntimeApi:
		sp_consensus_aura::AuraApi<Block, sp_consensus_aura::sr25519::AuthorityId>,
	Executor: 'static + sc_executor::NativeExecutionDispatch,
{
	// substrate
	use sc_client_api::HeaderBackend;

	let sc_service::PartialComponents {
		client,
		backend,
		mut task_manager,
		import_queue,
		keystore_container,
		select_chain,
		transaction_pool,
		other:
			(
				frontier_backend,
				filter_pool,
				fee_history_cache,
				fee_history_cache_limit,
				_block_import,
				_telemetry,
				_telemetry_worker_handle,
			),
	} = new_partial::<RuntimeApi, Executor>(&config, eth_rpc_config)?;
	let net_config = sc_network::config::FullNetworkConfiguration::new(&config.network);

	let (network, system_rpc_tx, tx_handler_controller, start_network, sync_service) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &config,
			net_config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue,
			block_announce_validator_builder: None,
			warp_sync_params: None,
		})?;

	if config.offchain_worker.enabled {
		task_manager.spawn_handle().spawn(
			"offchain-workers-runner",
			"offchain-work",
			sc_offchain::OffchainWorkers::new(sc_offchain::OffchainWorkerOptions {
				runtime_api_provider: client.clone(),
				keystore: None,
				offchain_db: backend.offchain_storage(),
				transaction_pool: Some(
					sc_transaction_pool_api::OffchainTransactionPoolFactory::new(
						transaction_pool.clone(),
					),
				),
				network_provider: network.clone(),
				is_validator: config.role.is_authority(),
				enable_http_requests: false,
				custom_extensions: move |_| vec![],
			})
			.run(client.clone(), task_manager.spawn_handle())
			.boxed(),
		);
	}

	let force_authoring = config.force_authoring;
	let backoff_authoring_blocks: Option<()> = None;
	let proposer_factory = sc_basic_authorship::ProposerFactory::new(
		task_manager.spawn_handle(),
		client.clone(),
		transaction_pool.clone(),
		None,
		None,
	);

	let slot_duration = sc_consensus_aura::slot_duration(&*client)?;
	let client_for_cidp = client.clone();
	if config.role.is_authority() {
		let aura = sc_consensus_aura::start_aura::<
			sp_consensus_aura::sr25519::AuthorityPair,
			_,
			_,
			_,
			_,
			_,
			_,
			_,
			_,
			_,
			_,
		>(sc_consensus_aura::StartAuraParams {
			slot_duration: sc_consensus_aura::slot_duration(&*client)?,
			client: client.clone(),
			select_chain,
			block_import: instant_finalize::InstantFinalizeBlockImport::new(client.clone()),
			proposer_factory,
			create_inherent_data_providers: move |block: Hash, ()| {
				let current_para_block = client_for_cidp
					.number(block)
					.expect("Header lookup should succeed")
					.expect("Header passed in as parent should be present in backend.");
				let client_for_xcm = client_for_cidp.clone();

				async move {
					let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

					let slot = sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
						*timestamp,
						slot_duration,
					);

					let mocked_parachain =
						cumulus_primitives_parachain_inherent::MockValidationDataInherentDataProvider {
							current_para_block,
							relay_offset: 1000,
							relay_blocks_per_para_block: 2,
							para_blocks_per_relay_epoch: 0,
							relay_randomness_config: (),
							xcm_config: cumulus_primitives_parachain_inherent::MockXcmConfig::new(
								&*client_for_xcm,
								block,
								Default::default(),
								Default::default(),
							),
							raw_downward_messages: Vec::new(),
							raw_horizontal_messages: Vec::new(),
						};

					Ok((slot, timestamp, mocked_parachain))
				}
			},
			force_authoring,
			backoff_authoring_blocks,
			keystore: keystore_container.keystore(),
			sync_oracle: sync_service.clone(),
			justification_sync_link: sync_service.clone(),
			// We got around 500ms for proposing
			block_proposal_slot_portion: cumulus_client_consensus_aura::SlotProportion::new(
				1f32 / 24f32,
			),
			// And a maximum of 750ms if slots are skipped
			max_block_proposal_slot_portion: Some(
				cumulus_client_consensus_aura::SlotProportion::new(1f32 / 16f32),
			),
			telemetry: None,
			compatibility_mode: Default::default(),
		})?;

		// the AURA authoring task is considered essential, i.e. if it
		// fails we take down the service with it.
		task_manager.spawn_essential_handle().spawn_blocking("aura", Some("block-authoring"), aura);
	} else {
		log::warn!("You could add --alice or --bob to make dev chain seal instantly.");
	}

	let prometheus_registry = config.prometheus_registry().cloned();
	let overrides = fc_storage::overrides_handle(client.clone());
	let block_data_cache = Arc::new(fc_rpc::EthBlockDataCacheTask::new(
		task_manager.spawn_handle(),
		overrides.clone(),
		eth_rpc_config.eth_log_block_cache,
		eth_rpc_config.eth_statuses_cache,
		prometheus_registry.clone(),
	));
	let pubsub_notification_sinks: fc_mapping_sync::EthereumBlockNotificationSinks<
		fc_mapping_sync::EthereumBlockNotification<Block>,
	> = Default::default();
	let pubsub_notification_sinks = Arc::new(pubsub_notification_sinks);
	// for ethereum-compatibility rpc.
	config.rpc_id_provider = Some(Box::new(fc_rpc::EthereumSubIdProvider));
	let tracing_requesters = frontier::spawn_tasks(
		&task_manager,
		client.clone(),
		backend.clone(),
		frontier_backend.clone(),
		filter_pool.clone(),
		overrides.clone(),
		fee_history_cache.clone(),
		fee_history_cache_limit,
		sync_service.clone(),
		pubsub_notification_sinks.clone(),
		eth_rpc_config.clone(),
		prometheus_registry,
	);
	let rpc_extensions_builder = {
		let client = client.clone();
		let pool = transaction_pool.clone();
		let network = network.clone();
		let filter_pool = filter_pool;
		let frontier_backend = frontier_backend;
		let overrides = overrides;
		let fee_history_cache = fee_history_cache;
		let max_past_logs = eth_rpc_config.max_past_logs;
		let collator = config.role.is_authority();
		let eth_rpc_config = eth_rpc_config.clone();
		let sync_service = sync_service.clone();

		let slot_duration = sc_consensus_aura::slot_duration(&*client)?;
		let pending_create_inherent_data_providers = move |_, ()| async move {
			let current = sp_timestamp::InherentDataProvider::from_system_time();
			let next_slot = current.timestamp().as_millis() + slot_duration.as_millis();
			let timestamp = sp_timestamp::InherentDataProvider::new(next_slot.into());
			let slot = sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
				*timestamp,
				slot_duration,
			);
			Ok((slot, timestamp))
		};

		Box::new(move |deny_unsafe, subscription_task_executor| {
			let deps = crate::rpc::FullDeps {
				client: client.clone(),
				pool: pool.clone(),
				graph: pool.pool().clone(),
				deny_unsafe,
				is_authority: collator,
				network: network.clone(),
				sync: sync_service.clone(),
				filter_pool: filter_pool.clone(),
				frontier_backend: match frontier_backend.clone() {
					fc_db::Backend::KeyValue(bd) => Arc::new(bd),
					fc_db::Backend::Sql(bd) => Arc::new(bd),
				},
				max_past_logs,
				fee_history_cache: fee_history_cache.clone(),
				fee_history_cache_limit,
				overrides: overrides.clone(),
				block_data_cache: block_data_cache.clone(),
				forced_parent_hashes: None,
				pending_create_inherent_data_providers,
			};

			if eth_rpc_config.tracing_api.contains(&crate::cli::TracingApi::Debug)
				|| eth_rpc_config.tracing_api.contains(&crate::cli::TracingApi::Trace)
			{
				crate::rpc::create_full::<_, _, _, _, crate::rpc::DefaultEthConfig<_, _>, _>(
					deps,
					subscription_task_executor,
					pubsub_notification_sinks.clone(),
					Some(crate::rpc::TracingConfig {
						tracing_requesters: tracing_requesters.clone(),
						trace_filter_max_count: eth_rpc_config.tracing_max_count,
					}),
				)
				.map_err(Into::into)
			} else {
				crate::rpc::create_full::<_, _, _, _, crate::rpc::DefaultEthConfig<_, _>, _>(
					deps,
					subscription_task_executor,
					pubsub_notification_sinks.clone(),
					None,
				)
				.map_err(Into::into)
			}
		})
	};

	sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		rpc_builder: Box::new(rpc_extensions_builder),
		client,
		transaction_pool,
		task_manager: &mut task_manager,
		config,
		keystore: keystore_container.keystore(),
		backend,
		network,
		sync_service,
		system_rpc_tx,
		tx_handler_controller,
		telemetry: None,
	})?;

	start_network.start_network();

	Ok(task_manager)
}
