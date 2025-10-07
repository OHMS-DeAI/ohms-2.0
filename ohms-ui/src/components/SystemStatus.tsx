import { useMemo, useState } from 'react'
import { useAdmin } from '../hooks/useAdmin'
import Badge from './Badge'
import type { ComponentHealth } from '@ohms/shared-types'

const badgeVariant = (status: ComponentHealth): 'success' | 'warning' | 'error' | 'info' => {
  switch (status) {
    case 'Healthy':
      return 'success'
    case 'Degraded':
      return 'warning'
    case 'Unknown':
      return 'info'
    default:
      return 'error'
  }
}

const SystemStatus = () => {
  const {
    isAdmin,
    systemHealth,
    getSystemAlerts,
    isSystemHealthy,
    agentHealth,
    agentStatus,
    econHealth,
    econStatus,
    getStatusLabel,
  } = useAdmin()
  const [showDetails, setShowDetails] = useState(false)

  if (!isAdmin) return null

  const healthy = isSystemHealthy()
  const alerts = getSystemAlerts()

  const agentLabel = useMemo(() => {
    if (!agentHealth) return 'Unavailable'
    if (!agentHealth.model_bound) return 'Unbound'
    if (agentHealth.queue_depth > 20) return 'Overloaded'
    if (agentHealth.queue_depth > 10) return 'Backlog'
    return 'Healthy'
  }, [agentHealth])

  const econLabel = useMemo(() => {
    if (!econHealth) return 'Unavailable'
    if (econHealth.pending_settlements === 0) return 'Healthy'
    const backlogRatio = econHealth.pending_settlements / Math.max(econHealth.total_escrows, 1)
    if (backlogRatio > 0.6) return 'Critical backlog'
    if (backlogRatio > 0.3) return 'Elevated backlog'
    return 'Operational'
  }, [econHealth])

  return (
    <div className="relative">
      {/* Status Indicator */}
      <button
        onClick={() => setShowDetails(!showDetails)}
        className="flex items-center gap-1 p-1 rounded-full hover:bg-accentGold/10 transition-colors"
        title="System Status"
      >
        <div className={`w-2 h-2 rounded-full ${
          healthy === null ? 'bg-gray-400' :
          healthy ? 'bg-green-400 animate-pulse' : 'bg-red-400 animate-pulse'
        }`} />
        {alerts.length > 0 && (
          <span className="text-xs bg-red-500 text-white rounded-full w-4 h-4 flex items-center justify-center">
            {alerts.length}
          </span>
        )}
      </button>

      {/* Details Dropdown */}
      {showDetails && (
        <div className="absolute right-0 top-full mt-2 w-64 bg-primary border border-accentGold/20 rounded-lg shadow-lg z-50">
          <div className="p-3 border-b border-accentGold/20">
            <h4 className="text-sm font-semibold text-accentGold">System Status</h4>
          </div>
          <div className="p-3 space-y-2">
            {systemHealth ? (
              <>
                <div className="flex items-center justify-between text-xs">
                  <span>Model</span>
                  <Badge variant={badgeVariant(systemHealth.model?.status ?? 'Unknown')} size="sm">
                    {systemHealth.model ? getStatusLabel(systemHealth.model.status) : 'Unavailable'}
                  </Badge>
                </div>
                <div className="flex items-center justify-between text-xs">
                  <span>Agent</span>
                  <Badge variant={badgeVariant(agentStatus)} size="sm">
                    {agentLabel}
                  </Badge>
                </div>
                <div className="flex items-center justify-between text-xs">
                  <span>Coordinator</span>
                  <Badge variant={badgeVariant(systemHealth.coordinator?.status ?? 'Unknown')} size="sm">
                    {systemHealth.coordinator ? getStatusLabel(systemHealth.coordinator.status) : 'Unavailable'}
                  </Badge>
                </div>
                <div className="flex items-center justify-between text-xs">
                  <span>Economics</span>
                  <Badge variant={badgeVariant(econStatus)} size="sm">
                    {econLabel}
                  </Badge>
                </div>
              </>
            ) : (
              <div className="text-xs text-textOnDark/60">Loading...</div>
            )}
            
            {alerts.length > 0 && (
              <div className="mt-3 pt-2 border-t border-accentGold/20">
                <div className="text-xs font-medium text-red-300 mb-1">Alerts</div>
                {alerts.map((alert, idx) => (
                  <div key={idx} className="text-xs text-red-300">
                    {alert.message}
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>
      )}
      
      {/* Click outside to close */}
      {showDetails && (
        <div 
          className="fixed inset-0 z-40" 
          onClick={() => setShowDetails(false)}
        />
      )}
    </div>
  )
}

export default SystemStatus
