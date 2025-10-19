/**
 * Global Error Handler - Catches and filters common development errors
 * Specifically handles fetchRootKey and UserInterrupt errors from IC agent
 */

interface ErrorPattern {
  pattern: RegExp
  action: 'suppress' | 'log' | 'warn'
  description: string
}

const GLOBAL_ERROR_PATTERNS: ErrorPattern[] = [
  {
    pattern: /net::ERR_BLOCKED_BY_CLIENT.*api\/v2\/status/i,
    action: 'suppress',
    description: 'Browser extension blocking IC replica API status requests'
  },
  {
    pattern: /userinterrupt|UserInterrupt/i,
    action: 'suppress',
    description: 'Internet Computer agent user interruption errors'
  },
  {
    pattern: /ERR_BLOCKED_BY_CLIENT.*localhost:4943/i,
    action: 'suppress',
    description: 'Browser extension blocking local IC replica requests'
  },
  {
    pattern: /fetchRootKey.*ERR_BLOCKED_BY_CLIENT/i,
    action: 'suppress',
    description: 'fetchRootKey calls blocked by browser extensions'
  },
  {
    pattern: /api\/v2\/status.*ERR_CONNECTION_REFUSED/i,
    action: 'suppress',
    description: 'Local replica not running or connection refused'
  },
  {
    pattern: /http:\/\/localhost:4943\/api\/v2\/status.*ERR_BLOCKED_BY_CLIENT/i,
    action: 'suppress',
    description: 'HTTP requests to local replica blocked by browser extensions'
  },
  {
    pattern: /USER_INTERRUPT.*Request interrupted.*browser extensions/i,
    action: 'suppress',
    description: 'User interrupt errors from browser extension blocking'
  },
  {
    pattern: /localhost:4943.*fetchRootKey/i,
    action: 'suppress',
    description: 'fetchRootKey requests to local replica'
  }
]

class GlobalErrorHandler {
  private static instance: GlobalErrorHandler
  private originalHandler: ((event: ErrorEvent) => void) | null = null
  private originalRejectionHandler: ((event: PromiseRejectionEvent) => void) | null = null
  private suppressedCount = 0

  private constructor() {}

  static getInstance(): GlobalErrorHandler {
    if (!GlobalErrorHandler.instance) {
      GlobalErrorHandler.instance = new GlobalErrorHandler()
    }
    return GlobalErrorHandler.instance
  }

  install(): void {
    if (typeof window === 'undefined') return

    // Handle regular errors
    this.originalHandler = window.onerror
    window.onerror = (message, source, lineno, colno, error) => {
      if (this.shouldSuppressError(message, error)) {
        this.suppressedCount++
        return true // Prevent default error handling
      }

      // Pass to original handler if exists
      if (this.originalHandler) {
        return this.originalHandler.call(window, message, source, lineno, colno, error)
      }

      return false
    }

    // Handle unhandled promise rejections
    this.originalRejectionHandler = window.onunhandledrejection
    window.onunhandledrejection = (event) => {
      if (this.shouldSuppressRejection(event.reason)) {
        this.suppressedCount++
        event.preventDefault() // Prevent default rejection handling
        return
      }

      // Pass to original handler if exists
      if (this.originalRejectionHandler) {
        this.originalRejectionHandler.call(window, event)
      }
    }
  }

  uninstall(): void {
    if (typeof window === 'undefined') return

    if (this.originalHandler) {
      window.onerror = this.originalHandler
      this.originalHandler = null
    }

    if (this.originalRejectionHandler) {
      window.onunhandledrejection = this.originalRejectionHandler
      this.originalRejectionHandler = null
    }
  }

  private shouldSuppressError(message: any, error: Error | null): boolean {
    const messageStr = message?.toString() || ''
    const errorStr = error?.message || error?.toString() || ''
    const fullText = `${messageStr} ${errorStr}`.toLowerCase()

    return GLOBAL_ERROR_PATTERNS.some(pattern =>
      pattern.action === 'suppress' && pattern.pattern.test(fullText)
    )
  }

  private shouldSuppressRejection(reason: any): boolean {
    const reasonStr = reason?.message || reason?.toString() || ''
    const fullText = reasonStr.toLowerCase()

    return GLOBAL_ERROR_PATTERNS.some(pattern =>
      pattern.action === 'suppress' && pattern.pattern.test(fullText)
    )
  }

  getStats(): { suppressedCount: number } {
    return { suppressedCount: this.suppressedCount }
  }

  // Method to manually check if an error should be filtered
  shouldFilter(message: string, error?: Error | string): boolean {
    const messageStr = message?.toString() || ''
    const errorStr = error?.message || error?.toString() || ''
    const fullText = `${messageStr} ${errorStr}`.toLowerCase()

    return GLOBAL_ERROR_PATTERNS.some(pattern =>
      pattern.action === 'suppress' && pattern.pattern.test(fullText)
    )
  }
}

// Create singleton instance
const globalErrorHandler = GlobalErrorHandler.getInstance()

// Export functions for easy usage
export const installGlobalErrorHandler = (): void => {
  globalErrorHandler.install()
}

export const uninstallGlobalErrorHandler = (): void => {
  globalErrorHandler.uninstall()
}

export const shouldFilterError = (message: string, error?: Error | string): boolean => {
  return globalErrorHandler.shouldFilter(message, error)
}

export const getGlobalErrorStats = () => globalErrorHandler.getStats()

// Auto-install in development
if (import.meta.env.DEV && typeof window !== 'undefined') {
  globalErrorHandler.install()
}