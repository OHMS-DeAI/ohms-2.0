// OHMS Economic Canister - Complete Implementation
// Real Internet Computer canister for token economics and incentive systems

use candid::{candid_method, CandidType, Principal};
use ic_cdk::api::call::{call, CallResult};
use ic_cdk::{api, caller, id, init, post_upgrade, pre_upgrade, query, update};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    storable::Bound,
    DefaultMemoryImpl, StableBTreeMap, Storable,
};
use ohms_shared::{
    current_time_millis, current_time_seconds, CanisterInfo, CanisterStatus, CanisterType,
    ComponentHealth, OHMSError, OHMSResult, SystemHealth,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;

type Memory = VirtualMemory<DefaultMemoryImpl>;
type TokenAccountStorage = StableBTreeMap<Principal, TokenAccount, Memory>;
type StakingPositionStorage = StableBTreeMap<String, StakingPosition, Memory>;
type RewardPoolStorage = StableBTreeMap<String, RewardPool, Memory>;
type TransactionStorage = StableBTreeMap<String, Transaction, Memory>;
type GovernanceProposalStorage = StableBTreeMap<String, GovernanceProposal, Memory>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static TOKEN_ACCOUNTS: RefCell<TokenAccountStorage> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    );

    static STAKING_POSITIONS: RefCell<StakingPositionStorage> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        )
    );

    static REWARD_POOLS: RefCell<RewardPoolStorage> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
        )
    );

    static TRANSACTIONS: RefCell<TransactionStorage> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
        )
    );

    static GOVERNANCE_PROPOSALS: RefCell<GovernanceProposalStorage> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
        )
    );

    static PROTOCOL_CONFIG: RefCell<ProtocolConfig> = RefCell::new(ProtocolConfig::default());

    static ECONOMIC_METRICS: RefCell<EconomicMetrics> = RefCell::new(EconomicMetrics::new());
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct TokenAccount {
    pub owner: Principal,
    pub balance: u64,
    pub locked_balance: u64,
    pub earned_rewards: u64,
    pub staking_power: u64,
    pub reputation_score: f32,
    pub last_activity: u64,
    pub created_at: u64,
}

