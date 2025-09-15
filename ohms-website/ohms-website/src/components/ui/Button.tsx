'use client'

import { forwardRef } from 'react'
import { ExternalLink, Loader2 } from 'lucide-react'
import { cn } from '@/lib/utils'

interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'outline' | 'ghost'
  size?: 'sm' | 'md' | 'lg'
  loading?: boolean
  external?: boolean
  icon?: React.ReactNode
  iconPosition?: 'left' | 'right'
}

const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  ({
    className,
    variant = 'primary',
    size = 'md',
    loading = false,
    external = false,
    icon,
    iconPosition = 'right',
    children,
    ...props
  }, ref) => {
    const variants = {
      primary: 'bg-primary hover:bg-primary/80 text-white border-transparent',
      secondary: 'bg-secondary hover:bg-secondary/80 text-white border-transparent',
      outline: 'bg-transparent hover:bg-glass text-text-primary border-glass-border',
      ghost: 'bg-transparent hover:bg-glass/50 text-text-primary border-transparent',
    }

    const sizes = {
      sm: 'px-3 py-2 text-sm',
      md: 'px-6 py-3 text-base',
      lg: 'px-8 py-4 text-lg',
    }

    const handleClick = (e: React.MouseEvent<HTMLButtonElement>) => {
      if (external && !loading) {
        // External link behavior will be handled by parent component
      }
      props.onClick?.(e)
    }

    return (
      <button
        ref={ref}
        className={cn(
          'inline-flex items-center justify-center rounded-lg font-medium transition-all duration-200',
          'border focus:outline-none focus:ring-2 focus:ring-primary/50 focus:ring-offset-2',
          'disabled:opacity-50 disabled:pointer-events-none',
          'hover:scale-105 active:scale-95',
          variants[variant],
          sizes[size],
          className
        )}
        onClick={handleClick}
        disabled={loading || props.disabled}
        {...props}
      >
        {loading && (
          <Loader2 className="mr-2 h-4 w-4 animate-spin" />
        )}

        {!loading && icon && iconPosition === 'left' && (
          <span className="mr-2">{icon}</span>
        )}

        {children}

        {!loading && icon && iconPosition === 'right' && (
          <span className="ml-2">{icon}</span>
        )}

        {external && !loading && (
          <ExternalLink className="ml-2 h-4 w-4" />
        )}
      </button>
    )
  }
)

Button.displayName = 'Button'

export default Button
