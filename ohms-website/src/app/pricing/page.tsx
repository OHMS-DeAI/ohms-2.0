'use client'

import { motion } from 'framer-motion'
import { Check, X, HelpCircle } from 'lucide-react'
import PricingCard from '@/components/ui/PricingCard'
import Card from '@/components/ui/Card'
import Button from '@/components/ui/Button'
import CTASection from '@/components/sections/CTASection'
import { PRICING_TIERS, OHMS_APP_URL } from '@/lib/constants'

const faqs = [
  {
    question: 'What is included in the Free tier?',
    answer: 'The Free tier includes up to 100 agents, basic orchestration capabilities, community support, and access to core OHMS features. It\'s perfect for getting started and small projects.'
  },
  {
    question: 'Can I upgrade or downgrade my plan at any time?',
    answer: 'Yes, you can upgrade or downgrade your plan at any time. Changes take effect immediately, and we prorate billing accordingly. No hidden fees or long-term commitments.'
  },
  {
    question: 'What payment methods do you accept?',
    answer: 'We accept all major credit cards, PayPal, and cryptocurrency payments. All payments are processed securely through our PCI-compliant payment partners.'
  },
  {
    question: 'Is there a setup fee?',
    answer: 'No, there are no setup fees for any of our plans. You only pay the monthly or annual subscription fee, and that\'s it. No hidden costs or surprise charges.'
  },
  {
    question: 'Do you offer enterprise discounts?',
    answer: 'Yes, we offer custom pricing for enterprise customers with specific requirements. Contact our sales team to discuss your needs and get a tailored quote.'
  },
  {
    question: 'What kind of support do you provide?',
    answer: 'Free tier users get community support, Pro users get priority email support, and Enterprise customers get 24/7 dedicated support with SLA guarantees.'
  },
  {
    question: 'Can I cancel my subscription anytime?',
    answer: 'Absolutely. You can cancel your subscription at any time with no penalties. Your account will remain active until the end of your current billing period.'
  },
  {
    question: 'Do you offer refunds?',
    answer: 'We offer a 30-day money-back guarantee for all paid plans. If you\'re not satisfied with OHMS, we\'ll refund your payment, no questions asked.'
  }
]

const comparisonFeatures = [
  { name: 'Number of Agents', free: '1', basic: '5', pro: '25', enterprise: '100' },
  { name: 'Monthly Creations', free: '3', basic: '10', pro: '50', enterprise: '200' },
  { name: 'Token Limit', free: '10K', basic: '100K', pro: '500K', enterprise: '2M' },
  { name: 'API Access', free: false, basic: true, pro: true, enterprise: true },
  { name: 'Custom Integrations', free: false, basic: false, pro: true, enterprise: true },
  { name: 'Advanced Analytics', free: false, basic: true, pro: true, enterprise: true },
  { name: 'Priority Support', free: false, basic: true, pro: true, enterprise: true },
  { name: 'White-label Solution', free: false, basic: false, pro: false, enterprise: true },
  { name: '24/7 Support', free: false, basic: false, pro: false, enterprise: true },
  { name: 'SLA Guarantee', free: false, basic: false, pro: false, enterprise: true },
]

