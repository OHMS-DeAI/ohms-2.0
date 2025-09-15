// Enhanced OHMS Canister Service with Shared Types
// This service provides a unified interface to all OHMS canisters using shared types

import { Actor, HttpAgent, Identity } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { AuthClient } from '@dfinity/auth-client';
import {
  SystemHealth,
  ModelInfo,
  AgentInfo,
  AgentSpec,
  InferenceRequest,
  InferenceResponse,
  CoordinationRequest,
  CoordinationStatus,
  JobCost,
  Balance,
  EscrowAccount,
  OHMSResult,
  OHMSError,
  CanisterConfig,
  isError,
  isSuccess,
  extractError,
  NOVAQCompressionResult,
  SubscriptionInfo,
  QuotaInfo
} from './index';

// Canister ID configuration
const CANISTER_IDS: CanisterConfig = {
  model: process.env.NEXT_PUBLIC_OHMS_MODEL_CANISTER_ID || 'rdmx6-jaaaa-aaaaa-aaadq-cai',
  agent: process.env.NEXT_PUBLIC_OHMS_AGENT_CANISTER_ID || 'rrkah-fqaaa-aaaaa-aaaaq-cai',
  coordinator: process.env.NEXT_PUBLIC_OHMS_COORDINATOR_CANISTER_ID || 'ryjl3-tyaaa-aaaaa-aaaba-cai',
  econ: process.env.NEXT_PUBLIC_OHMS_ECON_CANISTER_ID || 'rtlzz-xyaaa-aaaaa-aaacq-cai',
};

// Network configuration
const HOST = process.env.NODE_ENV === 'production' 
  ? 'https://ic0.app' 
  : 'http://localhost:4943';

// Initialize HTTP Agent
let agent: HttpAgent;
let authClient: AuthClient;

const initOHMSAgent = async (): Promise<void> => {
  agent = new HttpAgent({
    host: HOST,
  });

  // Fetch root key for local development
  if (process.env.NODE_ENV !== 'production') {
    await agent.fetchRootKey();
  }

  // Initialize auth client
  authClient = await AuthClient.create({
    idleOptions: {
      disableIdle: true,
      disableDefaultIdleCallback: true,
    },
  });
};

// ==============================================================================
// Model Repository Service
// ==============================================================================

export class ModelRepoService {
  private actor: any;

  constructor() {
    this.actor = Actor.createActor(this.getIDL(), {
      agent,
      canisterId: CANISTER_IDS.model,
    });
  }

  private getIDL() {
    // Simplified IDL - in production this would be generated from the Candid files
    return ({ IDL }: any) => {
      const ModelInfo = IDL.Record({
        model_id: IDL.Text,
        version: IDL.Text,
        state: IDL.Variant({
          Pending: IDL.Null,
          Active: IDL.Null,
          Deprecated: IDL.Null,
        }),
        compression_type: IDL.Variant({
          NOVAQ: IDL.Null,
          Uncompressed: IDL.Null,
        }),
        compression_ratio: IDL.Opt(IDL.Float32),
        accuracy_retention: IDL.Opt(IDL.Float32),
        size_bytes: IDL.Nat64,
        uploaded_at: IDL.Nat64,
        activated_at: IDL.Opt(IDL.Nat64),
      });

      const OHMSResult = (T: any) => IDL.Variant({
        Ok: T,
        Err: IDL.Variant({
          InvalidInput: IDL.Text,
          NotFound: IDL.Text,
          Unauthorized: IDL.Text,
          InternalError: IDL.Text,
          NetworkError: IDL.Text,
          QuotaExceeded: IDL.Text,
          InsufficientFunds: IDL.Text,
          ModelNotReady: IDL.Text,
          CompressionFailed: IDL.Text,
        }),
      });

      return IDL.Service({
        health: IDL.Func([], [IDL.Text], ['query']),
        get_model_info: IDL.Func([IDL.Text], [OHMSResult(ModelInfo)], ['query']),
        list_active_models: IDL.Func([], [IDL.Vec(ModelInfo)], ['query']),
        upload_model: IDL.Func([IDL.Text, IDL.Blob], [OHMSResult(IDL.Text)], []),
        activate_model: IDL.Func([IDL.Text], [OHMSResult(IDL.Text)], []),
        deprecate_model: IDL.Func([IDL.Text], [OHMSResult(IDL.Text)], []),
        get_compression_stats: IDL.Func([], [IDL.Text], ['query']),
      });
    };
  }

  async health(): Promise<string> {
    try {
      return await this.actor.health();
    } catch (error) {
      console.error('Model repo health check failed:', error);
      return 'Unhealthy';
    }
  }

