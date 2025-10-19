import { Actor, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { idlFactory } from '../declarations/ohms_coordinator';
import type { _SERVICE } from '../declarations/ohms_coordinator/ohms_coordinator.did.d';
import { getCanisterIdsFromEnv } from '../config/network';

export interface OrchestrationTask {
  task_id: string;
  user_id: string;
  instructions: string;
  queen_agent_id?: string;
  worker_agents: string[];
  status: TaskStatus;
  iterations: IterationRecord[];
  quality_score: number;
  quality_threshold: number;
  max_iterations: number;
  created_at: bigint;
  completed_at?: bigint;
  error_message?: string;
}

export interface IterationRecord {
  iteration_num: number;
  queen_plan: string;
  worker_executions: WorkerExecution[];
  peer_communications: PeerMessage[];
  queen_synthesis: string;
  quality_score: number;
  timestamp: bigint;
  duration_ms: bigint;
}

export interface WorkerExecution {
  agent_id: string;
  assigned_subtask: string;
  result: string;
  tokens_used: number;
  execution_time_ms: bigint;
  success: boolean;
  error_message?: string;
}

export interface PeerMessage {
  message_id: string;
  from_agent: string;
  to_agent: string;
  message_type: PeerMessageType;
  content: string;
  timestamp: bigint;
}

export type PeerMessageType = 'Question' | 'Answer' | 'Suggestion' | 'Status' | 'Error';

export type TaskStatus = 'Created' | 'Planning' | 'Executing' | 'Reviewing' | 'Completed' | 'Failed' | 'Cancelled';

export interface TaskProgress {
  task_id: string;
  status: TaskStatus;
  current_iteration: number;
  max_iterations: number;
  quality_score: number;
  quality_threshold: number;
  progress_percentage: number;
  estimated_completion_ms?: bigint;
  queen_agent?: string;
  active_workers: number;
  total_tokens_used: number;
}

export interface TaskUpdate {
  task_id: string;
  status: TaskStatus;
  iteration?: IterationRecord;
  progress: TaskProgress;
}

export class OrchestrationService {
  private agent: HttpAgent | null = null;
  private coordinatorCanisterId: string;

  constructor(coordinatorCanisterId?: string) {
    // Get canister ID from constructor, env var, or the network config default
    this.coordinatorCanisterId = coordinatorCanisterId || 
      import.meta.env.VITE_OHMS_COORDINATOR_CANISTER_ID ||
      process.env.CANISTER_ID_OHMS_COORDINATOR ||
      getCanisterIdsFromEnv().ohms_coordinator;
  }

  async initialize(agent: HttpAgent): Promise<void> {
    this.agent = agent;
  }

  private getActor(): _SERVICE {
    if (!this.agent) {
      throw new Error('OrchestrationService not initialized. Call initialize() first.');
    }

    return Actor.createActor<_SERVICE>(idlFactory, {
      agent: this.agent,
      canisterId: Principal.fromText(this.coordinatorCanisterId),
    });
  }

  async createTask(instructions: string): Promise<OrchestrationTask> {
    const actor = this.getActor();
    
    try {
      const result = await actor.create_orchestration_task(instructions);
      
      if ('Ok' in result) {
        return this.convertTask(result.Ok);
      } else {
        throw new Error(result.Err || 'Failed to create orchestration task');
      }
    } catch (error) {
      throw new Error(`Failed to create task: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  async iterateTask(taskId: string): Promise<IterationRecord> {
    const actor = this.getActor();
    
    try {
      const result = await actor.iterate_orchestration_task(taskId);
      
      if ('Ok' in result) {
        return this.convertIteration(result.Ok);
      } else {
        throw new Error(result.Err || 'Failed to iterate task');
      }
    } catch (error) {
      throw new Error(`Failed to iterate task: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  async getTaskStatus(taskId: string): Promise<OrchestrationTask> {
    const actor = this.getActor();
    
    try {
      const result = await actor.get_orchestration_task_status(taskId);
      
      if ('Ok' in result) {
        return this.convertTask(result.Ok);
      } else {
        throw new Error(result.Err || 'Failed to get task status');
      }
    } catch (error) {
      throw new Error(`Failed to get task status: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  async getTaskProgress(taskId: string): Promise<TaskProgress> {
    const actor = this.getActor();
    
    try {
      const result = await actor.get_orchestration_task_progress(taskId);
      
      if ('Ok' in result) {
        return this.convertProgress(result.Ok);
      } else {
        throw new Error(result.Err || 'Failed to get task progress');
      }
    } catch (error) {
      throw new Error(`Failed to get task progress: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  async listTasks(): Promise<OrchestrationTask[]> {
    const actor = this.getActor();
    
    try {
      const result = await actor.list_orchestration_tasks();
      
      if ('Ok' in result) {
        return result.Ok.map((task: any) => this.convertTask(task));
      } else {
        throw new Error(result.Err || 'Failed to list tasks');
      }
    } catch (error) {
      throw new Error(`Failed to list tasks: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  async cancelTask(taskId: string): Promise<void> {
    const actor = this.getActor();
    
    try {
      const result = await actor.cancel_orchestration_task(taskId);
      
      if ('Err' in result) {
        throw new Error(result.Err);
      }
    } catch (error) {
      throw new Error(`Failed to cancel task: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  async streamTaskUpdates(
    taskId: string,
    callback: (update: TaskUpdate) => void,
    intervalMs: number = 2000
  ): Promise<() => void> {
    let isStreaming = true;

    const poll = async () => {
      while (isStreaming) {
        try {
          const [task, progress] = await Promise.all([
            this.getTaskStatus(taskId),
            this.getTaskProgress(taskId)
          ]);

          const update: TaskUpdate = {
            task_id: taskId,
            status: task.status,
            iteration: task.iterations[task.iterations.length - 1],
            progress,
          };

          callback(update);

          if (task.status === 'Completed' || task.status === 'Failed' || task.status === 'Cancelled') {
            isStreaming = false;
            break;
          }

          await new Promise(resolve => setTimeout(resolve, intervalMs));
        } catch (error) {
          isStreaming = false;
          break;
        }
      }
    };

    poll();

    return () => {
      isStreaming = false;
    };
  }

  private convertTask(raw: any): OrchestrationTask {
    return {
      task_id: raw.task_id,
      user_id: raw.user_id,
      instructions: raw.instructions,
      queen_agent_id: raw.queen_agent_id[0],
      worker_agents: raw.worker_agents,
      status: Object.keys(raw.status)[0] as TaskStatus,
      iterations: raw.iterations.map((iter: any) => this.convertIteration(iter)),
      quality_score: raw.quality_score,
      quality_threshold: raw.quality_threshold,
      max_iterations: raw.max_iterations,
      created_at: raw.created_at,
      completed_at: raw.completed_at[0],
      error_message: raw.error_message[0],
    };
  }

  private convertIteration(raw: any): IterationRecord {
    return {
      iteration_num: raw.iteration_num,
      queen_plan: raw.queen_plan,
      worker_executions: raw.worker_executions.map((exec: any) => ({
        agent_id: exec.agent_id,
        assigned_subtask: exec.assigned_subtask,
        result: exec.result,
        tokens_used: exec.tokens_used,
        execution_time_ms: exec.execution_time_ms,
        success: exec.success,
        error_message: exec.error_message[0],
      })),
      peer_communications: raw.peer_communications.map((msg: any) => ({
        message_id: msg.message_id,
        from_agent: msg.from_agent,
        to_agent: msg.to_agent,
        message_type: Object.keys(msg.message_type)[0] as PeerMessageType,
        content: msg.content,
        timestamp: msg.timestamp,
      })),
      queen_synthesis: raw.queen_synthesis,
      quality_score: raw.quality_score,
      timestamp: raw.timestamp,
      duration_ms: raw.duration_ms,
    };
  }

  private convertProgress(raw: any): TaskProgress {
    return {
      task_id: raw.task_id,
      status: Object.keys(raw.status)[0] as TaskStatus,
      current_iteration: raw.current_iteration,
      max_iterations: raw.max_iterations,
      quality_score: raw.quality_score,
      quality_threshold: raw.quality_threshold,
      progress_percentage: raw.progress_percentage,
      estimated_completion_ms: raw.estimated_completion_ms[0],
      queen_agent: raw.queen_agent[0],
      active_workers: raw.active_workers,
      total_tokens_used: raw.total_tokens_used,
    };
  }
}

export const orchestrationService = new OrchestrationService();

