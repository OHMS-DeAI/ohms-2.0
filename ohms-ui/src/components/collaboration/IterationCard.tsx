import React from 'react';
import type { IterationRecord } from '../../services/orchestrationService';

interface IterationCardProps {
  iteration: IterationRecord;
  isExpanded: boolean;
  onToggle: () => void;
}

export const IterationCard: React.FC<IterationCardProps> = ({
  iteration,
  isExpanded,
  onToggle,
}) => {
  const formatDuration = (ms: bigint | number) => {
    const msNum = typeof ms === 'bigint' ? Number(ms) : ms;
    if (msNum < 1000) return `${msNum}ms`;
    return `${(msNum / 1000).toFixed(1)}s`;
  };

  return (
    <div className="border border-border-color rounded-lg overflow-hidden">
      {/* Header */}
      <button
        onClick={onToggle}
        className="w-full p-4 bg-primary-dark hover:bg-primary-dark/80 transition-colors flex items-center justify-between"
      >
        <div className="flex items-center space-x-4">
          <div className="text-accent-gold font-bold">
            Iteration {iteration.iteration_num}
          </div>
          <div className="text-sm text-text-secondary">
            Quality: {(iteration.quality_score * 100).toFixed(0)}%
          </div>
          <div className="text-sm text-text-secondary">
            Duration: {formatDuration(iteration.duration_ms)}
          </div>
          <div className="text-sm text-text-secondary">
            Workers: {iteration.worker_executions.length}
          </div>
        </div>
        <svg
          className={`w-5 h-5 text-text-secondary transition-transform ${
            isExpanded ? 'transform rotate-180' : ''
          }`}
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
        </svg>
      </button>

      {/* Expanded Content */}
      {isExpanded && (
        <div className="p-4 space-y-4 bg-primary">
          {/* Queen Plan */}
          <div>
            <h4 className="text-sm font-semibold text-accent-gold mb-2">Queen's Plan</h4>
            <p className="text-text-secondary text-sm whitespace-pre-wrap">{iteration.queen_plan}</p>
          </div>

          {/* Worker Executions */}
          <div>
            <h4 className="text-sm font-semibold text-accent-purple mb-2">Worker Executions</h4>
            <div className="space-y-2">
              {iteration.worker_executions.map((execution, idx) => (
                <div key={idx} className="p-3 bg-primary-dark rounded border border-border-color">
                  <div className="flex items-start justify-between mb-2">
                    <div className="text-xs font-mono text-text-secondary">{execution.agent_id}</div>
                    <div className={`text-xs font-medium ${execution.success ? 'text-green-500' : 'text-red-500'}`}>
                      {execution.success ? '✓ Success' : '✗ Failed'}
                    </div>
                  </div>
                  <div className="text-xs text-text-secondary mb-1">
                    Task: {execution.assigned_subtask}
                  </div>
                  {execution.success ? (
                    <p className="text-xs text-text-primary mt-2">{execution.result}</p>
                  ) : (
                    <p className="text-xs text-red-400 mt-2">{execution.error_message}</p>
                  )}
                  <div className="flex items-center space-x-3 mt-2 text-xs text-text-secondary">
                    <span>Tokens: {execution.tokens_used}</span>
                    <span>Time: {formatDuration(execution.execution_time_ms)}</span>
                  </div>
                </div>
              ))}
            </div>
          </div>

          {/* Queen Synthesis */}
          <div>
            <h4 className="text-sm font-semibold text-accent-gold mb-2">Queen's Synthesis</h4>
            <p className="text-text-secondary text-sm whitespace-pre-wrap">{iteration.queen_synthesis}</p>
          </div>

          {/* Peer Communications */}
          {iteration.peer_communications.length > 0 && (
            <div>
              <h4 className="text-sm font-semibold text-accent-cyan mb-2">
                Peer Communications ({iteration.peer_communications.length})
              </h4>
              <div className="space-y-2">
                {iteration.peer_communications.map((msg) => (
                  <div key={msg.message_id} className="p-2 bg-primary-dark rounded text-xs">
                    <div className="flex items-center space-x-2 mb-1">
                      <span className="font-mono text-accent-cyan">{msg.from_agent.slice(-8)}</span>
                      <span className="text-text-secondary">→</span>
                      <span className="font-mono text-accent-purple">{msg.to_agent.slice(-8)}</span>
                      <span className="text-text-secondary">({msg.message_type})</span>
                    </div>
                    <p className="text-text-secondary">{msg.content}</p>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
};
