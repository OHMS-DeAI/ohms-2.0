import { Actor, type ActorSubclass, HttpAgent } from '@dfinity/agent'
import type { Principal } from '@dfinity/principal'
import { HOST as RESOLVED_HOST, NETWORK, getCanisterIdsFromEnv } from '../config/network'
import { idlFactory as agentIdlFactory } from '../declarations/ohms_agent'
import { idlFactory as modelIdlFactory } from '../declarations/ohms_model'
import { idlFactory as coordinatorIdlFactory } from '../declarations/ohms_coordinator'
import { idlFactory as econIdlFactory } from '../declarations/ohms_econ'
import type {
  AgentCreationRequest as CandidAgentCreationRequest,
  AgentCreationResult as CandidAgentCreationResult,
  AgentHealth,
  AgentSummary as CandidAgentSummary,
  AgentStatus,
  AgentType as CandidAgentType,
  InferenceRequest as CandidInferenceRequest,
  InferenceResponse as CandidInferenceResponse,
  Result_Empty,
  Result_AgentCreation,
  Result_Inference,
  Result_Summaries,
  _SERVICE as AgentService,
} from '../declarations/ohms_agent/ohms_agent.did'
import type {
  AuditEvent,
  ModelInfo as CandidModelInfo,
  ModelManifest as CandidModelManifest,
  ModelState as CandidModelState,
  QuantizationFormat as CandidQuantizationFormat,
  QuantizedArtifactMetadata as CandidQuantizedMetadata,
  SystemHealth as CandidModelHealth,
  _SERVICE as ModelService,
} from '../declarations/ohms_model/ohms_model.did'
import type {
  AgentRegistration,
  RouteRequest,
  RouteResponse,
  RoutingStats,
  SwarmPolicy,
  SystemHealth as CandidCoordinatorHealth,
  _SERVICE as CoordinatorService,
} from '../declarations/ohms_coordinator/ohms_coordinator.did'
import type {
  EconHealth,
  FeePolicy,
  PaymentRequest,
  PaymentStats,
  QuotaValidation as CandidQuotaValidation,
  UserSubscription,
  _SERVICE as EconService,
} from '../declarations/ohms_econ/ohms_econ.did'
import type {
  AgentStatus as SharedAgentStatus,
  AgentType as SharedAgentType,
  ModelInfo as SharedModelInfo,
  ModelManifest as SharedModelManifest,
  QuantizationFormat as SharedQuantizationFormat,
  SystemHealth as SharedSystemHealth,
} from '@ohms/shared-types'

export type { CandidAgentCreationRequest, CandidAgentCreationResult, CandidInferenceRequest, CandidInferenceResponse }

export const host = RESOLVED_HOST

export const agent = new HttpAgent({ host })
if (NETWORK !== 'ic') {
  agent.fetchRootKey().catch(() => {
    /* Swallow errors for local development */
  })
}

const CANISTER_IDS = getCanisterIdsFromEnv()

type Optional<T> = T | undefined

type ActorOptions = {
  agent?: HttpAgent
  canisterId?: string
}

const optionToValue = <T>(input: [] | [T]): Optional<T> => (input.length === 0 ? undefined : input[0])

const tupleVecToRecord = (entries: Array<[string, string]>): Record<string, string> => {
  const result: Record<string, string> = {}
  for (const [key, value] of entries) {
    result[key] = value
  }
  return result
}

const buildActorFactory = <T>(idlFactory: Parameters<typeof Actor.createActor>[0], defaultCanisterId: string) => {
  const defaultActor = Actor.createActor<T>(idlFactory, { agent, canisterId: defaultCanisterId })
  return (options?: ActorOptions): ActorSubclass<T> => {
    if (!options || (!options.agent && !options.canisterId)) {
      return defaultActor
    }
    return Actor.createActor<T>(idlFactory, {
      agent: options.agent ?? agent,
      canisterId: options.canisterId ?? defaultCanisterId,
    })
  }
}

