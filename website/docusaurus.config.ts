import { themes as prismThemes } from "prism-react-renderer";
import type { Config } from "@docusaurus/types";
import type * as Preset from "@docusaurus/preset-classic";

// This runs in Node.js - Don't use client-side code here (browser APIs, JSX...)

const config: Config = {
  title: "Jilebi",
  tagline: "The secure MCP runtime with a powerful plugin ecosystem",
  favicon: "img/jilebi_logo.svg",

  // Future flags, see https://docusaurus.io/docs/api/docusaurus-config#future
  future: {
    v4: true, // Improve compatibility with the upcoming Docusaurus v4
  },

  // Set the production url of your site here
  url: "https://your-docusaurus-site.example.com",
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: "/",

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: "jilebi", // Usually your GitHub org/user name.
  projectName: "jilebi", // Usually your repo name.

  onBrokenLinks: "throw",
  onBrokenMarkdownLinks: "throw",

  // Even if you don't use internationalization, you can use this field to set
  // useful metadata like html lang. For example, if your site is Chinese, you
  // may want to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: "en",
    locales: ["en"],
  },

  presets: [
    [
      "classic",
      {
        docs: {
          sidebarPath: "./sidebars.ts",
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl: "https://github.com/datron/jilebi-plugins",
        },
        // blog: {
        // 	showReadingTime: true,
        // 	feedOptions: {
        // 		type: ['rss', 'atom'],
        // 		xslt: true,
        // 	},
        // 	// Please change this to your repo.
        // 	// Remove this to remove the "edit this page" links.
        // 	editUrl:
        // 		'https://github.com/datron/jilebi-plugins',
        // 	// Useful options to enforce blogging best practices
        // 	onInlineTags: 'warn',
        // 	onInlineAuthors: 'warn',
        // 	onUntruncatedBlogPosts: 'warn',
        // },
        theme: {
          customCss: "./src/css/custom.css",
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    // Replace with your project's social card
    image: "img/docusaurus-social-card.jpg",
    navbar: {
      title: "Jilebi",
      logo: {
        alt: "Jilebi Logo",
        src: "img/jilebi_logo.svg",
      },
      items: [
        {
          type: "docSidebar",
          sidebarId: "tutorialSidebar",
          position: "left",
          label: "Docs",
        },
        { to: "/docs/download", label: "Download", position: "left" },
        { to: "/plugins", label: "Plugins", position: "left" },
        // { to: '/blog', label: 'Blog', position: 'left' },
        // {
        // 	href: 'https://github.com/datron/jilebi',
        // 	label: 'GitHub',
        // 	position: 'right',
        // },
      ],
    },
    footer: {
      style: "dark",
      // links: [
      // 	{
      // 		title: 'Docs',
      // 		items: [
      // 			{
      // 				label: 'Get Started',
      // 				to: '/docs/intro',
      // 			},
      // 			{
      // 				label: 'Download',
      // 				to: '/docs/download',
      // 			},
      // 		],
      // 	},
      // 	{
      // 		title: 'Community',
      // 		items: [
      // 			{
      // 				label: 'Stack Overflow',
      // 				href: 'https://stackoverflow.com/questions/tagged/docusaurus',
      // 			},
      // 			{
      // 				label: 'Discord',
      // 				href: 'https://discordapp.com/invite/docusaurus',
      // 			},
      // 			{
      // 				label: 'X',
      // 				href: 'https://x.com/docusaurus',
      // 			},
      // 		],
      // 	},
      // 	{
      // 		title: 'More',
      // 		items: [
      // 			{
      // 				label: 'Blog',
      // 				to: '/blog',
      // 			},
      // 			{
      // 				label: 'GitHub',
      // 				href: 'https://github.com/datron/jilebi',
      // 			},
      // 		],
      // 	},
      // ],
      copyright: `Copyright © ${new Date().getFullYear()} Jilebi, Inc. Built with Docusaurus.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
    },
    colorMode: {
      defaultMode: "dark",
      disableSwitch: true,
      respectPrefersColorScheme: true,
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
