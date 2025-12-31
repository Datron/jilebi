import type { ReactNode } from 'react';
import Layout from '@theme/Layout';
import PluginDirectory from '@site/src/components/PluginDirectory';

export default function PluginsPage(): ReactNode {
	return (
		<Layout
			title="Plugin Directory"
			description="Explore the Jilebi plugin ecosystem. Find and install plugins to extend your MCP runtime.">
			<main>
				<PluginDirectory />
			</main>
		</Layout>
	);
}
