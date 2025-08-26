# OHMS UI: Admin-User Separation & Subscription Plan

## 🌐 Production Infrastructure Status

### Internet Computer Mainnet Deployment

| Component | Canister ID | Direct URL | Candid UI | Status |
|-----------|-------------|------------|-----------|--------|
| **OHMS Platform UI** | `xg5yr-zaaaa-aaaah-qqe5a-cai` | [🔗](https://xg5yr-zaaaa-aaaah-qqe5a-cai.icp0.io/) | [🎛️](https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=xg5yr-zaaaa-aaaah-qqe5a-cai) | ✅ Production |
| **OHMS Agent Factory** | `gavyi-uyaaa-aaaaa-qbu7q-cai` | [🔗](https://gavyi-uyaaa-aaaaa-qbu7q-cai.icp0.io/) | [🎛️](https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=gavyi-uyaaa-aaaaa-qbu7q-cai) | ✅ Production |
| **OHMS Economics** | `tetse-piaaa-aaaao-qkeyq-cai` | [🔗](https://tetse-piaaa-aaaao-qkeyq-cai.icp0.io/) | [🎛️](https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=tetse-piaaa-aaaao-qkeyq-cai) | ✅ Production |
| **OHMS Model Repository** | `3aes4-xyaaa-aaaal-qsryq-cai` | [🔗](https://3aes4-xyaaa-aaaal-qsryq-cai.icp0.io/) | [🎛️](https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=3aes4-xyaaa-aaaal-qsryq-cai) | ✅ Production |
| **OHMS Coordinator** | `xp6tn-piaaa-aaaah-qqe4q-cai` | [🔗](https://xp6tn-piaaa-aaaah-qqe4q-cai.icp0.io/) | [🎛️](https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=xp6tn-piaaa-aaaah-qqe4q-cai) | ✅ Production |
| **OHMS Marketing Website** | `rjeaj-jyaaa-aaaau-abyka-cai` | [🔗](https://rjeaj-jyaaa-aaaau-abyka-cai.icp0.io/) | [🎛️](https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=rjeaj-jyaaa-aaaau-abyka-cai) | ✅ Production |

### NOVAQ Compression Engine
- **Status**: ✅ Complete - Open Source CLI Tool
- **Repository**: [ohms-adaptq](https://github.com/OHMS-DeAI/ohms-adaptq)
- **Installation**: `cargo install --git https://github.com/OHMS-DeAI/ohms-adaptq.git`

---

## Architecture Overview

OHMS 2.0 implements strict separation between admin and user operations:

### Admin Operations (Restricted Access)
- NOVAQ model processing and compression (93-100x)
- ICP canister deployment and management
- User monitoring and platform analytics
- Subscription tier configuration and governance
- Rate limiting and quota management
- Model curation and quality assurance

### User Operations (Subscription-Based)
- **Basic tier**: 5 agents, 10 monthly creations, 100K tokens
- **Pro tier**: 25 agents, 50 monthly creations, 500K tokens
- **Enterprise tier**: 100 agents, 200 monthly creations, 2M tokens
- Agent spinning from natural language instructions
- Autonomous agent monitoring and management
- Subscription and billing management
- Usage tracking and analytics

## Authentication & Access Control

### Admin Authentication
- **Internet Identity v2**: Admin-level authentication via id.ai
- **Role-Based Access**: Only designated admin principals
- **Admin Dashboard**: Hidden from regular users
- **Security**: Admin routes protected by guard components

### User Authentication
- **Internet Identity v2**: User authentication via id.ai with Google account integration
- **Subscription Required**: Active subscription for agent creation
- **Quota Enforcement**: Monthly agent creation limits
- **Rate Limiting**: Token and inference limits per tier

## Subscription System

### Subscription Tiers

| Tier | Monthly Fee | Max Agents | Monthly Creations | Tokens | Inference Rate |
|------|-------------|------------|-------------------|--------|----------------|
| **Basic** | $29 | 5 | 10 | 100,000 | Standard |
| **Pro** | $99 | 25 | 50 | 500,000 | Priority |
| **Enterprise** | $299 | 100 | 200 | 2,000,000 | Premium |

### Payment Processing
- **Stripe Integration**: USD payment processing with real-time ICP conversion
- **Automated Billing**: Monthly subscription renewals
- **Usage Tracking**: Real-time token and agent usage monitoring
- **Quota Enforcement**: Automatic limits based on subscription tier

## UI Implementation

### Admin Interface Features
- **Model Management Dashboard**: APQ processing status and controls
- **User Monitoring**: Real-time user activity and agent creation
- **Platform Analytics**: Performance metrics and usage statistics
- **Subscription Administration**: User subscription management
- **System Configuration**: Rate limits, quotas, and platform settings

### User Interface Features
- **Instruction Input**: Natural language agent creation interface
- **Agent Dashboard**: Autonomous agent monitoring and management
- **Subscription Portal**: Plan selection and billing management
- **Usage Monitor**: Token usage and quota tracking
- **Performance Analytics**: Agent performance metrics

## Security Implementation

### Admin Security
```typescript
// Admin route protection
const AdminRoute = ({ children }: { children: React.ReactNode }) => {
  const { isAdmin, loading } = useAdmin();
  
  if (loading) return <LoadingSpinner />;
  if (!isAdmin) return <Navigate to="/" replace />;
  
  return <>{children}</>;
};

// Admin role verification
const useAdmin = () => {
  const [isAdmin, setIsAdmin] = useState(false);
  
  useEffect(() => {
    const checkAdminStatus = async () => {
      // Check against Internet Identity v2 admin principals
      const adminStatus = await verifyAdminRole();
      setIsAdmin(adminStatus);
    };
    checkAdminStatus();
  }, []);
  
  return { isAdmin };
};
```

### User Subscription Enforcement
```typescript
// Subscription quota enforcement
const useSubscriptionLimits = () => {
  const [limits, setLimits] = useState<SubscriptionLimits>();
  
  const checkQuota = async (action: 'create_agent' | 'inference') => {
    const usage = await getUserUsage();
    const tier = await getSubscriptionTier();
    
    return validateQuota(usage, tier, action);
  };
  
  return { limits, checkQuota };
};
```

## Environment Configuration

### Admin Configuration
```bash
# Admin principals (Internet Identity v2 addresses)
VITE_ADMIN_PRINCIPALS=["principal1", "principal2"]

# Admin interface settings
VITE_ADMIN_ENABLED=true
VITE_ADMIN_PATH="/admin"
```

### Authentication & Payment Configuration
```bash
# Internet Identity v2 configuration
VITE_II_CANISTER_ID="your-ii-canister-id"
VITE_II_HOST="https://id.ai"

# Stripe configuration
VITE_STRIPE_PUBLISHABLE_KEY="your-stripe-publishable-key"
VITE_STRIPE_SECRET_KEY="your-stripe-secret-key"

# Market data for ICP conversion
VITE_ICP_USD_RATE_API="https://api.coingecko.com/api/v3"

# Subscription tiers
VITE_SUBSCRIPTION_TIERS='{"basic": {...}, "pro": {...}, "enterprise": {...}}'

# Google account sync configuration
VITE_GOOGLE_ACCOUNT_SYNC_ENABLED=true
VITE_STRIPE_GOOGLE_SYNC_ENABLED=true
```

## Implementation Roadmap

### Phase 1: Authentication Migration (Week 1)
- [ ] Remove OISY wallet dependencies
- [ ] Implement Internet Identity v2 via id.ai
- [ ] Add Google account integration
- [ ] Update admin authentication system

### Phase 2: Payment Integration (Week 2)
- [ ] Integrate Stripe payment processing
- [ ] Implement real-time ICP/USD conversion
- [ ] Add subscription management
- [ ] Update billing and quota enforcement

### Phase 3: Agent Spinning Interface (Week 3)
- [ ] Instruction input interface
- [ ] Agent creation workflow
- [ ] Autonomous agent dashboard
- [ ] Performance monitoring

### Phase 4: Integration & Testing (Week 4)
- [ ] End-to-end testing
- [ ] Security audit
- [ ] Performance optimization
- [ ] Production deployment

## Success Metrics

### Admin Metrics
- Model upload and deployment time: <10 minutes
- Platform monitoring effectiveness: Real-time alerts
- User management efficiency: Complete control dashboard

### User Metrics
- Agent creation time: <30 seconds from instruction
- Subscription conversion rate: >10% trial to paid
- User satisfaction: >4.5/5 rating
- Autonomous operation uptime: >99.9%

---

> **Implementation Focus**: Complete separation of admin and user operations with Internet Identity v2 + Stripe integration and subscription-based economics.