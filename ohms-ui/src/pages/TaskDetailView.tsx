import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { useTaskProgress } from '../hooks/useTaskProgress';
import { orchestrationService } from '../services/orchestrationService';
import Button from '../components/Button';
import Card from '../components/Card';
import LoadingSpinner from '../components/LoadingSpinner';
import { AgentCollaborationView } from '../components/collaboration/AgentCollaborationView';
import { IterationCard } from '../components/collaboration/IterationCard';
import { QualityMeter } from '../components/collaboration/QualityMeter';
import { useAgent } from '../context/AgentContext';

const TaskDetailView: React.FC = () => {
  const { taskId } = useParams<{ taskId: string }>();
  const navigate = useNavigate();
  const { isConnected, createAuthAgent } = useAgent();
  const { task, progress, loading, error, refresh } = useTaskProgress(taskId || null);
  const [iterating, setIterating] = useState(false);
  const [iterationError, setIterationError] = useState<string | null>(null);
  const [expandedIteration, setExpandedIteration] = useState<number | null>(null);

  useEffect(() => {
    const initOrchestration = async () => {
      if (isConnected) {
        const agent = await createAuthAgent();
        if (agent) {
          await orchestrationService.initialize(agent);
        }
      }
    };
    initOrchestration();
  }, [isConnected, createAuthAgent]);

  const handleIterate = async () => {
    if (!taskId) return;

    try {
      setIterating(true);
      setIterationError(null);
      await orchestrationService.iterateTask(taskId);
      await refresh();
    } catch (err) {
      setIterationError(err instanceof Error ? err.message : 'Failed to iterate task');
    } finally {
      setIterating(false);
    }
  };

  const handleCancel = async () => {
    if (!taskId) return;

    if (window.confirm('Are you sure you want to cancel this task?')) {
      try {
        await orchestrationService.cancelTask(taskId);
        await refresh();
      } catch (err) {
        setIterationError(err instanceof Error ? err.message : 'Failed to cancel task');
      }
    }
  };

  if (loading) {
    return (
      <div className="max-w-6xl mx-auto p-6">
        <div className="flex justify-center py-12">
          <LoadingSpinner className="w-12 h-12" />
        </div>
      </div>
    );
  }

  if (error || !task) {
    return (
      <div className="max-w-6xl mx-auto p-6">
        <Card>
          <div className="text-center py-8">
            <h2 className="text-2xl font-bold mb-4 text-red-600">Error</h2>
            <p className="text-gray-600 mb-6">{error || 'Task not found'}</p>
            <Button onClick={() => navigate('/orchestrate')}>
              Back to Orchestrator
            </Button>
          </div>
        </Card>
      </div>
    );
  }

  const canIterate = task.status === 'Executing' || task.status === 'Reviewing';
  const isCompleted = task.status === 'Completed' || task.status === 'Failed' || task.status === 'Cancelled';

  return (
    <div className="max-w-6xl mx-auto p-6">
      <div className="mb-6">
        <Button
          onClick={() => navigate('/orchestrate')}
          variant="secondary"
          size="sm"
          className="mb-4"
        >
          ← Back to Tasks
        </Button>

        <div className="flex items-start justify-between">
          <div className="flex-1">
            <h1 className="text-3xl font-bold mb-2">Orchestration Task</h1>
            <p className="text-gray-600">{task.instructions}</p>
          </div>

          <div className="ml-4">
            <QualityMeter
              score={task.quality_score}
              threshold={task.quality_threshold}
            />
          </div>
        </div>
      </div>

      {progress && (
        <Card className="mb-6">
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
            <div>
              <p className="text-sm text-gray-600 mb-1">Status</p>
              <p className="text-lg font-semibold">{task.status}</p>
            </div>
            <div>
              <p className="text-sm text-gray-600 mb-1">Iterations</p>
              <p className="text-lg font-semibold">
                {progress.current_iteration} / {progress.max_iterations}
              </p>
            </div>
            <div>
              <p className="text-sm text-gray-600 mb-1">Queen Agent</p>
              <p className="text-lg font-semibold">
                {task.queen_agent_id ? `👑 ${task.queen_agent_id.substring(0, 8)}...` : 'Not assigned'}
              </p>
            </div>
            <div>
              <p className="text-sm text-gray-600 mb-1">Workers</p>
              <p className="text-lg font-semibold">{task.worker_agents.length}</p>
            </div>
          </div>

          <div className="mt-4 pt-4 border-t">
            <div className="flex items-center justify-between">
              <div className="flex-1">
                <p className="text-sm text-gray-600 mb-1">Quality Progress</p>
                <div className="w-full bg-gray-200 rounded-full h-2.5">
                  <div
                    className="bg-blue-600 h-2.5 rounded-full transition-all"
                    style={{ width: `${progress.progress_percentage}%` }}
                  />
                </div>
                <p className="text-xs text-gray-600 mt-1">
                  {progress.progress_percentage.toFixed(1)}% complete
                </p>
              </div>

              {!isCompleted && (
                <div className="ml-6 flex gap-2">
                  <Button
                    onClick={handleIterate}
                    disabled={!canIterate || iterating}
                    size="sm"
                  >
                    {iterating ? 'Iterating...' : 'Run Iteration'}
                  </Button>
                  <Button
                    onClick={handleCancel}
                    variant="secondary"
                    size="sm"
                  >
                    Cancel
                  </Button>
                </div>
              )}
            </div>
          </div>

          {iterationError && (
            <div className="mt-4 p-3 bg-red-50 border border-red-200 rounded-md">
              <p className="text-red-600 text-sm">{iterationError}</p>
            </div>
          )}
        </Card>
      )}

      {task.queen_agent_id && task.worker_agents.length > 0 && (
        <Card className="mb-6">
          <h2 className="text-xl font-bold mb-4">Agent Collaboration</h2>
          <AgentCollaborationView
            queenId={task.queen_agent_id}
            workerIds={task.worker_agents}
          />
        </Card>
      )}

      <div>
        <h2 className="text-2xl font-bold mb-4">Iteration History</h2>

        {task.iterations.length === 0 ? (
          <Card>
            <p className="text-center text-gray-600 py-8">
              No iterations yet. Click "Run Iteration" to start the orchestration process.
            </p>
          </Card>
        ) : (
          <div className="space-y-4">
            {task.iterations.map((iteration) => (
              <IterationCard
                key={iteration.iteration_num}
                iteration={iteration}
                isExpanded={expandedIteration === iteration.iteration_num}
                onToggle={() => setExpandedIteration(
                  expandedIteration === iteration.iteration_num ? null : iteration.iteration_num
                )}
              />
            ))}
          </div>
        )}
      </div>

      {isCompleted && task.iterations.length > 0 && (
        <Card className="mt-6 bg-green-50 border-green-200">
          <h3 className="text-xl font-bold mb-3 text-green-800">Final Result</h3>
          <p className="text-gray-800 whitespace-pre-wrap">
            {task.iterations[task.iterations.length - 1].queen_synthesis}
          </p>

          <div className="mt-4 pt-4 border-t border-green-200">
            <div className="flex items-center justify-between text-sm text-gray-600">
              <span>
                Total tokens used: {progress?.total_tokens_used.toLocaleString()}
              </span>
              <span>
                Completed: {task.completed_at ? new Date(Number(task.completed_at) / 1000000).toLocaleString() : 'N/A'}
              </span>
            </div>
          </div>
        </Card>
      )}
    </div>
  );
};

export default TaskDetailView;