const agentActorFactory = buildActorFactory<AgentService>(agentIdlFactory, CANISTER_IDS.ohms_agent)
const modelActorFactory = buildActorFactory<ModelService>(modelIdlFactory, CANISTER_IDS.ohms_model)
const coordinatorActorFactory = buildActorFactory<CoordinatorService>(coordinatorIdlFactory, CANISTER_IDS.ohms_coordinator)
const econActorFactory = buildActorFactory<EconService>(econIdlFactory, CANISTER_IDS.ohms_econ)

export const agentCanister = agentActorFactory()
export const modelCanister = modelActorFactory()
export const coordinatorCanister = coordinatorActorFactory()
export const econCanister = econActorFactory()

const parseActorArguments = (
  defaultCanisterId: string,
  param1?: HttpAgent | string,
  param2?: HttpAgent | string,
): ActorOptions => {
  let resolvedAgent: Optional<HttpAgent>
  let resolvedCanister: Optional<string>

  if (param1) {
    if (typeof param1 === 'string') {
      resolvedCanister = param1
    } else {
      resolvedAgent = param1
    }
  }

  if (param2) {
    if (typeof param2 === 'string') {
      resolvedCanister = param2
    } else {
      resolvedAgent = param2
    }
  }

  return {
    agent: resolvedAgent,
    canisterId: resolvedCanister ?? defaultCanisterId,
  }
}

export const createAgentActor = (
  param1?: HttpAgent | string,
  param2?: HttpAgent | string,
): ActorSubclass<AgentService> => {
  const options = parseActorArguments(CANISTER_IDS.ohms_agent, param1, param2)
  return agentActorFactory(options)
}

export const createModelActor = (
  param1?: HttpAgent | string,
  param2?: HttpAgent | string,
): ActorSubclass<ModelService> => {
  const options = parseActorArguments(CANISTER_IDS.ohms_model, param1, param2)
  return modelActorFactory(options)
}

export const createCoordinatorActor = (
  param1?: HttpAgent | string,
  param2?: HttpAgent | string,
): ActorSubclass<CoordinatorService> => {
  const options = parseActorArguments(CANISTER_IDS.ohms_coordinator, param1, param2)
  return coordinatorActorFactory(options)
}

export const createEconActor = (
  param1?: HttpAgent | string,
  param2?: HttpAgent | string,
): ActorSubclass<EconService> => {
  const options = parseActorArguments(CANISTER_IDS.ohms_econ, param1, param2)
  return econActorFactory(options)
}

const componentHealthToString = (health: CandidModelHealth['status']): SharedSystemHealth['status'] => {
  if ('Healthy' in health) return 'Healthy'
  if ('Degraded' in health) return 'Degraded'
  if ('Unhealthy' in health) return 'Unhealthy'
  return 'Unknown'
}

const coordinatorHealthToString = (health: CandidCoordinatorHealth['status']): SharedSystemHealth['status'] => {
  if ('Healthy' in health) return 'Healthy'
  if ('Degraded' in health) return 'Degraded'
  if ('Unhealthy' in health) return 'Unhealthy'
  return 'Unknown'
}

const quantizationFormatToShared = (format: CandidQuantizationFormat): SharedQuantizationFormat => {
  if ('NOVAQ' in format) return 'NOVAQ'
  if ('GGUF' in format) return 'GGUF'
  return { Custom: format.Custom }
}

const modelStateToShared = (state: CandidModelState): SharedModelInfo['state'] => {
  if ('Active' in state) return 'Active'
  if ('Deprecated' in state) return 'Deprecated'
  return 'Pending'
}

const agentTypeToShared = (agentType: CandidAgentType): SharedAgentType => {
  if ('Custom' in agentType) {
    return { Custom: agentType.Custom }
  }
  const [key] = Object.keys(agentType) as Array<keyof typeof agentType>
  return key as SharedAgentType
}

const agentStatusToShared = (status: AgentStatus): SharedAgentStatus => {
  if ('Error' in status) {
    return { Error: status.Error }
  }
  const [key] = Object.keys(status) as Array<keyof typeof status>
  return key as SharedAgentStatus
}

