// OHMS 2.0 Shared TypeScript Types
// This file contains TypeScript type definitions that mirror the Rust types

export type ComponentHealth = 'Healthy' | 'Degraded' | 'Unhealthy' | 'Unknown';

export interface SystemHealth {
  model: ComponentHealth;
  agent: ComponentHealth;
  coordinator: ComponentHealth;
  econ: ComponentHealth;
  timestamp: bigint;
}

// ==============================================================================
// Model Repository Types
// ==============================================================================

export type CompressionType = 'NOVAQ' | 'Uncompressed';
export type ModelState = 'Pending' | 'Active' | 'Deprecated';

export interface ModelInfo {
  model_id: string;
  version: string;
  state: ModelState;
  compression_type: CompressionType;
  compression_ratio?: number;
  accuracy_retention?: number;
  size_bytes: bigint;
  uploaded_at: bigint;
  activated_at?: bigint;
}

// ==============================================================================
// Agent Types
// ==============================================================================

export type AgentStatus = 
  | 'Creating' 
  | 'Ready' 
  | 'Active' 
  | 'Paused' 
  | 'Completed' 
  | { Error: string };

export type AgentType = 
  | 'GeneralAssistant'
  | 'CodeAssistant'
  | 'ContentCreator'
  | 'DataAnalyst'
  | 'ProblemSolver'
  | 'Coordinator'
  | 'Researcher'
  | 'Planner'
  | 'Executor'
  | { Custom: string };

export type ComplexityLevel = 'Simple' | 'Moderate' | 'Complex' | 'Expert';
export type UrgencyLevel = 'Low' | 'Normal' | 'High' | 'Critical';

export interface AgentInfo {
  agent_id: string;
  agent_type: AgentType;
  model_id: string;
  capabilities: string[];
  status: AgentStatus;
  created_at: bigint;
  last_active: bigint;
  health_score: number;
}

// ==============================================================================
// Economic Types
// ==============================================================================

export type JobPriority = 'Low' | 'Normal' | 'High' | 'Critical';
export type EscrowStatus = 'Pending' | 'Active' | 'Released' | 'Refunded' | 'Expired';

export interface JobCost {
  job_id: string;
  estimated_cost: bigint;
  base_cost: bigint;
  priority_multiplier: number;
  protocol_fee: bigint;
  total_cost: bigint;
}

export interface EscrowAccount {
  escrow_id: string;
  job_id: string;
  principal_id: string;
  amount: bigint;
  status: EscrowStatus;
  created_at: bigint;
  expires_at: bigint;
}

export interface Balance {
  principal_id: string;
  available_balance: bigint;
  escrowed_balance: bigint;
  total_earnings: bigint;
  last_updated: bigint;
}

// ==============================================================================
// Coordination Types
// ==============================================================================

export type CoordinationType = 'None' | 'Sequential' | 'Parallel' | 'Collaborative' | 'Hierarchical';

export interface CoordinationRequest {
  request_id: string;
  user_principal: string;
  instructions: string;
  coordination_type: CoordinationType;
  agent_requirements: AgentRequirement[];
  created_at: bigint;
}

export interface AgentRequirement {
  agent_type: AgentType;
  capabilities: string[];
  complexity: ComplexityLevel;
  urgency: UrgencyLevel;
  model_preferences: string[];
}

export interface CoordinationStatus {
  request_id: string;
  status: string;
  created_agents: string[];
  completed_tasks: string[];
  pending_tasks: string[];
  last_updated: bigint;
}

// ==============================================================================
// NOVAQ Integration Types
// ==============================================================================

export interface NOVAQConfig {
  target_bits: number;
  num_subspaces: number;
  codebook_size_l1: number;
  codebook_size_l2: number;
  outlier_threshold: number;
  teacher_model_path?: string;
  refinement_iterations: number;
  kl_weight: number;
  cosine_weight: number;
  learning_rate: number;
  seed: bigint;
}

export interface NOVAQCompressionResult {
  original_size_mb: number;
  compressed_size_mb: number;
  compression_ratio: number;
  accuracy_retention: number;
  compression_time_seconds: number;
  model_hash: string;
}

// ==============================================================================
// Inference Types
// ==============================================================================

export interface InferenceRequest {
  msg_id: string;
  prompt: string;
  max_tokens?: number;
  temperature?: number;
  top_p?: number;
  seed: bigint;
}

export interface InferenceResponse {
  generated_text: string;
  tokens: string[];
  inference_time_ms: bigint;
  cache_hits: number;
  cache_misses: number;
}

export interface AgentSpec {
  agent_id: string;
  agent_type: AgentType;
  model_id: string;
  capabilities: string[];
  complexity: ComplexityLevel;
  urgency: UrgencyLevel;
  estimated_tokens?: number;
  estimated_compute_cycles?: bigint;
  priority: JobPriority;
}

// ==============================================================================
// Error Types
// ==============================================================================

