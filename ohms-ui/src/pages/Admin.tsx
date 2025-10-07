import { useEffect, useMemo, useState } from 'react'
import { useAgent } from '../context/AgentContext'
import { useAdmin } from '../hooks/useAdmin'
import Card from '../components/Card'
import Button from '../components/Button'
import LoadingSpinner from '../components/LoadingSpinner'
import Badge from '../components/Badge'
import { createEconActor, type ModelSummary } from '../services/canisterService'
import type { RoutingStats } from '../declarations/ohms_coordinator/ohms_coordinator.did'
import type { AuditEvent } from '../services/canisterService'
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

const formatNumber = (value: number | bigint | undefined, options?: Intl.NumberFormatOptions): string => {
  if (value === undefined) return '—'
  const numeric = typeof value === 'bigint' ? Number(value) : value
  if (!Number.isFinite(numeric)) return '—'
  return new Intl.NumberFormat('en-US', options).format(numeric)
}

const formatBytes = (value: bigint | number | undefined): string => {
  if (value === undefined) return '—'
  const numeric = typeof value === 'bigint' ? Number(value) : value
  if (!Number.isFinite(numeric) || numeric <= 0) return '0 B'
  const units = ['B', 'KB', 'MB', 'GB', 'TB']
  const exponent = Math.min(Math.floor(Math.log10(numeric) / 3), units.length - 1)
  const scaled = numeric / 10 ** (exponent * 3)
  return `${scaled.toFixed(1)} ${units[exponent]}`
}

const formatTimestamp = (value: bigint | number | undefined): string => {
  if (value === undefined) return '—'
  const numeric = typeof value === 'bigint' ? Number(value) : value
  if (!Number.isFinite(numeric) || numeric === 0) return '—'
  return new Date(numeric / 1_000_000).toLocaleString()
}

