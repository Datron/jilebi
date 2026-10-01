import type { ReactNode } from 'react';
import Layout from '@theme/Layout';
import HomepageFeatures from '@site/src/components/HomepageFeatures';
import PluginConstellation from '@site/src/components/PluginConstellation';
import FAQ from '@site/src/components/FAQ';


export default function Home(): ReactNode {
	return (
		<Layout
			title="Secure MCP runtime with plugins"
			description="Jilebi is a secure MCP runtime with a powerful plugin ecosystem. Sandboxed plugins, explicit permissions, one-command installs.">
			<PluginConstellation />
			<main>
				<HomepageFeatures />
				<FAQ />
			</main>
		</Layout>
	);
}