export default function PricingPage() {
  const handleCtaClick = (tierName: string) => {
    if (tierName === 'Enterprise' || tierName === 'Free') {
      // Open contact form or email for Enterprise, or just launch for Free
      if (tierName === 'Enterprise') {
        window.location.href = 'mailto:sales@ohms.org?subject=Enterprise Inquiry'
      } else {
        window.open(OHMS_APP_URL, '_blank')
      }
    } else {
      // Launch OHMS app for Basic and Pro tiers
      window.open(OHMS_APP_URL, '_blank')
    }
  }

  return (
    <div className="min-h-screen">
      {/* Hero Section */}
      <section className="section-padding">
        <div className="container-medium content-center">
          <motion.h1
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8 }}
            className="text-4xl md:text-6xl font-bold mb-6"
          >
            Transparent
            <span className="gradient-text"> Pricing</span>
          </motion.h1>

          <motion.p
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8, delay: 0.2 }}
            className="text-xl text-text-muted mb-12 leading-relaxed"
          >
            Choose the perfect plan for your needs. No hidden fees, no surprise charges.
            Start free and scale as you grow.
          </motion.p>

          {/* Billing Toggle */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8, delay: 0.4 }}
            className="flex items-center justify-center space-x-4 mb-12"
          >
            <span className="text-text-secondary">Monthly</span>
            <button className="relative w-14 h-7 bg-glass rounded-full transition-colors">
              <div className="w-5 h-5 bg-primary rounded-full transition-transform absolute top-1 left-1"></div>
            </button>
            <span className="text-text-primary font-medium">Yearly</span>
            <span className="text-sm text-green-400 bg-green-500/10 px-2 py-1 rounded-full">
              Save 20%
            </span>
          </motion.div>
        </div>
      </section>

      {/* Pricing Cards */}
      <section className="py-12">
        <div className="container-wide content-center">
          <div className="grid-responsive-4">
            {PRICING_TIERS.map((tier, index) => (
              <motion.div
                key={tier.name}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.6, delay: index * 0.1 }}
                viewport={{ once: true }}
              >
                <PricingCard
                  name={tier.name}
                  price={tier.price}
                  period={tier.period}
                  description={tier.description}
                  features={tier.features}
                  cta={tier.cta}
                  popular={tier.popular}
                  onCtaClick={() => handleCtaClick(tier.name)}
                />
              </motion.div>
            ))}
          </div>
        </div>
      </section>

      {/* Comparison Table */}
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
              Feature Comparison
            </h2>
            <p className="text-xl text-text-muted">
              See exactly what&apos;s included in each plan
            </p>
          </motion.div>

          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.2 }}
            viewport={{ once: true }}
          >
            <Card className="overflow-hidden">
              <div className="overflow-x-auto">
                <table className="w-full">
                  <thead>
                    <tr className="border-b border-glass-border">
                      <th className="text-left py-4 px-4 font-semibold text-text-primary">
                        Features
                      </th>
                      <th className="text-center py-4 px-4 font-semibold text-text-primary">
                        Free
                      </th>
                      <th className="text-center py-4 px-4 font-semibold text-text-primary">
                        Basic
                      </th>
                      <th className="text-center py-4 px-4 font-semibold text-text-primary">
                        Pro
                      </th>
                      <th className="text-center py-4 px-4 font-semibold text-text-primary">
                        Enterprise
                      </th>
                    </tr>
                  </thead>
                  <tbody>
                    {comparisonFeatures.map((feature) => (
                      <tr key={feature.name} className="border-b border-glass-border/50">
                        <td className="py-4 px-6 text-text-primary font-medium">
                          {feature.name}
                        </td>
                        <td className="py-4 px-6 text-center">
                          {typeof feature.free === 'boolean' ? (
                            feature.free ? (
                              <Check className="w-5 h-5 text-green-400 mx-auto" />
                            ) : (
                              <X className="w-5 h-5 text-red-400 mx-auto" />
                            )
                          ) : (
                            <span className="text-text-secondary">{feature.free}</span>
                          )}
                        </td>
                        <td className="py-4 px-4 text-center">
                          {typeof feature.basic === 'boolean' ? (
                            feature.basic ? (
                              <Check className="w-5 h-5 text-green-400 mx-auto" />
                            ) : (
                              <X className="w-5 h-5 text-red-400 mx-auto" />
                            )
                          ) : (
                            <span className="text-text-secondary">{feature.basic}</span>
                          )}
                        </td>
                        <td className="py-4 px-4 text-center">
                          {typeof feature.pro === 'boolean' ? (
                            feature.pro ? (
                              <Check className="w-5 h-5 text-green-400 mx-auto" />
                            ) : (
                              <X className="w-5 h-5 text-red-400 mx-auto" />
                            )
                          ) : (
                            <span className="text-text-secondary">{feature.pro}</span>
                          )}
                        </td>
                        <td className="py-4 px-4 text-center">
                          {typeof feature.enterprise === 'boolean' ? (
                            feature.enterprise ? (
                              <Check className="w-5 h-5 text-green-400 mx-auto" />
                            ) : (
                              <X className="w-5 h-5 text-red-400 mx-auto" />
                            )
                          ) : (
                            <span className="text-text-secondary">{feature.enterprise}</span>
                          )}
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </Card>
          </motion.div>
        </div>
      </section>

      {/* FAQ Section */}
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
              Frequently Asked Questions
            </h2>
            <p className="text-xl text-text-muted">
              Everything you need to know about OHMS pricing
            </p>
          </motion.div>

          <div className="space-y-6 mobile-spacing">
            {faqs.map((faq, index) => (
              <motion.div
                key={index}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.6, delay: index * 0.1 }}
                viewport={{ once: true }}
              >
                <Card className="p-6">
                  <div className="flex items-start space-x-4">
                    <div className="flex-shrink-0 w-8 h-8 bg-primary/20 rounded-lg flex items-center justify-center mt-1">
                      <HelpCircle className="w-4 h-4 text-primary" />
                    </div>
                    <div>
                      <h3 className="text-lg font-semibold text-text-primary mb-3">
                        {faq.question}
                      </h3>
                      <p className="text-text-muted leading-relaxed">
                        {faq.answer}
                      </p>
                    </div>
                  </div>
                </Card>
              </motion.div>
            ))}
          </div>
        </div>
      </section>

      {/* Contact Section */}
      <section className="section-padding">
        <div className="container-medium content-center">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6 }}
            viewport={{ once: true }}
          >
            <Card className="p-8 md:p-12">
              <h2 className="heading-secondary mb-4">
                Need Custom Pricing?
              </h2>
              <p className="text-xl text-text-muted mb-8 leading-relaxed">
                Have specific requirements or need a custom solution?
                Our team is here to help you find the perfect plan for your organization.
              </p>
              <div className="flex flex-col sm:flex-row gap-4 justify-center">
                <Button
                  onClick={() => window.location.href = 'mailto:sales@ohms.org'}
                  size="lg"
                >
                  Contact Sales
                </Button>
                <Button
                  variant="outline"
                  size="lg"
                  onClick={() => window.location.href = 'tel:+1-555-OHMS'}
                >
                  Call Us
                </Button>
              </div>
            </Card>
          </motion.div>
        </div>
      </section>

      {/* CTA Section */}
      <CTASection
        title="Ready to Get Started?"
        subtitle="Choose your plan and start building with OHMS today. No credit card required for the free tier."
        primaryButtonText="Start Free Trial"
      />
    </div>
  )
}
