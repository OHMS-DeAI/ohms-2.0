'use client'

import { motion } from 'framer-motion'
import Link from 'next/link'
import { ArrowRight, ExternalLink } from 'lucide-react'
import { OHMS_APP_URL } from '@/lib/constants'
import ParticlesBackground from '@/components/effects/ParticlesBackground'

export default function HeroSection() {
  const launchOHMS = () => {
    window.open(OHMS_APP_URL, '_blank')
  }

  return (
    <section className="relative min-h-screen flex items-center justify-center overflow-hidden">
      <ParticlesBackground />

      <div className="relative z-10 container-narrow content-center py-20">
        <div className="content-center">
          {/* Main Headline */}
          <motion.h1
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8 }}
            className="heading-primary text-center"
          >
            <span className="gradient-text-primary">Turn Instructions</span>
            <br />
            <span className="text-text-primary">Into Autonomous AI Agents</span>
          </motion.h1>

          {/* Subheadline */}
          <motion.p
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8, delay: 0.2 }}
            className="text-body-large mb-16"
          >
            Revolutionary autonomous AI agent platform powered by NOVAQ compression technology. 
            Transform natural language instructions into intelligent, self-coordinating agents on the Internet Computer.
          </motion.p>

          {/* CTA Buttons */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8, delay: 0.4 }}
            className="flex flex-col sm:flex-row gap-6 justify-center items-center mb-20"
          >
            <button
              onClick={launchOHMS}
              className="btn-primary gap-3"
            >
              <span>Launch OHMS</span>
              <ExternalLink size={20} />
            </button>

            <Link href="/features">
              <button className="btn-secondary gap-3">
                <span>View Features</span>
                <ArrowRight size={20} />
              </button>
            </Link>
          </motion.div>

          {/* Feature Highlights */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8, delay: 0.6 }}
            className="grid grid-cols-1 md:grid-cols-3 gap-8 lg:gap-10 max-w-6xl mx-auto"
          >
            <div className="feature-card hover:bg-primary/5 border border-primary/10">
              <div className="w-16 h-16 bg-gradient-to-br from-primary/20 to-primary/30 rounded-2xl flex items-center justify-center mb-6 mx-auto">
                <span className="text-3xl">🧠</span>
              </div>
              <h3 className="heading-tertiary">Natural Language to Agents</h3>
              <p className="text-body">Simply describe what you need and OHMS creates intelligent agents automatically - no coding required</p>
            </div>

            <div className="feature-card hover:bg-secondary/5 border border-secondary/10">
              <div className="w-16 h-16 bg-gradient-to-br from-secondary/20 to-secondary/30 rounded-2xl flex items-center justify-center mb-6 mx-auto">
                <span className="text-3xl">⚡</span>
              </div>
              <h3 className="heading-tertiary">NOVAQ Compression</h3>
              <p className="text-body">Revolutionary 93-100x AI model compression with 99%+ capability retention for everyone</p>
            </div>

            <div className="feature-card hover:bg-accent/5 border border-accent/10">
              <div className="w-16 h-16 bg-gradient-to-br from-accent/20 to-accent/30 rounded-2xl flex items-center justify-center mb-6 mx-auto">
                <span className="text-3xl">🔗</span>
              </div>
              <h3 className="heading-tertiary">Internet Computer</h3>
              <p className="text-body">Built on ICP blockchain for true decentralization and autonomous agent operation</p>
            </div>
          </motion.div>
        </div>
      </div>

      {/* Scroll Indicator */}
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ duration: 1, delay: 1 }}
        className="absolute bottom-8 left-1/2 transform -translate-x-1/2"
      >
        <div className="w-6 h-10 border-2 border-text-muted rounded-full flex justify-center">
          <motion.div
            animate={{ y: [0, 12, 0] }}
            transition={{ duration: 2, repeat: Infinity }}
            className="w-1 h-3 bg-primary rounded-full mt-2"
          />
        </div>
      </motion.div>
    </section>
  )
}