const quantizedMetadataToShared = (metadata: CandidQuantizedMetadata): SharedModelManifest['quantization'] => ({
  format: quantizationFormatToShared(metadata.format),
  artifact_checksum: metadata.artifact_checksum,
  compression_ratio: metadata.compression_ratio,
  accuracy_retention: metadata.accuracy_retention,
  bits_per_weight: optionToValue(metadata.bits_per_weight),
  notes: optionToValue(metadata.notes),
})

const principalToText = (principal: Principal): string => principal.toText()

const modelManifestToShared = (manifest: CandidModelManifest): SharedModelManifest => ({
  model_id: manifest.model_id,
  version: manifest.version,
  state: modelStateToShared(manifest.state),
  uploaded_at: manifest.uploaded_at,
  activated_at: optionToValue(manifest.activated_at),
  total_size_bytes: manifest.total_size_bytes,
  chunk_count: manifest.chunk_count,
  checksum: manifest.checksum,
  quantization: quantizedMetadataToShared(manifest.quantization),
  metadata: tupleVecToRecord(manifest.metadata),
  chunks: manifest.chunks.map(chunk => ({
    chunk_id: chunk.chunk_id,
    offset: chunk.offset,
    size_bytes: chunk.size_bytes,
    sha256: chunk.sha256,
  })),
})

const modelInfoToShared = (info: CandidModelInfo): SharedModelInfo => ({
  model_id: info.model_id,
  version: info.version,
  state: modelStateToShared(info.state),
  quantization_format: quantizationFormatToShared(info.quantization_format),
  compression_ratio: optionToValue(info.compression_ratio),
  accuracy_retention: optionToValue(info.accuracy_retention),
  size_bytes: info.size_bytes,
  uploaded_at: info.uploaded_at,
  activated_at: optionToValue(info.activated_at),
})

const modelHealthToShared = (health: CandidModelHealth): SharedSystemHealth => ({
  canister_id: principalToText(health.canister_id),
  status: componentHealthToString(health.status),
  uptime_seconds: health.uptime_seconds,
  memory_usage_mb: health.memory_usage_mb,
  last_update: health.last_update,
  version: health.version,
  metrics: tupleVecToRecord(health.metrics),
})

const coordinatorHealthToShared = (health: CandidCoordinatorHealth): SharedSystemHealth => ({
  canister_id: principalToText(health.canister_id),
  status: coordinatorHealthToString(health.status),
  uptime_seconds: health.uptime_seconds,
  memory_usage_mb: health.memory_usage_mb,
  last_update: health.last_update,
  version: health.version,
  metrics: tupleVecToRecord(health.metrics),
})

export interface ModelSummary {
  info: SharedModelInfo
  manifest?: SharedModelManifest
}

export interface AgentSummaryView {
  agentId: string
  agentType: SharedAgentType
  status: SharedAgentStatus
  createdAt: bigint
  lastActive: bigint
}

export interface AgentLoaderStats {
  modelBound: boolean
  chunksLoaded: number
  totalChunks: number
  cacheUtilization: number
  cacheEntries: number
}

export interface SystemHealthSnapshot {
  model: Optional<SharedSystemHealth>
  coordinator: Optional<SharedSystemHealth>
  agent: Optional<AgentHealth>
  econ: Optional<EconHealth>
}

export interface AuditEvent {
  model_id: string
  event_type: Record<string, unknown>
  timestamp: bigint
  details?: string
}

export interface ModelStats {
  total: number
  active: number
  pending: number
  deprecated: number
}

const parseLoaderStats = (raw: string): AgentLoaderStats | undefined => {
  try {
    const parsed = JSON.parse(raw) as Record<string, unknown>
    return {
      modelBound: Boolean(parsed.model_bound),
      chunksLoaded: Number(parsed.chunks_loaded ?? 0),
      totalChunks: Number(parsed.total_chunks ?? 0),
      cacheUtilization: Number(parsed.cache_utilization ?? 0),
      cacheEntries: Number(parsed.cache_entries ?? 0),
    }
  } catch {
    return undefined
  }
}

