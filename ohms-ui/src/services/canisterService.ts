import { Actor, ActorSubclass, HttpAgent } from '@dfinity/agent';
import { HOST as RESOLVED_HOST, NETWORK, getCanisterIdsFromEnv } from '../config/network'
import { idlFactory as agentIdlFactory } from '../declarations/ohms_agent'
import type {
  _SERVICE as AgentCanister,
  AgentCreationRequest as CandidAgentCreationRequest,
  InferenceRequest as CandidInferenceRequest,
  Result_AgentCreation as CandidResultAgentCreation,
  Result_Inference as CandidResultInference,
  AgentSummary as CandidAgentSummary,
  Result_Summaries as CandidResultSummaries
} from '../declarations/ohms_agent/ohms_agent.did'

// Centralized host/network resolution
export const host = RESOLVED_HOST;

// Create an agent
export const agent = new HttpAgent({ host });
if (NETWORK !== 'ic') {
  // Fetch the root key for local development to validate certificates
  agent.fetchRootKey?.().catch(() => {
    // Ignore root key fetch errors in local development
  });
}

// Canister IDs from env via network config
const { ohms_model: OHMS_MODEL_CANISTER_ID, ohms_agent: OHMS_AGENT_CANISTER_ID, ohms_coordinator: OHMS_COORDINATOR_CANISTER_ID, ohms_econ: OHMS_ECON_CANISTER_ID } = getCanisterIdsFromEnv()

// Candid interface definitions - matching actual deployed interfaces
const modelCanisterIdl = ({ IDL }: any) => {
  const ComponentHealth = IDL.Variant({
    Healthy: IDL.Null,
    Degraded: IDL.Null,
    Unhealthy: IDL.Null,
    Unknown: IDL.Null,
  });

  const SystemHealth = IDL.Record({
    canister_id: IDL.Principal,
    status: ComponentHealth,
    uptime_seconds: IDL.Nat64,
    memory_usage_mb: IDL.Float32,
    last_update: IDL.Nat64,
    version: IDL.Text,
    metrics: IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
  });

  const QuantizationFormat = IDL.Variant({ NOVAQ: IDL.Null, GGUF: IDL.Null, Custom: IDL.Text });
  const ModelState = IDL.Variant({ Pending: IDL.Null, Active: IDL.Null, Deprecated: IDL.Null });
  const QuantizedArtifactMetadata = IDL.Record({
    format: QuantizationFormat,
    artifact_checksum: IDL.Text,
    compression_ratio: IDL.Float32,
    accuracy_retention: IDL.Float32,
    bits_per_weight: IDL.Opt(IDL.Float32),
    notes: IDL.Opt(IDL.Text),
  });
  const ArtifactChunkInfo = IDL.Record({
    chunk_id: IDL.Text,
    offset: IDL.Nat64,
    size_bytes: IDL.Nat64,
    sha256: IDL.Text,
  });
  const ModelManifest = IDL.Record({
    model_id: IDL.Text,
    version: IDL.Text,
    state: ModelState,
    uploaded_at: IDL.Nat64,
    activated_at: IDL.Opt(IDL.Nat64),
    total_size_bytes: IDL.Nat64,
    chunk_count: IDL.Nat32,
    checksum: IDL.Text,
    quantization: QuantizedArtifactMetadata,
    metadata: IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    chunks: IDL.Vec(ArtifactChunkInfo),
  });
  const ModelInfo = IDL.Record({
    model_id: IDL.Text,
    version: IDL.Text,
    state: ModelState,
    quantization_format: QuantizationFormat,
    compression_ratio: IDL.Opt(IDL.Float32),
    accuracy_retention: IDL.Opt(IDL.Float32),
    size_bytes: IDL.Nat64,
    uploaded_at: IDL.Nat64,
    activated_at: IDL.Opt(IDL.Nat64),
  });

  const ArtifactChunkUpload = IDL.Record({
    chunk_id: IDL.Text,
    order: IDL.Nat32,
    data: IDL.Vec(IDL.Nat8),
    sha256: IDL.Text,
  });

  const ModelUploadRequest = IDL.Record({
    name: IDL.Text,
    description: IDL.Text,
    model_type: IDL.Text,
    version: IDL.Text,
    quantization: QuantizedArtifactMetadata,
    metadata: IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    chunks: IDL.Vec(ArtifactChunkUpload),
  });

  const ModelUploadResponse = IDL.Record({
    model_id: IDL.Text,
    upload_time: IDL.Nat64,
    checksum: IDL.Text,
    total_size_bytes: IDL.Nat64,
    chunk_count: IDL.Nat32,
  });

  const ModelStatus = IDL.Variant({ Uploading: IDL.Null, Ready: IDL.Null, Deployed: IDL.Null, Failed: IDL.Null, Deleted: IDL.Null });

  const OHMSError = IDL.Variant({
    InvalidInput: IDL.Text,
    InvalidState: IDL.Text,
    AlreadyExists: IDL.Text,
    NotFound: IDL.Text,
    Unauthorized: IDL.Text,
    InternalError: IDL.Text,
    NetworkError: IDL.Text,
    CommunicationFailed: IDL.Text,
    QuotaExceeded: IDL.Text,
    InsufficientFunds: IDL.Text,
    ModelNotReady: IDL.Text,
    CompressionFailed: IDL.Text,
  });

  const ResultUpload = IDL.Variant({ Ok: ModelUploadResponse, Err: OHMSError });
  const ResultStatus = IDL.Variant({ Ok: ModelStatus, Err: OHMSError });
  const ResultUnit = IDL.Variant({ Ok: IDL.Null, Err: OHMSError });
  const ResultModelInfo = IDL.Variant({ Ok: ModelInfo, Err: OHMSError });

  return IDL.Service({
    health_check: IDL.Func([], [SystemHealth], ['query']),
    list_models: IDL.Func([IDL.Opt(IDL.Principal)], [IDL.Vec(ModelInfo)], ['query']),
    list_active_models: IDL.Func([], [IDL.Vec(ModelInfo)], ['query']),
    get_model_info: IDL.Func([IDL.Text], [ResultModelInfo], ['query']),
    get_manifest: IDL.Func([IDL.Text], [IDL.Opt(ModelManifest)], ['query']),
    upload_model: IDL.Func([ModelUploadRequest], [ResultUpload], []),
    deploy_model: IDL.Func([IDL.Text], [ResultUnit], []),
    delete_model: IDL.Func([IDL.Text], [ResultUnit], []),
    get_model_status: IDL.Func([IDL.Text], [ResultStatus], ['query']),
  });
};

