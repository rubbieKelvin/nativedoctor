// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

export default defineConfig({
	integrations: [
		starlight({
			title: 'nativedoctor.',
			components: { SiteTitle: './src/components/SiteTitle.astro' },
			description: 'API requests as plain YAML files, run from your terminal or CI.',
			social: [{ icon: 'github', label: 'GitHub', href: 'https://github.com/rubbieKelvin/nativedoctor' }],
			customCss: ['./src/styles/custom.css'],
			sidebar: [
				{
					label: 'Guides',
					items: [
						{ slug: 'guides/getting-started' },
						{ slug: 'guides/collections' },
						{ slug: 'guides/requests' },
						{ slug: 'guides/variables' },
						{ slug: 'guides/cookies' },
					],
				},
				{
					label: 'Reference',
					items: [
						{ slug: 'reference/commands' },
						{ slug: 'reference/request-items' },
						{ slug: 'reference/output' },
						{ slug: 'reference/exit-codes' },
					],
				},
			],
		}),
	],
});
