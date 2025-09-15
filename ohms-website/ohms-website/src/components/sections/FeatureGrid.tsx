'use client'

import { motion } from 'framer-motion'
import {
  Brain,
  Zap,
  Shield,
  Cpu,
  Database,
  Users,
  ArrowRight
} from 'lucide-react'
import { FEATURES } from '@/lib/constants'

const iconMap = {
  brain: Brain,
  zap: Zap,
  shield: Shield,
  cpu: Cpu,
  database: Database,
  users: Users,
}

export default function FeatureGrid() {
  return (
    <section className="bg-gradient-to-b from-background-dark to-background-darker">
      <div className="container-wide content-center">
        {/* Section Header */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6 }}
          viewport={{ once: true }}
          className="text-center mb-20"
        >
          <h2 className="heading-secondary text-center">
            Powerful Features for
            <span className="gradient-text-primary"> Modern AI Orchestration</span>
          </h2>
          <p className="text-body-large mt-6">
            Everything you need to create, deploy, and manage sophisticated AI agent networks
            with enterprise-grade security and performance.
          </p>
        </motion.div>

        {/* Features Grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8 lg:gap-10">
          {FEATURES.map((feature, index) => {
            const Icon = iconMap[feature.icon as keyof typeof iconMap]

            return (
              <motion.div
                key={feature.title}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.6, delay: index * 0.1 }}
                viewport={{ once: true }}
              >
                <div className="feature-card h-full">
                  <div className="w-16 h-16 bg-gradient-to-br from-primary/20 to-secondary/20 rounded-2xl flex items-center justify-center mb-6 mx-auto">
                    <Icon className="w-8 h-8 text-primary" />
                  </div>
                  <h3 className="heading-tertiary text-center">
                    {feature.title}
                  </h3>
                  <p className="text-body text-center">
                    {feature.description}
                  </p>
                </div>
              </motion.div>
            )
          })}
        </div>

        {/* CTA Section */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6 }}
          viewport={{ once: true }}
          className="text-center mt-20"
        >
          <div className="glass-card max-w-4xl mx-auto p-12 bg-gradient-to-br from-primary/10 via-secondary/5 to-accent/10">
            <h3 className="heading-secondary mb-6">
              Ready to Create Your First Agent?
            </h3>
            <p className="text-body-large mb-10">
              Transform your business with autonomous AI agents created from simple instructions.
            </p>
            <div className="flex flex-col sm:flex-row gap-6 justify-center items-center">
              <button 
                onClick={() => window.open('https://xg5yr-zaaaa-aaaah-qqe5a-cai.icp0.io/', '_blank')}
                className="btn-primary gap-3"
              >
                <span>Launch OHMS Platform</span>
                <ArrowRight size={20} />
              </button>
              <button className="btn-secondary">
                <span>View Pricing Plans</span>
              </button>
            </div>
            
            {/* Trust Indicators */}
            <div className="flex flex-wrap justify-center items-center gap-8 mt-10 pt-8 border-t border-white/10">
              <div className="flex items-center gap-2 text-sm text-text-muted">
                <div className="w-2 h-2 bg-green-400 rounded-full"></div>
                <span>No credit card required</span>
              </div>
              <div className="flex items-center gap-2 text-sm text-text-muted">
                <div className="w-2 h-2 bg-blue-400 rounded-full"></div>
                <span>Start in 2 minutes</span>
              </div>
              <div className="flex items-center gap-2 text-sm text-text-muted">
                <div className="w-2 h-2 bg-purple-400 rounded-full"></div>
                <span>Cancel anytime</span>
              </div>
            </div>
          </div>
        </motion.div>
      </div>
    </section>
  )
}