const coordinatorCanisterIdl = ({ IDL }: any) => {
  // Result variants matching the actual canister
  const Result = IDL.Variant({ Ok: IDL.Text, Err: IDL.Text });
  const Result_8 = IDL.Variant({ Ok: IDL.Null, Err: IDL.Text });

  const ComponentHealth = IDL.Variant({
    Healthy: IDL.Null,
    Degraded: IDL.Null,
    Unhealthy: IDL.Null,
    Unknown: IDL.Null,
  });

  const SystemHealth = IDL.Record({
    canister_id: IDL.Principal,
    status: ComponentHealth,
    uptime_seconds: IDL.Nat64,
    memory_usage_mb: IDL.Float32,
    last_update: IDL.Nat64,
    version: IDL.Text,
    metrics: IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
  });

  // OHMS 2.0 Types from actual ohms-coordinator.did
  const RoutingMode = IDL.Variant({ Unicast: IDL.Null, Broadcast: IDL.Null, AgentSpawning: IDL.Null });
  const AgentRegistration = IDL.Record({
    agent_id: IDL.Text,
    agent_principal: IDL.Text,
    canister_id: IDL.Text,
    capabilities: IDL.Vec(IDL.Text),
    model_id: IDL.Text,
    health_score: IDL.Float32,
    registered_at: IDL.Nat64,
    last_seen: IDL.Nat64,
  });
  const RouteRequest = IDL.Record({
    request_id: IDL.Text,
    requester: IDL.Text,
    capabilities_required: IDL.Vec(IDL.Text),
    payload: IDL.Vec(IDL.Nat8),
    routing_mode: RoutingMode,
  });
  const RouteResponse = IDL.Record({
    request_id: IDL.Text,
    selected_agents: IDL.Vec(IDL.Text),
    routing_time_ms: IDL.Nat64,
    selection_criteria: IDL.Text,
  });
  const RoutingStats = IDL.Record({
    agent_id: IDL.Text,
    total_requests: IDL.Nat64,
    success_rate: IDL.Float32,
    average_response_time_ms: IDL.Float64,
  });
  const SwarmTopology = IDL.Variant({ Mesh: IDL.Null, Hierarchical: IDL.Null, Ring: IDL.Null, Star: IDL.Null });
  const OrchestrationMode = IDL.Variant({ Parallel: IDL.Null, Sequential: IDL.Null, Adaptive: IDL.Null });
  const SwarmPolicy = IDL.Record({
    topology: SwarmTopology,
    mode: OrchestrationMode,
    top_k: IDL.Nat32,
    window_ms: IDL.Nat64,
  });

  // OHMS 2.0 Agent Spawning Types
  const InstructionRequest = IDL.Record({
    request_id: IDL.Text,
    user_principal: IDL.Text,
    instructions: IDL.Text,
    agent_count: IDL.Opt(IDL.Nat32),
    capabilities_required: IDL.Vec(IDL.Text),
    priority: IDL.Text,
    created_at: IDL.Nat64,
  });

  const AgentCreationStatus = IDL.Variant({ 
    Pending: IDL.Null, 
    InProgress: IDL.Null, 
    Completed: IDL.Null, 
    Failed: IDL.Null 
  });

  const AgentCreationResult = IDL.Record({
    request_id: IDL.Text,
    agent_ids: IDL.Vec(IDL.Text),
    status: AgentCreationStatus,
    created_at: IDL.Nat64,
    completed_at: IDL.Opt(IDL.Nat64),
    error_message: IDL.Opt(IDL.Text),
  });

  const AgentSpec = IDL.Record({
    agent_id: IDL.Text,
    capabilities: IDL.Vec(IDL.Text),
    behavior_rules: IDL.Vec(IDL.Text),
    coordination_network: IDL.Opt(IDL.Text),
  });

  const InstructionAnalysisResult = IDL.Record({
    request_id: IDL.Text,
    estimated_agents: IDL.Nat32,
    required_capabilities: IDL.Vec(IDL.Text),
    complexity_score: IDL.Float32,
    estimated_duration: IDL.Nat64,
    coordination_needs: IDL.Vec(IDL.Text),
  });

  const QuotaCheckResult = IDL.Record({
    allowed: IDL.Bool,
    remaining_quota: IDL.Opt(IDL.Record({
      agents_remaining: IDL.Nat32,
      tokens_remaining: IDL.Nat64,
      inferences_remaining: IDL.Nat32,
    })),
    reason: IDL.Opt(IDL.Text),
  });

  // OHMS 2.0 Economics Integration Types
  const UserSubscription = IDL.Record({
    principal_id: IDL.Text,
    tier: IDL.Record({
      name: IDL.Text,
      monthly_fee_usd: IDL.Nat32,
      max_agents: IDL.Nat32,
      monthly_agent_creations: IDL.Nat32,
      token_limit: IDL.Nat64,
      inference_rate: IDL.Variant({ Standard: IDL.Null, Priority: IDL.Null, Premium: IDL.Null }),
      features: IDL.Vec(IDL.Text),
    }),
    started_at: IDL.Nat64,
    expires_at: IDL.Nat64,
    auto_renew: IDL.Bool,
    current_usage: IDL.Record({
      agents_created_this_month: IDL.Nat32,
      tokens_used_this_month: IDL.Nat64,
      inferences_this_month: IDL.Nat32,
      last_reset_date: IDL.Nat64,
    }),
    payment_status: IDL.Variant({ Active: IDL.Null, Pending: IDL.Null, Failed: IDL.Null, Cancelled: IDL.Null }),
    created_at: IDL.Nat64,
    updated_at: IDL.Nat64,
  });

  const EconHealth = IDL.Record({
    total_escrows: IDL.Nat32,
    active_escrows: IDL.Nat32,
    total_receipts: IDL.Nat32,
    pending_settlements: IDL.Nat32,
    total_volume: IDL.Nat64,
    protocol_fees_collected: IDL.Nat64,
    average_job_cost: IDL.Float64,
  });

  const QuotaValidation = IDL.Record({
    allowed: IDL.Bool,
    reason: IDL.Opt(IDL.Text),
    remaining_quota: IDL.Opt(IDL.Record({
      agents_remaining: IDL.Nat32,
      tokens_remaining: IDL.Nat64,
      inferences_remaining: IDL.Nat32,
    })),
  });

  // Result variants
  const Result_1 = IDL.Variant({ Ok: AgentRegistration, Err: IDL.Text });
  const Result_2 = IDL.Variant({ Ok: RouteResponse, Err: IDL.Text });
  const Result_5 = IDL.Variant({ Ok: IDL.Vec(AgentRegistration), Err: IDL.Text });
  const Result_7 = IDL.Variant({ Ok: IDL.Vec(RoutingStats), Err: IDL.Text });
  const Result_InstructionAnalysis = IDL.Variant({ Ok: InstructionAnalysisResult, Err: IDL.Text });
  const Result_AgentCreation = IDL.Variant({ Ok: AgentCreationResult, Err: IDL.Text });
  const Result_QuotaCheck = IDL.Variant({ Ok: QuotaCheckResult, Err: IDL.Text });
  const Result_UserSubscription = IDL.Variant({ Ok: UserSubscription, Err: IDL.Text });
  const Result_EconHealth = IDL.Variant({ Ok: EconHealth, Err: IDL.Text });
  const Result_QuotaValidation = IDL.Variant({ Ok: QuotaValidation, Err: IDL.Text });

  return IDL.Service({
    // Health & registry - matching actual interface
    health: IDL.Func([], [SystemHealth], ['query']),
    list_agents: IDL.Func([], [Result_5], ['query']),
    get_agent: IDL.Func([IDL.Text], [Result_1], ['query']),
    register_agent: IDL.Func([AgentRegistration], [Result], []),
    update_agent_health: IDL.Func([IDL.Text, IDL.Float32], [Result_8], []),

    // Routing
    route_request: IDL.Func([RouteRequest], [Result_2], []),
    route_best_result: IDL.Func([RouteRequest, IDL.Nat32, IDL.Nat64], [Result_2], []),
    get_routing_stats: IDL.Func([IDL.Opt(IDL.Text)], [Result_7], ['query']),

    // OHMS 2.0 Agent Spawning APIs
    create_agents_from_instructions: IDL.Func([IDL.Text, IDL.Opt(IDL.Nat32), IDL.Vec(IDL.Text), IDL.Text], [Result_AgentCreation], []),
    get_agent_creation_status: IDL.Func([IDL.Text], [Result_AgentCreation], ['query']),
    get_user_quota_status: IDL.Func([], [Result_QuotaCheck], []),
    list_user_agents: IDL.Func([], [Result_5], ['query']),
    list_instruction_requests: IDL.Func([], [IDL.Vec(InstructionRequest)], ['query']),
    get_instruction_analysis: IDL.Func([IDL.Text], [Result_InstructionAnalysis], ['query']),
    update_agent_status: IDL.Func([IDL.Text, IDL.Text], [Result_8], []),
    get_agent_spawning_metrics: IDL.Func([], [IDL.Record({
      total_creations: IDL.Nat32,
      successful_creations: IDL.Nat32,
      failed_creations: IDL.Nat32,
      average_creation_time_ms: IDL.Float64,
    })], ['query']),
    get_coordination_networks: IDL.Func([], [IDL.Vec(IDL.Record({
      network_id: IDL.Text,
      agent_count: IDL.Nat32,
      coordination_type: IDL.Text,
      created_at: IDL.Nat64,
    }))], ['query']),

    // OHMS 2.0 Subscription Management
    upgrade_subscription_tier: IDL.Func([IDL.Text], [Result_UserSubscription], []),
    get_subscription_tier_info: IDL.Func([], [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Record({
      name: IDL.Text,
      monthly_fee_usd: IDL.Nat32,
      max_agents: IDL.Nat32,
      monthly_agent_creations: IDL.Nat32,
      token_limit: IDL.Nat64,
      inference_rate: IDL.Variant({ Standard: IDL.Null, Priority: IDL.Null, Premium: IDL.Null }),
      features: IDL.Vec(IDL.Text),
    })))], ['query']),

    // OHMS 2.0 Economics Integration
    get_economics_health: IDL.Func([], [Result_EconHealth], []),
    validate_token_usage_quota: IDL.Func([IDL.Nat64], [Result_QuotaValidation], []),

    // Swarm policy
    set_swarm_policy: IDL.Func([SwarmPolicy], [Result_8], []),
    get_swarm_policy: IDL.Func([], [SwarmPolicy], ['query']),
  });
};

