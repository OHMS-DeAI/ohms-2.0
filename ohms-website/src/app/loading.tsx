export default function Loading() {
  return (
    <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-background-dark to-background-darker">
      <div className="text-center">
        <div className="w-16 h-16 border-4 border-primary border-t-transparent rounded-full animate-spin mx-auto mb-4"></div>
        <h2 className="text-2xl font-bold text-text-primary mb-2">Loading OHMS</h2>
        <p className="text-text-muted">Preparing your decentralized AI experience...</p>
      </div>
    </div>
  )
}
