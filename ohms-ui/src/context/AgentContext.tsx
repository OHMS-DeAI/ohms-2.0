import React, { createContext, useContext, useState, useEffect, useCallback, type ReactNode } from 'react'
import { HttpAgent, type Identity } from '@dfinity/agent'
import { Principal } from '@dfinity/principal'
import { debugAdminConfig, getAdminStatus, CURRENT_NETWORK, ADMIN_PRINCIPALS } from '../config/adminConfig'
import { getCanisterIdsFromEnv, NETWORK, HOST } from '../config/network'
import {
  getUserFriendlyErrorMessage,
  getBrowserGuidance,
  isBraveBrowser,
  classifyWalletError,
  type WalletError,
} from '../utils/walletErrorHandler'
import { internetIdentityService, type IIv2User, type GoogleAccountInfo } from '../services/internetIdentityService'
import { getLlmService, type LlmState, type QuantizedModel, type ConversationSession, LlmError } from '../services/llmService'
import { createAgentActor, loadAdminSnapshot } from '../services/canisterService'
import type { AdminSnapshot } from '../services/canisterService'

// Define types for our canisters
export interface CanisterIds {
  ohms_model: string
  ohms_agent: string
  ohms_coordinator: string
  ohms_econ: string
}

interface UserProfile {
  name?: string
  accountId?: string
  principal: string
  email?: string // For Google account integration
  picture?: string // For Google profile picture
  googleId?: string // For Google account ID
  googleAccount?: GoogleAccountInfo // Full Google account info for Stripe
  isAnonymous?: boolean // From II v2
}

interface AgentContextType {
  canisterIds: CanisterIds
  isWalletAvailable: boolean
  principal: string | null
  userProfile: UserProfile | null
  isConnected: boolean
  isConnecting: boolean
  connectionError: WalletError | null
  createAuthAgent: () => Promise<HttpAgent | null>
  getPrincipal: () => Promise<string | null>
  connect: () => Promise<boolean>
  disconnect: () => void
  isAdmin: boolean
  checkAdminStatus: () => Promise<boolean>
  adminData: AdminData | null
  refreshAdminData: () => Promise<AdminData | null>
  initializeServices: () => Promise<void>
  clearConnectionError: () => void
  // LLM functionality
  llmState: LlmState
  createLlmConversation: (model: QuantizedModel) => Promise<ConversationSession>
  selectLlmConversation: (sessionId: string) => void
  sendLlmMessage: (message: string) => Promise<void>
  switchLlmModel: (model: QuantizedModel) => Promise<void>
  deleteLlmConversation: (sessionId: string) => Promise<void>
}

type AdminData = AdminSnapshot

const AgentContext = createContext<AgentContextType | undefined>(undefined)

// Default canister IDs (from centralized network env loader)
const defaultCanisterIds: CanisterIds = getCanisterIdsFromEnv()

interface AgentProviderProps {
  children: ReactNode
}