const econCanisterIdl = ({ IDL }: any) => {
  // Core economics types
  const JobPriority = IDL.Variant({ Low: IDL.Null, Normal: IDL.Null, High: IDL.Null, Critical: IDL.Null })
  const JobSpec = IDL.Record({ job_id: IDL.Text, model_id: IDL.Text, estimated_tokens: IDL.Nat32, estimated_compute_cycles: IDL.Nat64, priority: JobPriority })
  const CostQuote = IDL.Record({ job_id: IDL.Text, estimated_cost: IDL.Nat64, base_cost: IDL.Nat64, priority_multiplier: IDL.Float32, protocol_fee: IDL.Nat64, quote_expires_at: IDL.Nat64, quote_id: IDL.Text })
  const EscrowStatus = IDL.Variant({ Pending: IDL.Null, Active: IDL.Null, Released: IDL.Null, Refunded: IDL.Null, Expired: IDL.Null })
  const EscrowAccount = IDL.Record({ escrow_id: IDL.Text, job_id: IDL.Text, principal_id: IDL.Text, amount: IDL.Nat64, status: EscrowStatus, created_at: IDL.Nat64, expires_at: IDL.Nat64 })
  const SettlementStatus = IDL.Variant({ Pending: IDL.Null, Completed: IDL.Null, Failed: IDL.Null, Disputed: IDL.Null })
  const FeesBreakdown = IDL.Record({ base_amount: IDL.Nat64, protocol_fee: IDL.Nat64, agent_fee: IDL.Nat64, total_amount: IDL.Nat64 })
  const Receipt = IDL.Record({ receipt_id: IDL.Text, job_id: IDL.Text, escrow_id: IDL.Text, agent_id: IDL.Text, actual_cost: IDL.Nat64, fees_breakdown: FeesBreakdown, settlement_status: SettlementStatus, created_at: IDL.Nat64, settled_at: IDL.Opt(IDL.Nat64) })
  const Balance = IDL.Record({ principal_id: IDL.Text, available_balance: IDL.Nat64, escrowed_balance: IDL.Nat64, total_earnings: IDL.Nat64, last_updated: IDL.Nat64 })
  const FeePolicy = IDL.Record({ protocol_fee_percentage: IDL.Float32, agent_fee_percentage: IDL.Float32, minimum_fee: IDL.Nat64, priority_multipliers: IDL.Vec(IDL.Tuple(IDL.Text, IDL.Float32)), last_updated: IDL.Nat64 })
  const EconComponentHealth = IDL.Variant({
    Healthy: IDL.Null,
    Degraded: IDL.Null,
    Unhealthy: IDL.Null,
    Unknown: IDL.Null,
  })

  const EconHealth = IDL.Record({
    canister_id: IDL.Principal,
    status: EconComponentHealth,
    uptime_seconds: IDL.Nat64,
    memory_usage_mb: IDL.Float32,
    last_update: IDL.Nat64,
    version: IDL.Text,
    metrics: IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
  })
  
  // Subscription types
  const InferenceRate = IDL.Variant({ Standard: IDL.Null, Priority: IDL.Null, Premium: IDL.Null })
  const TierConfig = IDL.Record({ name: IDL.Text, monthly_fee_usd: IDL.Nat32, max_agents: IDL.Nat32, monthly_agent_creations: IDL.Nat32, token_limit: IDL.Nat64, inference_rate: InferenceRate, features: IDL.Vec(IDL.Text) })
  const PaymentStatus = IDL.Variant({ Active: IDL.Null, Pending: IDL.Null, Failed: IDL.Null, Cancelled: IDL.Null })
  const UsageMetrics = IDL.Record({ agents_created_this_month: IDL.Nat32, tokens_used_this_month: IDL.Nat64, inferences_this_month: IDL.Nat32, last_reset_date: IDL.Nat64 })
  const UserSubscription = IDL.Record({ principal_id: IDL.Text, tier: TierConfig, started_at: IDL.Nat64, expires_at: IDL.Nat64, auto_renew: IDL.Bool, current_usage: UsageMetrics, payment_status: PaymentStatus, created_at: IDL.Nat64, updated_at: IDL.Nat64 })
  const QuotaRemaining = IDL.Record({ agents_remaining: IDL.Nat32, tokens_remaining: IDL.Nat64, inferences_remaining: IDL.Nat32 })
  const QuotaValidation = IDL.Record({ allowed: IDL.Bool, reason: IDL.Opt(IDL.Text), remaining_quota: IDL.Opt(QuotaRemaining) })
  const SubscriptionStats = IDL.Record({ total_subscriptions: IDL.Nat32, active_subscriptions: IDL.Nat32, expired_subscriptions: IDL.Nat32, pending_payments: IDL.Nat32, tier_distribution: IDL.Vec(IDL.Tuple(IDL.Text, IDL.Nat32)), total_monthly_revenue_usd: IDL.Nat32 })
  
  // Payment types
  const PaymentRequest = IDL.Record({ payment_id: IDL.Text, subscription_tier: IDL.Text, amount_usd: IDL.Nat32, amount_icp_e8s: IDL.Nat64, expires_at: IDL.Nat64, created_at: IDL.Nat64 })
  const PaymentTransaction = IDL.Record({ transaction_id: IDL.Text, payment_request: PaymentRequest, from_principal: IDL.Text, amount_paid_e8s: IDL.Nat64, icp_usd_rate: IDL.Float64, status: IDL.Text, created_at: IDL.Nat64, completed_at: IDL.Opt(IDL.Nat64) })
  const PaymentVerification = IDL.Record({ verified: IDL.Bool, transaction_id: IDL.Text, amount_verified: IDL.Nat64, verification_time: IDL.Nat64 })
  const PaymentStats = IDL.Record({ total_transactions: IDL.Nat32, successful_transactions: IDL.Nat32, failed_transactions: IDL.Nat32, total_volume_icp_e8s: IDL.Nat64, total_volume_usd: IDL.Nat32 })
  
  // Result types
  const ResultText = IDL.Variant({ Ok: IDL.Text, Err: IDL.Text })
  const ResultQuote = IDL.Variant({ Ok: CostQuote, Err: IDL.Text })
  const ResultBalance = IDL.Variant({ Ok: Balance, Err: IDL.Text })
  const ResultEscrow = IDL.Variant({ Ok: EscrowAccount, Err: IDL.Text })
  const ResultReceipt = IDL.Variant({ Ok: Receipt, Err: IDL.Text })
  const ResultReceipts = IDL.Variant({ Ok: IDL.Vec(Receipt), Err: IDL.Text })
  const ResultUnit = IDL.Variant({ Ok: IDL.Null, Err: IDL.Text })
  const ResultUserSubscription = IDL.Variant({ Ok: UserSubscription, Err: IDL.Text })
  const ResultQuotaValidation = IDL.Variant({ Ok: QuotaValidation, Err: IDL.Text })
  const ResultPaymentRequest = IDL.Variant({ Ok: PaymentRequest, Err: IDL.Text })
  const ResultPaymentTransaction = IDL.Variant({ Ok: PaymentTransaction, Err: IDL.Text })
  const ResultPaymentVerification = IDL.Variant({ Ok: PaymentVerification, Err: IDL.Text })
  const ResultFloat64 = IDL.Variant({ Ok: IDL.Float64, Err: IDL.Text })
  const ResultNat64 = IDL.Variant({ Ok: IDL.Nat64, Err: IDL.Text })
  return IDL.Service({
    // Core economics APIs
    health: IDL.Func([], [EconHealth], ['query']),
    estimate: IDL.Func([JobSpec], [ResultQuote], ['query']),
    escrow: IDL.Func([IDL.Text, IDL.Nat64], [ResultText], []),
    get_balance: IDL.Func([IDL.Opt(IDL.Text)], [ResultBalance], ['query']),
    get_escrow: IDL.Func([IDL.Text], [ResultEscrow], ['query']),
    get_receipt: IDL.Func([IDL.Text], [ResultReceipt], ['query']),
    list_receipts: IDL.Func([IDL.Opt(IDL.Text), IDL.Opt(IDL.Nat32)], [ResultReceipts], ['query']),
    policy: IDL.Func([], [FeePolicy], ['query']),
    refund_escrow: IDL.Func([IDL.Text], [ResultUnit], []),
    update_policy: IDL.Func([FeePolicy], [ResultUnit], []),
    deposit: IDL.Func([IDL.Nat64], [ResultUnit], []),
    withdraw: IDL.Func([IDL.Nat64], [ResultUnit], []),
    settle: IDL.Func([Receipt], [ResultText], []),
    
    // Admin role APIs
    is_admin: IDL.Func([], [IDL.Bool], ['query']),
    list_admins: IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    add_admin: IDL.Func([IDL.Text], [ResultUnit], []),
    remove_admin: IDL.Func([IDL.Text], [ResultUnit], []),
    
    // Subscription APIs
    create_subscription: IDL.Func([IDL.Text, IDL.Bool], [ResultUserSubscription], []),
    get_user_subscription: IDL.Func([IDL.Opt(IDL.Text)], [IDL.Opt(UserSubscription)], ['query']),
    get_or_create_free_subscription: IDL.Func([], [ResultUserSubscription], []),
    update_payment_status: IDL.Func([PaymentStatus], [ResultUnit], []),
    validate_agent_creation_quota: IDL.Func([], [ResultQuotaValidation], []),
    validate_token_usage_quota: IDL.Func([IDL.Nat64], [ResultQuotaValidation], []),
    get_user_usage: IDL.Func([IDL.Opt(IDL.Text)], [IDL.Opt(UsageMetrics)], ['query']),
    cancel_subscription: IDL.Func([], [ResultUnit], []),
    renew_subscription: IDL.Func([], [ResultUnit], []),
    
    // Admin subscription APIs
    get_subscription_tiers: IDL.Func([], [IDL.Vec(IDL.Tuple(IDL.Text, TierConfig))], ['query']),
    list_all_subscriptions: IDL.Func([], [IDL.Vec(UserSubscription)], ['query']),
    get_subscription_stats: IDL.Func([], [SubscriptionStats], ['query']),
    
    // Payment APIs
    create_payment_request: IDL.Func([IDL.Text], [ResultPaymentRequest], []),
    process_subscription_payment: IDL.Func([PaymentRequest], [ResultPaymentTransaction], []),
    verify_payment: IDL.Func([IDL.Text], [ResultPaymentVerification], []),
    get_payment_transaction: IDL.Func([IDL.Text], [IDL.Opt(PaymentTransaction)], ['query']),
    list_user_payment_transactions: IDL.Func([IDL.Opt(IDL.Nat32)], [IDL.Vec(PaymentTransaction)], ['query']),
    get_icp_usd_rate: IDL.Func([], [ResultFloat64], ['query']),
    convert_usd_to_icp_e8s: IDL.Func([IDL.Nat32], [ResultNat64], []),
    
    // Admin payment APIs
    get_payment_stats: IDL.Func([], [PaymentStats], ['query']),
    list_all_payment_transactions: IDL.Func([IDL.Opt(IDL.Nat32)], [IDL.Vec(PaymentTransaction)], ['query']),
  })
}

