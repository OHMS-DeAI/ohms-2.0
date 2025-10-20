export const NAVIGATION_LINKS = [
  { name: 'Home', href: '/' },
  { name: 'Features', href: '/features' },
  { name: 'About', href: '/about' },
  { name: 'Pricing', href: '/pricing' },
  { name: 'FAQ', href: '/faq' },
  { name: 'Team', href: '/team' },
  { name: 'App', href: 'https://wrh5a-oaaaa-aaaah-arlbq-cai.icp0.io/', external: true },
]

export const SOCIAL_LINKS = [
  { name: 'GitHub', href: 'https://github.com/ohms-org', icon: 'github' },
  { name: 'Twitter', href: 'https://twitter.com/ohms_org', icon: 'twitter' },
  { name: 'Discord', href: 'https://discord.gg/ohms', icon: 'message-circle' },
  { name: 'LinkedIn', href: 'https://linkedin.com/company/ohms-org', icon: 'linkedin' },
]

export const FEATURES = [
  {
    icon: 'brain',
    title: 'Instruction-Based Agent Creation',
    description: 'Create autonomous AI agents from natural language instructions with configurable policies, tools, and guardrails.'
  },
  {
    icon: 'shield',
    title: 'Internet Computer Protocol',
    description: 'Built on the Internet Computer for transparent, tamper-evident coordination, storage, and billing.'
  },
  {
    icon: 'zap',
    title: 'Secure LLM Outcalls',
    description: 'Invoke external LLM endpoints via hardened HTTPS outcalls so you can bring your own provider while staying fully on-chain.'
  },
  {
    icon: 'cpu',
    title: 'Subscription Economics',
    description: 'Transparent usage-based billing with quotas, receipts, and tiered plans enforced by canisters.'
  },
  {
    icon: 'users',
    title: 'Autonomous Agent Networks',
    description: 'Self-coordinating multi-agent systems that plan, act, and hand off tasks with minimal human oversight.'
  },
  {
    icon: 'database',
    title: 'Operator Controls',
    description: 'Central registry for prompts, tools, and integrations so teams can enforce guardrails and quality.'
  },
]

export const STATS = [
  { label: 'LLM Providers Supported', value: '10+' },
  { label: 'On-Chain Operations', value: '100%' },
  { label: 'Coordination Latency', value: '<1s' },
  { label: 'Agent Setup Time', value: '2min' },
]

export const PRICING_TIERS = [
  {
    name: 'Basic',
    price: '$29',
    period: 'month',
    description: 'Perfect for getting started with autonomous agents',
    features: [
      'Up to 5 concurrent agents',
      '10 monthly agent creations',
      '100K processing tokens',
      'Standard LLM connectors',
      'Community support',
      'Basic analytics dashboard',
      'ICP blockchain deployment'
    ],
    cta: 'Start Basic Plan',
    popular: false,
  },
  {
    name: 'Pro',
    price: '$99',
    period: 'month',
    description: 'Ideal for growing teams and businesses',
    features: [
      'Up to 25 concurrent agents',
      '50 monthly agent creations',
      '500K processing tokens',
      'Priority HTTPS provider routing',
      'Priority support',
      'Advanced analytics',
      'Custom agent templates',
      'API access'
    ],
    cta: 'Start Pro Plan',
    popular: true,
  },
  {
    name: 'Enterprise',
    price: '$299',
    period: 'month',
    description: 'For large-scale deployments and organizations',
    features: [
      'Up to 100 concurrent agents',
      '200 monthly agent creations',
      '2M processing tokens',
      'Custom provider integrations',
      '24/7 dedicated support',
      'Enterprise analytics',
      'White-label deployment',
      'SLA guarantee',
      'Custom integrations'
    ],
    cta: 'Contact Sales',
    popular: false,
  },
]

export const FOOTER_LINKS = {
  company: [
    { name: 'About', href: '/about' },
    { name: 'Team', href: '/team' },
    { name: 'Careers', href: '/careers' },
    { name: 'Press', href: '/press' },
  ],
  product: [
    { name: 'Features', href: '/features' },
    { name: 'Pricing', href: '/pricing' },
    { name: 'FAQ', href: '/faq' },
    { name: 'Documentation', href: '/docs' },
    { name: 'API Reference', href: '/api' },
  ],
  resources: [
    { name: 'Blog', href: '/blog' },
    { name: 'Help Center', href: '/help' },
    { name: 'Community', href: '/community' },
    { name: 'Status', href: '/status' },
  ],
  legal: [
    { name: 'Privacy Policy', href: '/privacy' },
    { name: 'Terms of Service', href: '/terms' },
    { name: 'Cookie Policy', href: '/cookies' },
    { name: 'GDPR', href: '/gdpr' },
  ],
}

export const OHMS_APP_URL = 'https://wrh5a-oaaaa-aaaah-arlbq-cai.icp0.io/' // OHMS Main Application URL

export const METADATA = {
  title: 'OHMS - Revolutionary Autonomous AI Agent Platform | Create Agents from Instructions',
  description: 'Transform your business with OHMS - the subscription-based autonomous AI agent platform. Create intelligent agents from natural language instructions, orchestrate them entirely on the Internet Computer, and connect to external LLM providers through secure HTTPS outcalls.',
  keywords: 'AI agents, autonomous AI, agent platform, HTTPS outcalls, Internet Computer, ICP, subscription AI, natural language agents, decentralized AI, agent orchestration, LLM integration, blockchain AI, smart agents, AI automation, agent creation platform',
  author: 'OHMS Team',
  ogImage: '/og-image.jpg',
  siteName: 'OHMS - Autonomous AI Agent Platform',
  siteUrl: 'https://weamn-piaaa-aaaah-arlca-cai.icp0.io/',
  type: 'website',
  locale: 'en_US',
}