export const AgentProvider: React.FC<AgentProviderProps> = ({ children }) => {
  const [canisterIds] = useState<CanisterIds>(defaultCanisterIds)
  const [principal, setPrincipal] = useState<string | null>(null)
  const [userProfile, setUserProfile] = useState<UserProfile | null>(null)
  const [isWalletAvailable, setIsWalletAvailable] = useState(false)
  const [isConnected, setIsConnected] = useState(false)
  const [isConnecting, setIsConnecting] = useState(false)
  const [connectionError, setConnectionError] = useState<WalletError | null>(null)
  const [isAdmin, setIsAdmin] = useState(false)
  const [adminData, setAdminData] = useState<AdminData | null>(null)

  // LLM state
  const [llmState, setLlmState] = useState<LlmState>({
    conversations: new Map(),
    currentConversation: null,
    availableModels: [],
    userQuota: null,
    isLoading: false,
    error: null,
  })

  // Local storage keys
  const STORAGE_KEYS = {
    WAS_CONNECTED: 'ohms_was_connected',
    PRINCIPAL: 'ohms_principal',
    USER_PROFILE: 'ohms_user_profile',
    LAST_CONNECTION: 'ohms_last_connection'
  }
  
  // Initialize Internet Identity v2
  useEffect(() => {
    const initializeII = async () => {
      try {
        // Initialize Internet Identity v2
        const initialized = await internetIdentityService.initialize()
        setIsWalletAvailable(true)
        
        if (initialized) {
          // Restore existing session
          const authStatus = await internetIdentityService.getAuthStatus()
          if (authStatus.isAuthenticated && authStatus.user) {
            const userProfile = convertIIUserToProfile(authStatus.user)
            setPrincipal(authStatus.principal!)
            setUserProfile(userProfile)
            setIsConnected(true)
            storeConnection(authStatus.principal!, userProfile)
            // Restored session
          }
        }
      } catch (error) {
        // Failed to initialize
        setIsWalletAvailable(false)
      }
    }
    
    initializeII()
  }, [])
  
  const clearStoredConnection = () => {
    localStorage.removeItem(STORAGE_KEYS.WAS_CONNECTED)
    localStorage.removeItem(STORAGE_KEYS.PRINCIPAL)
    localStorage.removeItem(STORAGE_KEYS.USER_PROFILE)
    localStorage.removeItem(STORAGE_KEYS.LAST_CONNECTION)
  }
  
  const storeConnection = (principalId: string, profile: UserProfile) => {
    localStorage.setItem(STORAGE_KEYS.WAS_CONNECTED, 'true')
    localStorage.setItem(STORAGE_KEYS.PRINCIPAL, principalId)
    localStorage.setItem(STORAGE_KEYS.USER_PROFILE, JSON.stringify(profile))
    localStorage.setItem(STORAGE_KEYS.LAST_CONNECTION, Date.now().toString())
  }

  // Convert II v2 user to UserProfile
  const convertIIUserToProfile = (iiUser: IIv2User): UserProfile => {
    return {
      principal: iiUser.principal,
      name: iiUser.name,
      email: iiUser.email,
      picture: iiUser.picture,
      googleId: iiUser.googleAccount?.googleId,
      googleAccount: iiUser.googleAccount,
      isAnonymous: iiUser.isAnonymous
    }
  }
  
  // Check admin status when principal changes
  useEffect(() => {
    if (principal) {
      // Initialize services for all users
      initializeServices()

      checkAdminStatus().then(isAdminUser => {
        if (isAdminUser) {
          // Admin access granted - logging removed for security
          void refreshAdminData().catch(() => undefined)
        } else {
          // Regular user access - logging removed for security
        }
      })
    } else {
      // Clear admin status when no principal
      // User disconnected - clearing admin status
      setIsAdmin(false)
      setAdminData(null)
    }
  }, [principal])

  // Internet Identity v2 authentication
  const connect = async (): Promise<boolean> => {
    setIsConnecting(true)
    setConnectionError(null)
    
    try {
      // Starting authentication
      
      // Authenticate with II v2
      const authResult = await internetIdentityService.authenticate(true) // Prefer Google
      
      if (!authResult.success) {
        throw new Error(authResult.error || 'Authentication failed')
      }

      if (!authResult.user) {
        throw new Error('No user profile returned from authentication')
      }

      // Convert II user to UserProfile and update state
      const userProfile = convertIIUserToProfile(authResult.user)
      const principalString = authResult.user.principal

      setPrincipal(principalString)
      setUserProfile(userProfile)
      setIsConnected(true)
      setConnectionError(null)
      storeConnection(principalString, userProfile)

      // II v2 authentication successful - logging removed for security
      
      return true
    } catch (error) {
      const walletError = classifyWalletError(error)
      const friendlyMessage = getUserFriendlyErrorMessage(walletError)
      
      // Authentication failed
      setIsConnected(false)
      setConnectionError(walletError)
      throw walletError
    } finally {
      setIsConnecting(false)
    }
  }
  
  // Create authenticated agent with II v2
  const createAuthAgent = async (): Promise<HttpAgent | null> => {
    try {
      if (!isConnected) {
        // Try to connect first
        const connected = await connect()
        if (!connected) {
          throw new Error('Failed to establish II v2 connection')
        }
      }

      // Get agent from II v2 service
      const agent = internetIdentityService.createAgent()
      if (!agent) {
        throw new Error('Failed to create authenticated agent - no identity available')
      }

      // For local development, fetch root key
      if (NETWORK !== 'ic') {
        await agent.fetchRootKey()
      }

      // Created authenticated agent
      return agent
    } catch (error) {
      // Failed to create agent
      throw error
    }
  }

  // Get principal from II v2
  const getPrincipal = async (): Promise<string | null> => {
    try {
      const authStatus = await internetIdentityService.getAuthStatus()
      if (authStatus.isAuthenticated && authStatus.principal) {
        setPrincipal(authStatus.principal)
        return authStatus.principal
      }
      return null
    } catch (error) {
      const walletError = classifyWalletError(error)
      // Failed to get principal
      setConnectionError(walletError)
      return null
    }
  }

  const disconnect = async () => {
    try {
      // Sign out from II v2 service
      await internetIdentityService.signOut()
    } catch (error) {
      // Error signing out
    }
    
    setPrincipal(null)
    setUserProfile(null)
    setIsConnected(false)
    setIsAdmin(false)
    setAdminData(null)
    setConnectionError(null)
    clearStoredConnection()
    
    // Disconnected
  }

  const clearConnectionError = () => {
    setConnectionError(null)
  }

  // LLM methods
  const createLlmConversation = async (model: QuantizedModel): Promise<ConversationSession> => {
    try {
      const llmServiceInstance = getLlmService()
      const conversation = await llmServiceInstance.createConversation(model)
      setLlmState(prev => ({
        ...prev,
        conversations: new Map(prev.conversations).set(conversation.session_id, conversation),
        currentConversation: conversation,
      }))
      return conversation
    } catch (error) {
      // Failed to create LLM conversation
      throw error
    }
  }

  const sendLlmMessage = async (message: string): Promise<void> => {
    try {
      setLlmState(prev => ({ ...prev, isLoading: true, error: null }))

      const llmServiceInstance = getLlmService()
      await llmServiceInstance.sendMessage(message)

      // Update local state with new messages
      const currentState = llmServiceInstance.getState()
      setLlmState(prev => ({
        ...prev,
        conversations: currentState.conversations,
        currentConversation: currentState.currentConversation,
        isLoading: false,
      }))
    } catch (error) {
      // Failed to send LLM message
      const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred'
      setLlmState(prev => ({
        ...prev,
        isLoading: false,
        error: { error: LlmError.InternalError, message: errorMessage }
      }))
      throw error
    }
  }

  const selectLlmConversation = (sessionId: string): void => {
    const llmServiceInstance = getLlmService()
    const selected = llmServiceInstance.selectConversation(sessionId)
    const snapshot = llmServiceInstance.getState()
    setLlmState({
      ...snapshot,
      conversations: new Map(snapshot.conversations),
      currentConversation: selected,
    })
  }

  const switchLlmModel = async (model: QuantizedModel): Promise<void> => {
    try {
      if (llmState.currentConversation) {
        const llmServiceInstance = getLlmService()
        await llmServiceInstance.switchModel(model)
        setLlmState(prev => ({
          ...prev,
          currentConversation: prev.currentConversation ? {
            ...prev.currentConversation,
            model
          } : null,
        }))
      }
    } catch (error) {
      // Failed to switch LLM model
      throw error
    }
  }

  const deleteLlmConversation = async (sessionId: string): Promise<void> => {
    try {
      const llmServiceInstance = getLlmService()
      await llmServiceInstance.deleteConversation(sessionId)
      setLlmState(prev => {
        const newConversations = new Map(prev.conversations)
        newConversations.delete(sessionId)
        return {
          ...prev,
          conversations: newConversations,
          currentConversation: prev.currentConversation?.session_id === sessionId
            ? null
            : prev.currentConversation,
        }
      })
    } catch (error) {
      // Failed to delete conversation
      throw error
    }
  }

  const checkAdminStatus = async (): Promise<boolean> => {
    if (!isWalletAvailable || !principal) {
      setIsAdmin(false)
      return false
    }
    
    try {
      // Get detailed admin status
      const adminStatusInfo = getAdminStatus(principal)
      setIsAdmin(adminStatusInfo.isAdmin)
      
      // Comprehensive logging
      const logLevel = adminStatusInfo.isAdmin ? 'log' : 'info'
      // Admin Check - Status determined
      // Admin status details logged
      // End admin check
      
      // Debug admin configuration in development or when debug is enabled
      if (import.meta.env.DEV || import.meta.env.VITE_ADMIN_DEBUG === 'true') {
        debugAdminConfig()
      }
      
      return adminStatusInfo.isAdmin
    } catch (error) {
      // Failed to check admin status
      setIsAdmin(false)
      return false
    }
  }

  const initializeServices = async () => {
    try {
      // Initializing services
      
      const authAgent = await createAuthAgent()
      if (!authAgent) {
        // No authenticated agent available
        return
      }

      // Import API client dynamically to avoid initializing it unnecessarily
      const { getApiClient } = await import('../services/apiClient')

      const agentActor = createAgentActor(authAgent)

      // Get current identity from II v2 service
      const currentIdentity = internetIdentityService.getCurrentIdentity()

      // Get the API client instance and initialize it
      const apiClientInstance = getApiClient()
      await apiClientInstance.initialize(authAgent, currentIdentity || undefined)

      // Initialize LLM service with agent canister
      const llmServiceInstance = getLlmService()
      await llmServiceInstance.initialize(agentActor)
      // Services initialized
    } catch (error) {
      // Failed to initialize services
    }
  }

  const refreshAdminData = useCallback(async (): Promise<AdminData | null> => {
    if (!isAdmin) return null

    try {
      const authAgent = await createAuthAgent()
      if (!authAgent) {
        return null
      }

      const snapshot = await loadAdminSnapshot(authAgent)
      setAdminData(snapshot)
      return snapshot
    } catch (error) {
      setAdminData(null)
      throw error
    }
  }, [createAuthAgent, isAdmin])

  return (
    <AgentContext.Provider
      value={{
        canisterIds,
        isWalletAvailable,
        principal,
        userProfile,
        isConnected,
        isConnecting,
        connectionError,
        createAuthAgent,
        getPrincipal,
        connect,
        disconnect,
        isAdmin,
        checkAdminStatus,
        adminData,
        refreshAdminData,
        initializeServices,
        clearConnectionError,
        // LLM functionality
        llmState,
        createLlmConversation,
        selectLlmConversation,
        sendLlmMessage,
        switchLlmModel,
        deleteLlmConversation,
      }}
    >
      {children}
    </AgentContext.Provider>
  )
}

export const useAgent = () => {
  const context = useContext(AgentContext)
  if (context === undefined) {
    throw new Error('useAgent must be used within an AgentProvider')
  }
  return context
}