// Export IDL for creating custom actors
export { modelCanisterIdl, agentCanisterIdl, coordinatorCanisterIdl, econCanisterIdl };

// Create actor instances
export const modelCanister = Actor.createActor(modelCanisterIdl, {
  agent,
  canisterId: OHMS_MODEL_CANISTER_ID,
});

export const agentCanister: ActorSubclass<AgentCanister> = Actor.createActor(agentIdlFactory, {
  agent,
  canisterId: OHMS_AGENT_CANISTER_ID,
});

export const coordinatorCanister = Actor.createActor(coordinatorCanisterIdl, {
  agent,
  canisterId: OHMS_COORDINATOR_CANISTER_ID,
});

export const econCanister = Actor.createActor(econCanisterIdl, {
  agent,
  canisterId: OHMS_ECON_CANISTER_ID,
});

// Utility functions
export const getCanisterIds = () => ({
  model: OHMS_MODEL_CANISTER_ID,
  agent: OHMS_AGENT_CANISTER_ID,
  coordinator: OHMS_COORDINATOR_CANISTER_ID,
  econ: OHMS_ECON_CANISTER_ID,
});

// Health check for all canisters
export const healthCheck = async () => {
  try {
    const [modelHealth, agentHealth, coordinatorHealth, econHealth] = await Promise.allSettled([
      (modelCanister as any).health_check?.() ?? Promise.resolve(null),
      agentCanister.health(),
      coordinatorCanister.health(),
      econCanister.health(),
    ]);

    return {
      model: modelHealth.status === 'fulfilled' ? modelHealth.value : null,
      agent: agentHealth.status === 'fulfilled' ? agentHealth.value : null,
      coordinator: coordinatorHealth.status === 'fulfilled' ? coordinatorHealth.value : null,
      econ: econHealth.status === 'fulfilled' ? econHealth.value : null,
    };
  } catch (error) {
    // Removed console log
    throw error;
  }
};

