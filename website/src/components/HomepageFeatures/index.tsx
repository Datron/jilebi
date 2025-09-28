import type { ReactNode } from 'react';
import { useState } from 'react';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

type FeatureItem = {
	title: string;
	image: string;
	description: ReactNode;
};

const FeatureList: FeatureItem[] = [
	{
		title: 'Easy to Add MCPs',
		image: require('@site/static/img/adding_plugins.png').default,
		description: (
			<>
				Jilebi makes it really easy to add MCPs by converting them to plugins that can then be used by
				your AI tool or agent. You can add MCPs with a single command like this <br />
				<code>jilebi plugins add context7</code>
			</>
		),
	},
	{
		title: 'Plugins are better than MCP servers',
		image: require('@site/static/img/permissions.png').default,
		description: (
			<>
				Jilebi uses deno_core with help from <a href="https://github.com/rscarson/rustyscript">rustyscript</a> to sandbox and run plugins
				without giving them any access to your network, envs or file system. Any permission to use these resources needs to be explicitly allowed
				by the user during installation of the plugin
			</>
		),
	},
	{
		title: 'Simplify MCP development',
		image: require('@site/static/img/manifest.png').default,
		description: (
			<>
				Focus on tools, resources and prompts. Leave the server abstraction to Jilebi. Jilebi currently supports stdio, and
				will add support for SSE, HTTP and OAuth soon. Since its easy to make plugins, anyone can contribute, even AI tools.
				Jilebi can keep up with the MCP spec while you focus on your plugin
			</>
		),
	}
];

function Feature({ title, image, description, onImageClick }: FeatureItem & { onImageClick: (image: string, title: string) => void }) {
	return (
		<div className={styles.featureItem}>
			<div className={styles.featureContent}>
				<div className={styles.featureText}>
					<Heading as="h3">{title}</Heading>
					<p>{description}</p>
				</div>
				<div className={styles.featureImage}>
					<img
						src={image}
						className={styles.featureImg}
						alt={title}
						onClick={() => onImageClick(image, title)}
						role="button"
						tabIndex={0}
						onKeyDown={(e) => {
							if (e.key === 'Enter' || e.key === ' ') {
								e.preventDefault();
								onImageClick(image, title);
							}
						}}
					/>
				</div>
			</div>
		</div>
	);
}

export default function HomepageFeatures(): ReactNode {
	const [modalImage, setModalImage] = useState<{ src: string; title: string } | null>(null);

	const handleImageClick = (image: string, title: string) => {
		setModalImage({ src: image, title });
	};

	const closeModal = () => {
		setModalImage(null);
	};

	return (
		<section className={styles.features}>
			<div className="container">
				<div className={styles.featuresList}>
					{FeatureList.map((props, idx) => (
						<Feature key={idx} {...props} onImageClick={handleImageClick} />
					))}
				</div>
			</div>

			{modalImage && (
				<div className={styles.imageModal} onClick={closeModal}>
					<div className={styles.modalContent} onClick={(e) => e.stopPropagation()}>
						<button className={styles.closeButton} onClick={closeModal} aria-label="Close">
							×
						</button>
						<img
							src={modalImage.src}
							alt={modalImage.title}
							className={styles.modalImage}
						/>
						<p className={styles.modalTitle}>{modalImage.title}</p>
					</div>
				</div>
			)}
		</section>
	);
}
