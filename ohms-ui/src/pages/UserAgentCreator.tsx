import { useEffect, useMemo, useState } from 'react'
import { useAgent } from '../context/AgentContext'
import Card from '../components/Card'
import Button from '../components/Button'
import LoadingSpinner from '../components/LoadingSpinner'
import Badge from '../components/Badge'
import Input from '../components/Input'
import Textarea from '../components/Textarea'
import {
  createAgentsFromInstructionsTyped,
  createSubscription,
  getUserQuotaStatus,
  getUserSubscription,
  listModels,
  listUserAgents,
  type AgentSummaryView,
  type CandidAgentCreationResult,
} from '../services/canisterService'
import type { UserSubscription as EconUserSubscription, QuotaValidation } from '../declarations/ohms_econ/ohms_econ.did'

interface AgentCreationForm {
  instruction: string
  agentCount: number
  capabilities: string[]
  priority: 'low' | 'normal' | 'high' | 'critical'
}

const capabilityOptions = [
  'Code Assistant',
  'Data Analysis',
  'Content Creation',
  'Research',
  'Planning',
  'Problem Solving',
  'Translation',
  'Summarization',
  'Question Answering',
  'Creative Writing',
]

const priorityOptions: Array<{ value: AgentCreationForm['priority']; label: string }> = [
  { value: 'low', label: 'Low (background)' },
  { value: 'normal', label: 'Normal' },
  { value: 'high', label: 'High (priority)' },
  { value: 'critical', label: 'Critical (immediate)' },
]

const formatDate = (value: bigint): string => {
  if (!value) return '—'
  const millis = Number(value) / 1_000_000
  if (!Number.isFinite(millis)) return '—'
  return new Date(millis).toLocaleString()
}