export const listModels = async (agentOverride?: HttpAgent): Promise<ModelSummary[]> => {
  const actor = agentOverride ? createModelActor(agentOverride) : modelCanister
  const infos = await actor.list_models([])
  const manifests = await Promise.all(
    infos.map(async info => {
      const manifestOpt = await actor.get_manifest(info.model_id)
      const manifest = optionToValue(manifestOpt)
      return manifest ? modelManifestToShared(manifest) : undefined
    }),
  )

  return infos.map((info, index) => ({
    info: modelInfoToShared(info),
    manifest: manifests[index],
  }))
}

export const getModelManifest = async (
  modelId: string,
  agentOverride?: HttpAgent,
): Promise<Optional<SharedModelManifest>> => {
  const actor = agentOverride ? createModelActor(agentOverride) : modelCanister
  const manifestOpt = await actor.get_manifest(modelId)
  const manifest = optionToValue(manifestOpt)
  return manifest ? modelManifestToShared(manifest) : undefined
}

const readModelAuditLog = async (actor: ActorSubclass<ModelService>): Promise<AuditEvent[]> => {
  const candidate = actor as unknown as { get_audit_log?: () => Promise<AuditEvent[]> }
  if (typeof candidate.get_audit_log === 'function') {
    return candidate.get_audit_log()
  }
  return []
}

export const getModelAuditLog = async (agentOverride?: HttpAgent): Promise<AuditEvent[]> => {
  const actor = agentOverride ? createModelActor(agentOverride) : modelCanister
  return readModelAuditLog(actor)
}

export const getModelHealth = async (agentOverride?: HttpAgent): Promise<SharedSystemHealth> => {
  const actor = agentOverride ? createModelActor(agentOverride) : modelCanister
  const health = await actor.health_check()
  return modelHealthToShared(health)
}

export const getAgentHealth = async (agentOverride?: HttpAgent): Promise<AgentHealth> => {
  const actor = agentOverride ? createAgentActor(agentOverride) : agentCanister
  return actor.health()
}

export const getAgentLoaderStats = async (
  agentOverride?: HttpAgent,
): Promise<Optional<AgentLoaderStats>> => {
  const actor = agentOverride ? createAgentActor(agentOverride) : agentCanister
  const result = await actor.get_loader_stats()
  if ('Err' in result) {
    return undefined
  }
  return parseLoaderStats(result.Ok)
}

export const getCoordinatorHealth = async (
  agentOverride?: HttpAgent,
): Promise<SharedSystemHealth> => {
  const actor = agentOverride ? createCoordinatorActor(agentOverride) : coordinatorCanister
  const health = await actor.health()
  return coordinatorHealthToShared(health)
}

export const getEconomicsHealth = async (
  agentOverride?: HttpAgent,
): Promise<EconHealth> => {
  const actor = agentOverride ? createEconActor(agentOverride) : econCanister
  return actor.health()
}

export const getSystemHealthSnapshot = async (
  agentOverride?: HttpAgent,
): Promise<SystemHealthSnapshot> => {
  const [model, coordinator, agentHealth, econ] = await Promise.all([
    getModelHealth(agentOverride).catch(() => undefined),
    getCoordinatorHealth(agentOverride).catch(() => undefined),
    getAgentHealth(agentOverride).catch(() => undefined),
    getEconomicsHealth(agentOverride).catch(() => undefined),
  ])

  return {
    model,
    coordinator,
    agent: agentHealth,
    econ,
  }
}

export const calculateModelStats = (summaries: ModelSummary[]): ModelStats => {
  return summaries.reduce<ModelStats>(
    (acc, summary) => {
      acc.total += 1
      switch (summary.info.state) {
        case 'Active':
          acc.active += 1
          break
        case 'Deprecated':
          acc.deprecated += 1
          break
        case 'Pending':
        default:
          acc.pending += 1
          break
      }
      return acc
    },
    { total: 0, active: 0, pending: 0, deprecated: 0 },
  )
}

const unwrapAgentCreation = (result: Result_AgentCreation): CandidAgentCreationResult => {
  if ('Err' in result) {
    throw new Error(result.Err)
  }
  return result.Ok
}

const unwrapInference = (result: Result_Inference): CandidInferenceResponse => {
  if ('Err' in result) {
    throw new Error(result.Err)
  }
  return result.Ok
}

