/**
 * Dynamic Canister ID Configuration
 *
 * This file implements the proper ICP development pattern for consuming canister IDs.
 * Canister IDs are injected at build time by dfx as environment variables following the pattern:
 * import.meta.env.VITE_CANISTER_ID_<CANISTER_NAME_UPPERCASE>
 *
 * This approach ensures:
 * - No hardcoded canister IDs anywhere in the codebase
 * - Automatic canister ID resolution based on deployment network
 * - Seamless integration with dfx build system
 * - No manual updates required when canisters are recreated
 */

export interface CanisterConfig {
  ohms_model: string
  ohms_agent: string
  ohms_coordinator: string
  ohms_econ: string
}

/**
 * Get canister ID from dfx environment variables
 * dfx automatically injects these during build: import.meta.env.VITE_CANISTER_ID_<CANISTER_NAME>
 */
function getCanisterId(canisterName: string): string {
  const envVar = `VITE_CANISTER_ID_${canisterName.toUpperCase()}`
  const canisterId = import.meta.env[envVar]

  if (!canisterId) {
    throw new Error(`Canister ID not found for ${canisterName}. Expected environment variable: ${envVar}`)
  }

  return canisterId
}

/**
 * Get all canister IDs dynamically from dfx environment variables
 * This is the proper ICP way - no hardcoded values, fully dynamic
 */
export function getCanisters(): CanisterConfig {
  return {
    ohms_model: getCanisterId('ohms_model'),
    ohms_agent: getCanisterId('ohms_agent'),
    ohms_coordinator: getCanisterId('ohms_coordinator'),
    ohms_econ: getCanisterId('ohms_econ'),
  }
}

/**
 * Get a specific canister ID
 * @param canisterName - Name of the canister
 * @returns Canister ID string
 */
export function getCanister(canisterName: keyof CanisterConfig): string {
  const canisters = getCanisters()
  return canisters[canisterName]
}

/**
 * Validate that all required canister IDs are available
 * @returns boolean indicating if all canisters are configured
 */
export function validateCanisters(): boolean {
  try {
    getCanisters()
    return true
  } catch {
    return false
  }
}

/**
 * Network detection based on dfx environment
 * dfx sets DFX_NETWORK environment variable during build
 */
export function getNetwork(): string {
  return import.meta.env.VITE_DFX_NETWORK || 'local'
}

/**
 * Check if running on mainnet
 */
export function isMainnet(): boolean {
  return getNetwork() === 'ic'
}

/**
 * Check if running on local network
 */
export function isLocal(): boolean {
  return getNetwork() === 'local'
}

/**
 * Check if running on playground
 */
export function isPlayground(): boolean {
  return getNetwork() === 'playground'
}

/**
 * Get host configuration based on network
 */
export function getHost(): string {
  const network = getNetwork()

  switch (network) {
    case 'ic':
      return 'https://ic0.app'
    case 'playground':
      return 'https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io'
    default:
      return 'http://127.0.0.1:4943'
  }
}

export default {
  getCanisters,
  getCanister,
  validateCanisters,
  getNetwork,
  isMainnet,
  isLocal,
  isPlayground,
  getHost,
}