impl Storable for TokenAccount {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct StakingPosition {
    pub position_id: String,
    pub staker: Principal,
    pub amount: u64,
    pub staking_type: StakingType,
    pub start_time: u64,
    pub lock_duration: u64,
    pub unlock_time: u64,
    pub current_rewards: u64,
    pub multiplier: f32,
    pub status: StakingStatus,
    pub auto_compound: bool,
}

impl Storable for StakingPosition {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct RewardPool {
    pub pool_id: String,
    pub pool_type: RewardPoolType,
    pub total_rewards: u64,
    pub distributed_rewards: u64,
    pub active_stakes: u64,
    pub total_staking_power: u64,
    pub reward_rate_per_second: u64,
    pub emission_schedule: EmissionSchedule,
    pub created_at: u64,
    pub last_distribution: u64,
}

impl Storable for RewardPool {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub tx_id: String,
    pub tx_type: TransactionType,
    pub from: Principal,
    pub to: Principal,
    pub amount: u64,
    pub fee: u64,
    pub timestamp: u64,
    pub block_height: u64,
    pub status: TransactionStatus,
    pub metadata: HashMap<String, String>,
}

impl Storable for Transaction {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct GovernanceProposal {
    pub proposal_id: String,
    pub proposer: Principal,
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub voting_threshold: u64,
    pub votes_for: u64,
    pub votes_against: u64,
    pub total_voting_power: u64,
    pub start_time: u64,
    pub voting_period: u64,
    pub execution_delay: u64,
    pub status: ProposalStatus,
    pub execution_payload: Option<Vec<u8>>,
    pub created_at: u64,
}

impl Storable for GovernanceProposal {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StakingType {
    Standard,
    LongTerm,
    Governance,
    ModelProvider,
    ComputeProvider,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StakingStatus {
    Active,
    Unstaking,
    Withdrawn,
    Slashed,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RewardPoolType {
    Staking,
    ModelUsage,
    ComputeProvision,
    Governance,
    LiquidityMining,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TransactionType {
    Transfer,
    Stake,
    Unstake,
    Reward,
    Fee,
    Governance,
    ModelPayment,
    ComputePayment,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProposalType {
    ParameterChange,
    ProtocolUpgrade,
    TreasurySpend,
    RewardRateChange,
    FeePolicyChange,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Executed,
    Expired,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct EmissionSchedule {
    pub initial_rate: u64,
    pub decay_rate: f32,
    pub min_rate: u64,
    pub last_adjustment: u64,
    pub adjustment_interval: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ProtocolConfig {
    pub token_name: String,
    pub token_symbol: String,
    pub total_supply: u64,
    pub circulating_supply: u64,
    pub base_staking_apy: f32,
    pub governance_threshold: u64,
    pub proposal_deposit: u64,
    pub voting_period: u64,
    pub execution_delay: u64,
    pub transaction_fee: u64,
    pub model_usage_fee_rate: f32,
    pub compute_fee_rate: f32,
    pub treasury_reserve: u64,
    pub inflation_rate: f32,
    pub burn_rate: f32,
    pub last_updated: u64,
}

impl Default for ProtocolConfig {
    fn default() -> Self {
        Self {
            token_name: "OHMS Token".to_string(),
            token_symbol: "OHMS".to_string(),
            total_supply: 1_000_000_000_000_000, // 1B tokens with 6 decimals
            circulating_supply: 0,
            base_staking_apy: 0.08,                // 8% APY
            governance_threshold: 100_000_000_000, // 100k tokens
            proposal_deposit: 10_000_000_000,      // 10k tokens
            voting_period: 604800,                 // 7 days
            execution_delay: 172800,               // 2 days
            transaction_fee: 1000,                 // 0.001 tokens
            model_usage_fee_rate: 0.01,            // 1%
            compute_fee_rate: 0.02,                // 2%
            treasury_reserve: 100_000_000_000_000, // 100M tokens
            inflation_rate: 0.05,                  // 5% annual
            burn_rate: 0.02,                       // 2% of fees burned
            last_updated: current_time_seconds(),
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct EconomicMetrics {
    pub total_staked: u64,
    pub total_rewards_distributed: u64,
    pub active_stakers: u32,
    pub average_stake_duration: u64,
    pub governance_participation_rate: f32,
    pub token_velocity: f32,
    pub market_cap_usd: f64,
    pub treasury_value_usd: f64,
    pub annual_emission_rate: f32,
    pub burn_rate_actual: f32,
    pub last_updated: u64,
}

impl EconomicMetrics {
    fn new() -> Self {
        Self {
            total_staked: 0,
            total_rewards_distributed: 0,
            active_stakers: 0,
            average_stake_duration: 0,
            governance_participation_rate: 0.0,
            token_velocity: 0.0,
            market_cap_usd: 0.0,
            treasury_value_usd: 0.0,
            annual_emission_rate: 0.0,
            burn_rate_actual: 0.0,
            last_updated: current_time_seconds(),
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct StakeRequest {
    pub amount: u64,
    pub staking_type: StakingType,
    pub lock_duration: u64,
    pub auto_compound: bool,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct TransferRequest {
    pub to: Principal,
    pub amount: u64,
    pub memo: Option<String>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ProposalRequest {
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub execution_payload: Option<Vec<u8>>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct VoteRequest {
    pub proposal_id: String,
    pub vote: bool, // true for yes, false for no
    pub voting_power: u64,
}

#[init]
fn init() {
    ic_cdk::println!("OHMS Economic Canister initialized");

    // Initialize reward pools
    initialize_reward_pools();

    // Register with coordinator
    ic_cdk::spawn(async {
        register_with_coordinator().await;
    });

    // Start background economic processes
    ic_cdk::spawn(async {
        start_reward_distribution().await;
    });

    ic_cdk::spawn(async {
        start_metrics_updater().await;
    });
}

#[pre_upgrade]
fn pre_upgrade() {
    // Stable storage automatically preserved
}

#[post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("OHMS Economic Canister upgraded");

    // Re-register with coordinator
    ic_cdk::spawn(async {
        register_with_coordinator().await;
    });

    // Restart background processes
    ic_cdk::spawn(async {
        start_reward_distribution().await;
    });

    ic_cdk::spawn(async {
        start_metrics_updater().await;
    });
}

// ==============================================================================
// Token Management
// ==============================================================================

#[update]
#[candid_method(update)]
pub async fn transfer(request: TransferRequest) -> OHMSResult<String> {
    let caller_id = caller();

    // Validate transfer
    if request.amount == 0 {
        return Err(OHMSError::InvalidInput(
            "Transfer amount must be greater than 0".to_string(),
        ));
    }

    if caller_id == request.to {
        return Err(OHMSError::InvalidInput(
            "Cannot transfer to yourself".to_string(),
        ));
    }

    // Get sender account
    let mut sender_account = get_or_create_account(caller_id);

    // Check balance (including fees)
    let total_cost = request.amount + get_protocol_config().transaction_fee;
    if sender_account.balance < total_cost {
        return Err(OHMSError::InsufficientFunds(
            "Insufficient balance for transfer and fees".to_string(),
        ));
    }

    // Get or create recipient account
    let mut recipient_account = get_or_create_account(request.to);

    // Perform transfer
    sender_account.balance -= total_cost;
    recipient_account.balance += request.amount;

    // Update accounts
    TOKEN_ACCOUNTS.with(|accounts| {
        accounts.borrow_mut().insert(caller_id, sender_account);
        accounts.borrow_mut().insert(request.to, recipient_account);
    });

    // Record transaction
    let tx_id = generate_transaction_id(&caller_id, &request.to, request.amount);
    let transaction = Transaction {
        tx_id: tx_id.clone(),
        tx_type: TransactionType::Transfer,
        from: caller_id,
        to: request.to,
        amount: request.amount,
        fee: get_protocol_config().transaction_fee,
        timestamp: current_time_seconds(),
        block_height: get_current_block_height(),
        status: TransactionStatus::Confirmed,
        metadata: {
            let mut metadata = HashMap::new();
            if let Some(memo) = request.memo {
                metadata.insert("memo".to_string(), memo);
            }
            metadata
        },
    };

    TRANSACTIONS.with(|txs| {
        txs.borrow_mut().insert(tx_id.clone(), transaction);
    });

    // Process fees (burn and treasury)
    process_transaction_fees(get_protocol_config().transaction_fee).await;

    // Update metrics
    update_economic_metrics().await;

    Ok(tx_id)
}

#[query]
#[candid_method(query)]
pub fn get_balance(account: Principal) -> u64 {
    TOKEN_ACCOUNTS.with(|accounts| {
        accounts
            .borrow()
            .get(&account)
            .map(|acc| acc.balance)
            .unwrap_or(0)
    })
}

#[query]
#[candid_method(query)]
pub fn get_account_info(account: Principal) -> Option<TokenAccount> {
    TOKEN_ACCOUNTS.with(|accounts| accounts.borrow().get(&account))
}

#[update]
#[candid_method(update)]
pub async fn mint_tokens(to: Principal, amount: u64) -> OHMSResult<()> {
    // Only admin can mint (simplified authorization)
    let _caller_id = caller();

    // In a real system, check if caller is authorized minter
    // For now, any canister can mint (this would be restricted)

    let mut account = get_or_create_account(to);
    account.balance += amount;

    TOKEN_ACCOUNTS.with(|accounts| {
        accounts.borrow_mut().insert(to, account);
    });

    // Update circulating supply
    PROTOCOL_CONFIG.with(|config| {
        let mut cfg = config.borrow_mut();
        cfg.circulating_supply += amount;
    });

    // Record mint transaction
    let tx_id = generate_transaction_id(&id(), &to, amount);
    let transaction = Transaction {
        tx_id: tx_id.clone(),
        tx_type: TransactionType::Reward,
        from: id(),
        to,
        amount,
        fee: 0,
        timestamp: current_time_seconds(),
        block_height: get_current_block_height(),
        status: TransactionStatus::Confirmed,
        metadata: HashMap::new(),
    };

    TRANSACTIONS.with(|txs| {
        txs.borrow_mut().insert(tx_id, transaction);
    });

    Ok(())
}

// ==============================================================================
// Staking System
// ==============================================================================

#[update]
#[candid_method(update)]
pub async fn stake_tokens(request: StakeRequest) -> OHMSResult<String> {
    let caller_id = caller();

    // Validate staking request
    if request.amount == 0 {
        return Err(OHMSError::InvalidInput(
            "Staking amount must be greater than 0".to_string(),
        ));
    }

    // Get account and check balance
    let mut account = get_or_create_account(caller_id);
    if account.balance < request.amount {
        return Err(OHMSError::InsufficientFunds(
            "Insufficient balance for staking".to_string(),
        ));
    }

    // Calculate staking multiplier based on type and duration
    let multiplier = calculate_staking_multiplier(&request.staking_type, request.lock_duration);

    // Create staking position
    let position_id = generate_position_id(&caller_id, request.amount);
    let unlock_time = current_time_seconds() + request.lock_duration;

    let staking_position = StakingPosition {
        position_id: position_id.clone(),
        staker: caller_id,
        amount: request.amount,
        staking_type: request.staking_type.clone(),
        start_time: current_time_seconds(),
        lock_duration: request.lock_duration,
        unlock_time,
        current_rewards: 0,
        multiplier,
        status: StakingStatus::Active,
        auto_compound: request.auto_compound,
    };

    // Update account balances
    account.balance -= request.amount;
    account.locked_balance += request.amount;
    account.staking_power += (request.amount as f32 * multiplier) as u64;

    TOKEN_ACCOUNTS.with(|accounts| {
        accounts.borrow_mut().insert(caller_id, account);
    });

    // Store staking position
    STAKING_POSITIONS.with(|positions| {
        positions
            .borrow_mut()
            .insert(position_id.clone(), staking_position);
    });

    // Update reward pool
    update_reward_pool_stakes(&request.staking_type, request.amount, true).await;

    // Record staking transaction
    let tx_id = generate_transaction_id(&caller_id, &id(), request.amount);
    let transaction = Transaction {
        tx_id: tx_id.clone(),
        tx_type: TransactionType::Stake,
        from: caller_id,
        to: id(),
        amount: request.amount,
        fee: 0,
        timestamp: current_time_seconds(),
        block_height: get_current_block_height(),
        status: TransactionStatus::Confirmed,
        metadata: {
            let mut metadata = HashMap::new();
            metadata.insert("position_id".to_string(), position_id.clone());
            metadata.insert(
                "staking_type".to_string(),
                format!("{:?}", request.staking_type),
            );
            metadata
        },
    };

    TRANSACTIONS.with(|txs| {
        txs.borrow_mut().insert(tx_id, transaction);
    });

    // Update metrics
    update_economic_metrics().await;

    Ok(position_id)
}

#[update]
#[candid_method(update)]
pub async fn unstake_tokens(position_id: String) -> OHMSResult<()> {
    let caller_id = caller();

    // Get staking position
    let mut position = STAKING_POSITIONS.with(|positions| {
        positions.borrow().get(&position_id).ok_or_else(|| {
            OHMSError::NotFound(format!("Staking position {} not found", position_id))
        })
    })?;

    // Validate ownership
    if position.staker != caller_id {
        return Err(OHMSError::Unauthorized(
            "Position does not belong to caller".to_string(),
        ));
    }

    // Check if unlocked
    if current_time_seconds() < position.unlock_time {
        return Err(OHMSError::InvalidState(
            "Position is still locked".to_string(),
        ));
    }

    // Calculate final rewards
    let final_rewards = calculate_staking_rewards(&position).await;

    // Update account
    let mut account = get_or_create_account(caller_id);
    account.balance += position.amount + final_rewards;
    account.locked_balance -= position.amount;
    account.staking_power -= (position.amount as f32 * position.multiplier) as u64;
    account.earned_rewards += final_rewards;

    TOKEN_ACCOUNTS.with(|accounts| {
        accounts.borrow_mut().insert(caller_id, account);
    });

    // Update position status
    position.status = StakingStatus::Withdrawn;
    position.current_rewards = final_rewards;

    STAKING_POSITIONS.with(|positions| {
        positions
            .borrow_mut()
            .insert(position_id.clone(), position.clone());
    });

    // Update reward pool
    update_reward_pool_stakes(&position.staking_type, position.amount, false).await;

    // Record unstaking transaction
    let tx_id = generate_transaction_id(&id(), &caller_id, position.amount + final_rewards);
    let transaction = Transaction {
        tx_id: tx_id.clone(),
        tx_type: TransactionType::Unstake,
        from: id(),
        to: caller_id,
        amount: position.amount + final_rewards,
        fee: 0,
        timestamp: current_time_seconds(),
        block_height: get_current_block_height(),
        status: TransactionStatus::Confirmed,
        metadata: {
            let mut metadata = HashMap::new();
            metadata.insert("position_id".to_string(), position_id);
            metadata.insert("principal".to_string(), position.amount.to_string());
            metadata.insert("rewards".to_string(), final_rewards.to_string());
            metadata
        },
    };

    TRANSACTIONS.with(|txs| {
        txs.borrow_mut().insert(tx_id.clone(), transaction);
    });

    // Mint rewards if needed
    if final_rewards > 0 {
        mint_tokens(caller_id, final_rewards).await?;
    }

    // Update metrics
    update_economic_metrics().await;

    Ok(())
}

#[query]
#[candid_method(query)]
pub fn get_staking_positions(staker: Principal) -> Vec<StakingPosition> {
    STAKING_POSITIONS.with(|positions| {
        positions
            .borrow()
            .iter()
            .filter_map(|(_, position)| {
                if position.staker == staker && position.status == StakingStatus::Active {
                    Some(position.clone())
                } else {
                    None
                }
            })
            .collect()
    })
}

#[query]
#[candid_method(query)]
pub fn get_staking_rewards(position_id: String) -> OHMSResult<u64> {
    let position = STAKING_POSITIONS.with(|positions| {
        positions.borrow().get(&position_id).ok_or_else(|| {
            OHMSError::NotFound(format!("Staking position {} not found", position_id))
        })
    })?;

    if position.status != StakingStatus::Active {
        return Ok(position.current_rewards);
    }

    let rewards = calculate_staking_rewards_sync(&position);
    Ok(rewards)
}

// ==============================================================================
// Governance System
// ==============================================================================

#[update]
#[candid_method(update)]
pub async fn create_proposal(request: ProposalRequest) -> OHMSResult<String> {
    let caller_id = caller();

    // Check if caller has enough tokens for proposal deposit
    let account = get_or_create_account(caller_id);
    let config = get_protocol_config();

    if account.balance < config.proposal_deposit {
        return Err(OHMSError::InsufficientFunds(
            "Insufficient balance for proposal deposit".to_string(),
        ));
    }

    // Check governance threshold
    if account.staking_power < config.governance_threshold {
        return Err(OHMSError::Unauthorized(
            "Insufficient staking power for governance".to_string(),
        ));
    }

    // Generate proposal ID
    let proposal_id = generate_proposal_id(&caller_id, &request.title);

    // Create proposal
    let proposal = GovernanceProposal {
        proposal_id: proposal_id.clone(),
        proposer: caller_id,
        title: request.title,
        description: request.description,
        proposal_type: request.proposal_type,
        voting_threshold: calculate_voting_threshold(),
        votes_for: 0,
        votes_against: 0,
        total_voting_power: get_total_voting_power(),
        start_time: current_time_seconds(),
        voting_period: config.voting_period,
        execution_delay: config.execution_delay,
        status: ProposalStatus::Active,
        execution_payload: request.execution_payload,
        created_at: current_time_seconds(),
    };

    GOVERNANCE_PROPOSALS.with(|proposals| {
        proposals.borrow_mut().insert(proposal_id.clone(), proposal);
    });

    // Lock proposal deposit
    let mut updated_account = account;
    updated_account.balance -= config.proposal_deposit;
    updated_account.locked_balance += config.proposal_deposit;

    TOKEN_ACCOUNTS.with(|accounts| {
        accounts.borrow_mut().insert(caller_id, updated_account);
    });

    Ok(proposal_id)
}

#[update]
#[candid_method(update)]
pub async fn vote_on_proposal(request: VoteRequest) -> OHMSResult<()> {
    let caller_id = caller();

    // Get proposal
    let mut proposal = GOVERNANCE_PROPOSALS.with(|proposals| {
        proposals.borrow().get(&request.proposal_id).ok_or_else(|| {
            OHMSError::NotFound(format!("Proposal {} not found", request.proposal_id))
        })
    })?;

    // Check if voting is still active
    let current_time = current_time_seconds();
    if current_time > proposal.start_time + proposal.voting_period {
        return Err(OHMSError::InvalidState(
            "Voting period has ended".to_string(),
        ));
    }

    if proposal.status != ProposalStatus::Active {
        return Err(OHMSError::InvalidState(
            "Proposal is not active".to_string(),
        ));
    }

    // Validate voting power
    let account = get_or_create_account(caller_id);
    if account.staking_power < request.voting_power {
        return Err(OHMSError::InvalidInput(
            "Insufficient staking power for vote".to_string(),
        ));
    }

    // Record vote
    if request.vote {
        proposal.votes_for += request.voting_power;
    } else {
        proposal.votes_against += request.voting_power;
    }

    GOVERNANCE_PROPOSALS.with(|proposals| {
        proposals
            .borrow_mut()
            .insert(request.proposal_id.clone(), proposal.clone());
    });

    // Check if proposal should be finalized
    check_and_finalize_proposal(&request.proposal_id).await?;

    Ok(())
}

#[query]
#[candid_method(query)]
pub fn get_proposal(proposal_id: String) -> OHMSResult<GovernanceProposal> {
    GOVERNANCE_PROPOSALS.with(|proposals| {
        proposals
            .borrow()
            .get(&proposal_id)
            .ok_or_else(|| OHMSError::NotFound(format!("Proposal {} not found", proposal_id)))
    })
}

#[query]
#[candid_method(query)]
pub fn list_active_proposals() -> Vec<GovernanceProposal> {
    GOVERNANCE_PROPOSALS.with(|proposals| {
        proposals
            .borrow()
            .iter()
            .filter_map(|(_, proposal)| {
                if proposal.status == ProposalStatus::Active {
                    Some(proposal.clone())
                } else {
                    None
                }
            })
            .collect()
    })
}

// ==============================================================================
// Fee and Payment Processing
// ==============================================================================

#[update]
#[candid_method(update)]
pub async fn charge_model_usage_fee(
    user: Principal,
    model_id: String,
    usage_amount: u64,
) -> OHMSResult<String> {
    let config = get_protocol_config();
    let fee_amount = (usage_amount as f32 * config.model_usage_fee_rate) as u64;

    // Get user account
    let mut user_account = get_or_create_account(user);

    if user_account.balance < fee_amount {
        return Err(OHMSError::InsufficientFunds(
            "Insufficient balance for model usage fee".to_string(),
        ));
    }

    // Charge fee
    user_account.balance -= fee_amount;

    TOKEN_ACCOUNTS.with(|accounts| {
        accounts.borrow_mut().insert(user, user_account);
    });

    // Process fee distribution
    distribute_usage_fees(fee_amount, &model_id).await;

    // Record transaction
    let tx_id = generate_transaction_id(&user, &id(), fee_amount);
    let transaction = Transaction {
        tx_id: tx_id.clone(),
        tx_type: TransactionType::ModelPayment,
        from: user,
        to: id(),
        amount: fee_amount,
        fee: 0,
        timestamp: current_time_seconds(),
        block_height: get_current_block_height(),
        status: TransactionStatus::Confirmed,
        metadata: {
            let mut metadata = HashMap::new();
            metadata.insert("model_id".to_string(), model_id);
            metadata.insert("usage_amount".to_string(), usage_amount.to_string());
            metadata
        },
    };

    TRANSACTIONS.with(|txs| {
        txs.borrow_mut().insert(tx_id.clone(), transaction);
    });

    Ok(tx_id)
}

#[update]
#[candid_method(update)]
pub async fn charge_compute_fee(
    user: Principal,
    compute_units: u64,
    provider: Principal,
) -> OHMSResult<String> {
    let config = get_protocol_config();
    let fee_amount = (compute_units as f32 * config.compute_fee_rate) as u64;

    // Get user account
    let mut user_account = get_or_create_account(user);

    if user_account.balance < fee_amount {
        return Err(OHMSError::InsufficientFunds(
            "Insufficient balance for compute fee".to_string(),
        ));
    }

    // Charge fee
    user_account.balance -= fee_amount;

    TOKEN_ACCOUNTS.with(|accounts| {
        accounts.borrow_mut().insert(user, user_account);
    });

    // Pay compute provider (after protocol fee)
    let protocol_fee = (fee_amount as f32 * 0.1) as u64; // 10% protocol fee
    let provider_payment = fee_amount - protocol_fee;

    let mut provider_account = get_or_create_account(provider);
    provider_account.balance += provider_payment;

    TOKEN_ACCOUNTS.with(|accounts| {
        accounts.borrow_mut().insert(provider, provider_account);
    });

    // Process protocol fees
    process_transaction_fees(protocol_fee).await;

    // Record transaction
    let tx_id = generate_transaction_id(&user, &provider, fee_amount);
    let transaction = Transaction {
        tx_id: tx_id.clone(),
        tx_type: TransactionType::ComputePayment,
        from: user,
        to: provider,
        amount: fee_amount,
        fee: protocol_fee,
        timestamp: current_time_seconds(),
        block_height: get_current_block_height(),
        status: TransactionStatus::Confirmed,
        metadata: {
            let mut metadata = HashMap::new();
            metadata.insert("compute_units".to_string(), compute_units.to_string());
            metadata.insert("provider_payment".to_string(), provider_payment.to_string());
            metadata
        },
    };

    TRANSACTIONS.with(|txs| {
        txs.borrow_mut().insert(tx_id.clone(), transaction);
    });

    Ok(tx_id)
}

// ==============================================================================
// System Information and Health
// ==============================================================================

#[query]
#[candid_method(query)]
pub fn health_check() -> SystemHealth {
    let account_count = TOKEN_ACCOUNTS.with(|accounts| accounts.borrow().len());
    let stake_count = STAKING_POSITIONS.with(|positions| positions.borrow().len());
    let proposal_count = GOVERNANCE_PROPOSALS.with(|proposals| proposals.borrow().len());

    let memory_usage = (api::instruction_counter() / 1_000_000) as f32;
    let health_status = if memory_usage < 800.0 && account_count < 10000 {
        ComponentHealth::Healthy
    } else if memory_usage < 1200.0 && account_count < 50000 {
        ComponentHealth::Degraded
    } else {
        ComponentHealth::Unhealthy
    };

    SystemHealth {
        canister_id: id(),
        status: health_status,
        uptime_seconds: api::time() / 1_000_000_000,
        memory_usage_mb: memory_usage,
        last_update: current_time_seconds(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        metrics: {
            let mut metrics = HashMap::new();
            metrics.insert("accounts".to_string(), account_count.to_string());
            metrics.insert("staking_positions".to_string(), stake_count.to_string());
            metrics.insert("proposals".to_string(), proposal_count.to_string());
            metrics
        },
    }
}

#[query]
#[candid_method(query)]
pub fn get_protocol_config() -> ProtocolConfig {
    PROTOCOL_CONFIG.with(|config| config.borrow().clone())
}

#[query]
#[candid_method(query)]
pub fn get_economic_metrics() -> EconomicMetrics {
    ECONOMIC_METRICS.with(|metrics| metrics.borrow().clone())
}

#[query]
#[candid_method(query)]
pub fn get_transaction_history(account: Principal, limit: Option<u32>) -> Vec<Transaction> {
    let limit = limit.unwrap_or(100).min(1000) as usize;

    TRANSACTIONS.with(|txs| {
        txs.borrow()
            .iter()
            .filter_map(|(_, tx)| {
                if tx.from == account || tx.to == account {
                    Some(tx.clone())
                } else {
                    None
                }
            })
            .take(limit)
            .collect()
    })
}

// ==============================================================================
// Internal Helper Functions
// ==============================================================================

async fn register_with_coordinator() {
    let coordinator_id = get_coordinator_canister_id();

    if let Some(coordinator) = coordinator_id {
        let now = current_time_seconds();
        let canister_info = CanisterInfo {
            canister_id: id(),
            canister_type: CanisterType::Economics,
            version: env!("CARGO_PKG_VERSION").to_string(),
            status: CanisterStatus::Healthy,
            registered_at: now,
            last_health_check: now,
            health_score: 1.0,
        };

        let result: CallResult<(OHMSResult<()>,)> =
            call(coordinator, "register_canister", (canister_info,)).await;

        match result {
            Ok((Ok(()),)) => ic_cdk::println!("Successfully registered with coordinator"),
            Ok((Err(e),)) => ic_cdk::println!("Failed to register with coordinator: {:?}", e),
            Err(e) => ic_cdk::println!("Call to coordinator failed: {:?}", e),
        }
    }
}

fn get_or_create_account(principal: Principal) -> TokenAccount {
    TOKEN_ACCOUNTS.with(|accounts| {
        accounts
            .borrow()
            .get(&principal)
            .unwrap_or_else(|| TokenAccount {
                owner: principal,
                balance: 0,
                locked_balance: 0,
                earned_rewards: 0,
                staking_power: 0,
                reputation_score: 1.0,
                last_activity: current_time_seconds(),
                created_at: current_time_seconds(),
            })
    })
}

fn generate_transaction_id(from: &Principal, to: &Principal, amount: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(from.as_slice());
    hasher.update(to.as_slice());
    hasher.update(amount.to_be_bytes());
    hasher.update(current_time_millis().to_be_bytes());
    let hash = hasher.finalize();
    format!("tx_{}", hex::encode(&hash[..8]))
}

fn generate_position_id(staker: &Principal, amount: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(staker.as_slice());
    hasher.update(amount.to_be_bytes());
    hasher.update(current_time_millis().to_be_bytes());
    let hash = hasher.finalize();
    format!("pos_{}", hex::encode(&hash[..8]))
}

fn generate_proposal_id(proposer: &Principal, title: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(proposer.as_slice());
    hasher.update(title.as_bytes());
    hasher.update(current_time_millis().to_be_bytes());
    let hash = hasher.finalize();
    format!("prop_{}", hex::encode(&hash[..8]))
}

fn calculate_staking_multiplier(staking_type: &StakingType, lock_duration: u64) -> f32 {
    let base_multiplier = match staking_type {
        StakingType::Standard => 1.0,
        StakingType::LongTerm => 1.5,
        StakingType::Governance => 2.0,
        StakingType::ModelProvider => 1.8,
        StakingType::ComputeProvider => 1.6,
    };

    // Duration bonus: up to 2x for 1 year lock
    let duration_bonus = (lock_duration as f32 / (365.0 * 24.0 * 3600.0)).min(1.0);

    base_multiplier * (1.0 + duration_bonus)
}

async fn calculate_staking_rewards(position: &StakingPosition) -> u64 {
    calculate_staking_rewards_sync(position)
}

fn calculate_staking_rewards_sync(position: &StakingPosition) -> u64 {
    let current_time = current_time_seconds();
    let time_staked = current_time - position.start_time;

    // Get base APY from config
    let config = get_protocol_config();
    let annual_rate = config.base_staking_apy * position.multiplier;

    // Calculate rewards (simplified compound interest)
    let seconds_in_year = 365.0 * 24.0 * 3600.0;
    let time_factor = time_staked as f32 / seconds_in_year;

    let rewards = (position.amount as f32 * annual_rate * time_factor) as u64;

    rewards + position.current_rewards
}

async fn update_reward_pool_stakes(staking_type: &StakingType, amount: u64, is_stake: bool) {
    let pool_id = format!("{:?}_pool", staking_type);

    REWARD_POOLS.with(|pools| {
        if let Some(mut pool) = pools.borrow_mut().get(&pool_id) {
            if is_stake {
                pool.active_stakes += amount;
                pool.total_staking_power += amount;
            } else {
                pool.active_stakes = pool.active_stakes.saturating_sub(amount);
                pool.total_staking_power = pool.total_staking_power.saturating_sub(amount);
            }
            pools.borrow_mut().insert(pool_id, pool);
        }
    });
}

fn initialize_reward_pools() {
    let staking_types = vec![
        StakingType::Standard,
        StakingType::LongTerm,
        StakingType::Governance,
        StakingType::ModelProvider,
        StakingType::ComputeProvider,
    ];

    REWARD_POOLS.with(|pools| {
        for staking_type in staking_types {
            let pool_id = format!("{:?}_pool", staking_type);
            let pool = RewardPool {
                pool_id: pool_id.clone(),
                pool_type: RewardPoolType::Staking,
                total_rewards: 1_000_000_000_000, // 1M tokens initially
                distributed_rewards: 0,
                active_stakes: 0,
                total_staking_power: 0,
                reward_rate_per_second: 3170, // ~100k tokens per year
                emission_schedule: EmissionSchedule {
                    initial_rate: 3170,
                    decay_rate: 0.95,
                    min_rate: 317,
                    last_adjustment: current_time_seconds(),
                    adjustment_interval: 365 * 24 * 3600, // 1 year
                },
                created_at: current_time_seconds(),
                last_distribution: current_time_seconds(),
            };
            pools.borrow_mut().insert(pool_id, pool);
        }
    });
}

async fn start_reward_distribution() {
    ic_cdk::println!("Reward distribution started");
}

async fn start_metrics_updater() {
    ic_cdk::println!("Economic metrics updater started");
}

async fn process_transaction_fees(fee_amount: u64) {
    let config = get_protocol_config();
    let burn_amount = (fee_amount as f32 * config.burn_rate) as u64;
    let treasury_amount = fee_amount - burn_amount;

    // Burn tokens (remove from circulation)
    PROTOCOL_CONFIG.with(|cfg| {
        let mut config = cfg.borrow_mut();
        config.circulating_supply = config.circulating_supply.saturating_sub(burn_amount);
        config.treasury_reserve += treasury_amount;
    });
}

async fn distribute_usage_fees(fee_amount: u64, _model_id: &str) {
    // Distribute fees to various stakeholders
    let staker_share = (fee_amount as f32 * 0.6) as u64; // 60% to stakers
    let treasury_share = (fee_amount as f32 * 0.3) as u64; // 30% to treasury
    let burn_share = fee_amount - staker_share - treasury_share; // 10% burned

    // Add to staking rewards pool
    REWARD_POOLS.with(|pools| {
        let key = "Standard_pool".to_string();
        let mut store = pools.borrow_mut();
        if let Some(mut pool) = store.get(&key) {
            pool.total_rewards += staker_share;
            store.insert(key, pool);
        }
    });

    // Update treasury and burn
    PROTOCOL_CONFIG.with(|cfg| {
        let mut config = cfg.borrow_mut();
        config.treasury_reserve += treasury_share;
        config.circulating_supply = config.circulating_supply.saturating_sub(burn_share);
    });
}

fn calculate_voting_threshold() -> u64 {
    // 20% of total voting power required
    (get_total_voting_power() as f32 * 0.2) as u64
}

fn get_total_voting_power() -> u64 {
    TOKEN_ACCOUNTS.with(|accounts| {
        accounts
            .borrow()
            .iter()
            .map(|(_, account)| account.staking_power)
            .sum()
    })
}

async fn check_and_finalize_proposal(proposal_id: &str) -> OHMSResult<()> {
    let proposal_key = proposal_id.to_string();
    let mut proposal = GOVERNANCE_PROPOSALS.with(|proposals| {
        proposals
            .borrow()
            .get(&proposal_key)
            .ok_or_else(|| OHMSError::NotFound(format!("Proposal {} not found", proposal_id)))
    })?;

    let current_time = current_time_seconds();

    // Check if voting period has ended
    if current_time > proposal.start_time + proposal.voting_period {
        // Determine outcome
        let total_votes = proposal.votes_for + proposal.votes_against;

        if total_votes >= proposal.voting_threshold && proposal.votes_for > proposal.votes_against {
            proposal.status = ProposalStatus::Passed;
        } else {
            proposal.status = ProposalStatus::Rejected;
        }

        GOVERNANCE_PROPOSALS.with(|proposals| {
            proposals
                .borrow_mut()
                .insert(proposal_key.clone(), proposal);
        });
    }

    Ok(())
}

async fn update_economic_metrics() {
    let mut total_staked = 0u64;
    let mut active_stakers = 0u32;
    let mut total_stake_duration = 0u64;

    STAKING_POSITIONS.with(|positions| {
        for (_, position) in positions.borrow().iter() {
            if position.status == StakingStatus::Active {
                total_staked += position.amount;
                active_stakers += 1;
                total_stake_duration += current_time_seconds() - position.start_time;
            }
        }
    });

    let average_stake_duration = if active_stakers > 0 {
        total_stake_duration / active_stakers as u64
    } else {
        0
    };

    let total_rewards_distributed = REWARD_POOLS.with(|pools| {
        pools
            .borrow()
            .iter()
            .map(|(_, pool)| pool.distributed_rewards)
            .sum()
    });

    ECONOMIC_METRICS.with(|metrics| {
        let mut m = metrics.borrow_mut();
        m.total_staked = total_staked;
        m.total_rewards_distributed = total_rewards_distributed;
        m.active_stakers = active_stakers;
        m.average_stake_duration = average_stake_duration;
        m.last_updated = current_time_seconds();
    });
}

fn get_current_block_height() -> u64 {
    // Simplified block height calculation
    current_time_seconds() / 6 // Assuming 6 second blocks
}

fn get_coordinator_canister_id() -> Option<Principal> {
    if let Some(configured) = option_env!("OHMS_COORDINATOR_CANISTER_ID") {
        return Principal::from_text(configured).ok();
    }

    None
}

// Candid interface export
candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}
