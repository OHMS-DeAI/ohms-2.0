import { useState, useEffect, useCallback, useRef } from 'react';
import type {
  OrchestrationTask,
  TaskProgress,
  TaskUpdate,
} from '../services/orchestrationService';
import { orchestrationService } from '../services/orchestrationService';

export interface UseTaskProgressResult {
  task: OrchestrationTask | null;
  progress: TaskProgress | null;
  loading: boolean;
  error: string | null;
  isActive: boolean;
  refresh: () => Promise<void>;
}

export function useTaskProgress(taskId: string | null): UseTaskProgressResult {
  const [task, setTask] = useState<OrchestrationTask | null>(null);
  const [progress, setProgress] = useState<TaskProgress | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isActive, setIsActive] = useState(true);
  const stopStreamingRef = useRef<(() => void) | null>(null);

  const refresh = useCallback(async () => {
    if (!taskId) {
      return;
    }

    try {
      setError(null);
      const [taskData, progressData] = await Promise.all([
        orchestrationService.getTaskStatus(taskId),
        orchestrationService.getTaskProgress(taskId),
      ]);
      
      setTask(taskData);
      setProgress(progressData);
      
      const activeStatuses = ['Created', 'Planning', 'Executing', 'Reviewing'];
      setIsActive(activeStatuses.includes(taskData.status));
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch task data');
    } finally {
      setLoading(false);
    }
  }, [taskId]);

  useEffect(() => {
    if (!taskId) {
      setTask(null);
      setProgress(null);
      setLoading(false);
      return;
    }

    setLoading(true);

    const handleUpdate = (update: TaskUpdate) => {
      if (update.task_id === taskId) {
        setTask(prevTask => ({
          ...prevTask!,
          status: update.status,
          iterations: update.iteration ? [...(prevTask?.iterations || []), update.iteration] : prevTask?.iterations || [],
        }));
        setProgress(update.progress);
        
        const activeStatuses = ['Created', 'Planning', 'Executing', 'Reviewing'];
        setIsActive(activeStatuses.includes(update.status));
      }
    };

    orchestrationService.streamTaskUpdates(taskId, handleUpdate, 2000)
      .then(stopFn => {
        stopStreamingRef.current = stopFn;
      })
      .catch(err => {
        setError(err instanceof Error ? err.message : 'Failed to stream updates');
        setLoading(false);
      });

    refresh();

    return () => {
      if (stopStreamingRef.current) {
        stopStreamingRef.current();
        stopStreamingRef.current = null;
      }
    };
  }, [taskId, refresh]);

  return {
    task,
    progress,
    loading,
    error,
    isActive,
    refresh,
  };
}

export default useTaskProgress;

