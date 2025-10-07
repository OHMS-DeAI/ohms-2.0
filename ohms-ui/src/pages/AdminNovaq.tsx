import { useEffect, useMemo, useState } from 'react'
import { useAgent } from '../context/AgentContext'
import { useAdmin } from '../hooks/useAdmin'
import Card from '../components/Card'
import Button from '../components/Button'
import LoadingSpinner from '../components/LoadingSpinner'
import Badge from '../components/Badge'
import { createModelActor, type ModelSummary } from '../services/canisterService'
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

const formatPercentage = (value: number | undefined): string => {
  if (value === undefined) return '—'
  return `${(value * 100).toFixed(1)}%`
}

const formatRatio = (value: number | undefined): string => {
  if (value === undefined) return '—'
  return value.toFixed(2)
}

const formatBytes = (value: bigint | undefined): string => {
  if (!value) return '—'
  const numeric = Number(value)
  if (!Number.isFinite(numeric) || numeric <= 0) return '0 B'
  const units = ['B', 'KB', 'MB', 'GB', 'TB']
  const exponent = Math.min(Math.floor(Math.log10(numeric) / 3), units.length - 1)
  const scaled = numeric / 10 ** (exponent * 3)
  return `${scaled.toFixed(1)} ${units[exponent]}`
}

const AdminNovaq = () => {
  const { isWalletAvailable, createAuthAgent, checkAdminStatus } = useAgent()
  const {
    isAdmin,
    refreshAdminData,
    models,
    agentStatus,
    econStatus,
    systemHealth,
    getStatusLabel,
  } = useAdmin()

  const [authChecked, setAuthChecked] = useState(false)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    const verifyAdminAccess = async () => {
      if (!isWalletAvailable) {
        setAuthChecked(true)
        return
      }

      try {
        await createAuthAgent()
        await checkAdminStatus()
      } catch (err) {
        setError(prev => prev ?? (err instanceof Error ? err.message : 'Failed to verify admin access'))
      } finally {
        setAuthChecked(true)
      }
    }

    void verifyAdminAccess()
  }, [isWalletAvailable, createAuthAgent, checkAdminStatus])

  useEffect(() => {
    if (!authChecked || !isAdmin) return
    setLoading(true)
    setError(null)
    refreshAdminData()
      .catch(err => setError(err instanceof Error ? err.message : 'Failed to load NOVAQ data'))
      .finally(() => setLoading(false))
  }, [authChecked, isAdmin, refreshAdminData])

  const novaqModels = useMemo(
    () =>
      models.filter(summary => {
        const format = summary.info.quantization_format
        return format === 'NOVAQ'
      }),
    [models],
  )

  const pendingModels = useMemo(
    () => novaqModels.filter(summary => summary.info.state === 'Pending'),
    [novaqModels],
  )

  const activeModels = useMemo(
    () => novaqModels.filter(summary => summary.info.state === 'Active'),
    [novaqModels],
  )

  const deprecatedModels = useMemo(
    () => novaqModels.filter(summary => summary.info.state === 'Deprecated'),
    [novaqModels],
  )

  const performModelAction = async (
    modelId: string,
    action: (actor: ReturnType<typeof createModelActor>) => Promise<{ Err?: { [key: string]: string } } | void>,
  ) => {
    setLoading(true)
    setError(null)
    try {
      const agent = await createAuthAgent()
      if (!agent) {
        throw new Error('Authentication required. Please connect your wallet.')
      }

      const actor = createModelActor(agent)
      const result = await action(actor)
      if (result && 'Err' in result && result.Err) {
        const [[variant, message]] = Object.entries(result.Err)
        throw new Error(`${variant}: ${message}`)
      }

      await refreshAdminData()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Model operation failed')
    } finally {
      setLoading(false)
    }
  }

  const approveModel = (modelId: string) =>
    performModelAction(modelId, actor => actor.deploy_model(modelId))

  const rejectModel = (modelId: string) =>
    performModelAction(modelId, actor => actor.delete_model(modelId))

  const handleRefresh = async () => {
    setLoading(true)
    setError(null)
    try {
      await refreshAdminData()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to refresh NOVAQ data')
    } finally {
      setLoading(false)
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
          <h1 className="text-3xl font-bold text-accentGold mb-4">NOVAQ Admin</h1>
          <p className="text-textOnDark/70">Connect your wallet to review NOVAQ manifests.</p>
        </Card>
      </div>
    )
  }

  if (!isAdmin) {
    return (
      <div className="max-w-6xl mx-auto">
        <Card className="text-center py-12">
          <h1 className="text-3xl font-bold text-accentGold mb-4">NOVAQ Admin</h1>
          <p className="text-textOnDark/70">You must be an admin to review NOVAQ manifests.</p>
        </Card>
      </div>
    )
  }

  return (
    <div className="max-w-6xl mx-auto space-y-8">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-4xl font-bold text-accentGold mb-2">NOVAQ Manifests</h1>
          <p className="text-textOnDark/70">Review compression metadata and approve manifests for deployment.</p>
        </div>
        <Button variant="outline" onClick={handleRefresh} loading={loading}>Refresh</Button>
      </div>

      {error && (
        <Card className="border border-red-500/40">
          <p className="text-red-300">{error}</p>
        </Card>
      )}

      {loading ? (
        <div className="flex items-center justify-center py-16">
          <LoadingSpinner size="lg" />
        </div>
      ) : (
        <>
          <Card>
            <h3 className="text-lg font-semibold text-accentGold mb-4">System Signals</h3>
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
              <SignalTile label="Catalog" status={systemHealth?.model?.status ?? 'Unknown'} getStatusLabel={getStatusLabel} />
              <SignalTile label="Coordinator" status={systemHealth?.coordinator?.status ?? 'Unknown'} getStatusLabel={getStatusLabel} />
              <SignalTile label="Agent" status={agentStatus} getStatusLabel={getStatusLabel} />
              <SignalTile label="Economics" status={econStatus} getStatusLabel={getStatusLabel} />
            </div>
          </Card>

          <Card>
            <h3 className="text-lg font-semibold text-accentGold mb-4">Pending Reviews</h3>
            {pendingModels.length === 0 ? (
              <p className="text-sm text-textOnDark/60">No NOVAQ manifests awaiting review.</p>
            ) : (
              <NovaqTable models={pendingModels} onApprove={approveModel} onReject={rejectModel} />
            )}
          </Card>

          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <Card>
              <h3 className="text-lg font-semibold text-accentGold mb-4">Active NOVAQ Models</h3>
              {activeModels.length === 0 ? (
                <p className="text-sm text-textOnDark/60">No active NOVAQ manifests.</p>
              ) : (
                <NovaqTable models={activeModels} onReject={rejectModel} />
              )}
            </Card>

            <Card>
              <h3 className="text-lg font-semibold text-accentGold mb-4">Deprecated NOVAQ Models</h3>
              {deprecatedModels.length === 0 ? (
                <p className="text-sm text-textOnDark/60">No deprecated NOVAQ manifests.</p>
              ) : (
                <NovaqTable models={deprecatedModels} />
              )}
            </Card>
          </div>
        </>
      )}
    </div>
  )
}

