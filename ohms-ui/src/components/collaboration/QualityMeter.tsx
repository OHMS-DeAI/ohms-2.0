import React from 'react';

interface QualityMeterProps {
  score: number;
  threshold: number;
}

export const QualityMeter: React.FC<QualityMeterProps> = ({ score, threshold }) => {
  const percentage = (score / threshold) * 100;
  const isComplete = score >= threshold;

  return (
    <div className="flex items-center space-x-3">
      <div className="relative w-24 h-24">
        <svg className="w-full h-full transform -rotate-90">
          <circle
            cx="48"
            cy="48"
            r="40"
            stroke="currentColor"
            strokeWidth="8"
            fill="none"
            className="text-gray-700"
          />
          <circle
            cx="48"
            cy="48"
            r="40"
            stroke="currentColor"
            strokeWidth="8"
            fill="none"
            strokeDasharray={`${2 * Math.PI * 40}`}
            strokeDashoffset={`${2 * Math.PI * 40 * (1 - percentage / 100)}`}
            className={isComplete ? 'text-green-500' : 'text-accent-gold'}
            strokeLinecap="round"
          />
        </svg>
        <div className="absolute inset-0 flex items-center justify-center">
          <span className="text-lg font-bold text-text-primary">
            {Math.round(percentage)}%
          </span>
        </div>
      </div>
      <div>
        <div className="text-sm text-text-secondary">Quality Score</div>
        <div className="text-xl font-bold text-text-primary">
          {score.toFixed(2)} / {threshold.toFixed(2)}
        </div>
        {isComplete && (
          <div className="text-sm text-green-500 font-medium">✓ Threshold Met</div>
        )}
      </div>
    </div>
  );
};