// Swarm helpers
export const setSwarmPolicy = async (policy: { topology: any; mode: any; top_k: number; window_ms: bigint; }) => {
  return coordinatorCanister.set_swarm_policy(policy);
};

export const getSwarmPolicy = async () => {
  return coordinatorCanister.get_swarm_policy();
};

export const routeBestResult = async (req: {
  request_id: string;
  requester: string;
  capabilities_required: string[];
  payload: Uint8Array;
  routing_mode: any;
}, topK: number, windowMs: bigint) => {
  return coordinatorCanister.route_best_result(req, topK, windowMs);
};

// Agent Interaction Functions
export const sendMessageToAgent = async (
  agentId: string, 
  message: string, 
  capabilities?: string[]
): Promise<any> => {
  try {
    const agentActor = createAgentActor();

    const decodeParams = {
      max_tokens: [512],
      temperature: [0.7],
      top_p: [0.9],
      top_k: [],
      repetition_penalty: [1.05],
    } as CandidInferenceRequest['decode_params'];

    const inferenceRequest: CandidInferenceRequest = {
      seed: BigInt(Date.now()),
      prompt: message,
      decode_params: decodeParams,
      msg_id: `msg_${Date.now()}_${Math.random().toString(36).slice(2)}`,
    };

    const result: CandidResultInference = await agentActor.infer(inferenceRequest);

    if ('Err' in result) {
      throw new Error(`Agent inference failed: ${result.Err}`);
    }

    const responsePayload = result.Ok;
    return {
      success: true,
      agentId,
      response: responsePayload.generated_text,
      metadata: responsePayload,
    };
  } catch (error) {
    throw error;
  }
};

