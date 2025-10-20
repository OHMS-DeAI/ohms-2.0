'use client'

import { motion } from 'framer-motion'
import {
  Brain,
  Zap,
  Shield,
  Cpu,
  Users,
  CheckCircle,
  ExternalLink,
  Book,
  Play
} from 'lucide-react'
import Card from '@/components/ui/Card'
import Button from '@/components/ui/Button'
import CTASection from '@/components/sections/CTASection'
import { FEATURES, OHMS_APP_URL } from '@/lib/constants'

const featureDetails = [
  {
    ...FEATURES[0],
    details: [
      'Natural-language instruction builder with reusable blueprints',
      'Prompt libraries and contextual memory settings per agent',
      'Tool attachments for APIs, webhooks, and knowledge bases',
      'Policy guardrails and review steps baked into the workflow',
      'Team workspaces for sharing and versioning agent templates'
    ],
    benefits: [
      'Launch new agents in minutes without writing code',
      'Guarantee consistent outputs across teams and use cases',
      'Rapidly experiment by cloning and tweaking existing flows',
      'Ensure agents inherit the right governance policies',
      'Accelerate onboarding with ready-to-use instruction packs'
    ],
    useCase: 'Customer success teams ship a triage agent by describing intake questions and escalation policies—no developers required.',
    visualContent: {
      title: 'Instruction Engine',
      description: 'Compose agent behaviour from natural-language blueprints and guardrails',
      stats: [
        { label: 'Blueprint Library', value: '120+' },
        { label: 'Avg. Setup Time', value: '5 min' },
        { label: 'Reusable Templates', value: '40+' }
      ]
    }
  },
  {
    ...FEATURES[1],
    details: [
      'Immutable state and receipts persisted directly on ICP canisters',
      'Global replication across subnets for unstoppable availability',
      'Cycle-based economics with predictable on-chain costs',
      'Native Internet Identity support for seamless authentication',
      'Deterministic execution model for auditable outcomes'
    ],
    benefits: [
      'Prove every action with tamper-evident audit trails',
      'Deliver services without depending on centralized cloud',
      'Comply with regulatory requirements through verifiable logs',
      'Scale globally with ICP’s replicated compute environment',
      'Run 24/7 with transparent cost controls'
    ],
    useCase: 'Compliance teams deploy investigative agents that must retain immutable trails for regulators directly on-chain.',
    visualContent: {
      title: 'On-Chain Backbone',
      description: 'Harness the Internet Computer for transparent, verifiable agent coordination',
      stats: [
        { label: 'Replica Nodes', value: '1000+' },
        { label: 'Finality Time', value: '<2s' },
        { label: 'Uptime', value: '99.9%' }
      ]
    }
  },
  {
    ...FEATURES[2],
    details: [
      'Out-of-the-box connectors for OpenAI-compatible APIs and enterprise LLMs',
      'Secure API key vault with rotation policies and usage alerts',
      'Request signing, retry logic, and response validation inside the coordinator',
      'Provider profiles with latency, cost, and jurisdiction metadata',
      'Fallback routing and load-balancing across approved endpoints'
    ],
    benefits: [
      'Bring your own LLM vendor without rewriting workflows',
      'Meet regional compliance by pinning requests to approved providers',
      'Track provider-level spend with on-chain receipts',
      'Maintain uptime via automatic failover across configured endpoints',
      'Standardize prompt contracts even with heterogeneous APIs'
    ],
    useCase: 'Enterprises split workload across Azure OpenAI (for PII) and Anthropic (for research) with policy-based routing and auditing.',
    visualContent: {
      title: 'LLM Outcall Mesh',
      description: 'Bridge on-chain agents with the optimal external LLM for each task',
      stats: [
        { label: 'Integrated Providers', value: '10+' },
        { label: 'Latency Reduction', value: '35%' },
        { label: 'Key Rotations Automated', value: '100%' }
      ]
    }
  },
  {
    ...FEATURES[3],
    details: [
      'Quota enforcement per workspace, agent, and tool',
      'Token metering with granular receipt generation',
      'Usage dashboards and alerts for spend management',
      'Billing exports compatible with finance and ERP systems',
      'Multi-tenant safeguards with isolation between plans'
    ],
    benefits: [
      'Eliminate surprise invoices with live usage visibility',
      'Align pricing to the value agents deliver in production',
      'Segment customers across consumer, pro, and enterprise tiers',
      'Streamline reporting with automated, auditable receipts',
      'Scale monetization without building custom billing infrastructure'
    ],
    useCase: 'SaaS platforms embed OHMS agents and rely on native billing to track and recharge customer usage automatically.',
    visualContent: {
      title: 'Transparent Economics',
      description: 'Meter every action and enforce plan limits directly inside canisters',
      stats: [
        { label: 'Receipts Generated', value: '1k+' },
        { label: 'Quota Accuracy', value: '100%' },
        { label: 'Supported Plans', value: '5' }
      ]
    }
  },
  {
    ...FEATURES[4],
    details: [
      'Dependency graphs and role definitions for specialized agents',
      'Event-driven coordination bus for hand-offs and escalations',
      'Shared memory spaces with scoped access controls',
      'Observability layer tracking each agent’s contribution',
      'Simulation tools to test networks before deployment'
    ],
    benefits: [
      'Run complex missions with planner, executor, and reviewer agents',
      'Minimize human oversight while retaining accountability',
      'Blend autonomous steps with human approvals when required',
      'Reuse successful networks across new business units',
      'Gain visibility into how tasks flow across the collective'
    ],
    useCase: 'Research teams coordinate analyst, summarizer, and reviewer agents to deliver daily market briefings automatically.',
    visualContent: {
      title: 'Coordinated Networks',
      description: 'Design and monitor sophisticated agent collectives with clear hand-offs',
      stats: [
        { label: 'Agents per Mission', value: '25' },
        { label: 'Task Completion Boost', value: '70%' },
        { label: 'Human Handoffs Reduced', value: '60%' }
      ]
    }
  },
  {
    ...FEATURES[5],
    details: [
      'Central registry for prompts, datasets, and tool integrations',
      'Approval workflows and change logs for every resource',
      'Environment isolation across dev, staging, and production',
      'Runtime policy engine governing what agents may call',
      'Comprehensive audit log with exportable evidence trails'
    ],
    benefits: [
      'Maintain guardrails as teams expand the agent catalog',
      'Reduce risk with mandatory peer review and approvals',
      'Guarantee only vetted tools reach production environments',
      'Respond to audits quickly with structured historical data',
      'Empower operators to improve agents without code changes'
    ],
    useCase: 'Fintech operators manage payment and compliance tools through the registry, ensuring every change is reviewed and versioned.',
    visualContent: {
      title: 'Operator Console',
      description: 'Control prompts, tools, and policies from a single governance hub',
      stats: [
        { label: 'Registry Entries', value: '500+' },
        { label: 'Policy Checks Automated', value: '120' },
        { label: 'Daily Audit Events', value: '5k+' }
      ]
    }
  }
]

