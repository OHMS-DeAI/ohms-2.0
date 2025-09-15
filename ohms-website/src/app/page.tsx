import type { Metadata } from 'next'
import HeroSection from '@/components/sections/HeroSection'
import FeatureGrid from '@/components/sections/FeatureGrid'
import StatsSection from '@/components/sections/StatsSection'
import CTASection from '@/components/sections/CTASection'
import { METADATA } from '@/lib/constants'

export const metadata: Metadata = {
  title: METADATA.title,
  description: METADATA.description,
  keywords: METADATA.keywords,
}

export default function Home() {
  return (
    <div className="min-h-screen">
      <div className="section-padding">
        <HeroSection />
      </div>
      <div className="section-padding bg-gradient-to-b from-background-dark to-background-darker">
        <FeatureGrid />
      </div>
      <div className="section-padding">
        <StatsSection />
      </div>
      <div className="section-padding">
        <CTASection />
      </div>
    </div>
  )
}
