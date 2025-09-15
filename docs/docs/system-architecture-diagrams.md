# OHMS 2.0 System Architecture & Workflows - Real II v2 Principal Authentication

## 🌐 Production Infrastructure Status

### Internet Computer Mainnet Deployment

| Component | Canister ID | Direct URL | Candid UI | Status |
|-----------|-------------|------------|-----------|--------|
| **OHMS Agent Factory** | `gavyi-uyaaa-aaaaa-qbu7q-cai` | [🔗](https://gavyi-uyaaa-aaaaa-qbu7q-cai.icp0.io/) | [🎛️](https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=gavyi-uyaaa-aaaaa-qbu7q-cai) | ✅ Production |
| **OHMS Coordinator** | `xp6tn-piaaa-aaaah-qqe4q-cai` | [🔗](https://xp6tn-piaaa-aaaah-qqe4q-cai.icp0.io/) | [🎛️](https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=xp6tn-piaaa-aaaah-qqe4q-cai) | ✅ Production |
| **OHMS Economics** | `tetse-piaaa-aaaao-qkeyq-cai` | [🔗](https://tetse-piaaa-aaaao-qkeyq-cai.icp0.io/) | [🎛️](https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=tetse-piaaa-aaaao-qkeyq-cai) | ✅ Production |
| **OHMS Model Repository** | `3aes4-xyaaa-aaaal-qsryq-cai` | [🔗](https://3aes4-xyaaa-aaaal-qsryq-cai.icp0.io/) | [🎛️](https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=3aes4-xyaaa-aaaal-qsryq-cai) | ✅ Production |
| **OHMS Platform UI** | `xg5yr-zaaaa-aaaah-qqe5a-cai` | [🔗](https://xg5yr-zaaaa-aaaah-qqe5a-cai.icp0.io/) | [🎛️](https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=xg5yr-zaaaa-aaaah-qqe5a-cai) | ✅ Production |
| **OHMS Marketing Website** | `rjeaj-jyaaa-aaaau-abyka-cai` | [🔗](https://rjeaj-jyaaa-aaaau-abyka-cai.icp0.io/) | [🎛️](https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=rjeaj-jyaaa-aaaau-abyka-cai) | ✅ Production |

### NOVAQ Compression Engine
- **Status**: ✅ Complete - Open Source CLI Tool
- **Repository**: [ohms-adaptq](https://github.com/OHMS-DeAI/ohms-adaptq)
- **Installation**: `cargo install --git https://github.com/OHMS-DeAI/ohms-adaptq.git`

---

## System Architecture Diagram

```mermaid
%%{init: {
  'theme': 'base',
  'themeVariables': {
    'background': '#e5e5e5',
    'primaryColor': '#e5e5e5',
    'sectionBkgColor': '#e5e5e5',
    'altSectionBkgColor': '#cccccc',
    'primaryTextColor': '#000000',
    'primaryBorderColor': '#ffffff',
    'lineColor': '#dc3545',
    'lineWidth': '3px',
    'edgeStyle': 'solid',
    'fontFamily': 'Arial',
    'fontSize': '13px',
    'fontWeight': 'bold'
  }
}}%%

graph TB

%% -- LINK STYLE FOR ALL EDGES --
linkStyle default stroke:#dc3545,stroke-width:3px,fill:none

%% -- LAYERS --
subgraph "Admin Layer"
    AdminUI["Admin Interface"]
    AdminAuth["Admin Authentication"]
    NOVAQProcessor["NOVAQ Processor"]
    ModelValidator["Model Validator"]
    ICPDeployer["ICP Deployer"]
end

subgraph "User Layer"
    UserUI["User Interface"]
    UserAuth["User Authentication"]
    InstructionInput["Instruction Input"]
    AgentSpinner["Agent Spinner"]
    AgentDashboard["Agent Dashboard"]
end

subgraph "Authentication & User Management"
    IIv2["Internet Identity v2"]
    PrincipalAuth["Principal Authentication"]
    SessionManager["Session Manager"]
end

subgraph "Model Repository (ICP) - Admin Controlled"
    ICPCanister["ICP Canisters"]
    ModelStorage["Model Storage"]
    ModelAPI["Model APIs"]
    AccessControl["Access Control"]
end

subgraph "Agent Generation System - User Driven"
    InstructionAnalyzer["Instruction Analyzer"]
    ModelSelector["Model Selector"]
    AgentFactory["Agent Factory"]
    AutonomousAgents["Autonomous Agents"]
end

subgraph "Execution Engine"
    AutonomousOrchestrator["Autonomous Orchestrator"]
    ParallelExecutor["Parallel Executor"]
    SelfCoordinator["Self Coordinator"]
    PerformanceMonitor["Performance Monitor"]
end

subgraph "Economics & Control"
    RateLimiter["Rate Limiter"]
    TokenTracker["Token Tracker"]
    UsageMonitor["Usage Monitor"]
    BillingEngine["Billing Engine"]
    MonthlyQuotas["Monthly Quotas"]
end

subgraph "Monitoring & Analytics"
    PlatformMonitor["Platform Monitor"]
    UserAnalytics["User Analytics"]
    AdminAnalytics["Admin Analytics"]
    AlertSystem["Alert System"]
end

%% -- FLOWS --

AdminUI -->|Auth| AdminAuth
AdminAuth -->|Verify| IIv2
AdminUI -->|Process| NOVAQProcessor
NOVAQProcessor -->|Validate| ModelValidator
ModelValidator -->|Deploy| ICPDeployer
ICPDeployer -->|Store| ICPCanister
ICPCanister -->|Manage| ModelStorage
ICPCanister -->|Expose| ModelAPI
ICPCanister -->|Control| AccessControl

UserUI -->|Auth| UserAuth
UserAuth -->|Verify| IIv2
IIv2 -->|Principal| PrincipalAuth
UserUI -->|Input| InstructionInput
InstructionInput -->|Analyze| InstructionAnalyzer
InstructionAnalyzer -->|Select| ModelSelector
ModelSelector -->|Query| ModelAPI
ModelSelector -->|Create| AgentFactory
AgentFactory -->|Deploy| AutonomousAgents

AutonomousAgents -->|Orchestrate| AutonomousOrchestrator
AutonomousOrchestrator -->|Execute| ParallelExecutor
ParallelExecutor -->|Coordinate| SelfCoordinator
SelfCoordinator -->|Monitor| PerformanceMonitor

PrincipalAuth -->|Limit| RateLimiter
RateLimiter -->|Track| TokenTracker
TokenTracker -->|Monitor| UsageMonitor
UsageMonitor -->|Bill| BillingEngine
BillingEngine -->|Quota| MonthlyQuotas

PerformanceMonitor -->|Report| PlatformMonitor
PlatformMonitor -->|Analyze| UserAnalytics
PlatformMonitor -->|Admin Data| AdminAnalytics
AdminAnalytics -->|Display| AdminUI
UserAnalytics -->|Display| UserUI
PlatformMonitor -->|Alert| AlertSystem

MonthlyQuotas -->|Enforce| AgentFactory
MonthlyQuotas -->|Limit| InstructionAnalyzer
AlertSystem -->|Notify| AdminUI
UsageMonitor -->|Update| UserUI

%% -- NODE COLORING (Class Definitions) --

classDef default fill:#e5e5e5,stroke:#ffffff,stroke-width:1px,color:#000000,font-weight:bold
classDef adminLayer fill:#1976d2,stroke:#ffffff,stroke-width:1px,color:#ffffff,font-weight:bold
classDef userLayer fill:#388e3c,stroke:#ffffff,stroke-width:1px,color:#ffffff,font-weight:bold
classDef authLayer fill:#f57c00,stroke:#ffffff,stroke-width:1px,color:#ffffff,font-weight:bold
classDef modelLayer fill:#7b1fa2,stroke:#ffffff,stroke-width:1px,color:#ffffff,font-weight:bold
classDef agentLayer fill:#d32f2f,stroke:#ffffff,stroke-width:1px,color:#ffffff,font-weight:bold
classDef execLayer fill:#00796b,stroke:#ffffff,stroke-width:1px,color:#ffffff,font-weight:bold
classDef econLayer fill:#e91e63,stroke:#ffffff,stroke-width:1px,color:#ffffff,font-weight:bold
classDef monitorLayer fill:#689f38,stroke:#ffffff,stroke-width:1px,color:#ffffff,font-weight:bold

class AdminUI,AdminAuth,NOVAQProcessor,ModelValidator,ICPDeployer adminLayer
class UserUI,UserAuth,InstructionInput,AgentSpinner,AgentDashboard userLayer
class IIv2,PrincipalAuth,SessionManager authLayer
class ICPCanister,ModelStorage,ModelAPI,AccessControl modelLayer
class InstructionAnalyzer,ModelSelector,AgentFactory,AutonomousAgents agentLayer
class AutonomousOrchestrator,ParallelExecutor,SelfCoordinator,PerformanceMonitor execLayer
class RateLimiter,TokenTracker,UsageMonitor,BillingEngine,MonthlyQuotas econLayer
class PlatformMonitor,UserAnalytics,AdminAnalytics,AlertSystem monitorLayer

```

## Real II v2 Principal Authentication Flow

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'primaryColor': '#f8f9fa', 'primaryTextColor': '#000000', 'primaryBorderColor': '#ffffff', 'lineColor': '#dc3545', 'lineWidth': '4px', 'sectionBkgColor': '#f8f9fa', 'altSectionBkgColor': '#e9ecef', 'secondaryColor': '#6c757d', 'tertiaryColor': '#495057', 'fontFamily': 'Arial', 'fontSize': '12px', 'fontWeight': 'bold'}}}%%
flowchart TD
    Start(["<b>User Visits OHMS</b>"]) --> IIv2Auth["<b>Internet Identity v2 Authentication</b>"]
    
    IIv2Auth --> AuthChoice{"<b>Authentication Method</b>"}
    
    AuthChoice -->|"<b>Passkey</b>"| PasskeyFlow["<b>Passkey Authentication</b>"]
    AuthChoice -->|"<b>WebAuthn</b>"| WebAuthnFlow["<b>WebAuthn Authentication</b>"]
    
    PasskeyFlow --> PrincipalExtraction["<b>Extract Real Principal ID</b>"]
    WebAuthnFlow --> PrincipalExtraction
    
    PrincipalExtraction --> SubscriptionCheck{"<b>Active Subscription?</b>"}
    
    SubscriptionCheck -->|"<b>No</b>"| PaymentFlow["<b>Subscription Payment Flow</b>"]
    SubscriptionCheck -->|"<b>Yes</b>"| Dashboard["<b>OHMS Dashboard</b>"]
    
    PaymentFlow --> SelectPlan["<b>Select Subscription Plan</b>"]
    SelectPlan --> ICPPayment["<b>ICP Payment Processing</b>"]
    ICPPayment --> CyclesAllocation["<b>Cycles Allocation</b>"]
    CyclesAllocation --> Dashboard
    
    Dashboard --> AgentCreation["<b>Create Autonomous Agents</b>"]
    AgentCreation --> UsageTracking["<b>Track Usage & Cycles</b>"]
    UsageTracking --> BillingCycle["<b>Monthly Billing Cycle</b>"]
    BillingCycle --> AutoRenewal["<b>Auto-renewal Process</b>"]
    AutoRenewal --> Dashboard

    classDef default fill:#f8f9fa,stroke:#ffffff,stroke-width:1px,color:#000000,font-weight:bold
    classDef startEnd fill:#388e3c,stroke:#ffffff,stroke-width:1px,color:#ffffff,font-weight:bold
    classDef process fill:#1976d2,stroke:#ffffff,stroke-width:1px,color:#ffffff,font-weight:bold
    classDef decision fill:#f57c00,stroke:#ffffff,stroke-width:1px,color:#ffffff,font-weight:bold
    classDef payment fill:#7b1fa2,stroke:#ffffff,stroke-width:1px,color:#ffffff,font-weight:bold
    
    class Start,Dashboard startEnd
    class IIv2Auth,PasskeyFlow,WebAuthnFlow,PrincipalExtraction,AgentCreation,UsageTracking,BillingCycle,AutoRenewal process
    class AuthChoice,SubscriptionCheck decision
    class PaymentFlow,SelectPlan,ICPPayment,CyclesAllocation payment
    OHMSIntegration --> SubscriptionManagement[Subscription Management]
    OHMSIntegration --> UsageTracking[Usage Tracking]
```

## Real Principal Authentication Flow

```mermaid
%%{init: {
  'theme': 'base',
  'themeVariables': {
    'primaryColor': '#4a90e2',
    'primaryTextColor': '#000000',
    'primaryBorderColor': '#2c5aa0',
    'lineColor': '#1565c0',
    'sectionBkgColor': '#f8f9fa',
    'altSectionBkgColor': '#e9ecef',
    'secondaryColor': '#6c757d',
    'tertiaryColor': '#495057',
    'fontFamily': 'Arial',
    'fontSize': '12px',
    'fontWeight': 'bold'
  }
}}%%

sequenceDiagram
    participant User as "User"
    participant OHMS as "OHMS"
    participant IIv2 as "IIv2"
    participant ICPLedger as "ICPLedger"
    
    User->>+OHMS: Visit OHMS Platform
    OHMS->>+IIv2: Redirect to II v2 Authentication
    IIv2-->>-User: Show Authentication Options
    Note over IIv2: Continue with Passkey or WebAuthn
    
    User->>+IIv2: Select Authentication Method
    IIv2-->>User: Complete Authentication
    User->>IIv2: Authenticate with Chosen Method
    IIv2->>IIv2: Verify & Create Anchor
    IIv2-->>-OHMS: Return Real II v2 Principal ID
    
    OHMS->>OHMS: Establish User Session
    
    OHMS-->>User: Check Subscription Status
    User->>OHMS: No active subscription
    
    OHMS-->>User: Show subscription options
    User->>OHMS: Select subscription tier
    
    OHMS->>+ICPLedger: Process ICP Payment
    ICPLedger-->>-OHMS: Payment Confirmed
    
    OHMS->>OHMS: Activate subscription
    OHMS-->>-User: Subscription active - start creating agents

```

## System Workflow Diagram

```mermaid
%%{init: {
  'theme': 'base',
  'themeVariables': {
    'primaryColor': '#f8f9fa',
    'primaryTextColor': '#000000',
    'primaryBorderColor': '#1565c0',
    'lineColor': '#1565c0',
    'secondaryColor': '#e3f2fd',
    'tertiaryColor': '#f3e5f5',
    'fontFamily': 'Arial',
    'fontSize': '12px',
    'fontWeight': 'bold'
  }
}}%%

sequenceDiagram
    participant Admin as "Admin"
    participant AdminUI as "AdminUI"
    participant APQ as "APQ Processor"
    participant ICP as "ICP Canister"
    participant User as "User"
    participant UserUI as "UserUI"
    participant IIv2 as "Internet Identity v2"
    participant Stripe as "Stripe Payment"
    participant Agent as "Agent Factory"
    participant Autonomous as "Autonomous Agents"
    participant Billing as "Billing"
    participant Monitor as "Monitor"
    
    Note over Admin, ICP: Admin Model Management Phase
    Admin->>+AdminUI: Upload LLM/SLM Model
    AdminUI->>+APQ: Process with APQ Optimization
    APQ->>APQ: Apply 1000x Compression
    APQ->>APQ: Validate Performance
    APQ->>+ICP: Deploy Optimized Model
    ICP->>ICP: Store in Canister
    ICP->>ICP: Generate User APIs
    ICP-->>-AdminUI: Model Available for Users
    
    Note over User, Autonomous: User Authentication & Payment Phase
    User->>+UserUI: Access Platform
    UserUI->>+IIv2: Authenticate with Google
    IIv2-->>-UserUI: Authentication Success
    Note over IIv2: Continue with Google or Passkey
    
    UserUI->>+Stripe: Check Subscription Status
    Stripe-->>-UserUI: No Active Subscription
    UserUI-->>User: Show Subscription Options
    User->>UserUI: Select Subscription Tier
    UserUI->>+Stripe: Process Payment
    Stripe-->>-UserUI: Payment Success
    UserUI->>Billing: Activate Subscription
    
    Note over User, Autonomous: User Agent Spinning Phase
    User->>UserUI: Provide Instructions/Prompts
    UserUI->>+Billing: Check Subscription & Quotas
    Billing-->>-UserUI: Quota Available
    UserUI->>+Agent: Analyze Instructions
    Agent->>+ICP: Query Available Models
    ICP-->>-Agent: Return Suitable Models
    Agent->>Agent: Create Specialized Agents
    Agent-->>-Autonomous: Deploy Autonomous Agents
    
    Note over Autonomous, Monitor: Autonomous Execution Phase
    Autonomous->>Autonomous: Execute Based on Instructions
    Autonomous->>Autonomous: Self-Coordinate Tasks
    Autonomous->>Autonomous: Parallel Processing
    Autonomous->>+Monitor: Report Performance
    Monitor->>+Billing: Update Usage
    Billing->>Billing: Track Tokens & Inference
    Billing-->>-UserUI: Usage Update
    
    Note over User, Monitor: Ongoing Operations
    User->>UserUI: Monitor Agent Performance
    UserUI->>+Autonomous: Request Status
    Autonomous-->>-UserUI: Autonomous Performance Data
    Monitor-->>AdminUI: Platform Analytics
    Monitor-->>-Billing: Billing Updates

```

## User Workflow Diagram

```mermaid
%%{init: {
  "theme": "base",
  "themeVariables": {
    "background": "#e5e5e5",
    "primaryTextColor": "#000000",
    "fontFamily": "Arial",
    "fontSize": "13px",
    "fontWeight": "bold",
    "lineColor": "#dc3545",
    "lineWidth": "3px"
  }
}}%%

flowchart TD

    linkStyle default stroke:#dc3545,stroke-width:3px

    Start([User Starts Session])

    subgraph "Authentication"
        Login[Connect Internet Identity v2]
        GoogleAuth[Google OAuth Flow]
        AuthCheck{Authentication Valid?}
        AuthFail[Authentication Failed]
    end

    subgraph "Subscription Verification"
        SubCheck{Subscription Active?}
        SubExpired[Subscription Expired]
        PaymentFlow[Stripe Payment Process]
        SelectPlan[Select Subscription Plan]
        QuotaCheck{Monthly Quota Available?}
        QuotaExceeded[Monthly Agent Limit Reached]
    end

    subgraph "Agent Creation from Instructions"
        Dashboard[User Dashboard]
        ProvideInstructions[Provide Instructions/Prompts]
        AnalyzeInstructions[System Analyzes Instructions]
        SelectModels[System Selects Models]
        ConfigureAgents[Configure Agent Count]
        SpinAgents[Spin Up Autonomous Agents]
        AgentsReady[Agents Ready & Autonomous]
    end

    subgraph "Agent Monitoring"
        MonitorAgents[Monitor Agent Performance]
        ViewResults[View Agent Results]
        CheckUsage[Check Token/Inference Usage]
        ManageAgents[Manage Active Agents]
    end

    subgraph "Account Management"
        ViewSubscription[View Subscription Details]
        ViewBilling[View Billing & Usage]
        UpgradePlan[Upgrade Subscription]
        ManageSettings[Manage Settings]
    end

    %% Flow Connections
    Start --> Login
    Login --> GoogleAuth
    GoogleAuth --> AuthCheck
    AuthCheck -->|Yes| SubCheck
    AuthCheck -->|No| AuthFail
    AuthFail --> Login

    SubCheck -->|Active| QuotaCheck
    SubCheck -->|Expired| SubExpired
    SubExpired --> SelectPlan
    SelectPlan --> PaymentFlow
    PaymentFlow --> Dashboard

    QuotaCheck -->|Available| Dashboard
    QuotaCheck -->|Exceeded| QuotaExceeded
    QuotaExceeded --> UpgradePlan

    Dashboard --> ProvideInstructions
    Dashboard --> MonitorAgents
    Dashboard --> ViewSubscription

    ProvideInstructions --> AnalyzeInstructions
    AnalyzeInstructions --> SelectModels
    SelectModels --> ConfigureAgents
    ConfigureAgents --> SpinAgents
    SpinAgents --> AgentsReady

    AgentsReady --> MonitorAgents
    MonitorAgents --> ViewResults
    MonitorAgents --> CheckUsage
    MonitorAgents --> ManageAgents

    ViewSubscription --> ViewBilling
    ViewBilling --> UpgradePlan
    ViewBilling --> ManageSettings
    UpgradePlan --> PaymentFlow

    ViewResults --> Dashboard
    CheckUsage --> Dashboard
    ManageSettings --> Dashboard

```

## Admin Workflow Diagram

```mermaid
flowchart TD
    AdminStart([Admin Login])
    
    subgraph "Admin Authentication"
        AdminAuth[Admin II v2 Authentication]
        AdminAuthCheck{Admin Rights Valid?}
        AdminAccessDenied[Access Denied]
    end
    
    subgraph "Admin Dashboard"
        AdminDash[Admin Dashboard]
        ModelManagement[Model Management]
        UserMonitoring[User Monitoring]
        PlatformConfig[Platform Configuration]
        SystemAnalytics[System Analytics]
        PlatformControl[Platform Control]
    end
    
    subgraph "Model Management Operations"
        UploadModel[Upload LLM/SLM Model]
        APQProcessing[APQ Processing & Optimization]
        ValidateModel[Validate Compressed Model]
        DeployToICP[Deploy to ICP Canister]
        ConfigureAccess[Configure User Access]
        ManageModelLibrary[Manage Model Library]
    end
    
    subgraph "User Monitoring & Management"
        ViewAllUsers[View All Users]
        UserAgentActivity[Monitor User Agent Activity]
        SubscriptionMonitoring[Monitor Subscriptions]
        UsageAnalytics[User Usage Analytics]
        QuotaManagement[Manage User Quotas]
        UserSupport[User Support & Issues]
    end
    
    subgraph "Platform Configuration"
        ConfigSubscriptionTiers[Configure Subscription Tiers]
        SetRateLimits[Set Rate Limits]
        ManageTokenQuotas[Manage Token Quotas]
        ConfigureMonthlyLimits[Configure Monthly Agent Limits]
        SecuritySettings[Security Settings]
        SystemParameters[System Parameters]
    end
    
    subgraph "System Analytics & Control"
        RealTimeMonitoring[Real-time System Monitoring]
        PlatformPerformance[Platform Performance Analytics]
        RevenueAnalytics[Revenue & Billing Analytics]
        UserBehaviorAnalytics[User Behavior Analytics]
        SystemHealth[System Health Monitoring]
        EmergencyControls[Emergency Controls]
    end
    
    AdminStart --> AdminAuth
    AdminAuth --> AdminAuthCheck
    AdminAuthCheck -->|Valid| AdminDash
    AdminAuthCheck -->|Invalid| AdminAccessDenied
    AdminAccessDenied --> AdminAuth
    
    AdminDash --> ModelManagement
    AdminDash --> UserMonitoring
    AdminDash --> PlatformConfig
    AdminDash --> SystemAnalytics
    AdminDash --> PlatformControl
    
    ModelManagement --> UploadModel
    UploadModel --> APQProcessing
    APQProcessing --> ValidateModel
    ValidateModel --> DeployToICP
    DeployToICP --> ConfigureAccess
    ConfigureAccess --> ManageModelLibrary
    
    UserMonitoring --> ViewAllUsers
    ViewAllUsers --> UserAgentActivity
    UserAgentActivity --> SubscriptionMonitoring
    SubscriptionMonitoring --> UsageAnalytics
    UsageAnalytics --> QuotaManagement
    QuotaManagement --> UserSupport
    
    PlatformConfig --> ConfigSubscriptionTiers
    ConfigSubscriptionTiers --> SetRateLimits
    SetRateLimits --> ManageTokenQuotas
    ManageTokenQuotas --> ConfigureMonthlyLimits
    ConfigureMonthlyLimits --> SecuritySettings
    SecuritySettings --> SystemParameters
    
    SystemAnalytics --> RealTimeMonitoring
    RealTimeMonitoring --> PlatformPerformance
    PlatformPerformance --> RevenueAnalytics
    RevenueAnalytics --> UserBehaviorAnalytics
    UserBehaviorAnalytics --> SystemHealth
    SystemHealth --> EmergencyControls
    
    %% Return flows
    ManageModelLibrary --> AdminDash
    UserSupport --> AdminDash
    SystemParameters --> AdminDash
    EmergencyControls --> AdminDash
```

## Agent Spinning & Autonomous Operation Flow

```mermaid
sequenceDiagram
    participant User
    participant UI as User Interface
    participant IIv2 as Internet Identity v2
    participant Analyzer as Instruction Analyzer
    participant Selector as Model Selector
    participant Factory as Agent Factory
    participant Agent1 as Autonomous Agent 1
    participant Agent2 as Autonomous Agent 2
    participant Agent3 as Autonomous Agent 3
    participant Coord as Self-Coordinator
    participant Monitor as Performance Monitor
    
    User->>UI: "Create coding agents for Python development and testing"
    UI->>IIv2: Verify Authentication
    IIv2-->>UI: Authentication Valid
    UI->>Analyzer: Analyze Instructions
    Analyzer->>Analyzer: Parse: "coding", "Python", "development", "testing"
    Analyzer->>Selector: Request suitable models
    Selector->>Selector: Find Python-capable models
    Selector-->>Factory: Models + Capabilities mapping
    
    Factory->>Factory: Create 3 specialized agents
    Factory->>Agent1: Deploy as "Python Developer"
    Factory->>Agent2: Deploy as "Code Reviewer" 
    Factory->>Agent3: Deploy as "Test Generator"
    
    Note over Agent1, Agent3: Autonomous Operation Begins
    Agent1->>Agent1: Start autonomous development tasks
    Agent2->>Agent2: Start autonomous code review
    Agent3->>Agent3: Start autonomous test generation
    
    Agent1->>Coord: Share development progress
    Agent2->>Coord: Share review findings
    Agent3->>Coord: Share test results
    Coord->>Coord: Coordinate collaborative work
    
    Agent1->>Monitor: Report performance metrics
    Agent2->>Monitor: Report performance metrics
    Agent3->>Monitor: Report performance metrics
    Monitor-->>UI: Combined autonomous performance
    
    Note over User, Monitor: Ongoing Autonomous Operation
    UI-->>User: "Agents working autonomously on Python tasks"
```

## Subscription Tiers & Quotas Diagram

```mermaid
graph LR
    subgraph "Subscription Tiers"
        Basic[Basic - $29/month]
        Pro[Pro - $99/month]
        Enterprise[Enterprise - $299/month]
    end
    
    subgraph "Basic Tier Limits"
        B1[Max Agents: 5]
        B2[Monthly Agent Creations: 10]
        B3[Tokens: 100,000]
        B4[Inference Rate: Standard]
    end
    
    subgraph "Pro Tier Limits"
        P1[Max Agents: 25]
        P2[Monthly Agent Creations: 50]
        P3[Tokens: 500,000]
        P4[Inference Rate: Priority]
    end
    
    subgraph "Enterprise Tier Limits"
        E1[Max Agents: 100]
        E2[Monthly Agent Creations: 200]
        E3[Tokens: 2,000,000]
        E4[Inference Rate: Premium]
    end
    
    Basic --> B1
    Basic --> B2
    Basic --> B3
    Basic --> B4
    
    Pro --> P1
    Pro --> P2
    Pro --> P3
    Pro --> P4
    
    Enterprise --> E1
    Enterprise --> E2
    Enterprise --> E3
    Enterprise --> E4
```

## Technology Stack Overview (Admin-User Separated)

```mermaid
graph LR
    subgraph "Admin Frontend"
        AdminReact[React Admin Interface]
        APQDashboard[APQ Processing Dashboard]
        ModelManager[Model Management UI]
        UserMonitor[User Monitoring UI]
    end
    
    subgraph "User Frontend"
        UserReact[React User Interface]
        InstructionUI[Instruction Input UI]
        AgentDashboard[Agent Dashboard]
        UsageMonitor[Usage Monitoring UI]
    end
    
    subgraph "Authentication Layer"
        IIv2Auth[Internet Identity v2]
        GoogleOAuth[Google OAuth Integration]
        RoleBasedAuth[Role-Based Authorization]
        AdminAuth[Admin Authentication]
        UserAuth[User Authentication]
    end
    
    subgraph "Payment Layer"
        StripeService[Stripe Payment Service]
        MarketDataAPI[Market Data API]
        ICPConversion[ICP Conversion Service]
        SubscriptionManager[Subscription Manager]
    end
    
    subgraph "Backend Services"
        AdminAPI[Admin APIs]
        UserAPI[User APIs]
        APQService[APQ Processing Service]
        AgentService[Agent Spinning Service]
        BillingService[Billing Service]
    end
    
    subgraph "ICP Infrastructure"
        AdminCanisters[Admin Canisters]
        ModelCanisters[Model Storage Canisters]
        AgentCanisters[Agent Execution Canisters]
        BillingCanisters[Billing Canisters]
    end
    
    subgraph "Data Layer"
        AdminDB[(Admin Database)]
        ModelDB[(Model Repository)]
        UserDB[(User Database)]
        AgentDB[(Agent Database)]
        BillingDB[(Billing Database)]
    end
    
    AdminReact --> AdminAPI
    UserReact --> UserAPI
    IIv2Auth --> GoogleOAuth
    GoogleOAuth --> RoleBasedAuth
    RoleBasedAuth --> AdminAuth
    RoleBasedAuth --> UserAuth
    
    StripeService --> MarketDataAPI
    MarketDataAPI --> ICPConversion
    ICPConversion --> SubscriptionManager
    
    AdminAPI --> APQService
    UserAPI --> AgentService
    APQService --> AdminCanisters
    AgentService --> AgentCanisters
    BillingService --> BillingCanisters
    
    AdminCanisters --> ModelCanisters
    ModelCanisters --> AgentCanisters
    
    AdminAPI --> AdminDB
    APQService --> ModelDB
    UserAPI --> UserDB
    AgentService --> AgentDB
    BillingService --> BillingDB
```

This comprehensive architecture separates admin model management from user agent spinning, with clear subscription-based quotas and autonomous agent operations, powered by Internet Identity v2 + Stripe integration.