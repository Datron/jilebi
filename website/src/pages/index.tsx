import type { ReactNode } from 'react';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import HomepageFeatures from '@site/src/components/HomepageFeatures';
import PluginConstellation from '@site/src/components/PluginConstellation';
import FAQ from '@site/src/components/FAQ';


export default function Home(): ReactNode {
	const { siteConfig } = useDocusaurusContext();
	return (
		<Layout
			title={`Hello from ${siteConfig.title}`}
			description="Description will go into a meta tag in <head />">
			<PluginConstellation />
			<main>
				<HomepageFeatures />
				<FAQ />
			</main>
		</Layout>
	);
}
