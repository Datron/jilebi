import type { ReactNode } from 'react';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Heading from '@theme/Heading';
import styles from './PluginConstellation.module.css';

type Plugin = {
	id: string;
	name: string;
	icon: string;
	description: string;
};

const plugins: Plugin[] = [
	{
		id: 'filesystem',
		name: 'Filesystem',
		icon: '📁',
		description: 'File operations'
	},
	{
		id: 'database',
		name: 'Database',
		icon: '🗄️',
		description: 'Data storage'
	},
	{
		id: 'http',
		name: 'HTTP Client',
		icon: '🌐',
		description: 'Web requests'
	},
	{
		id: 'ai',
		name: 'AI Models',
		icon: '🤖',
		description: 'LLM integration'
	},
	{
		id: 'time',
		name: 'Time Utils',
		icon: '⏰',
		description: 'Date & time'
	},
	{
		id: 'crypto',
		name: 'Cryptography',
		icon: '🔐',
		description: 'Security ops'
	},
	{
		id: 'notifications',
		name: 'Notifications',
		icon: '🔔',
		description: 'Alert system'
	},
	{
		id: 'git',
		name: 'Git',
		icon: '📚',
		description: 'Version control'
	},
	{
		id: 'email',
		name: 'Email',
		icon: '📧',
		description: 'Mail services'
	},
	{
		id: 'json',
		name: 'JSON Utils',
		icon: '📄',
		description: 'JSON processing'
	},
	{
		id: 'pdf',
		name: 'PDF Tools',
		icon: '📑',
		description: 'PDF generation'
	},
	{
		id: 'image',
		name: 'Image Proc',
		icon: '🖼️',
		description: 'Image editing'
	},
	{
		id: 'weather',
		name: 'Weather',
		icon: '🌤️',
		description: 'Weather data'
	},
	{
		id: 'calendar',
		name: 'Calendar',
		icon: '📅',
		description: 'Schedule mgmt'
	},
	{
		id: 'analytics',
		name: 'Analytics',
		icon: '📊',
		description: 'Data insights'
	},
	{
		id: 'search',
		name: 'Search',
		icon: '🔍',
		description: 'Full-text search'
	},
	{
		id: 'auth',
		name: 'Auth',
		icon: '🔑',
		description: 'Authentication'
	},
	{
		id: 'cache',
		name: 'Cache',
		icon: '💾',
		description: 'Data caching'
	},
	{
		id: 'log',
		name: 'Logging',
		icon: '📝',
		description: 'System logs'
	},
	{
		id: 'queue',
		name: 'Queue',
		icon: '🚀',
		description: 'Task queuing'
	}
];

function PluginCard({ plugin, position }: { plugin: Plugin; position: { x: number; y: number } }) {
	return (
		<div
			className={styles.pluginCard}
			style={{
				'--x': `${position.x}px`,
				'--y': `${position.y}px`
			} as React.CSSProperties}
		>
			<div className={styles.cardContent}>
				<div className={styles.pluginIcon}>{plugin.icon}</div>
				<div className={styles.pluginInfo}>
					<h4 className={styles.pluginName}>{plugin.name}</h4>
					<p className={styles.pluginDescription}>{plugin.description}</p>
				</div>
			</div>
		</div>
	);
}

export default function PluginConstellation(): ReactNode {
	const { siteConfig } = useDocusaurusContext();

	// Define custom positions based on your drawing
	const positions = [
		// Top row
		{ x: -360, y: -240 }, { x: -180, y: -240 }, { x: 0, y: -240 }, { x: 180, y: -240 }, { x: 360, y: -240 },
		// Upper middle row
		{ x: -420, y: -120 }, { x: -240, y: -120 }, { x: 240, y: -120 }, { x: 420, y: -120 },
		// Middle row (around CPU)
		{ x: -420, y: 0 }, { x: -240, y: 0 }, { x: 240, y: 0 }, { x: 420, y: 0 },
		// Lower middle row
		{ x: -420, y: 120 }, { x: -240, y: 120 }, { x: 240, y: 120 }, { x: 420, y: 120 },
		// Bottom row
		{ x: -360, y: 240 }, { x: -180, y: 240 }, { x: 0, y: 240 }, { x: 180, y: 240 }, { x: 360, y: 240 }
	];

	return (
		<header className={styles.motherboard}>
			<div className="container">
				<div className={styles.motherboardContainer}>
					{/* Central CPU socket */}
					<div className={styles.cpuSocket}>
						<div className={styles.cpu}>
							<img
								src="/img/jilebi_logo.svg"
								alt="Jilebi Logo"
								className={styles.logoImage}
							/>
							<h2 className={styles.logoText}>Jilebi</h2>
							<p className={styles.logoSubtext}>Runtime Core</p>
						</div>
					</div>

					{/* Plugin modules with custom positions */}
					<div className={styles.pluginContainer}>
						{plugins.map((plugin, index) => {
							const position = positions[index] || { x: 0, y: 0 };
							return (
								<PluginCard key={plugin.id} plugin={plugin} position={position} />
							);
						})}
					</div>
				</div>

				{/* Homepage Header Content */}
				<div className={styles.heroContent}>
					<Heading as="h1" className={styles.heroTitle}>
						{siteConfig.title}
					</Heading>
					<p className={styles.heroSubtitle}>{siteConfig.tagline}</p>
					<div className={styles.buttons}>
						<Link
							className="button button--secondary button--lg"
							to="/docs/download">
							Download Jilebi
						</Link>
						<Link
							className="button button--outline button--secondary button--lg"
							to="/docs/intro">
							Find Plugins
						</Link>
					</div>
				</div>
			</div>
		</header>
	);
}
