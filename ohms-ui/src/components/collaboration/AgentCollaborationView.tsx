import React from 'react';

interface AgentCollaborationViewProps {
  queenId?: string;
  workerIds: string[];
  peerMessages?: any[];
  iterations?: any[];
  status?: any;
}

export const AgentCollaborationView: React.FC<AgentCollaborationViewProps> = ({
  queenId,
  workerIds,
}) => {
  return (
    <div className="p-4 bg-primary-dark rounded-lg">
      <div className="space-y-4">
        {/* Queen Agent */}
        {queenId && (
          <div className="flex items-center space-x-3">
            <div className="w-12 h-12 rounded-full bg-accent-gold flex items-center justify-center text-text-on-dark font-bold">
              Q
            </div>
            <div>
              <div className="text-sm text-text-secondary">Queen Agent</div>
              <div className="text-text-primary font-mono text-sm">{queenId}</div>
            </div>
          </div>
        )}

        {/* Worker Agents */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
          {workerIds.map((workerId, idx) => (
            <div key={workerId} className="flex items-center space-x-3 p-3 bg-primary rounded">
              <div className="w-10 h-10 rounded-full bg-accent-purple flex items-center justify-center text-text-on-dark font-bold text-sm">
                W{idx + 1}
              </div>
              <div>
                <div className="text-xs text-text-secondary">Worker Agent</div>
                <div className="text-text-primary font-mono text-xs">{workerId}</div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};
