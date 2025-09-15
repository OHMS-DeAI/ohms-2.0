'use client'

import { motion } from 'framer-motion'
import { Github, Twitter, Linkedin, Mail, Award, Users, Heart, Lightbulb } from 'lucide-react'
import Image from 'next/image'
import Card from '@/components/ui/Card'
import Button from '@/components/ui/Button'
import CTASection from '@/components/sections/CTASection'

const teamMembers = [
  {
    name: 'Dedan Okware',
    role: 'Senior Software Engineer',
    bio: 'Specialist in Blockchain ICP technology with extensive experience in software infrastructure and decentralized systems. Leading the technical architecture and development of OHMS platform.',
    image: '/team/dedan-okware.jpg',
    social: {
      linkedin: 'https://linkedin.com/in/dedan-okware',
      twitter: 'https://twitter.com/dedan_okware',
      github: 'https://github.com/dedan-okware',
      email: 'dedan@ohms.org'
    },
    skills: ['Blockchain ICP', 'Software Infrastructure', 'System Architecture', 'Rust', 'TypeScript']
  },
  {
    name: 'Judith Karuku',
    role: 'Blockchain Developer & Business Developer',
    bio: 'Experienced blockchain developer with a strong background in business development and community growth. Focused on building partnerships and driving adoption of the OHMS platform.',
    image: '/team/judith-karuku.jpg',
    social: {
      linkedin: 'https://linkedin.com/in/judith-karuku',
      twitter: 'https://twitter.com/judith_karuku',
      github: 'https://github.com/judith-karuku',
      email: 'judith@ohms.org'
    },
    skills: ['Blockchain Development', 'Business Development', 'Community Building', 'Smart Contracts', 'Marketing']
  }
]

const cultureValues = [
  {
    icon: Heart,
    title: 'People First',
    description: 'We believe that great products are built by great teams. We prioritize work-life balance, continuous learning, and personal growth.'
  },
  {
    icon: Lightbulb,
    title: 'Innovation Mindset',
    description: 'We encourage creative thinking, experimentation, and calculated risk-taking to push the boundaries of what\'s possible.'
  },
  {
    icon: Users,
    title: 'Collaboration',
    description: 'We work together across disciplines, share knowledge openly, and support each other in achieving our common goals.'
  },
  {
    icon: Award,
    title: 'Excellence',
    description: 'We strive for the highest quality in everything we do, from code to customer experience, and never compromise on our standards.'
  }
]