  async getModelInfo(modelId: string): Promise<OHMSResult<ModelInfo>> {
    try {
      return await this.actor.get_model_info(modelId);
    } catch (error) {
      console.error('Failed to get model info:', error);
      return { Err: { NetworkError: 'Failed to communicate with model repository' } };
    }
  }

  async listActiveModels(): Promise<ModelInfo[]> {
    try {
      return await this.actor.list_active_models();
    } catch (error) {
      console.error('Failed to list models:', error);
      return [];
    }
  }

  async uploadModel(modelId: string, modelData: Uint8Array): Promise<OHMSResult<string>> {
    try {
      return await this.actor.upload_model(modelId, modelData);
    } catch (error) {
      console.error('Failed to upload model:', error);
      return { Err: { NetworkError: 'Failed to upload model' } };
    }
  }

  async activateModel(modelId: string): Promise<OHMSResult<string>> {
    try {
      return await this.actor.activate_model(modelId);
    } catch (error) {
      console.error('Failed to activate model:', error);
      return { Err: { NetworkError: 'Failed to activate model' } };
    }
  }

  async getCompressionStats(): Promise<string> {
    try {
      return await this.actor.get_compression_stats();
    } catch (error) {
      console.error('Failed to get compression stats:', error);
      return 'Unable to retrieve stats';
    }
  }
}

// ==============================================================================
// Agent Service
// ==============================================================================

export class AgentService {
  private actor: any;

  constructor() {
    this.actor = Actor.createActor(this.getIDL(), {
      agent,
      canisterId: CANISTER_IDS.agent,
    });
  }

  private getIDL() {
    return ({ IDL }: any) => {
      const AgentType = IDL.Variant({
        GeneralAssistant: IDL.Null,
        CodeAssistant: IDL.Null,
        ContentCreator: IDL.Null,
        DataAnalyst: IDL.Null,
        ProblemSolver: IDL.Null,
        Coordinator: IDL.Null,
        Researcher: IDL.Null,
        Planner: IDL.Null,
        Executor: IDL.Null,
        Custom: IDL.Text,
      });

      const AgentStatus = IDL.Variant({
        Creating: IDL.Null,
        Ready: IDL.Null,
        Active: IDL.Null,
        Paused: IDL.Null,
        Completed: IDL.Null,
        Error: IDL.Text,
      });

      const AgentInfo = IDL.Record({
        agent_id: IDL.Text,
        agent_type: AgentType,
        model_id: IDL.Text,
        capabilities: IDL.Vec(IDL.Text),
        status: AgentStatus,
        created_at: IDL.Nat64,
        last_active: IDL.Nat64,
        health_score: IDL.Float32,
      });

      const InferenceRequest = IDL.Record({
        msg_id: IDL.Text,
        prompt: IDL.Text,
        max_tokens: IDL.Opt(IDL.Nat32),
        temperature: IDL.Opt(IDL.Float32),
        top_p: IDL.Opt(IDL.Float32),
        seed: IDL.Nat64,
      });

      const InferenceResponse = IDL.Record({
        generated_text: IDL.Text,
        tokens: IDL.Vec(IDL.Text),
        inference_time_ms: IDL.Nat64,
        cache_hits: IDL.Nat32,
        cache_misses: IDL.Nat32,
      });

      const OHMSResult = (T: any) => IDL.Variant({
        Ok: T,
        Err: IDL.Variant({
          InvalidInput: IDL.Text,
          NotFound: IDL.Text,
          Unauthorized: IDL.Text,
          InternalError: IDL.Text,
          NetworkError: IDL.Text,
          QuotaExceeded: IDL.Text,
          InsufficientFunds: IDL.Text,
          ModelNotReady: IDL.Text,
          CompressionFailed: IDL.Text,
        }),
      });

      return IDL.Service({
        health: IDL.Func([], [IDL.Text], ['query']),
        create_agent: IDL.Func([AgentInfo], [OHMSResult(AgentInfo)], []),
        get_agent_info: IDL.Func([IDL.Text], [OHMSResult(AgentInfo)], ['query']),
        list_user_agents: IDL.Func([], [IDL.Vec(AgentInfo)], ['query']),
        inference: IDL.Func([InferenceRequest], [OHMSResult(InferenceResponse)], []),
        pause_agent: IDL.Func([IDL.Text], [OHMSResult(IDL.Text)], []),
        resume_agent: IDL.Func([IDL.Text], [OHMSResult(IDL.Text)], []),
        delete_agent: IDL.Func([IDL.Text], [OHMSResult(IDL.Text)], []),
      });
    };
  }

