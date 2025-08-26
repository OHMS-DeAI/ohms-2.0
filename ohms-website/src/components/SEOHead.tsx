import Head from 'next/head'
import { METADATA } from '@/lib/constants'

interface SEOHeadProps {
  title?: string
  description?: string
  keywords?: string[]
  ogImage?: string
  structuredData?: object
}

export default function SEOHead({
  title,
  description,
  keywords,
  ogImage,
  structuredData
}: SEOHeadProps) {
  const fullTitle = title ? `${title} - ${METADATA.title}` : METADATA.title
  const metaDescription = description || METADATA.description
  const metaKeywords = keywords ? [...METADATA.keywords.split(', '), ...keywords].join(', ') : METADATA.keywords

  const defaultStructuredData = {
    "@context": "https://schema.org",
    "@type": "SoftwareApplication",
    "name": "OHMS",
    "description": metaDescription,
    "url": "https://ohms.org",
    "applicationCategory": "DeveloperApplication",
    "operatingSystem": "Web, Internet Computer Protocol",
    "offers": {
      "@type": "Offer",
      "price": "0",
      "priceCurrency": "USD",
      "description": "Free tier available, paid plans start at $49/month"
    },
    "publisher": {
      "@type": "Organization",
      "name": "OHMS",
      "url": "https://ohms.org"
    },
    "featureList": [
      "AI Agent Orchestration",
      "Decentralized Processing",
      "Enterprise Security",
      "High Performance",
      "Developer Friendly",
      "Multi-Agent Support"
    ]
  }

  const finalStructuredData = structuredData || defaultStructuredData

  return (
    <Head>
      <title>{fullTitle}</title>
      <meta name="description" content={metaDescription} />
      <meta name="keywords" content={metaKeywords} />
      <meta name="author" content={METADATA.author} />

      {/* Open Graph */}
      <meta property="og:title" content={fullTitle} />
      <meta property="og:description" content={metaDescription} />
      <meta property="og:type" content="website" />
      <meta property="og:url" content="https://ohms.org" />
      <meta property="og:image" content={ogImage || "/og-image.jpg"} />
      <meta property="og:site_name" content="OHMS" />

      {/* Twitter */}
      <meta name="twitter:card" content="summary_large_image" />
      <meta name="twitter:title" content={fullTitle} />
      <meta name="twitter:description" content={metaDescription} />
      <meta name="twitter:image" content={ogImage || "/og-image.jpg"} />

      {/* Additional Meta Tags */}
      <meta name="robots" content="index, follow" />
      <meta name="language" content="English" />
      <meta name="revisit-after" content="7 days" />
      <meta name="viewport" content="width=device-width, initial-scale=1.0" />

      {/* Canonical URL */}
      <link rel="canonical" href="https://ohms.org" />

      {/* Structured Data */}
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{
          __html: JSON.stringify(finalStructuredData)
        }}
      />
    </Head>
  )
}
