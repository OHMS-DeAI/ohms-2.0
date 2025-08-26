export const NAVIGATION_LINKS = [
  { name: 'Home', href: '/' },
  { name: 'Features', href: '/features' },
  { name: 'About', href: '/about' },
  { name: 'Pricing', href: '/pricing' },
  { name: 'Team', href: '/team' },
  { name: 'App', href: 'https://xg5yr-zaaaa-aaaah-qqe5a-cai.icp0.io/', external: true },
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
    description: 'Create autonomous AI agents from simple natural language instructions. No coding required - just describe what you need and OHMS generates intelligent agents.'
  },
  {
    icon: 'zap',
    title: 'NOVAQ Model Compression',
    description: 'Revolutionary 93-100x AI model compression with 99%+ capability retention. Democratized access to advanced AI compression technology for everyone.'
  },
  {
    icon: 'shield',
    title: 'Internet Computer Protocol',
    description: 'Built on ICP blockchain for true decentralization, censorship resistance, and autonomous operation without traditional cloud dependencies.'
  },
  {
    icon: 'cpu',
    title: 'Subscription Economics',
    description: 'Transparent pricing with agent quotas, monthly creation limits, and usage tracking. Scale from Basic to Enterprise tiers as your needs grow.'
  },
  {
    icon: 'users',
    title: 'Autonomous Agent Networks',
    description: 'Self-coordinating multi-agent systems that work together intelligently, requiring minimal human intervention once deployed.'
  },
  {
    icon: 'database',
    title: 'Admin-Curated Quality',
    description: 'Professional model curation ensures reliability and performance. All models are validated and optimized before platform deployment.'
  },
]

export const STATS = [
  { label: 'Compression Ratio', value: '93-100x' },
  { label: 'Model Capability Retention', value: '99%+' },
  { label: 'ICP Blockchain Powered', value: '100%' },
  { label: 'Agent Creation Time', value: '2min' },
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
      'Standard NOVAQ models',
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
      'Premium NOVAQ models',
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
      'Custom NOVAQ models',
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

export const OHMS_APP_URL = 'https://xg5yr-zaaaa-aaaah-qqe5a-cai.icp0.io/' // OHMS Main Application URL

export const METADATA = {
  title: 'OHMS - Revolutionary Autonomous AI Agent Platform | Create Agents from Instructions',
  description: 'Transform your business with OHMS - the subscription-based autonomous AI agent platform. Create intelligent agents from natural language instructions, powered by NOVAQ compression technology and Internet Computer Protocol. 93-100x model compression with 99%+ capability retention.',
  keywords: 'AI agents, autonomous AI, agent platform, NOVAQ compression, Internet Computer, ICP, subscription AI, natural language agents, decentralized AI, agent orchestration, AI model compression, blockchain AI, smart agents, AI automation, agent creation platform',
  author: 'OHMS Team',
  ogImage: '/og-image.jpg',
  siteName: 'OHMS - Autonomous AI Agent Platform',
  siteUrl: 'https://xg5yr-zaaaa-aaaah-qqe5a-cai.icp0.io/',
  type: 'website',
  locale: 'en_US',
}
