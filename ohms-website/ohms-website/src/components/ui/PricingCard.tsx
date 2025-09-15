'use client'

import { motion } from 'framer-motion'
import { Check } from 'lucide-react'
import { cn } from '@/lib/utils'
import Button from './Button'
import Badge from './Badge'

interface PricingCardProps {
  name: string
  price: string
  period: string
  description: string
  features: string[]
  cta: string
  popular?: boolean
  onCtaClick?: () => void
}

export default function PricingCard({
  name,
  price,
  period,
  description,
  features,
  cta,
  popular = false,
  onCtaClick,
}: PricingCardProps) {
  return (
    <motion.div
      className={cn(
        'relative glass-card p-8',
        popular && 'ring-2 ring-primary border-primary/50'
      )}
      whileHover={{ y: -5 }}
      transition={{ duration: 0.3 }}
    >
      {popular && (
        <div className="absolute -top-4 left-1/2 transform -translate-x-1/2">
          <Badge variant="primary">Most Popular</Badge>
        </div>
      )}

      <div className="text-center mb-8">
        <h3 className="text-2xl font-bold text-text-primary mb-2">{name}</h3>
        <div className="mb-4">
          <span className="text-4xl font-bold gradient-text">{price}</span>
          <span className="text-text-muted">/{period}</span>
        </div>
        <p className="text-text-muted">{description}</p>
      </div>

      <ul className="space-y-3 mb-8">
        {features.map((feature, index) => (
          <li key={index} className="flex items-center space-x-3">
            <div className="flex-shrink-0 w-5 h-5 bg-primary/20 rounded-full flex items-center justify-center">
              <Check className="w-3 h-3 text-primary" />
            </div>
            <span className="text-text-secondary">{feature}</span>
          </li>
        ))}
      </ul>

      <Button
        onClick={onCtaClick}
        variant={popular ? 'primary' : 'outline'}
        className="w-full"
        size="lg"
      >
        {cta}
      </Button>
    </motion.div>
  )
}