const unwrapAgentSummaries = (result: Result_Summaries): CandidAgentSummary[] => {
  if ('Err' in result) {
    throw new Error(result.Err)
  }
  return result.Ok
}

const unwrapEmpty = (result: Result_Empty): void => {
  if ('Err' in result) {
    throw new Error(result.Err)
  }
}

export const listAgents = async (
  userPrincipal: Optional<string>,
  agentOverride?: HttpAgent,
): Promise<AgentSummaryView[]> => {
  const actor = agentOverride ? createAgentActor(agentOverride) : agentCanister
  const summaries = unwrapAgentSummaries(await actor.list_user_agents(userPrincipal ?? ''))
  return summaries.map(summary => ({
    agentId: summary.agent_id,
    agentType: agentTypeToShared(summary.agent_type),
    status: agentStatusToShared(summary.status),
    createdAt: summary.created_at,
    lastActive: summary.last_active,
  }))
}

export const listUserAgents = async (
  userPrincipal: Optional<string>,
  agentOverride?: HttpAgent,
): Promise<AgentSummaryView[]> => listAgents(userPrincipal, agentOverride)

type AgentInstructionInput = CandidAgentCreationRequest | {
  instruction: string
  agentCount?: number
  capabilities?: string[]
  priority?: string
}

const normalizeAgentCreationRequest = (input: AgentInstructionInput): CandidAgentCreationRequest => {
  if ('agentCount' in input || 'capabilities' in input || 'priority' in input) {
    return {
      instruction: input.instruction,
      agent_count: input.agentCount !== undefined ? [input.agentCount] : [],
      capabilities: input.capabilities && input.capabilities.length ? [input.capabilities] : [],
      priority: input.priority ? [input.priority] : [],
    }
  }
  return input
}

export const createAgentsFromInstructions = async (
  instructionOrRequest: AgentInstructionInput | string,
  maybeAgentCountOrAgent?: number | HttpAgent,
  maybeCapabilities?: string[],
  maybePriority?: string,
  agentOverride?: HttpAgent,
): Promise<Result_AgentCreation> => {
  let request: CandidAgentCreationRequest
  let override: Optional<HttpAgent>

  if (typeof instructionOrRequest === 'string') {
    request = normalizeAgentCreationRequest({
      instruction: instructionOrRequest,
      agentCount: typeof maybeAgentCountOrAgent === 'number' ? maybeAgentCountOrAgent : undefined,
      capabilities: maybeCapabilities,
      priority: maybePriority,
    })
    override = (typeof maybeAgentCountOrAgent !== 'number' ? maybeAgentCountOrAgent : agentOverride) as Optional<HttpAgent>
  } else {
    request = normalizeAgentCreationRequest(instructionOrRequest)
    override = (maybeAgentCountOrAgent as HttpAgent | undefined) ?? agentOverride
  }

  const actor = override ? createAgentActor(override) : agentCanister
  return actor.create_agent_from_instruction(request)
}

export const createAgentsFromInstructionsTyped = async (
  instructionOrRequest: AgentInstructionInput | string,
  maybeAgentCountOrAgent?: number | HttpAgent,
  maybeCapabilities?: string[],
  maybePriority?: string,
  agentOverride?: HttpAgent,
): Promise<CandidAgentCreationResult> => {
  const result = await createAgentsFromInstructions(
    instructionOrRequest,
    maybeAgentCountOrAgent,
    maybeCapabilities,
    maybePriority,
    agentOverride,
  )
  return unwrapAgentCreation(result)
}

export const runInference = async (
  request: CandidInferenceRequest,
  agentOverride?: HttpAgent,
): Promise<CandidInferenceResponse> => {
  const actor = agentOverride ? createAgentActor(agentOverride) : agentCanister
  const result = await actor.infer(request)
  return unwrapInference(result)
}

export const getSwarmPolicy = async (agentOverride?: HttpAgent): Promise<SwarmPolicy> => {
  const actor = agentOverride ? createCoordinatorActor(agentOverride) : coordinatorCanister
  return actor.get_swarm_policy()
}

