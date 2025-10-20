'use client'

import { motion } from 'framer-motion'
import { useRouter } from 'next/navigation'
import { ArrowRight } from 'lucide-react'
import { OHMS_APP_URL } from '@/lib/constants'
import Button from '@/components/ui/Button'

interface CTAButtonConfig {
  label: string
  href: string
  external?: boolean
}

interface CTASectionProps {
  title?: string
  subtitle?: string
  primaryButtonText?: string
  secondaryButtonText?: string
  showSecondaryButton?: boolean
  primaryCta?: CTAButtonConfig
  secondaryCta?: CTAButtonConfig | null
}

export default function CTASection({
  title = 'Ready to Transform Your AI Workflow?',
  subtitle = 'Join thousands of developers building the future of autonomous AI agents. Start your journey with OHMS today.',
  primaryButtonText = 'Launch OHMS',
  secondaryButtonText = 'View Documentation',
  showSecondaryButton = true,
  primaryCta,
  secondaryCta,
}: CTASectionProps) {
  const router = useRouter()

  const defaultPrimary: CTAButtonConfig = {
    label: primaryButtonText,
    href: OHMS_APP_URL,
    external: true,
  }

  const defaultSecondary: CTAButtonConfig | null =
    showSecondaryButton ? { label: secondaryButtonText, href: '' } : null

  const resolvedPrimary = primaryCta ?? defaultPrimary
  const resolvedSecondary =
    secondaryCta === null ? null : secondaryCta ?? defaultSecondary

  const handleNavigate = (cta: CTAButtonConfig) => {
    if (!cta.href) return
    if (cta.external ?? cta.href.startsWith('http')) {
      window.open(cta.href, '_blank')
      return
    }
    router.push(cta.href)
  }

  return (
    <section className="py-20">
      <div className="container-medium content-center">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6 }}
          viewport={{ once: true }}
          className="glass-card p-8 md:p-12 text-center"
        >
          <motion.h2
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.1 }}
            viewport={{ once: true }}
            className="heading-secondary mb-4"
          >
            {title}
          </motion.h2>

          <motion.p
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.2 }}
            viewport={{ once: true }}
            className="text-xl text-text-muted mb-8 leading-relaxed"
          >
            {subtitle}
          </motion.p>

          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.3 }}
            viewport={{ once: true }}
            className="flex flex-col sm:flex-row gap-4 justify-center items-center"
          >
            <Button
              onClick={() => handleNavigate(resolvedPrimary)}
              size="lg"
              external={resolvedPrimary.external}
              className="bg-gradient-to-r from-primary to-secondary hover:from-primary/80 hover:to-secondary/80"
            >
              {resolvedPrimary.label}
            </Button>

            {resolvedSecondary && (
              <Button
                variant="outline"
                size="lg"
                className="group"
                onClick={
                  resolvedSecondary.href
                    ? () => handleNavigate(resolvedSecondary)
                    : undefined
                }
              >
                {resolvedSecondary.label}
                <ArrowRight
                  size={16}
                  className="ml-2 group-hover:translate-x-1 transition-transform"
                />
              </Button>
            )}
          </motion.div>

          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.4 }}
            viewport={{ once: true }}
            className="mt-12 grid grid-cols-1 md:grid-cols-3 gap-6"
          >
            <div className="flex items-center space-x-3">
              <div className="w-8 h-8 bg-primary/20 rounded-full flex items-center justify-center">
                <span className="text-primary text-sm">✓</span>
              </div>
              <span className="text-text-secondary">Free tier available</span>
            </div>
            <div className="flex items-center space-x-3">
              <div className="w-8 h-8 bg-secondary/20 rounded-full flex items-center justify-center">
                <span className="text-secondary text-sm">✓</span>
              </div>
              <span className="text-text-secondary">No credit card required</span>
            </div>
            <div className="flex items-center space-x-3">
              <div className="w-8 h-8 bg-accent/20 rounded-full flex items-center justify-center">
                <span className="text-accent text-sm">✓</span>
              </div>
              <span className="text-text-secondary">Cancel anytime</span>
            </div>
          </motion.div>
        </motion.div>
      </div>
    </section>
  )
}