export type OHMSError = 
  | { InvalidInput: string }
  | { NotFound: string }
  | { Unauthorized: string }
  | { InternalError: string }
  | { NetworkError: string }
  | { QuotaExceeded: string }
  | { InsufficientFunds: string }
  | { ModelNotReady: string }
  | { CompressionFailed: string };

export type OHMSResult<T> = { Ok: T } | { Err: OHMSError };

// ==============================================================================
// Utility Types for Frontend
// ==============================================================================

export interface CanisterConfig {
  model: string;
  agent: string;
  coordinator: string;
  econ: string;
}

export interface AppState {
  isConnected: boolean;
  currentUser?: string;
  systemHealth?: SystemHealth;
  availableModels: ModelInfo[];
  userAgents: AgentInfo[];
  activeCoordinations: CoordinationStatus[];
  userBalance?: Balance;
}

// ==============================================================================
// Event Types for Real-time Updates
// ==============================================================================

export type AppEvent =
  | { type: 'SYSTEM_HEALTH_UPDATE'; payload: SystemHealth }
  | { type: 'MODEL_ACTIVATED'; payload: ModelInfo }
  | { type: 'AGENT_CREATED'; payload: AgentInfo }
  | { type: 'AGENT_STATUS_CHANGED'; payload: { agent_id: string; status: AgentStatus } }
  | { type: 'COORDINATION_UPDATE'; payload: CoordinationStatus }
  | { type: 'BALANCE_UPDATE'; payload: Balance }
  | { type: 'INFERENCE_COMPLETE'; payload: { agent_id: string; response: InferenceResponse } }
  | { type: 'ERROR'; payload: { message: string; error?: OHMSError } };

// ==============================================================================
// API Response Types
// ==============================================================================

export interface APIResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
  timestamp: number;
}

export interface PaginatedResponse<T> {
  items: T[];
  total: number;
  page: number;
  limit: number;
  has_more: boolean;
}

// ==============================================================================
// Configuration Types
// ==============================================================================

export interface OHMSConfig {
  network: 'local' | 'ic';
  canisters: CanisterConfig;
  features: {
    novaq_compression: boolean;
    real_time_updates: boolean;
    payment_integration: boolean;
    multi_agent_coordination: boolean;
  };
}

// ==============================================================================
// Subscription Types
// ==============================================================================

export type SubscriptionTier = 'Basic' | 'Pro' | 'Enterprise';

export interface SubscriptionInfo {
  tier: SubscriptionTier;
  max_agents: number;
  monthly_creations: number;
  token_limit: bigint;
  inference_rate: string;
  agents_created_this_month: number;
  tokens_used_this_month: bigint;
  last_reset_date: bigint;
}

export interface QuotaInfo {
  agents_remaining: number;
  tokens_remaining: bigint;
  inferences_remaining: number;
  next_reset: bigint;
}

// ==============================================================================
// Utility Functions
// ==============================================================================

export const isError = <T>(result: OHMSResult<T>): result is { Err: OHMSError } => {
  return 'Err' in result;
};

export const isSuccess = <T>(result: OHMSResult<T>): result is { Ok: T } => {
  return 'Ok' in result;
};

export const extractError = (error: OHMSError): string => {
  if ('InvalidInput' in error) return `Invalid input: ${error.InvalidInput}`;
  if ('NotFound' in error) return `Not found: ${error.NotFound}`;
  if ('Unauthorized' in error) return `Unauthorized: ${error.Unauthorized}`;
  if ('InternalError' in error) return `Internal error: ${error.InternalError}`;
  if ('NetworkError' in error) return `Network error: ${error.NetworkError}`;
  if ('QuotaExceeded' in error) return `Quota exceeded: ${error.QuotaExceeded}`;
  if ('InsufficientFunds' in error) return `Insufficient funds: ${error.InsufficientFunds}`;
  if ('ModelNotReady' in error) return `Model not ready: ${error.ModelNotReady}`;
  if ('CompressionFailed' in error) return `Compression failed: ${error.CompressionFailed}`;
  return 'Unknown error';
};

export const formatAgentType = (agentType: AgentType): string => {
  if (typeof agentType === 'string') {
    return agentType.replace(/([A-Z])/g, ' $1').trim();
  }
  if ('Custom' in agentType) {
    return agentType.Custom;
  }
  return 'Unknown';
};

export const formatComplexity = (complexity: ComplexityLevel): string => {
  return complexity;
};

export const formatUrgency = (urgency: UrgencyLevel): string => {
  return urgency;
};

export const formatPriority = (priority: JobPriority): string => {
  return priority;
};

export const formatBalance = (balance: bigint): string => {
  // Convert from smallest unit to ICP (assuming 8 decimal places)
  return (Number(balance) / 100_000_000).toFixed(8);
};

export const parseBalance = (balance: string): bigint => {
  // Convert from ICP to smallest unit (assuming 8 decimal places)
  return BigInt(Math.floor(parseFloat(balance) * 100_000_000));
};