export const setSwarmPolicy = async (
  policy: SwarmPolicy,
  agentOverride?: HttpAgent,
): Promise<void> => {
  const actor = agentOverride ? createCoordinatorActor(agentOverride) : coordinatorCanister
  const result = await actor.set_swarm_policy(policy)
  if ('Err' in result) {
    throw new Error(result.Err)
  }
}

export const routeBestResult = async (
  request: RouteRequest,
  limit: number,
  timeoutNs: bigint,
  agentOverride?: HttpAgent,
): Promise<RouteResponse> => {
  const actor = agentOverride ? createCoordinatorActor(agentOverride) : coordinatorCanister
  const result = await actor.route_best_result(request, limit, timeoutNs)
  if ('Err' in result) {
    throw new Error(result.Err)
  }
  return result.Ok
}

export const getRoutingStats = async (
  agentOverride?: HttpAgent,
): Promise<RoutingStats[]> => {
  const actor = agentOverride ? createCoordinatorActor(agentOverride) : coordinatorCanister
  const result = await actor.get_routing_stats([])
  if ('Err' in result) {
    throw new Error(result.Err)
  }
  return result.Ok
}

export const getUserQuotaStatus = async (
  agentOverride?: HttpAgent,
): Promise<CandidQuotaValidation | undefined> => {
  const actor = agentOverride ? createCoordinatorActor(agentOverride) : coordinatorCanister
  const result = await actor.get_user_quota_status()
  if ('Err' in result) {
    throw new Error(result.Err)
  }
  return result.Ok
}

export const validateTokenUsageQuota = async (
  tokens: bigint,
  agentOverride?: HttpAgent,
): Promise<CandidQuotaValidation> => {
  const actor = agentOverride ? createCoordinatorActor(agentOverride) : coordinatorCanister
  const result = await actor.validate_token_usage_quota(tokens)
  if ('Err' in result) {
    throw new Error(result.Err)
  }
  return result.Ok
}

export const registerAgent = async (
  registration: AgentRegistration,
  agentOverride?: HttpAgent,
): Promise<void> => {
  const actor = agentOverride ? createCoordinatorActor(agentOverride) : coordinatorCanister
  const result = await actor.register_agent(registration)
  if ('Err' in result) {
    throw new Error(result.Err)
  }
}

export const getEconomicsPolicy = async (
  agentOverride?: HttpAgent,
): Promise<FeePolicy> => {
  const actor = agentOverride ? createEconActor(agentOverride) : econCanister
  return actor.policy()
}

export const listEconomicsAdmins = async (
  agentOverride?: HttpAgent,
): Promise<string[]> => {
  const actor = agentOverride ? createEconActor(agentOverride) : econCanister
  return actor.list_admins()
}

export const getUserSubscription = async (
  userPrincipal: Optional<string>,
  agentOverride?: HttpAgent,
): Promise<Optional<UserSubscription>> => {
  const actor = agentOverride ? createEconActor(agentOverride) : econCanister
  const result = await actor.get_user_subscription(userPrincipal ? [userPrincipal] : [])
  return optionToValue(result)
}

export const createSubscription = async (
  tier: string,
  autoRenew: boolean,
  agentOverride?: HttpAgent,
): Promise<UserSubscription> => {
  const actor = agentOverride ? createEconActor(agentOverride) : econCanister
  const result = await actor.create_subscription(tier, autoRenew)
  if ('Err' in result) {
    throw new Error(result.Err)
  }
  return result.Ok
}

export const getPaymentRequest = async (
  tier: string,
  agentOverride?: HttpAgent,
): Promise<PaymentRequest> => {
  const actor = agentOverride ? createEconActor(agentOverride) : econCanister
  const result = await actor.create_payment_request(tier)
  if ('Err' in result) {
    throw new Error(result.Err)
  }
  return result.Ok
}

export const getPaymentStats = async (
  agentOverride?: HttpAgent,
): Promise<PaymentStats> => {
  const actor = agentOverride ? createEconActor(agentOverride) : econCanister
  return actor.get_payment_stats()
}

export interface RoutingOverview {
  policy: SwarmPolicy
  stats: RoutingStats[]
}

export const getRoutingOverview = async (
  agentOverride?: HttpAgent,
): Promise<RoutingOverview> => {
  const [policy, stats] = await Promise.all([
    getSwarmPolicy(agentOverride),
    getRoutingStats(agentOverride),
  ])
  return { policy, stats }
}