const Admin = () => {
  const { isWalletAvailable, createAuthAgent, checkAdminStatus, principal } = useAgent()
  const {
    isAdmin,
    refreshAdminData,
    systemHealth,
    modelStats,
    models,
    auditLog,
    agentHealth,
    agentLoader,
    routing,
    econHealth,
    econPolicy,
    econAdmins,
    agentStatus,
    econStatus,
    getStatusLabel,
  } = useAdmin()

  const [authChecked, setAuthChecked] = useState(false)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [canisterAdmin, setCanisterAdmin] = useState<boolean | null>(null)

  useEffect(() => {
    const verifyAdminAccess = async () => {
      if (!isWalletAvailable) {
        setAuthChecked(true)
        return
      }

      try {
        const agent = await createAuthAgent()
        await checkAdminStatus()

        if (agent) {
          const econ = createEconActor(agent)
          const res = await econ.is_admin()
          setCanisterAdmin(Boolean(res))
        } else {
          setCanisterAdmin(null)
        }
      } catch (err) {
        setError(prev => prev ?? (err instanceof Error ? err.message : 'Failed to verify admin access'))
        setCanisterAdmin(null)
      } finally {
        setAuthChecked(true)
      }
    }

    void verifyAdminAccess()
  }, [isWalletAvailable, createAuthAgent, checkAdminStatus])

  useEffect(() => {
    if (!authChecked || !isAdmin) {
      return
    }

    setLoading(true)
    setError(null)
    refreshAdminData()
      .catch(err => {
        setError(err instanceof Error ? err.message : 'Failed to load admin data')
      })
      .finally(() => setLoading(false))
  }, [authChecked, isAdmin, refreshAdminData])

  const refreshAll = async () => {
    setLoading(true)
    setError(null)
    try {
      await refreshAdminData()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load admin data')
    } finally {
      setLoading(false)
    }
  }

  const activateCanisterAdmin = async () => {
    try {
      const agent = await createAuthAgent()
      if (!agent || !principal) {
        throw new Error('Connect your identity before requesting admin access')
      }

      const econ = createEconActor(agent)
      const result = await econ.add_admin(principal)
      if (result && 'Err' in result) {
        throw new Error(result.Err)
      }

      const check = await econ.is_admin()
      setCanisterAdmin(Boolean(check))
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to grant admin permissions')
    }
  }

  if (!authChecked) {
    return (
      <div className="max-w-6xl mx-auto">
        <div className="flex items-center justify-center py-16">
          <LoadingSpinner size="lg" />
        </div>
      </div>
    )
  }

  if (!isWalletAvailable) {
    return (
      <div className="max-w-6xl mx-auto">
        <Card className="text-center py-12">
          <h1 className="text-3xl font-bold text-accentGold mb-4">Admin</h1>
          <p className="text-textOnDark/70">Internet Identity v2 authentication is required to access admin tooling.</p>
        </Card>
      </div>
    )
  }

  if (!isAdmin) {
    return (
      <div className="max-w-6xl mx-auto">
        <Card className="text-center py-12">
          <h1 className="text-3xl font-bold text-accentGold mb-4">Admin</h1>
          <p className="text-textOnDark/70 mb-4">You must be on the admin allowlist to view this dashboard.</p>
          {canisterAdmin === false && (
            <p className="text-sm text-textOnDark/60 mb-3">
              Your principal is recognised by the UI, but the economics canister does not list you as an admin.
              If you are a deployer, promote yourself below.
            </p>
          )}
          <div className="flex items-center justify-center gap-3">
            <Button onClick={refreshAll}>Reconnect &amp; Re-check</Button>
            {canisterAdmin === false && (
              <Button variant="outline" onClick={activateCanisterAdmin}>Grant Canister Admin</Button>
            )}
          </div>
        </Card>
      </div>
    )
  }

  const healthTiles = useMemo(
    () => [
      { name: 'Model', status: systemHealth?.model?.status ?? 'Unknown' },
      { name: 'Coordinator', status: systemHealth?.coordinator?.status ?? 'Unknown' },
      { name: 'Agent', status: agentStatus },
      { name: 'Economics', status: econStatus },
    ],
    [systemHealth?.model?.status, systemHealth?.coordinator?.status, agentStatus, econStatus],
  )

  const topRoutingStats = useMemo(() => routing?.stats.slice(0, 5) ?? [], [routing?.stats])
  const manifestedModels = useMemo(() => models.slice(0, 5), [models])
  const recentAuditEvents = useMemo(() => auditLog.slice(-8).reverse(), [auditLog])

  return (
    <div className="max-w-7xl mx-auto">
      <div className="flex items-center justify-between mb-8">
        <div>
          <h1 className="text-4xl font-bold text-accentGold mb-2">Admin Dashboards</h1>
          <p className="text-textOnDark/70">Operations • Finance • Catalog</p>
        </div>
        <Button variant="outline" onClick={refreshAll} loading={loading}>Refresh</Button>
      </div>

      {error && (
        <Card className="mb-6 border border-red-500/50">
          <p className="text-red-300">{error}</p>
        </Card>
      )}

      {loading ? (
        <div className="flex items-center justify-center py-16">
          <LoadingSpinner size="lg" />
        </div>
      ) : (
        <div className="space-y-8">
          <Card>
            <h3 className="text-lg font-semibold text-accentGold mb-4">Health Overview</h3>
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
              {healthTiles.map(tile => (
                <div key={tile.name} className="p-3 bg-primary/40 rounded border border-accentGold/20 flex items-center justify-between">
                  <span className="text-textOnDark/80">{tile.name}</span>
                  <Badge variant={badgeVariant(tile.status)} size="sm">{getStatusLabel(tile.status)}</Badge>
                </div>
              ))}
            </div>
          </Card>

          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <Card>
              <h3 className="text-lg font-semibold text-accentGold mb-4">Agent Operations</h3>
              <div className="grid grid-cols-2 gap-4 text-sm">
                <Stat label="Model Bound" value={agentHealth ? String(agentHealth.model_bound) : '—'} />
                <Stat label="Queue Depth" value={formatNumber(agentHealth?.queue_depth)} />
                <Stat label="Cache Hit Rate" value={`${(agentHealth?.cache_hit_rate ?? 0).toFixed(2)}%`} />
                <Stat label="Warm Set Utilization" value={`${(agentHealth?.warm_set_utilization ?? 0).toFixed(2)}%`} />
              </div>
              {agentLoader && (
                <div className="mt-4 grid grid-cols-2 gap-4 text-sm">
                  <Stat label="Chunks Loaded" value={`${agentLoader.chunksLoaded}/${agentLoader.totalChunks}`} />
                  <Stat label="Cache Entries" value={formatNumber(agentLoader.cacheEntries)} />
                </div>
              )}
            </Card>

            <Card>
              <h3 className="text-lg font-semibold text-accentGold mb-4">Routing &amp; Swarm</h3>
              {routing ? (
                <div className="space-y-4 text-sm">
                  <div className="grid grid-cols-2 gap-4">
                    <Stat label="Topology" value={Object.keys(routing.policy.topology)[0]} />
                    <Stat label="Mode" value={Object.keys(routing.policy.mode)[0]} />
                    <Stat label="Top K" value={String(routing.policy.top_k)} />
                    <Stat label="Window (ms)" value={formatNumber(Number(routing.policy.window_ms))} />
                  </div>
                  {topRoutingStats.length > 0 && (
                    <div>
                      <p className="text-textOnDark/70 mb-1">Top Agents</p>
                      <div className="space-y-1">
                        {topRoutingStats.map((stat: RoutingStats) => (
                          <div key={stat.agent_id} className="flex justify-between bg-primary/40 rounded px-2 py-1 border border-accentGold/20">
                            <span className="truncate mr-3">{stat.agent_id}</span>
                            <span>
                              {(stat.success_rate * 100).toFixed(1)}% • {stat.total_requests} req
                            </span>
                          </div>
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              ) : (
                <p className="text-textOnDark/60">Routing data unavailable.</p>
              )}
            </Card>
          </div>

          <Card>
            <h3 className="text-lg font-semibold text-accentGold mb-4">Economics</h3>
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
              <Stat label="Total Volume" value={`${formatNumber(econHealth?.total_volume, { notation: 'compact' })} cycles`} />
              <Stat label="Protocol Fees" value={`${formatNumber(econHealth?.protocol_fees_collected, { notation: 'compact' })} cycles`} />
              <Stat label="Active Escrows" value={formatNumber(econHealth?.active_escrows)} />
              <Stat label="Pending Settlements" value={formatNumber(econHealth?.pending_settlements)} />
            </div>
            {econPolicy && (
              <div className="mt-4 grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                <Stat label="Protocol Fee %" value={`${econPolicy.protocol_fee_percentage.toFixed(2)}%`} />
                <Stat label="Agent Fee %" value={`${econPolicy.agent_fee_percentage.toFixed(2)}%`} />
                <Stat label="Minimum Fee" value={formatNumber(econPolicy.minimum_fee)} />
                <Stat label="Registered Admins" value={String(econAdmins.length)} />
              </div>
            )}
          </Card>

          <Card>
            <h3 className="text-lg font-semibold text-accentGold mb-4">Model Catalog</h3>
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm mb-4">
              <Stat label="Total Models" value={formatNumber(modelStats.total)} />
              <Stat label="Active" value={formatNumber(modelStats.active)} />
              <Stat label="Pending" value={formatNumber(modelStats.pending)} />
              <Stat label="Deprecated" value={formatNumber(modelStats.deprecated)} />
            </div>
            <div className="overflow-x-auto">
              <table className="min-w-full text-sm">
                <thead>
                  <tr className="text-left text-textOnDark/60">
                    <th className="py-2 pr-4">Model</th>
                    <th className="py-2 pr-4">Version</th>
                    <th className="py-2 pr-4">Quantization</th>
                    <th className="py-2 pr-4">Size</th>
                    <th className="py-2">State</th>
                  </tr>
                </thead>
                <tbody>
                  {manifestedModels.map((summary: ModelSummary) => (
                    <tr key={`${summary.info.model_id}-${summary.info.version}`} className="border-t border-accentGold/10">
                      <td className="py-2 pr-4 font-mono text-xs">{summary.info.model_id}</td>
                      <td className="py-2 pr-4">{summary.info.version}</td>
                      <td className="py-2 pr-4">{typeof summary.info.quantization_format === 'string' ? summary.info.quantization_format : summary.info.quantization_format.Custom}</td>
                      <td className="py-2 pr-4">{formatBytes(summary.info.size_bytes)}</td>
                      <td className="py-2">
                        <Badge variant={badgeVariant(summary.info.state === 'Active' ? 'Healthy' : 'Degraded')} size="sm">
                          {summary.info.state}
                        </Badge>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>

            <div className="mt-6">
              <p className="text-textOnDark/70 mb-2">Recent Audit Events</p>
              {recentAuditEvents.length === 0 ? (
                <p className="text-sm text-textOnDark/60">No audit entries recorded.</p>
              ) : (
                <div className="space-y-2 max-h-56 overflow-y-auto">
                  {recentAuditEvents.map((event: AuditEvent, idx: number) => (
                    <div key={`${event.model_id}-${idx}`} className="text-sm bg-primary/40 border border-accentGold/20 rounded px-3 py-2">
                      <div className="flex justify-between">
                        <span className="font-mono text-xs">{event.model_id}</span>
                        <span className="text-textOnDark/60 text-xs">{formatTimestamp(event.timestamp)}</span>
                      </div>
                      <div className="flex justify-between mt-1">
                        <span className="text-textOnDark/80">{Object.keys(event.event_type)[0] ?? 'Event'}</span>
                        {event.details && (
                          <span className="text-textOnDark/60 truncate ml-3">{event.details}</span>
                        )}
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </Card>
        </div>
      )}
    </div>
  )
}

const Stat = ({ label, value }: { label: string; value: string }) => (
  <div className="p-3 bg-primary/40 rounded border border-accentGold/20">
    <p className="text-sm text-textOnDark/60">{label}</p>
    <p className="text-textOnDark font-semibold">{value}</p>
  </div>
)

export default Admin
