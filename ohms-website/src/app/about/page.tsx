'use client'

import { motion } from 'framer-motion'
import { Cpu, Shield, Globe, Users, Zap, Target } from 'lucide-react'
import Card from '@/components/ui/Card'
import CTASection from '@/components/sections/CTASection'
import ParticlesBackground from '@/components/effects/ParticlesBackground'

const values = [
  {
    icon: Target,
    title: 'Innovation First',
    description: 'We push the boundaries of AI technology to create truly autonomous agent systems that learn and adapt.'
  },
  {
    icon: Shield,
    title: 'Security by Design',
    description: 'Every component is built with enterprise-grade security, ensuring your data remains private and protected.'
  },
  {
    icon: Globe,
    title: 'Decentralized Future',
    description: 'We believe in a decentralized web where users control their data and AI agents operate transparently.'
  },
  {
    icon: Users,
    title: 'Community Driven',
    description: 'Our platform grows stronger with every developer and organization that joins our ecosystem.'
  },
]

const technologies = [
  {
    icon: Cpu,
    title: 'Internet Computer Protocol',
    description: 'Built on ICP for true decentralization, scalability, and cost-effective computation.'
  },
  {
    icon: Zap,
    title: 'Advanced AI Models',
    description: 'Integration with state-of-the-art language models and machine learning frameworks.'
  },
  {
    icon: Shield,
    title: 'End-to-End Encryption',
    description: 'All data transmission and storage is protected with military-grade encryption protocols.'
  },
]

