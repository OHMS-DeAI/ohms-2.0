'use client'

import { METADATA } from '@/lib/constants'

interface StructuredDataProps {
  type?: 'website' | 'organization' | 'product'
}

export default function StructuredData({ type = 'website' }: StructuredDataProps) {
  const getStructuredData = () => {
    const baseData = {
      '@context': 'https://schema.org',
      '@type': type === 'website' ? 'WebSite' : type === 'organization' ? 'Organization' : 'SoftwareApplication',
      name: 'OHMS - Autonomous AI Agent Platform',
      url: METADATA.siteUrl,
      description: METADATA.description,
      sameAs: [
        'https://github.com/ohms-org',
        'https://twitter.com/ohms_org',
        'https://linkedin.com/company/ohms-org'
      ]
    }

    if (type === 'website') {
      return {
        ...baseData,
        '@type': 'WebSite',
        potentialAction: {
          '@type': 'SearchAction',
          target: `${METADATA.siteUrl}/search?q={search_term_string}`,
          'query-input': 'required name=search_term_string'
        }
      }
    }

    if (type === 'organization') {
      return {
        ...baseData,
        '@type': 'Organization',
        foundingDate: '2024',
        founders: [
          {
            '@type': 'Person',
            name: 'Dedan Okware',
            jobTitle: 'Senior Software Engineer',
            sameAs: [
              'https://linkedin.com/in/dedan-okware',
              'https://github.com/dedan-okware'
            ]
          },
          {
            '@type': 'Person',
            name: 'Judith Kauruku',
            jobTitle: 'Blockchain Developer & Business Developer',
            sameAs: [
              'https://linkedin.com/in/judith-kauruku',
              'https://github.com/judith-kauruku'
            ]
          }
        ],
        contactPoint: {
          '@type': 'ContactPoint',
          contactType: 'customer support',
          email: 'support@ohms-platform.com'
        }
      }
    }

    if (type === 'product') {
      return {
        ...baseData,
        '@type': 'SoftwareApplication',
        applicationCategory: 'AI Platform',
        operatingSystem: 'Web Browser',
        offers: [
          {
            '@type': 'Offer',
            name: 'Basic Plan',
            price: '29',
            priceCurrency: 'USD',
            priceSpecification: {
              '@type': 'UnitPriceSpecification',
              price: 29,
              priceCurrency: 'USD',
              unitText: 'monthly'
            }
          },
          {
            '@type': 'Offer',
            name: 'Pro Plan',
            price: '99',
            priceCurrency: 'USD',
            priceSpecification: {
              '@type': 'UnitPriceSpecification',
              price: 99,
              priceCurrency: 'USD',
              unitText: 'monthly'
            }
          },
          {
            '@type': 'Offer',
            name: 'Enterprise Plan',
            price: '299',
            priceCurrency: 'USD',
            priceSpecification: {
              '@type': 'UnitPriceSpecification',
              price: 299,
              priceCurrency: 'USD',
              unitText: 'monthly'
            }
          }
        ],
        featureList: [
          'AI Agent Creation from Natural Language',
          'NOVAQ Model Compression Technology',
          'Internet Computer Blockchain',
          'Autonomous Agent Networks',
          'Subscription-based Economics'
        ]
      }
    }

    return baseData
  }

  return (
    <script
      type="application/ld+json"
      dangerouslySetInnerHTML={{
        __html: JSON.stringify(getStructuredData(), null, 2)
      }}
    />
  )
}