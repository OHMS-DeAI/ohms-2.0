import { defineConfig, loadEnv } from 'vite'
import path from 'path'
import fs from 'fs'
import react from '@vitejs/plugin-react'

// Load canister IDs from dfx canister_ids.json files
function loadCanisterIds(network: string): Record<string, string> {
  try {
    const canisterIdsPath = path.resolve(__dirname, '../.dfx', network, 'canister_ids.json')

    if (!fs.existsSync(canisterIdsPath)) {
      console.warn(`Canister IDs file not found: ${canisterIdsPath}`)
      return {}
    }

    const canisterIds = JSON.parse(fs.readFileSync(canisterIdsPath, 'utf8'))
    const envVars: Record<string, string> = {}

    for (const [canisterName, ids] of Object.entries(canisterIds)) {
      if (typeof ids === 'object' && ids !== null) {
        // Get the network-specific canister ID
        const networkId = (ids as any)[network]
        if (networkId) {
          envVars[`VITE_CANISTER_ID_${canisterName.toUpperCase()}`] = networkId
          console.log(`Loaded canister ID: ${canisterName} -> ${networkId}`)
        } else {
          console.warn(`No ${network} ID found for canister: ${canisterName}`)
        }
      }
    }

    return envVars
  } catch (error) {
    console.error('Error loading canister IDs:', error)
    return {}
  }
}

// https://vite.dev/config/
export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), '')

  // Get network from command line env var or fallback to local
  const network = process.env.DFX_NETWORK || env.DFX_NETWORK || 'local'
  console.log(`Building for network: ${network}`)

  // Load canister IDs for the current network
  const canisterEnvVars = loadCanisterIds(network)

  return {
    plugins: [react()],
    resolve: {
      alias: {
        '@dfinity/agent': path.resolve(__dirname, 'src/shims/dfinity-agent.ts'),
        '@dfinity-agent-real': path.resolve(__dirname, 'node_modules/@dfinity/agent/lib/esm/index.js'),
        '@dfinity/identity/lib/cjs/identity/partial': path.resolve(__dirname, 'node_modules/@dfinity/identity/lib/cjs/identity/partial.js'),
      },
    },
    build: {
      outDir: 'dist',
      assetsDir: 'assets',
      rollupOptions: {
        // Workaround: force-resolve @dfinity/identity subpath exports
        external: [],
        output: {
          manualChunks: undefined,
        },
      },
      // Ensure compatibility with ICP
      target: 'es2020',
      sourcemap: false,
      // Remove console logs in production builds
      terserOptions: {
        compress: {
          drop_console: true,
          drop_debugger: true,
        },
      },
    },
    server: {
      port: 3000,
      host: true,
    },
    // Define environment variables for different networks
    define: {
      // Inject dfx network
      'import.meta.env.VITE_DFX_NETWORK': JSON.stringify(network),
      'import.meta.env.NODE_ENV': JSON.stringify(process.env.NODE_ENV || 'development'),

      // Inject canister IDs from dfx
      ...Object.fromEntries(
        Object.entries(canisterEnvVars).map(([key, value]) => [
          `import.meta.env.${key}`,
          JSON.stringify(value)
        ])
      ),
    },
    // Optimize for production builds
    optimizeDeps: {
      include: [
        '@dfinity/agent',
        '@dfinity/principal',
        '@dfinity/candid',
        '@dfinity/auth-client',
        '@dfinity/identity',
        '@dfinity/ledger-icp',
        '@slide-computer/signer',
        '@slide-computer/signer-agent',
      ],
    },
  }
})
