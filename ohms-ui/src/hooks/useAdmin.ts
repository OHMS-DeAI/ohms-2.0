import { useMemo } from 'react'
import { useAgent } from '../context/AgentContext'
import type { AgentHealth } from '../declarations/ohms_agent/ohms_agent.did'
import type { EconHealth } from '../declarations/ohms_econ/ohms_econ.did'
import type { ComponentHealth } from '@ohms/shared-types'

type Alert = { type: 'info' | 'warning' | 'error'; message: string }

const evaluateAgentStatus = (health?: AgentHealth): ComponentHealth => {
  if (!health) return 'Unknown'
  if (!health.model_bound) return 'Unhealthy'
  if (health.queue_depth > 20 || health.warm_set_utilization > 0.95) return 'Unhealthy'
  if (health.queue_depth > 10 || health.cache_hit_rate < 0.3) return 'Degraded'
  return 'Healthy'
}

const evaluateEconStatus = (health?: EconHealth): ComponentHealth => {
  if (!health) return 'Unknown'
  const backlogRatio = health.pending_settlements / Math.max(health.total_escrows, 1)
  if (backlogRatio > 0.6) return 'Unhealthy'
  if (backlogRatio > 0.3) return 'Degraded'
  return 'Healthy'
}

const getStatusLabel = (status: ComponentHealth): string => status

export const useAdmin = () => {
  const { isAdmin, adminData, refreshAdminData, checkAdminStatus } = useAgent()

  const systemHealth = adminData?.systemHealth ?? null
  const agentHealth = adminData?.agentHealth ?? null
  const econHealth = adminData?.econHealth ?? null

  const agentStatus = evaluateAgentStatus(agentHealth)
  const econStatus = evaluateEconStatus(econHealth)
  const modelStatus = systemHealth?.model?.status ?? 'Unknown'
  const coordinatorStatus = systemHealth?.coordinator?.status ?? 'Unknown'

  const isSystemHealthy = useMemo(() => {
    if (!systemHealth) return null
    return [modelStatus, agentStatus, coordinatorStatus, econStatus].every(status => status === 'Healthy')
  }, [systemHealth, modelStatus, agentStatus, coordinatorStatus, econStatus])

  const alerts = useMemo<Alert[]>(() => {
    if (!systemHealth) return []
    const notices: Alert[] = []

    if (!systemHealth.model || systemHealth.model.status === 'Unhealthy') {
      notices.push({ type: 'error', message: 'Model canister reporting unhealthy status' })
    } else if (systemHealth.model.status === 'Degraded') {
      notices.push({ type: 'warning', message: 'Model canister operating in degraded mode' })
    }

    if (!systemHealth.coordinator || systemHealth.coordinator.status === 'Unhealthy') {
      notices.push({ type: 'error', message: 'Coordinator canister unavailable' })
    } else if (systemHealth.coordinator.status === 'Degraded') {
      notices.push({ type: 'warning', message: 'Coordinator canister degraded' })
    }

    if (agentStatus === 'Unhealthy') {
      notices.push({ type: 'error', message: 'Agent canister unreachable or unbound' })
    } else if (agentStatus === 'Degraded') {
      notices.push({ type: 'warning', message: 'Agent canister experiencing backlog' })
    }

    if (econStatus === 'Unhealthy') {
      notices.push({ type: 'error', message: 'Economics canister settlement backlog critical' })
    } else if (econStatus === 'Degraded') {
      notices.push({ type: 'warning', message: 'Economics canister settlement backlog elevated' })
    }

    return notices
  }, [systemHealth, agentStatus, econStatus])

  return {
    isAdmin,
    adminData,
    refreshAdminData,
    checkAdminStatus,
    systemHealth,
    modelStats: adminData?.stats ?? { total: 0, active: 0, pending: 0, deprecated: 0 },
    agentHealth,
    agentStatus,
    econHealth,
    econStatus,
    routing: adminData?.routing ?? null,
    econPolicy: adminData?.econPolicy ?? null,
    econAdmins: adminData?.econAdmins ?? [],
    models: adminData?.summaries ?? [],
    auditLog: adminData?.auditLog ?? [],
    agentLoader: adminData?.agentLoader,
    isSystemHealthy,
    getStatusLabel,
    getSystemAlerts: () => alerts,
  }
}
