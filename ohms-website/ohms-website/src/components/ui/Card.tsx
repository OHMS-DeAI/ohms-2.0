'use client'

import { forwardRef } from 'react'
import { cn } from '@/lib/utils'

interface CardProps extends React.HTMLAttributes<HTMLDivElement> {
  variant?: 'default' | 'feature' | 'pricing' | 'team'
  hover?: boolean
  gradient?: boolean
}

const Card = forwardRef<HTMLDivElement, CardProps>(
  ({
    className,
    variant = 'default',
    hover = true,
    gradient = false,
    children,
    ...props
  }, ref) => {
    const variants = {
      default: 'glass-card',
      feature: 'glass-card p-6 text-center w-full',
      pricing: 'glass-card p-8 relative text-center w-full',
      team: 'glass-card p-6 text-center w-full',
    }

    return (
      <div
        ref={ref}
        className={cn(
          'rounded-xl transition-all duration-300 mx-auto',
          gradient && 'bg-gradient-to-br from-primary/10 to-secondary/10',
          hover && 'hover:-translate-y-1 hover:shadow-2xl hover:shadow-black/50',
          variants[variant],
          className
        )}
        {...props}
      >
        {children}
      </div>
    )
  }
)

Card.displayName = 'Card'

export default Card