// Bind agent and wire inference routes
export const bindAgentAndWireRoutes = async (
  agentId: string,
  modelId: string
): Promise<any> => {
  try {
    const agentActor = createAgentActor();

    const bindResult = await agentActor.bind_model(modelId);

    if ('Err' in bindResult) {
      throw new Error(`Failed to bind model: ${bindResult.Err}`);
    }

    return {
      success: true,
      agentId,
      modelId,
      status: 'ready',
      message: 'Agent bound successfully',
      canisterId: OHMS_AGENT_CANISTER_ID,
    };
  } catch (error) {
    throw error;
  }
};

// Execute coordinator workflow
export const executeCoordinatorWorkflow = async (
  workflow: any
): Promise<any> => {
  // Removed console log
  
  try {
    const results = [];
    
    // Process each node in the workflow
    for (const node of workflow.nodes) {
      if (node.type === 'agent' && node.data.config?.instructions) {
        // Removed console log
        
        // Create agent if not already created
        let agentId = node.data.config.agentId;
        if (!agentId) {
          const agentResult = await createAgentsFromInstructions(
            node.data.config.instructions,
            1,
            node.data.config.capabilities || [],
            node.data.config.priority || 'normal'
          );
          
          if ('Ok' in agentResult) {
            agentId = agentResult.Ok.agent_id;
          } else {
            throw new Error(`Failed to create agent: ${agentResult.Err}`);
          }
        }
        
        // Bind agent and wire routes
        await bindAgentAndWireRoutes(agentId, 'default');
        
        // Test agent communication
        const testMessage = `Execute task: ${node.data.config.instructions}`;
        const response = await sendMessageToAgent(
          agentId, 
          testMessage, 
          node.data.config.capabilities
        );
        
        results.push({
          nodeId: node.id,
          agentId,
          response: response.response,
          status: 'completed'
        });
        
        // Removed console log
      }
    }
    
    return {
      success: true,
      workflowId: workflow.id,
      results,
      message: 'Workflow executed successfully'
    };
    
  } catch (error) {
    // Removed console log
    throw error;
  }
};

