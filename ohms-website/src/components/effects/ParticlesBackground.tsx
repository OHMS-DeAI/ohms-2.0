'use client'

import { useEffect, useMemo, useState, useCallback } from 'react'
import Particles, { initParticlesEngine } from '@tsparticles/react'
import { loadSlim } from '@tsparticles/slim'
import type { ISourceOptions } from '@tsparticles/engine'

export default function ParticlesBackground() {
  const [init, setInit] = useState(false)
  const [isMobile, setIsMobile] = useState(false)
  const [isVisible, setIsVisible] = useState(true)

  useEffect(() => {
    // Check if device is mobile
    const checkMobile = () => {
      setIsMobile(window.innerWidth < 768)
    }

    checkMobile()
    window.addEventListener('resize', checkMobile)

    // Performance optimization: reduce particle count on mobile
    initParticlesEngine(async (engine) => {
      await loadSlim(engine)
    }).then(() => {
      setInit(true)
    })

    return () => window.removeEventListener('resize', checkMobile)
  }, [])

  // Performance optimization: pause particles when not visible
  const handleVisibilityChange = useCallback(() => {
    setIsVisible(!document.hidden)
  }, [])

  useEffect(() => {
    document.addEventListener('visibilitychange', handleVisibilityChange)
    return () => document.removeEventListener('visibilitychange', handleVisibilityChange)
  }, [handleVisibilityChange])

  const particlesOptions: ISourceOptions = useMemo(
    () => ({
      background: {
        color: {
          value: 'transparent',
        },
      },
      fpsLimit: isMobile ? 30 : 60, // Reduce FPS on mobile for performance
      pauseOnBlur: true,
      pauseOnOutsideViewport: true,
      interactivity: {
        events: {
          onClick: {
            enable: !isMobile, // Disable on mobile for better performance
            mode: 'push',
          },
          onHover: {
            enable: !isMobile, // Disable on mobile for better performance
            mode: 'repulse',
          },
          resize: { enable: true },
        },
        modes: {
          push: {
            quantity: isMobile ? 2 : 4,
          },
          repulse: {
            distance: 200,
            duration: 0.4,
          },
        },
      },
      particles: {
        color: {
          value: ['#6366f1', '#8b5cf6', '#06b6d4', '#f1f5f9'],
        },
        links: {
          color: '#6366f1',
          distance: 150,
          enable: !isMobile, // Disable links on mobile for performance
          opacity: 0.3,
          width: 1,
        },
        move: {
          direction: 'none',
          enable: isVisible, // Pause when not visible
          outModes: {
            default: 'bounce',
          },
          random: true,
          speed: isMobile ? 0.5 : 1, // Slower on mobile
          straight: false,
        },
        number: {
          density: {
            enable: true,
            area: isMobile ? 1200 : 800,
          },
          value: isMobile ? 30 : 80, // Fewer particles on mobile
        },
        opacity: {
          value: { min: 0.1, max: 0.8 },
          animation: {
            enable: !isMobile, // Disable animation on mobile for performance
            speed: 1,
            minimumValue: 0.1,
            sync: false,
          },
        },
        shape: {
          type: 'circle',
        },
        size: {
          value: { min: 1, max: isMobile ? 2 : 3 },
          animation: {
            enable: !isMobile, // Disable animation on mobile for performance
            speed: 2,
            minimumValue: 1,
            sync: false,
          },
        },
      },
      detectRetina: !isMobile, // Disable retina detection on mobile for performance
    }),
    [isMobile, isVisible]
  )

  if (!init) {
    return null
  }

  return (
    <div className="particles-container">
      <Particles
        id="tsparticles"
        options={particlesOptions}
        className="w-full h-full"
      />
    </div>
  )
}
