'use client'

import { motion } from 'framer-motion'
import { HelpCircle, ShieldCheck, Zap, ExternalLink } from 'lucide-react'
import Card from '@/components/ui/Card'
import Button from '@/components/ui/Button'
import CTASection from '@/components/sections/CTASection'
import { OHMS_APP_URL } from '@/lib/constants'

const faqSections = [
  {
    title: 'Getting Started',
    icon: HelpCircle,
    description: 'Understand the fundamentals of building agents with OHMS.',
    items: [
      {
        question: 'What makes OHMS different from other agent platforms?',
        answer:
          'OHMS runs entirely on the Internet Computer, giving you verifiable execution, on-chain receipts, and direct ownership of your agent state. You describe intent in natural language and the platform assembles workflow-ready agents automatically.'
      },
      {
        question: 'Do I need to write code to create an agent?',
        answer:
          'No. You provide natural-language instructions, objectives, and guardrails. OHMS converts that input into a deployable multi-agent workflow with shared memory, policies, and tool access. Advanced users can extend agents with custom integrations through the registry.'
      },
      {
        question: 'How fast can I deploy my first agent?',
        answer:
          'Most teams launch a functional agent in under ten minutes. The blueprint library, reusable prompt packs, and guided onboarding get you from idea to deployment quickly.'
      }
    ]
  },
  {
    title: 'Architecture & Providers',
    icon: ShieldCheck,
    description: 'Learn how OHMS connects on-chain coordination with external LLMs.',
    items: [
      {
        question: 'How do HTTPS LLM outcalls work?',
        answer:
          'The coordinator canister issues signed HTTPS requests to approved LLM endpoints. Responses stream back into the agent network while receipts record cost, latency, and provider identity on-chain.'
      },
      {
        question: 'Can I bring my own LLM provider?',
        answer:
          'Yes. You can register OpenAI-compatible or custom enterprise endpoints, configure authentication, and set routing policies per workspace. Failover and latency-aware routing are handled automatically.'
      },
      {
        question: 'Where is my data stored?',
        answer:
          'Agent state, receipts, and policies persist inside ICP canisters. Only the prompt payloads you explicitly approve are sent to external LLMs, and each call is logged with hashed metadata for auditing.'
      }
    ]
  },
  {
    title: 'Billing & Plans',
    icon: Zap,
    description: 'See how subscriptions, quotas, and receipts operate.',
    items: [
      {
        question: 'How are usage quotas enforced?',
        answer:
          'Each plan defines concurrent agent limits, monthly creation caps, and processing token budgets. The economics canister meters every action, issues receipts, and blocks requests that exceed policy thresholds.'
      },
      {
        question: 'Can I mix providers across tiers?',
        answer:
          'Absolutely. You can assign different providers to agents within the same workspace. Costs are attributed to the initiating user or tenant, and you can export receipts for reconciliation.'
      },
      {
        question: 'Do you support enterprise procurement workflows?',
        answer:
          'Enterprise plans include custom invoicing, dedicated subnets, and contract support. Our team works with procurement to align SLAs, security reviews, and compliance requirements.'
      }
    ]
  }
]

const contactOptions = [
  {
    title: 'Talk to Sales',
    description: 'Tailor OHMS to your organisation with custom plans, SLAs, and dedicated subnets.',
    action: { label: 'Contact sales@ohms.ai', href: 'mailto:sales@ohms.ai' }
  },
  {
    title: 'Join the Community',
    description: 'Meet other builders shaping autonomous agent workflows in our Discord.',
    action: { label: 'Open Discord', href: 'https://discord.gg/ohms' }
  },
  {
    title: 'Launch the App',
    description: 'Ready to experiment? Spin up your first agent on the main platform.',
    action: { label: 'Launch OHMS', href: OHMS_APP_URL }
  }
]

export default function FAQPage() {
  return (
    <div className="min-h-screen">
      {/* Hero */}
      <section className="section-padding">
        <div className="container-medium content-center text-center">
          <motion.h1
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8 }}
            className="heading-primary"
          >
            Frequently Asked <span className="gradient-text-primary">Questions</span>
          </motion.h1>
          <motion.p
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8, delay: 0.2 }}
            className="text-body-large mt-6 max-w-2xl mx-auto text-text-muted"
          >
            Everything you need to know about launching, operating, and scaling autonomous agents with secure LLM outcalls on the Internet Computer.
          </motion.p>
        </div>
      </section>

      {/* FAQ Sections */}
      <section className="section-padding pt-0">
        <div className="container-wide space-y-16">
          {faqSections.map((section, sectionIndex) => {
            const Icon = section.icon
            return (
              <motion.div
                key={section.title}
                initial={{ opacity: 0, y: 24 }}
                whileInView={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.5, delay: sectionIndex * 0.1 }}
                viewport={{ once: true, margin: '-100px' }}
                className="space-y-8"
              >
                <div className="flex items-start gap-4">
                  <div className="w-10 h-10 rounded-xl bg-primary/15 flex items-center justify-center text-primary">
                    <Icon className="w-5 h-5" />
                  </div>
                  <div>
                    <h2 className="heading-secondary">{section.title}</h2>
                    <p className="text-text-muted">{section.description}</p>
                  </div>
                </div>

                <div className="grid gap-6 md:grid-cols-2">
                  {section.items.map((item, itemIndex) => (
                    <Card
                      key={item.question}
                      className="p-6 text-left"
                      hover={false}
                    >
                      <motion.h3
                        initial={{ opacity: 0, y: 12 }}
                        whileInView={{ opacity: 1, y: 0 }}
                        transition={{ duration: 0.4, delay: itemIndex * 0.05 }}
                        viewport={{ once: true }}
                        className="text-lg font-semibold text-text-primary mb-3"
                      >
                        {item.question}
                      </motion.h3>
                      <p className="text-sm leading-relaxed text-text-muted">
                        {item.answer}
                      </p>
                    </Card>
                  ))}
                </div>
              </motion.div>
            )
          })}
        </div>
      </section>

      {/* Contact Section */}
      <section className="section-padding bg-gradient-to-b from-background-darker/60 to-background-dark/80">
        <div className="container-wide">
          <div className="grid gap-6 md:grid-cols-3">
            {contactOptions.map((option) => (
              <Card key={option.title} className="p-6" gradient hover>
                <div className="flex flex-col h-full justify-between space-y-6">
                  <div>
                    <h3 className="text-xl font-semibold text-text-primary mb-2">{option.title}</h3>
                    <p className="text-sm text-text-muted">{option.description}</p>
                  </div>
                  <Button
                    className="w-full justify-center gap-2"
                    onClick={() => window.open(option.action.href, '_blank')}
                  >
                    {option.action.label}
                    <ExternalLink size={16} />
                  </Button>
                </div>
              </Card>
            ))}
          </div>
        </div>
      </section>

      <CTASection
        title="Ready to orchestrate your own agent network?"
        subtitle="Deploy your first agent with transparent billing, provider flexibility, and on-chain coordination."
        primaryCta={{ label: 'Launch OHMS', href: OHMS_APP_URL, external: true }}
        secondaryCta={{ label: 'Explore Features', href: '/features' }}
      />
    </div>
  )
}
