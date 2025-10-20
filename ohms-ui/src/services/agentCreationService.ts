/**
 * Agent Creation Service for OHMS 2.0
 * Follows the working pattern from llmService.ts
 * Handles real agent creation via canister calls - no mocks or placeholders
 */

import { createAgentsFromInstructions, coordinatorCanister, bindAgentAndWireRoutes } from './canisterService';
import type { HttpAgent } from '@dfinity/agent';
import type { EventEmitter } from './llmService';

const IC_LLAMA_MODEL_ID = 'llama3.1-8b';

export interface AgentCreationRequest {
  instructions: string;
  capabilities: string[];
  priority: 'low' | 'normal' | 'high' | 'critical';
  agentCount?: number;
}

export interface AgentCreationResult {
  requestId: string;
  agentIds: string[];
  status: 'pending' | 'in_progress' | 'completed' | 'failed';
  createdAt: bigint;
  completedAt?: bigint;
  errorMessage?: string;
}

export interface CreatedAgent {
  agentId: string;
  canisterId: string;
  capabilities: string[];
  status: 'creating' | 'ready' | 'active' | 'error';
  modelId: string;
  createdAt: bigint;
}

export enum AgentCreationError {
  QuotaExceeded = 'quota_exceeded',
  InvalidInstructions = 'invalid_instructions',
  CanisterError = 'canister_error',
  NetworkError = 'network_error',
  InternalError = 'internal_error'
}

export interface AgentCreationState {
  isLoading: boolean;
  error: { type: AgentCreationError; message: string } | null;
  creationRequests: Map<string, AgentCreationResult>;
  createdAgents: Map<string, CreatedAgent>;
  currentRequest: AgentCreationResult | null;
}

export interface AgentCreationEvent {
  type: 'agent_creation_started' | 'agent_creation_completed' | 'agent_creation_failed' | 'agent_ready';
  requestId?: string;
  agentId?: string;
  data: any;
}

/**
 * Agent Creation Service - mirrors the llmService pattern exactly
 */
export class AgentCreationService implements EventEmitter<AgentCreationEvent> {
  private state: AgentCreationState = {
    isLoading: false,
    error: null,
    creationRequests: new Map(),
    createdAgents: new Map(),
    currentRequest: null
  };

  private listeners: ((event: AgentCreationEvent) => void)[] = [];

  constructor() {
    // Initialize service - no mocks, only real implementations
  }

  // EventEmitter implementation
  on(callback: (event: AgentCreationEvent) => void): () => void {
    this.listeners.push(callback);
    return () => {
      const index = this.listeners.indexOf(callback);
      if (index > -1) this.listeners.splice(index, 1);
    };
  }

  private emit(event: AgentCreationEvent): void {
    this.listeners.forEach(callback => {
      try {
        callback(event);
      } catch (error) {
        console.error('Event listener error:', error);
      }
    });
  }

  // State management - mirrors llmService pattern
  getState(): AgentCreationState {
    return { ...this.state };
  }

  private setLoading(loading: boolean): void {
    this.state.isLoading = loading;
  }

  private handleError(type: AgentCreationError, message: string): void {
    this.state.error = { type, message };
    this.setLoading(false);
  }

  private clearError(): void {
    this.state.error = null;
  }

