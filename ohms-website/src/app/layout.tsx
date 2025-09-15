import type { Metadata } from "next";
import { Inter, JetBrains_Mono } from "next/font/google";
import "./globals.css";
import Navbar from "@/components/layout/Navbar";
import Footer from "@/components/layout/Footer";
import SEOHead from "@/components/SEOHead";
import StructuredData from "@/components/StructuredData";
import { METADATA } from "@/lib/constants";

const inter = Inter({
  variable: "--font-inter",
  subsets: ["latin"],
  display: "swap",
});

const jetbrainsMono = JetBrains_Mono({
  variable: "--font-mono",
  subsets: ["latin"],
  display: "swap",
});

export const metadata: Metadata = {
  title: {
    default: METADATA.title,
    template: '%s | OHMS - Autonomous AI Agent Platform'
  },
  description: METADATA.description,
  keywords: METADATA.keywords,
  authors: [{ name: METADATA.author }],
  creator: METADATA.author,
  publisher: METADATA.author,
  robots: {
    index: true,
    follow: true,
    googleBot: {
      index: true,
      follow: true,
      'max-video-preview': -1,
      'max-image-preview': 'large',
      'max-snippet': -1,
    },
  },
  openGraph: {
    type: "website",
    locale: METADATA.locale,
    url: METADATA.siteUrl,
    siteName: METADATA.siteName,
    title: METADATA.title,
    description: METADATA.description,
    images: [
      {
        url: METADATA.ogImage,
        width: 1200,
        height: 630,
        alt: METADATA.title,
      },
    ],
  },
  twitter: {
    card: "summary_large_image",
    title: METADATA.title,
    description: METADATA.description,
    images: [METADATA.ogImage],
    creator: "@ohms_org",
    site: "@ohms_org",
  },
  alternates: {
    canonical: METADATA.siteUrl,
  },
  verification: {
    google: "your-google-verification-code",
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <head>
        <link rel="icon" href="/favicon.ico" sizes="any" />
        <link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png" />
        <link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png" />
        <link rel="apple-touch-icon" sizes="180x180" href="/favicon-192x192.png" />
        <meta name="msapplication-TileImage" content="/favicon-192x192.png" />
      </head>
      <SEOHead />
      <body
        className={`${inter.variable} ${jetbrainsMono.variable} antialiased`}
      >
        <StructuredData type="website" />
        <StructuredData type="organization" />
        <StructuredData type="product" />
        <Navbar />
        <main className="min-h-screen">
          {children}
        </main>
        <Footer />
      </body>
    </html>
  );
}