const UserAgentCreator = () => {
  const { isWalletAvailable, createAuthAgent, principal } = useAgent()
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [success, setSuccess] = useState<string | null>(null)

  const [form, setForm] = useState<AgentCreationForm>({
    instruction: '',
    agentCount: 1,
    capabilities: [],
    priority: 'normal',
  })

  const [agents, setAgents] = useState<AgentSummaryView[]>([])
  const [subscription, setSubscription] = useState<EconUserSubscription | null>(null)
  const [quota, setQuota] = useState<QuotaValidation | null>(null)
  const [availableModels, setAvailableModels] = useState<string[]>([])

  const loadUserData = async () => {
    setLoading(true)
    setError(null)

    try {
      const agent = await createAuthAgent()
      if (!agent) {
        throw new Error('Authentication required. Please connect your wallet.')
      }

      const [subscriptionResult, quotaResult, agentSummaries, modelSummaries] = await Promise.all([
        getUserSubscription(principal ?? undefined, agent),
        getUserQuotaStatus(agent),
        listUserAgents(principal ?? undefined, agent),
        listModels(agent),
      ])

      if (subscriptionResult) {
        setSubscription(subscriptionResult)
      } else {
        const created = await createSubscription('pro', false, agent)
        setSubscription(created)
      }

      setQuota(quotaResult ?? null)
      setAgents(agentSummaries)
      setAvailableModels(modelSummaries.map(summary => summary.info.model_id))
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load agent data')
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    if (isWalletAvailable && principal) {
      void loadUserData()
    }
  }, [isWalletAvailable, principal])

  const quotaStatus = useMemo(() => {
    if (!quota) return 'Unknown'
    return quota.allowed ? 'Within quota' : quota.reason?.[0] ?? 'Quota exceeded'
  }, [quota])

  const quotaAllowsCreation = quota?.allowed ?? true

  const handleCapabilityToggle = (capability: string) => {
    setForm(prev => {
      const exists = prev.capabilities.includes(capability)
      return {
        ...prev,
        capabilities: exists
          ? prev.capabilities.filter(item => item !== capability)
          : [...prev.capabilities, capability],
      }
    })
  }

  const handleCreateAgent = async () => {
    if (!form.instruction.trim()) {
      setError('Please provide instructions for your agent')
      return
    }

    if (!quotaAllowsCreation) {
      setError('Quota validation failed. Please review your subscription limits.')
      return
    }

    setLoading(true)
    setError(null)
    setSuccess(null)

    try {
      const agent = await createAuthAgent()
      if (!agent) {
        throw new Error('Authentication required')
      }

      const creation: CandidAgentCreationResult = await createAgentsFromInstructionsTyped(
        {
          instruction: form.instruction.trim(),
          agentCount: form.agentCount,
          capabilities: form.capabilities,
          priority: form.priority,
        },
        agent,
      )

      setSuccess(`Agent ${creation.agent_id} created successfully`)
      setForm({ instruction: '', agentCount: 1, capabilities: [], priority: 'normal' })

      // Refresh data to capture new agent and quota updates
      await loadUserData()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create agent')
    } finally {
      setLoading(false)
    }
  }

  if (!isWalletAvailable) {
    return (
      <div className="max-w-6xl mx-auto">
        <Card className="text-center py-12">
          <h1 className="text-3xl font-bold text-accentGold mb-4">Agent Creator</h1>
          <p className="text-textOnDark/70">Connect your wallet to create autonomous agents.</p>
        </Card>
      </div>
    )
  }

  return (
    <div className="max-w-6xl mx-auto space-y-8">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-4xl font-bold text-accentGold mb-2">Design Your Agent</h1>
          <p className="text-textOnDark/70">Define capabilities, concurrency, and routing preferences.</p>
        </div>
        <Button variant="outline" onClick={() => loadUserData()} loading={loading}>Refresh</Button>
      </div>

      {error && (
        <Card className="border border-red-500/40">
          <p className="text-red-300">{error}</p>
        </Card>
      )}

      {success && (
        <Card className="border border-green-500/40">
          <p className="text-green-300">{success}</p>
        </Card>
      )}

      <Card>
        <h3 className="text-lg font-semibold text-accentGold mb-4">Quota &amp; Subscription</h3>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
          <Stat label="Tier" value={subscription?.tier?.name ?? '—'} />
          <Stat label="Agents This Month" value={String(subscription?.current_usage.agents_created_this_month ?? 0)} />
          <Stat label="Monthly Creations" value={String(subscription?.tier?.monthly_agent_creations ?? 0)} />
          <Stat label="Token Limit" value={String(subscription?.tier?.token_limit ?? 0n)} />
        </div>
        <div className="mt-4 flex items-center justify-between">
          <span className="text-sm text-textOnDark/70">Quota status: {quotaStatus}</span>
          <Badge variant={quotaAllowsCreation ? 'success' : 'warning'}>{quotaAllowsCreation ? 'Eligible' : 'Blocked'}</Badge>
        </div>
      </Card>

      <Card>
        <h3 className="text-lg font-semibold text-accentGold mb-4">Agent Blueprint</h3>
        <div className="space-y-4">
          <Textarea
            label="Instruction"
            placeholder="Describe the agent's autonomous mission"
            value={form.instruction}
            onChange={event => setForm(prev => ({ ...prev, instruction: event.target.value }))}
            rows={5}
          />

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <Input
              label="Number of collaborating agents"
              type="number"
              min={1}
              max={10}
              value={form.agentCount}
              onChange={event => setForm(prev => ({ ...prev, agentCount: Number(event.target.value) }))}
            />

            <div>
              <label className="block text-sm font-medium text-textOnDark mb-2">Priority</label>
              <select
                className="w-full bg-primary/40 border border-accentGold/20 rounded px-3 py-2"
                value={form.priority}
                onChange={event => setForm(prev => ({ ...prev, priority: event.target.value as AgentCreationForm['priority'] }))}
              >
                {priorityOptions.map(option => (
                  <option key={option.value} value={option.value}>{option.label}</option>
                ))}
              </select>
            </div>
          </div>

          <div>
            <p className="text-sm text-textOnDark/70 mb-2">Capabilities</p>
            <div className="flex flex-wrap gap-2">
              {capabilityOptions.map(option => {
                const active = form.capabilities.includes(option)
                return (
                  <button
                    key={option}
                    type="button"
                    onClick={() => handleCapabilityToggle(option)}
                    className={`px-3 py-1 rounded-full text-sm border transition ${
                      active
                        ? 'bg-accentGold/20 text-accentGold border-accentGold/40'
                        : 'bg-primary/40 text-textOnDark border-accentGold/10'
                    }`}
                  >
                    {option}
                  </button>
                )
              })}
            </div>
          </div>

          <div>
            <p className="text-sm text-textOnDark/70 mb-2">Available Models</p>
            <div className="flex flex-wrap gap-2 text-sm">
              {availableModels.length === 0 ? (
                <span className="text-textOnDark/60">No models available</span>
              ) : (
                availableModels.map(model => (
                  <Badge key={model} variant="info">{model}</Badge>
                ))
              )}
            </div>
          </div>

          <div className="flex justify-end">
            <Button onClick={handleCreateAgent} disabled={!quotaAllowsCreation || loading} loading={loading}>
              Create Agent
            </Button>
          </div>
        </div>
      </Card>

      <Card>
        <h3 className="text-lg font-semibold text-accentGold mb-4">Your Agents</h3>
        {agents.length === 0 ? (
          <p className="text-sm text-textOnDark/60">No agents created yet.</p>
        ) : (
          <div className="space-y-3">
            {agents.map(agentSummary => (
              <div key={agentSummary.agentId} className="p-3 bg-primary/40 rounded border border-accentGold/20 flex justify-between items-center">
                <div>
                  <p className="font-mono text-xs text-textOnDark/80">{agentSummary.agentId}</p>
                  <p className="text-sm text-textOnDark/60">
                    Last active: {formatDate(agentSummary.lastActive)}
                  </p>
                </div>
                <div className="flex items-center gap-2">
                  <Badge variant="info">{typeof agentSummary.agentType === 'string' ? agentSummary.agentType : agentSummary.agentType.Custom}</Badge>
                  <Badge variant="success">{typeof agentSummary.status === 'string' ? agentSummary.status : 'Error' in agentSummary.status ? agentSummary.status.Error : 'Unknown'}</Badge>
                </div>
              </div>
            ))}
          </div>
        )}
      </Card>
    </div>
  )
}

const Stat = ({ label, value }: { label: string; value: string }) => (
  <div className="p-3 bg-primary/40 rounded border border-accentGold/20">
    <p className="text-sm text-textOnDark/60">{label}</p>
    <p className="text-textOnDark font-semibold">{value}</p>
  </div>
)

export default UserAgentCreator
