/**
 * Network Configuration using dfx environment variables
 *
 * This file now uses the proper ICP development pattern where canister IDs
 * are injected at build time by dfx and consumed dynamically at runtime.
 *
 * No hardcoded canister IDs - everything is resolved from dfx build system.
 */

import { getCanisters, getNetwork, getHost, isMainnet, isLocal, isPlayground } from './canisters'

/**
 * Get current network from dfx environment
 */
export const NETWORK = getNetwork()

/**
 * Get boundary node host for current network
 */
export const HOST = getHost()

/**
 * Network type guards
 */
export const IS_MAINNET = isMainnet()
export const IS_LOCAL = isLocal()
export const IS_PLAYGROUND = isPlayground()

/**
 * Canister ID type
 */
export type CanisterIds = {
  ohms_model: string
  ohms_agent: string
  ohms_coordinator: string
  ohms_econ: string
}

/**
 * Get canister IDs dynamically from dfx environment variables
 * This is the proper ICP way - fully dynamic, no hardcoded values
 */
export const getCanisterIdsFromEnv = (): CanisterIds => {
  return getCanisters()
}

/**
 * Get a specific canister ID by name
 */
export const getCanisterId = (name: keyof CanisterIds): string => {
  const canisters = getCanisters()
  return canisters[name]
}

/**
 * Runtime network detection based on hostname
 * This is used for client-side network detection when needed
 */
const getRuntimeNetwork = (): string => {
  if (typeof window === 'undefined') return NETWORK

  const hostname = window.location.hostname.toLowerCase()

  // Mainnet detection
  if (hostname.endsWith('.ic0.app') ||
      hostname.endsWith('.icp0.io') && !hostname.includes('-cai.icp0.io')) {
    return 'ic'
  }

  // Playground detection (canister URLs)
  if (hostname.includes('-cai.icp0.io')) {
    return 'playground'
  }

  // Default to local
  return 'local'
}

/**
 * Get runtime host based on current network
 */
export const getRuntimeHost = (): string => {
  const runtimeNetwork = getRuntimeNetwork()

  switch (runtimeNetwork) {
    case 'ic':
      return 'https://ic0.app'
    case 'playground':
      return 'https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io'
    default:
      return 'http://127.0.0.1:4943'
  }
}