  async health(): Promise<string> {
    try {
      return await this.actor.health();
    } catch (error) {
      console.error('Agent health check failed:', error);
      return 'Unhealthy';
    }
  }

  async createAgent(agentInfo: AgentInfo): Promise<OHMSResult<AgentInfo>> {
    try {
      return await this.actor.create_agent(agentInfo);
    } catch (error) {
      console.error('Failed to create agent:', error);
      return { Err: { NetworkError: 'Failed to create agent' } };
    }
  }

  async getAgentInfo(agentId: string): Promise<OHMSResult<AgentInfo>> {
    try {
      return await this.actor.get_agent_info(agentId);
    } catch (error) {
      console.error('Failed to get agent info:', error);
      return { Err: { NetworkError: 'Failed to get agent info' } };
    }
  }

  async listUserAgents(): Promise<AgentInfo[]> {
    try {
      return await this.actor.list_user_agents();
    } catch (error) {
      console.error('Failed to list user agents:', error);
      return [];
    }
  }

  async sendInferenceRequest(request: InferenceRequest): Promise<OHMSResult<InferenceResponse>> {
    try {
      return await this.actor.inference(request);
    } catch (error) {
      console.error('Failed to send inference request:', error);
      return { Err: { NetworkError: 'Failed to send inference request' } };
    }
  }

  async pauseAgent(agentId: string): Promise<OHMSResult<string>> {
    try {
      return await this.actor.pause_agent(agentId);
    } catch (error) {
      console.error('Failed to pause agent:', error);
      return { Err: { NetworkError: 'Failed to pause agent' } };
    }
  }

  async resumeAgent(agentId: string): Promise<OHMSResult<string>> {
    try {
      return await this.actor.resume_agent(agentId);
    } catch (error) {
      console.error('Failed to resume agent:', error);
      return { Err: { NetworkError: 'Failed to resume agent' } };
    }
  }

  async deleteAgent(agentId: string): Promise<OHMSResult<string>> {
    try {
      return await this.actor.delete_agent(agentId);
    } catch (error) {
      console.error('Failed to delete agent:', error);
      return { Err: { NetworkError: 'Failed to delete agent' } };
    }
  }
}

// ==============================================================================
// Coordinator Service
// ==============================================================================

export class CoordinatorService {
  private actor: any;

  constructor() {
    this.actor = Actor.createActor(this.getIDL(), {
      agent,
      canisterId: CANISTER_IDS.coordinator,
    });
  }

  private getIDL() {
    return ({ IDL }: any) => {
      const CoordinationType = IDL.Variant({
        None: IDL.Null,
        Sequential: IDL.Null,
        Parallel: IDL.Null,
        Collaborative: IDL.Null,
        Hierarchical: IDL.Null,
      });

      const CoordinationRequest = IDL.Record({
        request_id: IDL.Text,
        user_principal: IDL.Text,
        instructions: IDL.Text,
        coordination_type: CoordinationType,
        agent_requirements: IDL.Vec(IDL.Text), // Simplified for now
        created_at: IDL.Nat64,
      });

      const CoordinationStatus = IDL.Record({
        request_id: IDL.Text,
        status: IDL.Text,
        created_agents: IDL.Vec(IDL.Text),
        completed_tasks: IDL.Vec(IDL.Text),
        pending_tasks: IDL.Vec(IDL.Text),
        last_updated: IDL.Nat64,
      });

      const OHMSResult = (T: any) => IDL.Variant({
        Ok: T,
        Err: IDL.Variant({
          InvalidInput: IDL.Text,
          NotFound: IDL.Text,
          Unauthorized: IDL.Text,
          InternalError: IDL.Text,
          NetworkError: IDL.Text,
          QuotaExceeded: IDL.Text,
          InsufficientFunds: IDL.Text,
          ModelNotReady: IDL.Text,
          CompressionFailed: IDL.Text,
        }),
      });

      return IDL.Service({
        health: IDL.Func([], [IDL.Text], ['query']),
        analyze_instruction_and_spawn_agents: IDL.Func([CoordinationRequest], [OHMSResult(IDL.Text)], []),
        get_coordination_status: IDL.Func([IDL.Text], [OHMSResult(CoordinationStatus)], ['query']),
        list_user_coordinations: IDL.Func([], [IDL.Vec(CoordinationStatus)], ['query']),
        cancel_coordination: IDL.Func([IDL.Text], [OHMSResult(IDL.Text)], []),
      });
    };
  }

  async health(): Promise<string> {
    try {
      return await this.actor.health();
    } catch (error) {
      console.error('Coordinator health check failed:', error);
      return 'Unhealthy';
    }
  }