// Helpers
export const createAgentActor = (
  param1?: HttpAgent | string,
  param2?: HttpAgent | string
): ActorSubclass<AgentCanister> => {
  let resolvedAgent: HttpAgent = agent;
  let resolvedCanister = OHMS_AGENT_CANISTER_ID;

  if (param1) {
    if (typeof param1 === 'string') {
      resolvedCanister = param1;
    } else {
      resolvedAgent = param1;
    }
  }

  if (param2) {
    if (typeof param2 === 'string') {
      resolvedCanister = param2;
    } else {
      resolvedAgent = param2;
    }
  }

  return Actor.createActor(agentIdlFactory, {
    agent: resolvedAgent,
    canisterId: resolvedCanister,
  });
};

export const createCoordinatorActor = (agentOverride?: HttpAgent, canisterId?: string) =>
  Actor.createActor(coordinatorCanisterIdl, {
    agent: agentOverride || agent,
    canisterId: canisterId || OHMS_COORDINATOR_CANISTER_ID,
  });

export const createModelActor = (agentOverride?: HttpAgent, canisterId?: string) =>
  Actor.createActor(modelCanisterIdl, {
    agent: agentOverride || agent,
    canisterId: canisterId || OHMS_MODEL_CANISTER_ID,
  });

export const listAgents = async (userId: string = 'anonymous', agentOverride?: HttpAgent): Promise<CandidAgentSummary[]> => {
  const actor = agentOverride ? createAgentActor(agentOverride) : agentCanister;
  const res: CandidResultSummaries = await actor.list_user_agents(userId);
  if ('Err' in res) {
    throw new Error(res.Err || 'Failed to list agents');
  }
  return res.Ok;
};