export const listInstructionRequests = async (
  agentOverride?: HttpAgent,
) => {
  const actor = agentOverride ? createCoordinatorActor(agentOverride) : coordinatorCanister
  const result = await actor.list_instruction_requests()
  if ('Err' in result) {
    throw new Error(result.Err)
  }
  return result.Ok
}

export const getInstructionAnalysis = async (
  requestId: string,
  agentOverride?: HttpAgent,
) => {
  const actor = agentOverride ? createCoordinatorActor(agentOverride) : coordinatorCanister
  const result = await actor.get_instruction_analysis(requestId)
  if ('Err' in result) {
    throw new Error(result.Err)
  }
  return result.Ok
}

export const updateAgentStatus = async (
  agentId: string,
  status: string,
  agentOverride?: HttpAgent,
): Promise<void> => {
  const actor = agentOverride ? createCoordinatorActor(agentOverride) : coordinatorCanister
  const result = await actor.update_agent_status(agentId, status)
  if ('Err' in result) {
    throw new Error(result.Err)
  }
}

export const getAgentSpawningMetrics = async (
  agentOverride?: HttpAgent,
) => {
  const actor = agentOverride ? createCoordinatorActor(agentOverride) : coordinatorCanister
  const result = await actor.get_agent_spawning_metrics()
  if ('Err' in result) {
    throw new Error(result.Err)
  }
  return result.Ok
}

export const getCoordinationNetworks = async (
  agentOverride?: HttpAgent,
) => {
  const actor = agentOverride ? createCoordinatorActor(agentOverride) : coordinatorCanister
  const result = await actor.get_coordination_networks()
  if ('Err' in result) {
    throw new Error(result.Err)
  }
  return result.Ok
}

export const upgradeSubscriptionTier = async (
  tierName: string,
  agentOverride?: HttpAgent,
): Promise<void> => {
  const actor = agentOverride ? createCoordinatorActor(agentOverride) : coordinatorCanister
  const result = await actor.upgrade_subscription_tier(tierName)
  if ('Err' in result) {
    throw new Error(result.Err)
  }
}

export const getSubscriptionTierInfo = async (
  agentOverride?: HttpAgent,
) => {
  const actor = agentOverride ? createCoordinatorActor(agentOverride) : coordinatorCanister
  const result = await actor.get_subscription_tier_info()
  if ('Err' in result) {
    throw new Error(result.Err)
  }
  return result.Ok
}

export const listEconomicsReceipts = async (
  principalFilter: Optional<string>,
  limit: Optional<number>,
  agentOverride?: HttpAgent,
) => {
  const actor = agentOverride ? createEconActor(agentOverride) : econCanister
  const result = await actor.list_receipts(principalFilter ? [principalFilter] : [], limit ? [limit] : [])
  if ('Err' in result) {
    throw new Error(result.Err)
  }
  return result.Ok
}

export const getEconomicsReceipt = async (
  receiptId: string,
  agentOverride?: HttpAgent,
) => {
  const actor = agentOverride ? createEconActor(agentOverride) : econCanister
  const result = await actor.get_receipt(receiptId)
  if ('Err' in result) {
    throw new Error(result.Err)
  }
  return result.Ok
}

export const getEconomicsBalance = async (
  principalFilter: Optional<string>,
  agentOverride?: HttpAgent,
) => {
  const actor = agentOverride ? createEconActor(agentOverride) : econCanister
  const result = await actor.get_balance(principalFilter ? [principalFilter] : [])
  if ('Err' in result) {
    throw new Error(result.Err)
  }
  return result.Ok
}

export interface AdminSnapshot {
  summaries: ModelSummary[]
  stats: ModelStats
  systemHealth: SystemHealthSnapshot
  auditLog: AuditEvent[]
  agentHealth: Optional<AgentHealth>
  agentLoader: Optional<AgentLoaderStats>
  routing: RoutingOverview
  econHealth: Optional<EconHealth>
  econPolicy: FeePolicy
  econAdmins: string[]
}

