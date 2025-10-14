import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { orchestrationService } from '../services/orchestrationService';
import type { OrchestrationTask } from '../services/orchestrationService';
import Button from '../components/Button';
import Textarea from '../components/Textarea';
import Card from '../components/Card';
import LoadingSpinner from '../components/LoadingSpinner';
import { useAgent } from '../context/AgentContext';

const MultiAgentOrchestrator: React.FC = () => {
  const navigate = useNavigate();
  const { isConnected, createAuthAgent } = useAgent();
  const [instructions, setInstructions] = useState('');
  const [tasks, setTasks] = useState<OrchestrationTask[]>([]);
  const [loading, setLoading] = useState(false);
  const [loadingTasks, setLoadingTasks] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (isConnected) {
      loadTasks();
    }
  }, [isConnected]);

  const loadTasks = async () => {
    try {
      setLoadingTasks(true);
      const agent = await createAuthAgent();
      if (agent) {
        await orchestrationService.initialize(agent);
        const taskList = await orchestrationService.listTasks();
        setTasks(taskList.sort((a, b) => Number(b.created_at - a.created_at)));
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load tasks');
    } finally {
      setLoadingTasks(false);
    }
  };

  const handleCreateTask = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!instructions.trim()) {
      setError('Please enter task instructions');
      return;
    }

    try {
      setLoading(true);
      setError(null);

      const agent = await createAuthAgent();
      if (agent) {
        await orchestrationService.initialize(agent);
        const task = await orchestrationService.createTask(instructions);
        navigate(`/orchestrate/${task.task_id}`);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create task');
    } finally {
      setLoading(false);
    }
  };

  const getStatusColor = (status: string): string => {
    switch (status) {
      case 'Completed':
        return 'text-green-600 bg-green-50';
      case 'Failed':
      case 'Cancelled':
        return 'text-red-600 bg-red-50';
      case 'Executing':
      case 'Planning':
      case 'Reviewing':
        return 'text-blue-600 bg-blue-50';
      default:
        return 'text-gray-600 bg-gray-50';
    }
  };

  const getStatusIcon = (status: string): string => {
    switch (status) {
      case 'Completed':
        return '✓';
      case 'Failed':
        return '✗';
      case 'Cancelled':
        return '⊘';
      case 'Executing':
        return '⚡';
      case 'Planning':
        return '🧠';
      case 'Reviewing':
        return '👁';
      default:
        return '○';
    }
  };

  if (!isConnected) {
    return (
      <div className="max-w-4xl mx-auto p-6">
        <Card>
          <div className="text-center py-8">
            <h2 className="text-2xl font-bold mb-4">Authentication Required</h2>
            <p className="text-gray-600 mb-6">Please connect your wallet to use the Multi-Agent Orchestrator</p>
          </div>
        </Card>
      </div>
    );
  }

  return (
    <div className="max-w-6xl mx-auto p-6">
      <div className="mb-8">
        <h1 className="text-4xl font-bold mb-2">Multi-Agent Orchestrator</h1>
        <p className="text-gray-600">
          Create complex tasks and watch multiple agents collaborate to complete them
        </p>
      </div>

      <Card className="mb-8">
        <h2 className="text-2xl font-bold mb-4">Create New Orchestration Task</h2>
        <form onSubmit={handleCreateTask}>
          <div className="mb-4">
            <label className="block text-sm font-medium mb-2">Task Instructions</label>
            <Textarea
              value={instructions}
              onChange={(e) => setInstructions(e.target.value)}
              placeholder="Describe the task you want the agents to complete. Be as detailed as possible. Example: 'Research the latest developments in quantum computing, create a summary report, and generate a presentation outline.'"
              rows={6}
              className="w-full"
              disabled={loading}
            />
          </div>

          {error && (
            <div className="mb-4 p-4 bg-red-50 border border-red-200 rounded-md">
              <p className="text-red-600 text-sm">{error}</p>
            </div>
          )}

          <Button
            type="submit"
            disabled={loading || !instructions.trim()}
            className="w-full"
          >
            {loading ? (
              <span className="flex items-center justify-center">
                <LoadingSpinner className="w-5 h-5 mr-2" />
                Creating Orchestration...
              </span>
            ) : (
              'Start Multi-Agent Task'
            )}
          </Button>
        </form>
      </Card>

      <div>
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-2xl font-bold">Your Orchestration Tasks</h2>
          <Button
            onClick={loadTasks}
            disabled={loadingTasks}
            variant="secondary"
            size="sm"
          >
            {loadingTasks ? 'Loading...' : 'Refresh'}
          </Button>
        </div>

        {loadingTasks ? (
          <div className="flex justify-center py-12">
            <LoadingSpinner className="w-8 h-8" />
          </div>
        ) : tasks.length === 0 ? (
          <Card>
            <div className="text-center py-8">
              <p className="text-gray-600">No orchestration tasks yet. Create one to get started!</p>
            </div>
          </Card>
        ) : (
          <div className="space-y-4">
            {tasks.map((task) => (
              <Card
                key={task.task_id}
                className="hover:shadow-lg transition-shadow cursor-pointer"
                onClick={() => navigate(`/orchestrate/${task.task_id}`)}
              >
                <div className="flex items-start justify-between">
                  <div className="flex-1">
                    <div className="flex items-center gap-2 mb-2">
                      <span className={`px-3 py-1 rounded-full text-sm font-medium ${getStatusColor(task.status)}`}>
                        <span className="mr-1">{getStatusIcon(task.status)}</span>
                        {task.status}
                      </span>
                      {task.queen_agent_id && (
                        <span className="text-sm text-gray-600">
                          👑 Queen: {task.queen_agent_id.substring(0, 8)}...
                        </span>
                      )}
                      <span className="text-sm text-gray-600">
                        Workers: {task.worker_agents.length}
                      </span>
                    </div>

                    <p className="text-gray-800 mb-2 line-clamp-2">{task.instructions}</p>

                    <div className="flex items-center gap-4 text-sm text-gray-600">
                      <span>Iterations: {task.iterations.length} / {task.max_iterations}</span>
                      <span>Quality: {(task.quality_score * 100).toFixed(1)}%</span>
                      <span>
                        Created: {new Date(Number(task.created_at) / 1000000).toLocaleDateString()}
                      </span>
                    </div>

                    {task.error_message && (
                      <p className="text-red-600 text-sm mt-2">{task.error_message}</p>
                    )}
                  </div>

                  <div className="ml-4">
                    <div className="w-24 h-24">
                      <svg className="w-full h-full" viewBox="0 0 100 100">
                        <circle
                          cx="50"
                          cy="50"
                          r="40"
                          fill="none"
                          stroke="#e5e7eb"
                          strokeWidth="8"
                        />
                        <circle
                          cx="50"
                          cy="50"
                          r="40"
                          fill="none"
                          stroke="#3b82f6"
                          strokeWidth="8"
                          strokeDasharray={`${task.quality_score * 251.2} 251.2`}
                          strokeLinecap="round"
                          transform="rotate(-90 50 50)"
                        />
                        <text
                          x="50"
                          y="50"
                          textAnchor="middle"
                          dy="0.3em"
                          className="text-xl font-bold fill-current text-gray-800"
                        >
                          {(task.quality_score * 100).toFixed(0)}%
                        </text>
                      </svg>
                    </div>
                  </div>
                </div>
              </Card>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

export default MultiAgentOrchestrator;