// OHMS 2.0 Agent Spawning Functions
export const createAgentsFromInstructions = async (
  instructions: string, 
  agentCount?: number, 
  capabilities: string[] = [], 
  priority: string = 'normal'
): Promise<any> => {
  const actor = createAgentActor();
  const request: CandidAgentCreationRequest = {
    instruction: instructions,
    agent_count: agentCount ? [agentCount] : [],
    capabilities: capabilities.length ? [capabilities] : [],
    priority: priority ? [priority] : [],
  };

  const result: CandidResultAgentCreation = await actor.create_agent_from_instruction(request);
  return result;
};

export const getAgentCreationStatus = async (requestId: string): Promise<any> => {
  return coordinatorCanister.get_agent_creation_status(requestId);
};

export const getUserQuotaStatus = async (): Promise<any> => {
  return coordinatorCanister.get_user_quota_status();
};

export const listUserAgents = async (userId: string, agentOverride?: HttpAgent): Promise<CandidAgentSummary[]> => {
  return listAgents(userId, agentOverride);
};

export const listInstructionRequests = async (): Promise<any[]> => {
  const result = await coordinatorCanister.list_instruction_requests();
  return result as any[];
};

export const getInstructionAnalysis = async (requestId: string): Promise<any> => {
  return coordinatorCanister.get_instruction_analysis(requestId);
};

export const updateAgentStatus = async (agentId: string, status: string): Promise<any> => {
  return coordinatorCanister.update_agent_status(agentId, status);
};

export const getAgentSpawningMetrics = async (): Promise<any> => {
  return coordinatorCanister.get_agent_spawning_metrics();
};

export const getCoordinationNetworks = async (): Promise<any[]> => {
  const result = await coordinatorCanister.get_coordination_networks();
  return result as any[];
};

// OHMS 2.0 Subscription Management Functions
export const upgradeSubscriptionTier = async (tierName: string): Promise<any> => {
  return coordinatorCanister.upgrade_subscription_tier(tierName);
};

export const getSubscriptionTierInfo = async (): Promise<any[]> => {
  const result = await coordinatorCanister.get_subscription_tier_info();
  return result as any[];
};

// OHMS 2.0 Economics Integration Functions
export const getEconomicsHealth = async (): Promise<any> => {
  return coordinatorCanister.get_economics_health();
};

export const validateTokenUsageQuota = async (tokens: bigint): Promise<any> => {
  return coordinatorCanister.validate_token_usage_quota(tokens);
};

export const listModels = async (state?: any, agentOverride?: HttpAgent): Promise<any[]> => {
  const modelActor = agentOverride ? createModelActor(agentOverride) : modelCanister;
  const res = await modelActor.list_models(state ? [state] : []);
  return res as any[];
};

// Dynamic actor creators (useful to attach a specific agent/identity)
export const createEconActor = (agentOverride?: HttpAgent, canisterId?: string) =>
  Actor.createActor(econCanisterIdl, {
    agent: agentOverride ?? agent,
    canisterId: canisterId || OHMS_ECON_CANISTER_ID,
  });