export const loadAdminSnapshot = async (
  authAgent: HttpAgent,
): Promise<AdminSnapshot> => {
  const modelActor = createModelActor(authAgent)
  const [summaries, systemHealth, auditLog, agentHealth, agentLoader, routing, econHealth, econPolicy, econAdmins] =
    await Promise.all([
      listModels(authAgent),
      getSystemHealthSnapshot(authAgent),
      readModelAuditLog(modelActor),
      getAgentHealth(authAgent).catch(() => undefined),
      getAgentLoaderStats(authAgent),
      getRoutingOverview(authAgent),
      getEconomicsHealth(authAgent).catch(() => undefined),
      getEconomicsPolicy(authAgent),
      listEconomicsAdmins(authAgent),
    ])

  const stats = calculateModelStats(summaries)

  return {
    summaries,
    stats,
    systemHealth,
    auditLog,
    agentHealth,
    agentLoader,
    routing,
    econHealth,
    econPolicy,
    econAdmins,
  }
}

export interface WorkflowNodeExecution {
  nodeId: string
  agentId: string
  response: CandidInferenceResponse
}

export interface CoordinatorWorkflowResult {
  success: boolean
  workflowId: string
  results: WorkflowNodeExecution[]
}

export const sendMessageToAgent = async (
  agentId: string,
  message: string,
  agentOverride?: HttpAgent,
): Promise<CandidInferenceResponse> => {
  const actor = agentOverride ? createAgentActor(agentOverride) : agentCanister
  const request: CandidInferenceRequest = {
    prompt: message,
    msg_id: `msg-${Date.now()}`,
    seed: BigInt(Date.now()),
    max_tokens: [],
    temperature: [],
    top_p: [],
  }
  await actor.bind_model(agentId)
  return runInference(request, agentOverride)
}

export const bindAgentAndWireRoutes = async (
  agentId: string,
  modelId: string,
  agentOverride?: HttpAgent,
  coordinatorOverride?: HttpAgent,
): Promise<void> => {
  const agentActor = agentOverride ? createAgentActor(agentOverride) : agentCanister
  const coordinatorActor = coordinatorOverride ? createCoordinatorActor(coordinatorOverride) : coordinatorCanister

  unwrapEmpty(await agentActor.bind_model(modelId))

  let agentPrincipal = ''
  try {
    const principal = await (agentOverride ?? agent).getPrincipal()
    agentPrincipal = principalToText(principal)
  } catch {
    agentPrincipal = ''
  }

  const registration: AgentRegistration = {
    agent_id: agentId,
    agent_principal: agentPrincipal,
    canister_id: CANISTER_IDS.ohms_agent,
    capabilities: [],
    model_id: modelId,
    health_score: 1,
    registered_at: BigInt(Date.now()),
    last_seen: BigInt(Date.now()),
  }

  await registerAgent(registration, agentOverride)
  const updateStatus = await coordinatorActor.update_agent_status(agentId, 'Ready')
  if ('Err' in updateStatus) {
    throw new Error(updateStatus.Err)
  }
}

export const executeCoordinatorWorkflow = async (
  workflow: { id: string; nodes: Array<{ id: string; type: string; data: { config?: Record<string, unknown> } }> },
  agentOverride?: HttpAgent,
): Promise<CoordinatorWorkflowResult> => {
  const results: WorkflowNodeExecution[] = []

  for (const node of workflow.nodes) {
    if (node.type !== 'agent') continue
    const instructions = node.data.config?.instructions
    if (typeof instructions !== 'string' || instructions.trim().length === 0) {
      continue
    }

    const creationRequest: CandidAgentCreationRequest = {
      instruction: instructions,
      agent_count: [],
      capabilities: [],
      priority: [],
    }

    const creation = await createAgentsFromInstructionsTyped(creationRequest, agentOverride)
    const agentId = creation.agent_id
    const message = `Execute node: ${node.id} -> ${instructions}`
    const response = await sendMessageToAgent(agentId, message, agentOverride)

    results.push({
      nodeId: node.id,
      agentId,
      response,
    })
  }

  return {
    success: true,
    workflowId: workflow.id,
    results,
  }
}
