import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

export default defineConfig({
  site: 'https://digitalhumanitiescraft.github.io',
  base: '/ivory-kleio',
  trailingSlash: 'always',
  integrations: [
    starlight({
      title: 'Ivory KLEIO',
      description: 'An agentic research environment with a database kernel for historical information.',
      favicon: '/favicon.svg',
      customCss: ['./src/styles/theme.css'],
      components: {
        Header: './src/components/Header.astro',
        Footer: './src/components/Footer.astro',
      },
      sidebar: [
        { label: 'Project', items: [
          { label: 'Specification overview', slug: '' },
          { label: 'Project scope', slug: 'introduction' },
          { label: 'Temporal alternatives', slug: 'concepts/temporal-alternatives' },
          { label: 'The kernel and the harness', slug: 'concepts/kernel-and-harness' },
        ] },
        { label: 'Specification', items: [
          { label: 'Requirements', slug: 'generated/specification' },
          { label: 'Requirements integration', slug: 'generated/requirements-integration' },
          { label: 'Temporal contract', slug: 'generated/temporal-specification' },
          { label: 'Architecture', slug: 'generated/architecture' },
          { label: 'Acceptance case', slug: 'generated/temporal-case' },
        ] },
        { label: 'Research', items: [
          { label: 'Scientific proposal', slug: 'generated/proposal' },
          { label: 'Research objectives', slug: 'generated/project' },
          { label: 'Methodology', slug: 'generated/methodology' },
          { label: 'Sources and reading', slug: 'generated/sources' },
        ] },
        { label: 'Development', items: [
          { label: 'Testing and evaluation', slug: 'generated/testing' },
          { label: 'Contributing', slug: 'generated/contributing' },
          { label: 'Build this website', slug: 'contributing/website' },
        ] },
      ],
      social: [{ icon: 'github', label: 'GitHub', href: 'https://github.com/DigitalHumanitiesCraft/ivory-kleio' }],
      editLink: { baseUrl: 'https://github.com/DigitalHumanitiesCraft/ivory-kleio/edit/main/site/src/content/docs/' },
      pagination: true,
    }),
  ],
});
