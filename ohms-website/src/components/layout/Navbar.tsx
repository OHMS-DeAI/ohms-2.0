'use client'

import { useState } from 'react'
import Link from 'next/link'
import Image from 'next/image'
import { motion, AnimatePresence } from 'framer-motion'
import { Menu, X, ExternalLink } from 'lucide-react'
import { NAVIGATION_LINKS, OHMS_APP_URL } from '@/lib/constants'


export default function Navbar() {
  const [isOpen, setIsOpen] = useState(false)

  const launchOHMS = () => {
    window.open(OHMS_APP_URL, '_blank')
  }

  return (
    <motion.nav
      initial={{ y: -100 }}
      animate={{ y: 0 }}
      transition={{ duration: 0.6 }}
      className="fixed top-0 left-0 right-0 z-50 glass"
    >
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex items-center justify-between h-16">
          {/* Logo */}
          <Link href="/" className="flex items-center space-x-3">
            <div className="w-10 h-10 flex items-center justify-center">
              <Image
                src="/ohms-logo.png"
                alt="OHMS Logo"
                width={40}
                height={40}
                unoptimized
                className="w-full h-full object-contain"
              />
            </div>
            <span className="text-xl font-bold gradient-text">OHMS</span>
          </Link>

          {/* Desktop Navigation */}
          <div className="hidden md:flex items-center space-x-8">
            {NAVIGATION_LINKS.map((link) => (
              link.external ? (
                <a
                  key={link.name}
                  href={link.href}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-text-secondary hover:text-primary transition-colors duration-200 font-medium"
                >
                  {link.name}
                </a>
              ) : (
                <Link
                  key={link.name}
                  href={link.href}
                  className="text-text-secondary hover:text-primary transition-colors duration-200 font-medium"
                >
                  {link.name}
                </Link>
              )
            ))}
          </div>

          {/* Desktop CTA Button */}
          <div className="hidden md:flex items-center space-x-4">
            <button
              onClick={launchOHMS}
              className="flex items-center space-x-2 bg-primary hover:bg-primary/80 text-white px-6 py-2 rounded-lg font-medium transition-all duration-200 hover:scale-105"
            >
              <span>Launch OHMS</span>
              <ExternalLink size={16} />
            </button>
          </div>

          {/* Mobile Menu Button */}
          <button
            onClick={() => setIsOpen(!isOpen)}
            className="md:hidden p-2 text-text-secondary hover:text-primary transition-colors"
          >
            {isOpen ? <X size={24} /> : <Menu size={24} />}
          </button>
        </div>
      </div>

      {/* Mobile Menu */}
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
                link.external ? (
                  <a
                    key={link.name}
                    href={link.href}
                    target="_blank"
                    rel="noopener noreferrer"
                    onClick={() => setIsOpen(false)}
                    className="block text-text-secondary hover:text-primary transition-colors duration-200 font-medium py-2"
                  >
                    {link.name}
                  </a>
                ) : (
                  <Link
                    key={link.name}
                    href={link.href}
                    onClick={() => setIsOpen(false)}
                    className="block text-text-secondary hover:text-primary transition-colors duration-200 font-medium py-2"
                  >
                    {link.name}
                  </Link>
                )
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
    </motion.nav>
  )
}
