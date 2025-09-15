'use client'

import { motion } from 'framer-motion'
import { STATS } from '@/lib/constants'

export default function StatsSection() {
  return (
    <section className="section-padding bg-gradient-to-b from-background-darker to-background-dark">
      <div className="container-wide">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6 }}
          viewport={{ once: true }}
          className="text-center mb-20"
        >
          <h2 className="heading-secondary">
            Trusted by Developers
            <span className="gradient-text-primary"> Worldwide</span>
          </h2>
          <p className="text-body-large mt-6">
            Join a growing community of developers and organizations building the future of AI
          </p>
        </motion.div>

        <div className="grid grid-cols-2 lg:grid-cols-4 gap-8 lg:gap-10">
          {STATS.map((stat, index) => (
            <motion.div
              key={stat.label}
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.6, delay: index * 0.1 }}
              viewport={{ once: true }}
              className="text-center"
            >
              <div className="stat-card">
                <div className="stat-value">
                  {stat.value}
                </div>
                <div className="stat-label">
                  {stat.label}
                </div>
              </div>
            </motion.div>
          ))}
        </div>

        {/* Social Proof */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6, delay: 0.4 }}
          viewport={{ once: true }}
          className="mt-16 text-center"
        >
          <p className="text-text-muted mb-8">
            Trusted by leading organizations and developers
          </p>
          <div className="flex flex-wrap justify-center items-center gap-8 opacity-60">
            {/* Placeholder for company logos */}
            <div className="text-text-muted font-semibold">Company A</div>
            <div className="text-text-muted font-semibold">Company B</div>
            <div className="text-text-muted font-semibold">Company C</div>
            <div className="text-text-muted font-semibold">Company D</div>
            <div className="text-text-muted font-semibold">Company E</div>
          </div>
        </motion.div>
      </div>
    </section>
  )
}
