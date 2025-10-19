import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'
import { logger } from './utils/professionalLogger'
import consoleManager from './utils/consoleOverride'
import { initializeExtensionErrorSupport } from './utils/extensionErrorSupport'
import { installConsoleErrorFilter } from './utils/consoleErrorFilter'
import { installGlobalErrorHandler } from './utils/globalErrorHandler'
import { fixAriaHiddenIssue } from './utils/accessibilityFix'

// Initialize professional logging system
logger.info('OHMS 2.0 Application Starting', {
  environment: import.meta.env.MODE,
  version: import.meta.env.VITE_APP_VERSION || '2.0.0',
  production: import.meta.env.PROD
});

// Install console override system
// Temporarily disabled due to recursion issue
// consoleManager.install();
// consoleManager.captureGlobalErrors();

// Initialize extension error support for legacy wallet extension noise
initializeExtensionErrorSupport()

// Install console error filtering to suppress extension errors
installConsoleErrorFilter()

// Install global error handler to suppress fetchRootKey and UserInterrupt errors
installGlobalErrorHandler()

// Fix aria-hidden accessibility issue
fixAriaHiddenIssue()

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