const apiEndpoints = [
  { method: 'POST', path: '/agents', description: 'Create new AI agent' },
  { method: 'GET', path: '/agents/{id}', description: 'Get agent details' },
  { method: 'POST', path: '/workflows', description: 'Create agent workflow' },
  { method: 'POST', path: '/deploy', description: 'Deploy to ICP network' },
  { method: 'GET', path: '/metrics', description: 'Get performance metrics' },
  { method: 'POST', path: '/security', description: 'Configure security settings' }
]

export default function FeaturesPage() {
  return (
    <div className="min-h-screen">
      {/* Hero Section */}
      <section className="section-padding">
        <div className="container-medium content-center">
          <motion.h1
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8 }}
            className="heading-primary"
          >
            Powerful
            <span className="gradient-text-primary"> Features</span>
          </motion.h1>

          <motion.p
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8, delay: 0.2 }}
            className="text-body-large mb-16"
          >
            Discover how OHMS blends natural-language agent creation, secure HTTPS LLM outcalls,
            and Internet Computer deployment for verifiable autonomous workflows.
          </motion.p>

          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8, delay: 0.4 }}
            className="flex flex-col sm:flex-row gap-6 justify-center"
          >
            <button
              onClick={() => window.open(OHMS_APP_URL, '_blank')}
              className="btn-primary gap-3"
            >
              <span>Try OHMS Now</span>
              <ExternalLink size={20} />
            </button>
            <button
              className="btn-secondary"
              onClick={() => document.getElementById('platform-interface')?.scrollIntoView({ behavior: 'smooth' })}
            >
              View Platform Interface
            </button>
          </motion.div>
        </div>
      </section>

      {/* Feature Details */}
      <section className="section-padding">
        <div className="container-wide">
          <div className="space-y-20">
            {featureDetails.map((feature, index) => {
              const Icon = [Brain, Zap, Shield, Cpu, CheckCircle, Users][index]

              return (
                <motion.div
                  key={feature.title}
                  initial={{ opacity: 0, y: 20 }}
                  whileInView={{ opacity: 1, y: 0 }}
                  transition={{ duration: 0.6 }}
                  viewport={{ once: true }}
                  className="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center"
                >
                  <div className={index % 2 === 1 ? 'lg:order-2' : ''}>
                    <div className="flex items-center space-x-4 mb-6">
                      <div className="w-12 h-12 bg-primary/20 rounded-lg flex items-center justify-center">
                        <Icon className="w-6 h-6 text-primary" />
                      </div>
                      <h2 className="text-3xl font-bold text-text-primary">
                        {feature.title}
                      </h2>
                    </div>

                    <p className="text-xl text-text-muted mb-8 leading-relaxed">
                      {feature.description}
                    </p>

                    <ul className="space-y-3 mb-8">
                      {feature.details.map((detail, idx) => (
                        <li key={idx} className="flex items-center space-x-3">
                          <CheckCircle className="w-5 h-5 text-green-400 flex-shrink-0" />
                          <span className="text-text-secondary">{detail}</span>
                        </li>
                      ))}
                    </ul>

                    <div className="bg-background-darker rounded-lg p-4 mb-6">
                      <h3 className="text-lg font-semibold text-text-primary mb-2">
                        Use Case
                      </h3>
                      <p className="text-text-muted">{feature.useCase}</p>
                    </div>

                    <div className="bg-gradient-to-br from-primary/10 to-secondary/10 rounded-lg p-4 mb-6">
                      <h3 className="text-lg font-semibold text-text-primary mb-2">
                        Key Benefits
                      </h3>
                      <ul className="space-y-2">
                        {feature.benefits.map((benefit, idx) => (
                          <li key={idx} className="flex items-center space-x-2">
                            <div className="w-2 h-2 bg-primary rounded-full"></div>
                            <span className="text-text-secondary">{benefit}</span>
                          </li>
                        ))}
                      </ul>
                    </div>
                  </div>

                  <div className={index % 2 === 1 ? 'lg:order-1' : ''}>
                    <Card className="p-6">
                      <div className="flex items-center space-x-2 mb-4">
                        <div className="w-5 h-5 bg-gradient-to-br from-primary to-secondary rounded"></div>
                        <span className="text-sm font-medium text-text-primary">{feature.visualContent.title}</span>
                      </div>
                      <p className="text-text-muted mb-4">{feature.visualContent.description}</p>
                      <div className="grid grid-cols-3 gap-4">
                        {feature.visualContent.stats.map((stat, idx) => (
                          <div key={idx} className="text-center">
                            <div className="text-2xl font-bold gradient-text-primary mb-1">{stat.value}</div>
                            <div className="text-xs text-text-muted">{stat.label}</div>
                          </div>
                        ))}
                      </div>
                    </Card>
                  </div>
                </motion.div>
              )
            })}
          </div>
        </div>
      </section>

      {/* Platform Interface */}
      <section id="platform-interface" className="section-padding bg-gradient-to-b from-background-darker to-background-dark">
        <div className="container-narrow content-center">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6 }}
            viewport={{ once: true }}
            className="text-center mb-16"
          >
            <h2 className="heading-secondary mb-4">
              Platform Interface
            </h2>
            <p className="text-xl text-text-muted max-w-2xl mx-auto">
              Simple web interface for creating and managing autonomous AI agents
            </p>
          </motion.div>

          <div className="grid grid-cols-1 lg:grid-cols-2 gap-8 lg:gap-12 mb-12">
            <Card className="p-6">
              <h3 className="text-xl font-semibold text-text-primary mb-4 flex items-center">
                <Book className="w-5 h-5 mr-2" />
                REST API Reference
              </h3>
              <p className="text-text-muted mb-4">
                Complete API documentation with examples, schemas, and interactive playground.
              </p>
              <Button variant="outline" size="sm">
                View Full Docs <ExternalLink className="w-4 h-4 ml-1" />
              </Button>
            </Card>

            <Card className="p-6">
              <h3 className="text-xl font-semibold text-text-primary mb-4 flex items-center">
                <Play className="w-5 h-5 mr-2" />
                Interactive Playground
              </h3>
              <p className="text-text-muted mb-4">
                Test API endpoints in real-time with our interactive developer playground.
              </p>
              <Button variant="outline" size="sm">
                Open Playground <ExternalLink className="w-4 h-4 ml-1" />
              </Button>
            </Card>
          </div>

          <Card className="overflow-hidden">
            <div className="p-6 border-b border-glass-border">
              <h3 className="text-xl font-semibold text-text-primary">API Endpoints</h3>
            </div>
            <div className="overflow-x-auto">
              <table className="w-full">
                <thead className="bg-background-darker">
                  <tr>
                    <th className="px-6 py-4 text-left text-text-primary font-semibold">Method</th>
                    <th className="px-6 py-4 text-left text-text-primary font-semibold">Endpoint</th>
                    <th className="px-6 py-4 text-left text-text-primary font-semibold">Description</th>
                  </tr>
                </thead>
                <tbody>
                  {apiEndpoints.map((endpoint, index) => (
                    <tr key={index} className="border-b border-glass-border">
                      <td className="px-6 py-4">
                        <span className={`px-2 py-1 rounded text-xs font-medium ${
                          endpoint.method === 'GET' ? 'bg-blue-500/20 text-blue-400' :
                          endpoint.method === 'POST' ? 'bg-green-500/20 text-green-400' :
                          'bg-yellow-500/20 text-yellow-400'
                        }`}>
                          {endpoint.method}
                        </span>
                      </td>
                      <td className="px-6 py-4 font-mono text-sm text-text-primary">
                        {endpoint.path}
                      </td>
                      <td className="px-6 py-4 text-text-secondary">
                        {endpoint.description}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </Card>
        </div>
      </section>

      {/* Agent Creation Process */}
      <section className="section-padding">
        <div className="container-narrow content-center">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6 }}
            viewport={{ once: true }}
            className="text-center mb-16"
          >
            <h2 className="heading-secondary mb-4">
              Simple Agent Creation Process
            </h2>
            <p className="text-xl text-text-muted max-w-2xl mx-auto">
              Create powerful autonomous agents in just a few steps with our intuitive platform
            </p>
          </motion.div>

          <div className="grid-responsive-4">
            {[
              { step: '1', title: 'Describe Your Need', description: 'Tell OHMS what kind of agent you want in natural language' },
              { step: '2', title: 'Choose Your Tier', description: 'Select Basic, Pro, or Enterprise tier based on your needs' },
              { step: '3', title: 'Agent Generation', description: 'OHMS assembles your agent with verified prompts and secure LLM connectors' },
              { step: '4', title: 'Deploy & Monitor', description: 'Your agent runs autonomously on the Internet Computer' }
            ].map((process, index) => (
              <motion.div
                key={process.step}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.6, delay: index * 0.1 }}
                viewport={{ once: true }}
              >
                <div className="feature-card">
                  <div className="w-12 h-12 bg-gradient-to-r from-primary to-secondary rounded-full flex items-center justify-center mx-auto mb-4 text-white font-bold text-lg">
                    {process.step}
                  </div>
                  <h3 className="text-lg font-semibold text-text-primary mb-2">
                    {process.title}
                  </h3>
                  <p className="text-sm text-text-muted leading-relaxed">
                    {process.description}
                  </p>
                </div>
              </motion.div>
            ))}
          </div>
        </div>
      </section>

      {/* Getting Started */}
      <section className="section-padding bg-gradient-to-br from-background-darker to-background-dark">
        <div className="container-medium content-center">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6 }}
            viewport={{ once: true }}
          >
            <Card className="p-8 md:p-12">
              <h2 className="heading-secondary mb-4">
                Ready to Get Started?
              </h2>
              <p className="text-xl text-text-muted mb-8 leading-relaxed">
                Jump into the OHMS ecosystem with our comprehensive getting started guide,
                tutorials, and community resources.
              </p>

              <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 lg:gap-8 mb-8">
                <div className="text-center">
                  <div className="w-16 h-16 bg-primary/20 rounded-2xl flex items-center justify-center mx-auto mb-4">
                    <Book className="w-8 h-8 text-primary" />
                  </div>
                  <h3 className="text-lg font-semibold text-text-primary mb-2">
                    Documentation
                  </h3>
                  <p className="text-text-muted text-sm">
                    Step-by-step guides and API references
                  </p>
                </div>
                <div className="text-center">
                  <div className="w-16 h-16 bg-secondary/20 rounded-2xl flex items-center justify-center mx-auto mb-4">
                    <Play className="w-8 h-8 text-secondary" />
                  </div>
                  <h3 className="text-lg font-semibold text-text-primary mb-2">
                    Tutorials
                  </h3>
                  <p className="text-text-muted text-sm">
                    Video tutorials and hands-on examples
                  </p>
                </div>
                <div className="text-center">
                  <div className="w-16 h-16 bg-accent/20 rounded-2xl flex items-center justify-center mx-auto mb-4">
                    <Users className="w-8 h-8 text-accent" />
                  </div>
                  <h3 className="text-lg font-semibold text-text-primary mb-2">
                    Community
                  </h3>
                  <p className="text-text-muted text-sm">
                    Join our developer community and forum
                  </p>
                </div>
              </div>

              <Button
                onClick={() => window.open(OHMS_APP_URL, '_blank')}
                external
                size="lg"
              >
                Start Building Today
              </Button>
            </Card>
          </motion.div>
        </div>
      </section>

      {/* CTA Section */}
      <CTASection
        title="Experience OHMS Features"
        subtitle="Ready to explore all that OHMS has to offer? Create your first agent and see the power of decentralized AI orchestration."
        primaryButtonText="Launch OHMS"
        secondaryButtonText="View Examples"
      />
    </div>
  )
}