export default function AboutPage() {
  return (
    <div className="min-h-screen">
      {/* Hero Section */}
      <section className="relative min-h-[60vh] flex items-center justify-center overflow-hidden">
        <ParticlesBackground />
        <div className="relative z-10 container-medium content-center py-20">
          <motion.h1
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8 }}
            className="text-4xl md:text-6xl font-bold mb-6"
          >
            Building the Future of
            <span className="gradient-text"> Autonomous AI</span>
          </motion.h1>

          <motion.p
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8, delay: 0.2 }}
            className="text-xl md:text-2xl text-text-muted mb-8 leading-relaxed"
          >
            OHMS is revolutionizing how we interact with AI through decentralized,
            secure, and intelligent agent orchestration.
          </motion.p>
        </div>
      </section>

      {/* Mission Section */}
      <section className="section-padding">
        <div className="container-medium content-center">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6 }}
            viewport={{ once: true }}
            className="text-center mb-16"
          >
            <h2 className="heading-secondary mb-6">
              Our Mission
            </h2>
            <p className="text-xl text-text-muted leading-relaxed mb-8">
              To democratize access to powerful AI agents while maintaining the highest standards
              of security, privacy, and decentralization. We envision a world where AI agents
              work seamlessly together to solve complex problems, create value, and enhance
              human potential.
            </p>
          </motion.div>

          <div className="grid grid-cols-1 lg:grid-cols-2 gap-8 lg:gap-12 mb-20">
            <motion.div
              initial={{ opacity: 0, x: -20 }}
              whileInView={{ opacity: 1, x: 0 }}
              transition={{ duration: 0.6 }}
              viewport={{ once: true }}
            >
              <Card className="h-full p-8">
                <h3 className="text-2xl font-bold text-text-primary mb-4">What We Do</h3>
                <p className="text-text-muted leading-relaxed mb-4">
                  We provide a comprehensive platform for creating, deploying, and managing
                  AI agents on the Internet Computer Protocol. Our orchestration engine
                  enables complex multi-agent workflows with intelligent task distribution
                  and conflict resolution.
                </p>
                <ul className="space-y-2 text-text-secondary">
                  <li>• Decentralized AI agent hosting</li>
                  <li>• Intelligent orchestration algorithms</li>
                  <li>• Enterprise-grade security features</li>
                  <li>• Real-time performance monitoring</li>
                </ul>
              </Card>
            </motion.div>

            <motion.div
              initial={{ opacity: 0, x: 20 }}
              whileInView={{ opacity: 1, x: 0 }}
              transition={{ duration: 0.6 }}
              viewport={{ once: true }}
            >
              <Card className="h-full p-8">
                <h3 className="text-2xl font-bold text-text-primary mb-4">Why It Matters</h3>
                <p className="text-text-muted leading-relaxed mb-4">
                  Traditional AI systems are centralized, expensive, and limited by single points
                  of failure. OHMS changes this paradigm by providing a decentralized,
                  scalable, and secure alternative that puts users in control.
                </p>
                <ul className="space-y-2 text-text-secondary">
                  <li>• True decentralization and ownership</li>
                  <li>• Cost-effective at any scale</li>
                  <li>• Enhanced privacy and security</li>
                  <li>• Interoperable agent ecosystems</li>
                </ul>
              </Card>
            </motion.div>
          </div>
        </div>
      </section>

      {/* Values Section */}
      <section className="section-padding bg-gradient-to-br from-background-darker to-background-dark">
        <div className="container-wide content-center">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6 }}
            viewport={{ once: true }}
            className="text-center mb-16"
          >
            <h2 className="heading-secondary mb-4">
              Our Core Values
            </h2>
            <p className="text-xl text-text-muted max-w-2xl mx-auto">
              The principles that guide everything we build and every decision we make
            </p>
          </motion.div>

          <div className="grid-responsive-4">
            {values.map((value, index) => {
              const Icon = value.icon
              return (
                <motion.div
                  key={value.title}
                  initial={{ opacity: 0, y: 20 }}
                  whileInView={{ opacity: 1, y: 0 }}
                  transition={{ duration: 0.6, delay: index * 0.1 }}
                  viewport={{ once: true }}
                >
                  <Card variant="feature" className="text-center">
                    <div className="w-16 h-16 bg-primary/20 rounded-2xl flex items-center justify-center mb-6 mx-auto">
                      <Icon className="w-8 h-8 text-primary" />
                    </div>
                    <h3 className="text-xl font-semibold text-text-primary mb-4">
                      {value.title}
                    </h3>
                    <p className="text-text-muted leading-relaxed">
                      {value.description}
                    </p>
                  </Card>
                </motion.div>
              )
            })}
          </div>
        </div>
      </section>

      {/* Technology Section */}
      <section className="section-padding">
        <div className="container-wide content-center">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6 }}
            viewport={{ once: true }}
            className="text-center mb-16"
          >
            <h2 className="heading-secondary mb-4">
              Technology Foundation
            </h2>
            <p className="text-xl text-text-muted max-w-2xl mx-auto">
              Built on cutting-edge technologies that ensure performance, security, and scalability
            </p>
          </motion.div>

          <div className="grid-responsive">
            {technologies.map((tech, index) => {
              const Icon = tech.icon
              return (
                <motion.div
                  key={tech.title}
                  initial={{ opacity: 0, y: 20 }}
                  whileInView={{ opacity: 1, y: 0 }}
                  transition={{ duration: 0.6, delay: index * 0.2 }}
                  viewport={{ once: true }}
                >
                  <Card className="p-8 text-center">
                    <div className="w-20 h-20 bg-secondary/20 rounded-2xl flex items-center justify-center mb-6 mx-auto">
                      <Icon className="w-10 h-10 text-secondary" />
                    </div>
                    <h3 className="text-2xl font-bold text-text-primary mb-4">
                      {tech.title}
                    </h3>
                    <p className="text-text-muted leading-relaxed">
                      {tech.description}
                    </p>
                  </Card>
                </motion.div>
              )
            })}
          </div>
        </div>
      </section>

      {/* Roadmap Section */}
      <section className="section-padding bg-gradient-to-br from-background-darker to-background-dark">
        <div className="container-medium content-center">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6 }}
            viewport={{ once: true }}
            className="text-center mb-16"
          >
            <h2 className="heading-secondary mb-4">
              Development Roadmap
            </h2>
            <p className="text-xl text-text-muted">
              Our vision for the future of decentralized AI agent orchestration
            </p>
          </motion.div>

          <div className="space-y-8">
            {[
              {
                phase: 'Phase 1',
                title: 'Core Platform',
                status: 'Completed',
                description: 'Basic agent creation, deployment, and orchestration capabilities'
              },
              {
                phase: 'Phase 2',
                title: 'Advanced Features',
                status: 'In Progress',
                description: 'Multi-agent collaboration, advanced security features, and performance optimizations'
              },
              {
                phase: 'Phase 3',
                title: 'Enterprise Solutions',
                status: 'Upcoming',
                description: 'Large-scale deployments, custom integrations, and enterprise support'
              },
              {
                phase: 'Phase 4',
                title: 'Ecosystem Expansion',
                status: 'Future',
                description: 'Third-party integrations, marketplace, and community governance'
              }
            ].map((item, index) => (
              <motion.div
                key={item.phase}
                initial={{ opacity: 0, x: -20 }}
                whileInView={{ opacity: 1, x: 0 }}
                transition={{ duration: 0.6, delay: index * 0.1 }}
                viewport={{ once: true }}
                className="flex items-start space-x-6"
              >
                <div className="flex-shrink-0 w-16 h-16 bg-primary/20 rounded-2xl flex items-center justify-center">
                  <span className="text-primary font-bold">{item.phase}</span>
                </div>
                <div className="flex-1">
                  <div className="flex items-center space-x-3 mb-2">
                    <h3 className="text-xl font-semibold text-text-primary">
                      {item.title}
                    </h3>
                    <span className={`px-3 py-1 rounded-full text-sm font-medium ${
                      item.status === 'Completed' ? 'bg-green-500/20 text-green-400' :
                      item.status === 'In Progress' ? 'bg-yellow-500/20 text-yellow-400' :
                      item.status === 'Upcoming' ? 'bg-blue-500/20 text-blue-400' :
                      'bg-gray-500/20 text-gray-400'
                    }`}>
                      {item.status}
                    </span>
                  </div>
                  <p className="text-text-muted leading-relaxed">
                    {item.description}
                  </p>
                </div>
              </motion.div>
            ))}
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <CTASection
        title="Join the OHMS Revolution"
        subtitle="Be part of the future of decentralized AI agent orchestration. Start building with OHMS today."
        primaryButtonText="Get Started"
        secondaryButtonText="View Roadmap"
      />
    </div>
  )
}