const SignalTile = ({
  label,
  status,
  getStatusLabel,
}: {
  label: string
  status: ComponentHealth
  getStatusLabel: (status: ComponentHealth) => string
}) => (
  <div className="p-3 bg-primary/40 rounded border border-accentGold/20 flex items-center justify-between">
    <span className="text-textOnDark/80">{label}</span>
    <Badge variant={badgeVariant(status)} size="sm">{getStatusLabel(status)}</Badge>
  </div>
)

const NovaqTable = ({
  models,
  onApprove,
  onReject,
}: {
  models: ModelSummary[]
  onApprove?: (modelId: string) => void
  onReject?: (modelId: string) => void
}) => (
  <div className="overflow-x-auto">
    <table className="min-w-full text-sm">
      <thead>
        <tr className="text-left text-textOnDark/60">
          <th className="py-2 pr-4">Model</th>
          <th className="py-2 pr-4">Version</th>
          <th className="py-2 pr-4">Compression</th>
          <th className="py-2 pr-4">Capability</th>
          <th className="py-2 pr-4">Size</th>
          <th className="py-2 pr-4">Chunks</th>
          {(onApprove || onReject) && <th className="py-2">Actions</th>}
        </tr>
      </thead>
      <tbody>
        {models.map(summary => {
          const manifest = summary.manifest
          const quantization = manifest?.quantization
          return (
            <tr key={`${summary.info.model_id}-${summary.info.version}`} className="border-t border-accentGold/10">
              <td className="py-2 pr-4 font-mono text-xs">{summary.info.model_id}</td>
              <td className="py-2 pr-4">{summary.info.version}</td>
              <td className="py-2 pr-4">{formatRatio(quantization?.compression_ratio)}</td>
              <td className="py-2 pr-4">{formatPercentage(quantization?.accuracy_retention)}</td>
              <td className="py-2 pr-4">{formatBytes(summary.info.size_bytes)}</td>
              <td className="py-2 pr-4">{manifest?.chunk_count ?? '—'}</td>
              {(onApprove || onReject) && (
                <td className="py-2 flex gap-2">
                  {onApprove && (
                    <Button size="xs" onClick={() => onApprove(summary.info.model_id)}>Approve</Button>
                  )}
                  {onReject && (
                    <Button size="xs" variant="outline" onClick={() => onReject(summary.info.model_id)}>Reject</Button>
                  )}
                </td>
              )}
            </tr>
          )
        })}
      </tbody>
    </table>
  </div>
)

export default AdminNovaq
