'use client'

import { motion, AnimatePresence } from 'framer-motion'
import Link from 'next/link'
import { ExternalLink } from 'lucide-react'
import { NAVIGATION_LINKS, OHMS_APP_URL } from '@/lib/constants'

interface MobileMenuProps {
  isOpen: boolean
  onClose: () => void
}

export default function MobileMenu({ isOpen, onClose }: MobileMenuProps) {
  const launchOHMS = () => {
    window.open(OHMS_APP_URL, '_blank')
    onClose()
  }

  return (
    <AnimatePresence>
      {isOpen && (
        <motion.div
          initial={{ opacity: 0, height: 0 }}
          animate={{ opacity: 1, height: 'auto' }}
          exit={{ opacity: 0, height: 0 }}
          transition={{ duration: 0.3 }}
          className="md:hidden glass border-t border-glass-border"
        >
          <div className="px-4 py-6 space-y-4">
            {NAVIGATION_LINKS.map((link) => (
              <Link
                key={link.name}
                href={link.href}
                onClick={onClose}
                className="block text-text-secondary hover:text-primary transition-colors duration-200 font-medium py-2"
              >
                {link.name}
              </Link>
            ))}
            <div className="pt-4 border-t border-glass-border">
              <button
                onClick={launchOHMS}
                className="flex items-center space-x-2 bg-primary hover:bg-primary/80 text-white px-6 py-3 rounded-lg font-medium transition-all duration-200 w-full justify-center"
              >
                <span>Launch OHMS</span>
                <ExternalLink size={16} />
              </button>
            </div>
          </div>
        </motion.div>
      )}
    </AnimatePresence>
  )
}