  async requestCoordination(request: CoordinationRequest): Promise<OHMSResult<string>> {
    try {
      return await this.actor.analyze_instruction_and_spawn_agents(request);
    } catch (error) {
      console.error('Failed to request coordination:', error);
      return { Err: { NetworkError: 'Failed to request coordination' } };
    }
  }

  async getCoordinationStatus(requestId: string): Promise<OHMSResult<CoordinationStatus>> {
    try {
      return await this.actor.get_coordination_status(requestId);
    } catch (error) {
      console.error('Failed to get coordination status:', error);
      return { Err: { NetworkError: 'Failed to get coordination status' } };
    }
  }

  async listUserCoordinations(): Promise<CoordinationStatus[]> {
    try {
      return await this.actor.list_user_coordinations();
    } catch (error) {
      console.error('Failed to list user coordinations:', error);
      return [];
    }
  }

  async cancelCoordination(requestId: string): Promise<OHMSResult<string>> {
    try {
      return await this.actor.cancel_coordination(requestId);
    } catch (error) {
      console.error('Failed to cancel coordination:', error);
      return { Err: { NetworkError: 'Failed to cancel coordination' } };
    }
  }
}

// ==============================================================================
// Economics Service
// ==============================================================================

export class EconService {
  private actor: any;

  constructor() {
    this.actor = Actor.createActor(this.getIDL(), {
      agent,
      canisterId: CANISTER_IDS.econ,
    });
  }

  private getIDL() {
    return ({ IDL }: any) => {
      const JobPriority = IDL.Variant({
        Low: IDL.Null,
        Normal: IDL.Null,
        High: IDL.Null,
        Critical: IDL.Null,
      });

      const JobCost = IDL.Record({
        job_id: IDL.Text,
        estimated_cost: IDL.Nat64,
        base_cost: IDL.Nat64,
        priority_multiplier: IDL.Float32,
        protocol_fee: IDL.Nat64,
        total_cost: IDL.Nat64,
      });

      const Balance = IDL.Record({
        principal_id: IDL.Text,
        available_balance: IDL.Nat64,
        escrowed_balance: IDL.Nat64,
        total_earnings: IDL.Nat64,
        last_updated: IDL.Nat64,
      });

      const OHMSResult = (T: any) => IDL.Variant({
        Ok: T,
        Err: IDL.Variant({
          InvalidInput: IDL.Text,
          NotFound: IDL.Text,
          Unauthorized: IDL.Text,
          InternalError: IDL.Text,
          NetworkError: IDL.Text,
          QuotaExceeded: IDL.Text,
          InsufficientFunds: IDL.Text,
          ModelNotReady: IDL.Text,
          CompressionFailed: IDL.Text,
        }),
      });

      return IDL.Service({
        health: IDL.Func([], [IDL.Text], ['query']),
        get_cost_quote: IDL.Func([IDL.Text, IDL.Nat32, JobPriority], [OHMSResult(JobCost)], ['query']),
        create_escrow: IDL.Func([JobCost], [OHMSResult(IDL.Text)], []),
        get_balance: IDL.Func([], [OHMSResult(Balance)], ['query']),
        withdraw_funds: IDL.Func([IDL.Nat64], [OHMSResult(IDL.Text)], []),
        get_transaction_history: IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
      });
    };
  }

  async health(): Promise<string> {
    try {
      return await this.actor.health();
    } catch (error) {
      console.error('Econ health check failed:', error);
      return 'Unhealthy';
    }
  }

  async getCostQuote(modelId: string, estimatedTokens: number, priority: string): Promise<OHMSResult<JobCost>> {
    try {
      return await this.actor.get_cost_quote(modelId, estimatedTokens, { [priority]: null });
    } catch (error) {
      console.error('Failed to get cost quote:', error);
      return { Err: { NetworkError: 'Failed to get cost quote' } };
    }
  }

  async createEscrow(jobCost: JobCost): Promise<OHMSResult<string>> {
    try {
      return await this.actor.create_escrow(jobCost);
    } catch (error) {
      console.error('Failed to create escrow:', error);
      return { Err: { NetworkError: 'Failed to create escrow' } };
    }
  }

  async getBalance(): Promise<OHMSResult<Balance>> {
    try {
      return await this.actor.get_balance();
    } catch (error) {
      console.error('Failed to get balance:', error);
      return { Err: { NetworkError: 'Failed to get balance' } };
    }
  }

  async withdrawFunds(amount: bigint): Promise<OHMSResult<string>> {
    try {
      return await this.actor.withdraw_funds(amount);
    } catch (error) {
      console.error('Failed to withdraw funds:', error);
      return { Err: { NetworkError: 'Failed to withdraw funds' } };
    }
  }