  /**
   * Create agents from natural language instructions
   * Uses real canister calls following the chat pattern
   */
  async createAgents(request: AgentCreationRequest): Promise<AgentCreationResult> {
    this.setLoading(true);
    this.clearError();

    try {
      // Validate request
      if (!request.instructions.trim()) {
        throw new Error('Instructions cannot be empty');
      }

      // Create request ID
      const requestId = `req_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;

      // Call the coordinator canister - real implementation
      const response = await createAgentsFromInstructions(
        request.instructions,
        request.agentCount || 1,
        request.capabilities,
        request.priority
      );

      // Handle response following the chat pattern (Ok/Err variants)
      if ('Err' in response) {
        throw new Error(`Agent creation failed: ${response.Err}`);
      }

      // Extract agent creation result
      const result = response.Ok;
      
      const creationResult: AgentCreationResult = {
        requestId,
        agentIds: [result.agent_id || requestId], // Use returned agent_id or fallback
        status: result.status === 'Completed' ? 'completed' : 'in_progress',
        createdAt: BigInt(Date.now()) * BigInt(1000000),
        completedAt: result.status === 'Completed' ? BigInt(Date.now()) * BigInt(1000000) : undefined,
        errorMessage: undefined
      };

      // Store in state
      this.state.creationRequests.set(requestId, creationResult);
      this.state.currentRequest = creationResult;

      // Emit event
      this.emit({
        type: 'agent_creation_started',
        requestId,
        data: creationResult
      });

      // Immediately bind and register the new agent so it is ready for inference
      if (creationResult.agentIds.length > 0) {
        await this.setupCreatedAgent(creationResult.agentIds[0], request, agent);
      }

      return creationResult;

    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error';
      
      // Determine error type
      let errorType = AgentCreationError.InternalError;
      if (errorMessage.includes('quota')) {
        errorType = AgentCreationError.QuotaExceeded;
      } else if (errorMessage.includes('canister')) {
        errorType = AgentCreationError.CanisterError;
      } else if (errorMessage.includes('network') || errorMessage.includes('timeout')) {
        errorType = AgentCreationError.NetworkError;
      }

      this.handleError(errorType, errorMessage);
      
      // Create failed result
      const failedResult: AgentCreationResult = {
        requestId: `failed_${Date.now()}`,
        agentIds: [],
        status: 'failed',
        createdAt: BigInt(Date.now()) * BigInt(1000000),
        errorMessage
      };

      this.emit({
        type: 'agent_creation_failed',
        requestId: failedResult.requestId,
        data: failedResult
      });

      throw error;
    } finally {
      this.setLoading(false);
    }
  }

  /**
   * Setup a newly created agent - bind model and wire routes
  */
  private async setupCreatedAgent(agentId: string, request: AgentCreationRequest, authAgent: HttpAgent): Promise<void> {
    try {
      // Bind agent to the IC-managed capacity pool and wire inference routes through the coordinator
      await bindAgentAndWireRoutes(agentId, IC_LLAMA_MODEL_ID, authAgent);

      // Create agent record
      const agent: CreatedAgent = {
        agentId,
        canisterId: 'ohms-agent',
        capabilities: request.capabilities,
        status: 'ready',
        modelId: IC_LLAMA_MODEL_ID,
        createdAt: BigInt(Date.now()) * BigInt(1000000)
      };

      // Store agent
      this.state.createdAgents.set(agentId, agent);

      // Emit ready event
      this.emit({
        type: 'agent_ready',
        agentId,
        data: agent
      });

      // Update creation request status
      if (this.state.currentRequest?.agentIds.includes(agentId)) {
        this.state.currentRequest.status = 'completed';
        this.state.currentRequest.completedAt = BigInt(Date.now()) * BigInt(1000000);
      }

      this.emit({
        type: 'agent_creation_completed',
        requestId: this.state.currentRequest?.requestId,
        agentId,
        data: agent
      });

    } catch (error) {
      // Update agent status to error
      const agent: CreatedAgent = {
        agentId,
        canisterId: 'unknown',
        capabilities: request.capabilities,
        status: 'error',
        modelId: IC_LLAMA_MODEL_ID,
        createdAt: BigInt(Date.now()) * BigInt(1000000)
      };

      this.state.createdAgents.set(agentId, agent);

      this.emit({
        type: 'agent_creation_failed',
        agentId,
        data: { agent, error }
      });

      throw error;
    }
  }

  /**
   * Get creation status for a request
   */
  async getCreationStatus(requestId: string): Promise<AgentCreationResult | null> {
    // First check local state
    const localResult = this.state.creationRequests.get(requestId);
    if (localResult) {
      return localResult;
    }

    // Query coordinator canister for status
    try {
      const response = await coordinatorCanister.get_agent_creation_status(requestId);
      
      if ('Ok' in response) {
        const result = response.Ok;
        const creationResult: AgentCreationResult = {
          requestId,
          agentIds: result.agent_ids || [],
          status: result.status === 'Completed' ? 'completed' : 'in_progress',
          createdAt: BigInt(result.created_at || Date.now()),
          completedAt: result.completed_at ? BigInt(result.completed_at) : undefined,
          errorMessage: result.error_message || undefined
        };

        // Cache the result
        this.state.creationRequests.set(requestId, creationResult);
        return creationResult;
      }
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Failed to get creation status';
      this.handleError(AgentCreationError.CanisterError, errorMessage);
    }

    return null;
  }

  /**
   * Get a created agent by ID
   */
  getAgent(agentId: string): CreatedAgent | null {
    return this.state.createdAgents.get(agentId) || null;
  }

  /**
   * List all created agents
   */
  listAgents(): CreatedAgent[] {
    return Array.from(this.state.createdAgents.values());
  }

  /**
   * Test agent communication (following directLlmService pattern)
   */
  async testAgent(agentId: string, message: string = "Hello! Are you responsive?"): Promise<boolean> {
    try {
      const agent = this.getAgent(agentId);
      if (!agent || agent.status !== 'ready') {
        return false;
      }

      // Use the working direct LLM service pattern
      const { createDirectLlmService } = await import('./directLlmService');
      const llmService = createDirectLlmService();
      
      const response = await llmService.chatWithAgent(agentId, message);
      return response.success && response.content.length > 0;

    } catch (error) {
      console.error('Agent test failed:', error);
      return false;
    }
  }
}

// Export singleton instance (following llmService pattern)
let agentCreationServiceInstance: AgentCreationService | null = null;

export const getAgentCreationService = (): AgentCreationService => {
  if (!agentCreationServiceInstance) {
    agentCreationServiceInstance = new AgentCreationService();
  }
  return agentCreationServiceInstance;
};

// Export already done above with class declaration
