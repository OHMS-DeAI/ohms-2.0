import { useAgent } from '../context/AgentContext'

export const useAdmin = () => {
  const { isAdmin, adminData, refreshAdminData, checkAdminStatus } = useAgent()
  
  return {
    isAdmin,
    adminData,
    refreshAdminData,
    checkAdminStatus,
    
    // Convenience getters for specific admin data
    systemHealth: adminData?.health || null,
    modelStats: adminData?.modelStats || { total: 0, active: 0, pending: 0, deprecated: 0 },
    agentHealth: adminData?.agentHealth || null,
    routingHealth: adminData?.routingHealth || null,
    econHealth: adminData?.econHealth || null,
    
    // System status helpers
    isSystemHealthy: () => {
      if (!adminData?.health) return null
      const { model, agent, coordinator, econ } = adminData.health

      const coordinatorStatus = coordinator?.status || 'Unknown'
      const econStatus = econ?.status || 'Unknown'
      const modelStatus = model?.status || 'Unknown'
      const agentStatus = agent ? (agent.queue_depth > 5 ? 'Degraded' : 'Healthy') : 'Unknown'

      return [modelStatus, agentStatus, coordinatorStatus, econStatus].every(status => status === 'Healthy')
    },
    
    getSystemAlerts: (): Array<{type: string, message: string}> => {
      const alerts: Array<{type: string, message: string}> = []
      if (!adminData?.health) return alerts
      
      const { health } = adminData

      if (!health.model || health.model.status === 'Unhealthy') {
        alerts.push({ type: 'error', message: 'Model canister reporting unhealthy status' })
      } else if (health.model.status === 'Degraded') {
        alerts.push({ type: 'warning', message: 'Model canister operating in degraded mode' })
      }

      if (!health.coordinator || health.coordinator.status === 'Unhealthy') {
        alerts.push({ type: 'error', message: 'Coordinator canister unavailable' })
      } else if (health.coordinator.status === 'Degraded') {
        alerts.push({ type: 'warning', message: 'Coordinator canister degraded' })
      }

      if (!health.econ || health.econ.status === 'Unhealthy') {
        alerts.push({ type: 'error', message: 'Economics canister unavailable' })
      } else if (health.econ.status === 'Degraded') {
        alerts.push({ type: 'warning', message: 'Economics canister degraded' })
      }

      if (!health.agent || (health.agent.queue_depth ?? 0) > 10) {
        alerts.push({ type: 'warning', message: 'Agent canister backlog detected' })
      }
      
      return alerts
    }
  }
}
