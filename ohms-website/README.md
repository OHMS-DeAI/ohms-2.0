# OHMS Website Canister

A professional, high-performance marketing website for OHMS - the decentralized AI agent orchestration platform. Built with Next.js 14, TypeScript, and Tailwind CSS, optimized for deployment on the Internet Computer Protocol (ICP).

## 🚀 Features

- **Modern Tech Stack**: Next.js 14 with App Router, TypeScript, and Tailwind CSS v4
- **Stunning Design**: Glass morphism effects, particle animations, and responsive layouts
- **Performance Optimized**: Core Web Vitals optimized with <2s load times
- **Mobile-First**: Fully responsive design across all breakpoints
- **SEO Ready**: Comprehensive meta tags, structured data, and sitemap
- **ICP Optimized**: Configured for seamless deployment on Internet Computer
- **Interactive Elements**: Smooth animations with Framer Motion and particle effects

## 📦 Tech Stack

- **Framework**: Next.js 14 (App Router)
- **Language**: TypeScript
- **Styling**: Tailwind CSS v4
- **Animations**: Framer Motion
- **Particles**: tsParticles
- **Icons**: Lucide React
- **Deployment**: Internet Computer Protocol

## 🏗️ Project Structure

```
ohms-website/
├── src/
│   ├── app/
│   │   ├── layout.tsx          # Root layout with navbar/footer
│   │   ├── page.tsx           # Home page
│   │   ├── about/page.tsx     # About page
│   │   ├── pricing/page.tsx   # Pricing page
│   │   ├── team/page.tsx      # Team page
│   │   ├── features/page.tsx  # Features page
│   │   ├── globals.css        # Global styles with custom theme
│   │   └── loading.tsx        # Loading component
│   ├── components/
│   │   ├── layout/
│   │   │   ├── Navbar.tsx     # Navigation with glass effects
│   │   │   ├── Footer.tsx     # Footer with social links
│   │   │   └── MobileMenu.tsx # Mobile navigation
│   │   ├── ui/
│   │   │   ├── Button.tsx     # Reusable button component
│   │   │   ├── Card.tsx       # Card with glass effects
│   │   │   ├── Badge.tsx      # Status badges
│   │   │   └── PricingCard.tsx # Pricing tier cards
│   │   ├── sections/
│   │   │   ├── HeroSection.tsx      # Hero with particle background
│   │   │   ├── FeatureGrid.tsx      # Features showcase
│   │   │   ├── StatsSection.tsx     # Statistics display
│   │   │   └── CTASection.tsx       # Call-to-action sections
│   │   ├── effects/
│   │   │   └── ParticlesBackground.tsx # Particle system
│   │   └── ErrorBoundary.tsx # Error handling
│   └── lib/
│       ├── utils.ts           # Utility functions
│       └── constants.ts       # App constants and data
├── public/
│   ├── sitemap.xml           # SEO sitemap
│   └── robots.txt           # Search engine directives
├── .ic-assets.json5         # ICP asset configuration
├── canister_ids.json        # Canister IDs for deployment
├── dfx.json                # DFINITY project configuration
├── next.config.ts          # Next.js configuration
├── package.json            # Dependencies and scripts
└── tailwind.config.js      # Tailwind CSS configuration
```

## 🚀 Quick Start

### Prerequisites

- Node.js 18+
- npm or yarn
- DFINITY SDK (dfx) for ICP deployment

### Installation

1. **Clone and navigate to the project:**
   ```bash
   cd ohms-website
   ```

2. **Install dependencies:**
   ```bash
   npm install
   ```

3. **Start development server:**
   ```bash
   npm run dev
   ```

4. **Open [http://localhost:3000](http://localhost:3000) in your browser**

### Build for Production

```bash
# Build the application
npm run build

# Export static files for ICP deployment
npm run export
```

## 🌐 Deployment on Internet Computer

### Local Development

1. **Start local ICP replica:**
   ```bash
   dfx start --clean
   ```

2. **Deploy canister locally:**
   ```bash
   dfx deploy ohms_website
   ```

3. **Access the website:**
   ```
   http://127.0.0.1:4943/?canisterId=be2us-64aaa-aaaaa-qaabq-cai
   ```

### Production Deployment

1. **Build and export:**
   ```bash
   npm run build
   ```

2. **Deploy to mainnet:**
   ```bash
   dfx deploy --network ic ohms_website
   ```

3. **Update canister ID:**
   The deployment will provide a canister ID. Update `canister_ids.json` with the production canister ID.

## 🎨 Design System

### Color Scheme
- **Primary**: `#6366f1` (Indigo)
- **Secondary**: `#8b5cf6` (Purple)
- **Accent**: `#06b6d4` (Cyan)
- **Background**: Dark gradient (`#0f0f23` to `#1a1a2e`)
- **Text**: `#ffffff`, `#f1f5f9`, `#94a3b8`

### Typography
- **Headings**: Inter (700, 600, 500)
- **Body**: Inter (400, 300)
- **Code**: JetBrains Mono

### Components

#### Button Variants
- `primary`: Main call-to-action
- `secondary`: Alternative action
- `outline`: Subtle action
- `ghost`: Minimal action

#### Card Variants
- `default`: Basic card
- `feature`: Feature showcase
- `pricing`: Pricing tier
- `team`: Team member profile

## 📊 Performance

### Core Web Vitals Targets
- **LCP**: <2.5s (Largest Contentful Paint)
- **FID**: <100ms (First Input Delay)
- **CLS**: <0.1 (Cumulative Layout Shift)

### Optimizations
- Image optimization with WebP/AVIF
- Code splitting and lazy loading
- Particle system performance optimizations
- Mobile-specific performance adjustments
- Compression and caching headers

## 🔧 Configuration

### Environment Variables

Create a `.env.local` file for local development:

```env
# OHMS App URL for external linking
OHMS_APP_URL=https://ohms-main-app-url

# Development settings
NODE_ENV=development
ANALYZE=false
```

### Build Configuration

The `next.config.ts` includes:
- Package import optimization
- Image optimization settings
- Security headers
- Performance optimizations
- Bundle analysis (when `ANALYZE=true`)

## 📱 Mobile Responsiveness

The website is fully responsive with breakpoints:
- **Mobile**: 320px - 767px
- **Tablet**: 768px - 1023px
- **Desktop**: 1024px+

### Mobile Optimizations
- Touch-friendly button sizes (min 44px)
- Optimized particle system for mobile
- Efficient animations and interactions
- Performance optimizations for mobile networks

## 🔍 SEO & Accessibility

### SEO Features
- Comprehensive meta tags
- Structured data (JSON-LD)
- Sitemap and robots.txt
- Open Graph and Twitter Card support
- Semantic HTML structure

### Accessibility
- WCAG 2.1 AA compliant
- Keyboard navigation support
- Screen reader friendly
- High contrast ratios
- Focus management

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 📄 License

This project is part of the OHMS ecosystem. See the main OHMS repository for license information.

## 🆘 Support

- **Documentation**: Check the features page for API documentation
- **Issues**: Report bugs on the OHMS GitHub repository
- **Community**: Join our Discord community for support

## 🎯 Roadmap

- [ ] Multi-language support
- [ ] Advanced analytics integration
- [ ] Custom domain support
- [ ] A/B testing framework
- [ ] Progressive Web App (PWA) features

---

Built with ❤️ for the future of decentralized AI