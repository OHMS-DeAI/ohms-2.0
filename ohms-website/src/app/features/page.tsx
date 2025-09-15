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
    ...FEATURES[0], // AI Agent Orchestration
    details: [
      'Intelligent task distribution across multiple agents',
      'Conflict resolution algorithms',
      'Real-time agent coordination',
      'Custom workflow orchestration',
      'Agent performance monitoring'
    ],
    benefits: [
      'Reduce human workload by 70%',
      'Complete complex tasks in minutes',
      'Ensure consistent quality across all outputs',
      'Scale operations without hiring more staff',
      '24/7 automated processing capabilities'
    ],
    useCase: 'Content creation pipeline where multiple AI agents collaborate to research, write, and edit articles.',
    visualContent: {
      title: 'Workflow Automation',
      description: 'See how OHMS orchestrates multiple AI agents working together seamlessly',
      stats: [
        { label: 'Tasks Automated', value: '500+' },
        { label: 'Time Saved', value: '80%' },
        { label: 'Error Reduction', value: '95%' }
      ]
    }
  },
  {
    ...FEATURES[1], // Decentralized Processing
    details: [
      'Internet Computer Protocol integration',
      'Distributed computing across nodes',
      'Cost-effective processing',
      'Global scalability',
      'Censorship resistance'
    ],
    benefits: [
      'Pay only for what you use',
      'No single point of failure',
      'Global network reliability',
      'Enhanced security and privacy',
      'Automatic scaling capabilities'
    ],
    useCase: 'Running AI processing workloads across a global network of nodes without centralized control.',
    visualContent: {
      title: 'Global Network',
      description: 'Leverage the power of decentralized computing with ICP blockchain',
      stats: [
        { label: 'Network Nodes', value: '1000+' },
        { label: 'Global Reach', value: '50+ Countries' },
        { label: 'Uptime', value: '99.9%' }
      ]
    }
  },
  {
    ...FEATURES[2], // Enterprise Security
    details: [
      'End-to-end encryption',
      'Multi-party computation',
      'Zero-knowledge proofs',
      'Secure key management',
      'Compliance certifications'
    ],
    benefits: [
      'Bank-level security standards',
      'GDPR and HIPAA compliance',
      'Zero-trust architecture',
      'Regular security audits',
      'Enterprise-grade encryption'
    ],
    useCase: 'Healthcare data analysis where sensitive patient information remains encrypted throughout processing.',
    visualContent: {
      title: 'Security First',
      description: 'Military-grade security protecting your most sensitive data',
      stats: [
        { label: 'Security Score', value: 'A+' },
        { label: 'Compliance', value: 'GDPR/HIPAA' },
        { label: 'Encryption', value: 'AES-256' }
      ]
    }
  },
  {
    ...FEATURES[3], // High Performance
    details: [
      'Sub-second response times',
      'Advanced caching mechanisms',
      'Parallel processing',
      'Auto-scaling infrastructure',
      'Performance optimization'
    ],
    benefits: [
      'Lightning-fast response times',
      'Handle millions of requests',
      'Intelligent resource allocation',
      'Automatic performance optimization',
      'Cost-effective scaling'
    ],
    useCase: 'Real-time customer support chatbot handling thousands of concurrent conversations.',
    visualContent: {
      title: 'Lightning Fast',
      description: 'Experience unmatched speed and performance with OHMS',
      stats: [
        { label: 'Response Time', value: '<100ms' },
        { label: 'Concurrent Users', value: '10M+' },
        { label: 'Uptime', value: '99.99%' }
      ]
    }
  },
  {
    ...FEATURES[4], // Admin-Curated Quality
    details: [
      'Professional model validation',
      'Performance optimization',
      'Security auditing',
      'Quality assurance testing',
      'Deployment certification'
    ],
    benefits: [
      'Expertly validated models',
      'Consistent high-quality outputs',
      'Regular performance updates',
      'Comprehensive testing suite',
      'Professional support included'
    ],
    useCase: 'Enterprise deployment where model quality and reliability are critical for business operations.',
    visualContent: {
      title: 'Quality Assured',
      description: 'Every model is thoroughly tested and validated by our experts',
      stats: [
        { label: 'Models Tested', value: '50+' },
        { label: 'Quality Score', value: '99.9%' },
        { label: 'Expert Reviews', value: '1000+' }
      ]
    }
  },
  {
    ...FEATURES[5], // Multi-Agent Support
    details: [
      'Complex agent networks',
      'Communication protocols',
      'Task dependencies',
      'Agent marketplace',
      'Cross-platform compatibility'
    ],
    benefits: [
      'Seamless agent collaboration',
      'Specialized agent roles',
      'Intelligent task routing',
      'Scalable network architecture',
      'Interoperability across platforms'
    ],
    useCase: 'Financial trading system where multiple AI agents analyze market data, make decisions, and execute trades.',
    visualContent: {
      title: 'Agent Ecosystem',
      description: 'Build powerful networks of specialized AI agents working together',
      stats: [
        { label: 'Agent Types', value: '25+' },
        { label: 'Integrations', value: '50+' },
        { label: 'Network Size', value: 'Unlimited' }
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
            Discover everything OHMS has to offer with revolutionary AI agent creation,
            NOVAQ compression technology, and Internet Computer deployment.
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
              { step: '3', title: 'Agent Generation', description: 'OHMS creates your agent using curated NOVAQ models' },
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