export default function TeamPage() {
  return (
    <div className="min-h-screen">
      {/* Hero Section */}
      <section className="py-20 px-4 sm:px-6 lg:px-8">
        <div className="max-w-4xl mx-auto text-center">
          <motion.h1
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8 }}
            className="text-4xl md:text-6xl font-bold mb-6"
          >
            Meet the
            <span className="gradient-text"> OHMS Team</span>
          </motion.h1>

          <motion.p
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8, delay: 0.2 }}
            className="text-xl text-text-muted mb-12 leading-relaxed"
          >
            We&apos;re a diverse group of passionate individuals united by our vision
            of democratizing AI through decentralized technology.
          </motion.p>
        </div>
      </section>

      {/* Team Members */}
      <section className="py-20 px-4 sm:px-6 lg:px-8">
        <div className="max-w-7xl mx-auto">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6 }}
            viewport={{ once: true }}
            className="text-center mb-16"
          >
            <h2 className="text-3xl md:text-4xl font-bold text-text-primary mb-4">
              Core Team
            </h2>
            <p className="text-xl text-text-muted max-w-2xl mx-auto">
              The talented individuals driving OHMS forward
            </p>
          </motion.div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
            {teamMembers.map((member, index) => (
              <motion.div
                key={member.name}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.6, delay: index * 0.1 }}
                viewport={{ once: true }}
              >
                <Card className="p-6 text-center group">
                  <div className="w-24 h-24 rounded-full mx-auto mb-4 overflow-hidden">
                    <Image
                      src={member.image}
                      alt={member.name}
                      width={96}
                      height={96}
                      className="w-full h-full object-cover"
                      unoptimized
                    />
                  </div>

                  <h3 className="text-xl font-semibold text-text-primary mb-1">
                    {member.name}
                  </h3>
                  <p className="text-primary font-medium mb-3">
                    {member.role}
                  </p>

                  <p className="text-text-muted text-sm mb-4 leading-relaxed">
                    {member.bio}
                  </p>

                  <div className="flex flex-wrap gap-2 mb-4 justify-center">
                    {member.skills.map((skill) => (
                      <span
                        key={skill}
                        className="px-2 py-1 bg-primary/10 text-primary text-xs rounded-full"
                      >
                        {skill}
                      </span>
                    ))}
                  </div>

                  <div className="flex justify-center space-x-3">
                    <a
                      href={member.social.linkedin}
                      className="text-text-muted hover:text-primary transition-colors"
                    >
                      <Linkedin size={18} />
                    </a>
                    <a
                      href={member.social.twitter}
                      className="text-text-muted hover:text-primary transition-colors"
                    >
                      <Twitter size={18} />
                    </a>
                    <a
                      href={member.social.github}
                      className="text-text-muted hover:text-primary transition-colors"
                    >
                      <Github size={18} />
                    </a>
                    <a
                      href={`mailto:${member.social.email}`}
                      className="text-text-muted hover:text-primary transition-colors"
                    >
                      <Mail size={18} />
                    </a>
                  </div>
                </Card>
              </motion.div>
            ))}
          </div>
        </div>
      </section>

      {/* Culture Section */}
      <section className="py-20 px-4 sm:px-6 lg:px-8 bg-gradient-to-br from-background-darker to-background-dark">
        <div className="max-w-7xl mx-auto">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6 }}
            viewport={{ once: true }}
            className="text-center mb-16"
          >
            <h2 className="text-3xl md:text-4xl font-bold text-text-primary mb-4">
              Our Culture
            </h2>
            <p className="text-xl text-text-muted max-w-2xl mx-auto">
              The values that guide our decisions and shape our company culture
            </p>
          </motion.div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
            {cultureValues.map((value, index) => {
              const Icon = value.icon
              return (
                <motion.div
                  key={value.title}
                  initial={{ opacity: 0, y: 20 }}
                  whileInView={{ opacity: 1, y: 0 }}
                  transition={{ duration: 0.6, delay: index * 0.1 }}
                  viewport={{ once: true }}
                >
                  <Card variant="feature" className="text-center">
                    <div className="w-16 h-16 bg-secondary/20 rounded-2xl flex items-center justify-center mb-6 mx-auto">
                      <Icon className="w-8 h-8 text-secondary" />
                    </div>
                    <h3 className="text-xl font-semibold text-text-primary mb-4">
                      {value.title}
                    </h3>
                    <p className="text-text-muted leading-relaxed">
                      {value.description}
                    </p>
                  </Card>
                </motion.div>
              )
            })}
          </div>
        </div>
      </section>


      {/* Join Us Section */}
      <section className="py-20 px-4 sm:px-6 lg:px-8 bg-gradient-to-br from-background-darker to-background-dark">
        <div className="max-w-4xl mx-auto text-center">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6 }}
            viewport={{ once: true }}
          >
            <Card className="p-8 md:p-12">
              <h2 className="text-3xl md:text-4xl font-bold text-text-primary mb-4">
                Join Our Team
              </h2>
              <p className="text-xl text-text-muted mb-8 leading-relaxed">
                We&apos;re always looking for talented individuals who share our passion
                for building the future of decentralized AI. If you&apos;re excited about
                making a difference, we&apos;d love to hear from you.
              </p>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
                <div className="text-left">
                  <h3 className="text-lg font-semibold text-text-primary mb-2">
                    Open Positions
                  </h3>
                  <ul className="space-y-2 text-text-muted">
                    <li>• Senior Rust Developer</li>
                    <li>• AI Research Engineer</li>
                    <li>• DevOps Engineer</li>
                    <li>• Product Designer</li>
                  </ul>
                </div>
                <div className="text-left">
                  <h3 className="text-lg font-semibold text-text-primary mb-2">
                    Benefits
                  </h3>
                  <ul className="space-y-2 text-text-muted">
                    <li>• Competitive salary</li>
                    <li>• Remote work options</li>
                    <li>• Learning stipend</li>
                    <li>• Health benefits</li>
                  </ul>
                </div>
              </div>

              <div className="flex flex-col sm:flex-row gap-4 justify-center">
                <Button
                  onClick={() => window.location.href = 'mailto:careers@ohms.org'}
                  size="lg"
                >
                  View Open Positions
                </Button>
                <Button
                  variant="outline"
                  size="lg"
                  onClick={() => window.location.href = 'mailto:careers@ohms.org?subject=General Inquiry'}
                >
                  Send Us a Message
                </Button>
              </div>
            </Card>
          </motion.div>
        </div>
      </section>

      {/* CTA Section */}
      <CTASection
        title="Ready to Work Together?"
        subtitle="Whether you're looking to join our team or collaborate on exciting projects, we'd love to connect with you."
        primaryButtonText="Get in Touch"
        secondaryButtonText="View Careers"
      />
    </div>
  )
}