  async getTransactionHistory(): Promise<string[]> {
    try {
      return await this.actor.get_transaction_history();
    } catch (error) {
      console.error('Failed to get transaction history:', error);
      return [];
    }
  }
}

// ==============================================================================
// Unified OHMS Service
// ==============================================================================

export class OHMSService {
  public model: ModelRepoService;
  public agent: AgentService;
  public coordinator: CoordinatorService;
  public econ: EconService;

  constructor() {
    this.model = new ModelRepoService();
    this.agent = new AgentService();
    this.coordinator = new CoordinatorService();
    this.econ = new EconService();
  }

  async systemHealthCheck(): Promise<SystemHealth> {
    const [modelHealth, agentHealth, coordinatorHealth, econHealth] = await Promise.allSettled([
      this.model.health(),
      this.agent.health(),
      this.coordinator.health(),
      this.econ.health(),
    ]);

    const mapHealth = (result: PromiseSettledResult<string>) => {
      if (result.status === 'fulfilled') {
        return result.value === 'Healthy' ? 'Healthy' : 'Degraded';
      }
      return 'Unhealthy';
    };

    return {
      model: mapHealth(modelHealth),
      agent: mapHealth(agentHealth),
      coordinator: mapHealth(coordinatorHealth),
      econ: mapHealth(econHealth),
      timestamp: BigInt(Date.now()),
    };
  }

  async createAgentWithPayment(spec: AgentSpec): Promise<OHMSResult<AgentInfo>> {
    try {
      // 1. Get cost quote
      const quoteResult = await this.econ.getCostQuote(
        spec.model_id,
        spec.estimated_tokens || 1000,
        spec.priority
      );

      if (isError(quoteResult)) {
        return quoteResult;
      }

      const quote = quoteResult.Ok;

      // 2. Create escrow
      const escrowResult = await this.econ.createEscrow(quote);
      if (isError(escrowResult)) {
        return { Err: escrowResult.Err };
      }

      // 3. Create agent
      const agentInfo: AgentInfo = {
        agent_id: spec.agent_id,
        agent_type: spec.agent_type,
        model_id: spec.model_id,
        capabilities: spec.capabilities,
        status: 'Creating',
        created_at: BigInt(Date.now()),
        last_active: BigInt(Date.now()),
        health_score: 1.0,
      };

      const createResult = await this.agent.createAgent(agentInfo);
      if (isError(createResult)) {
        // TODO: Refund escrow
        return createResult;
      }

      return createResult;
    } catch (error) {
      console.error('Failed to create agent with payment:', error);
      return { Err: { InternalError: 'Failed to create agent with payment' } };
    }
  }

  async getCanisterIds(): Promise<CanisterConfig> {
    return CANISTER_IDS;
  }
}

// ==============================================================================
// Authentication Utilities
// ==============================================================================

export class AuthService {
  static async login(): Promise<boolean> {
    try {
      if (!authClient) {
        await initOHMSAgent();
      }

      return new Promise((resolve) => {
        authClient.login({
          identityProvider: process.env.NODE_ENV === 'production' 
            ? 'https://identity.ic0.app/'
            : `http://localhost:4943/?canisterId=${process.env.NEXT_PUBLIC_INTERNET_IDENTITY_CANISTER_ID}`,
          onSuccess: () => {
            resolve(true);
          },
          onError: (error) => {
            console.error('Login failed:', error);
            resolve(false);
          },
        });
      });
    } catch (error) {
      console.error('Login initialization failed:', error);
      return false;
    }
  }

  static async logout(): Promise<void> {
    if (authClient) {
      await authClient.logout();
    }
  }

  static async isAuthenticated(): Promise<boolean> {
    if (!authClient) {
      await initOHMSAgent();
    }
    return await authClient.isAuthenticated();
  }

  static async getIdentity(): Promise<Identity | null> {
    if (!authClient) {
      await initOHMSAgent();
    }
    const isAuth = await authClient.isAuthenticated();
    if (isAuth) {
      return authClient.getIdentity();
    }
    return null;
  }

  static async getPrincipal(): Promise<Principal | null> {
    const identity = await this.getIdentity();
    if (identity) {
      return identity.getPrincipal();
    }
    return null;
  }
}

// ==============================================================================
// Exports
// ==============================================================================

export {
  CANISTER_IDS,
  initOHMSAgent,
  isError,
  isSuccess,
  extractError,
};

// Default export
const ohmsService = new OHMSService();
export default ohmsService;
