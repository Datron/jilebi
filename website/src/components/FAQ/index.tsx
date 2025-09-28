import type { ReactNode } from 'react';
import { useState } from 'react';
import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

type FAQItem = {
	question: string;
	answer: ReactNode;
};

const FAQList: FAQItem[] = [
	{
		question: "What is Jilebi and how is it different from MCP?",
		answer: (
			<>
				Jilebi is a plugin runtime that converts Model Context Protocol (MCP) servers into secure,
				sandboxed plugins. Unlike traditional MCP servers that require direct network and system access,
				Jilebi plugins run in a secure sandbox with explicit permission controls, making them safer
				and easier to manage.
			</>
		),
	},
	{
		question: "Why did you build this?",
		answer: (
			<>
				I wanted to use MCP servers but were concerned they would steal my ssh keys. Setting up and using each MCP server with my editor
				was also a pain. I just wanted to simplify this whole setup, and created jilebi to do that.
			</>
		),
	},
	{
		question: "Can I develop my own plugins for Jilebi?",
		answer: (
			<>
				Absolutely! Jilebi makes plugin development simple by focusing on tools, resources, and prompts
				rather than server abstractions. You can create plugins using JavaScript/TypeScript, and Jilebi
				handles the MCP protocol implementation. Check our documentation for plugin development guides
				and examples.
			</>
		),
	},
	{
		question: "Is jilebi free to use and open source?",
		answer: (
			<>
				Currently no, jilebi is not open source. It is free to download and use in stdio mode. 
				The plugins I write for it are open source and can be found on <a href="https://github.com/datron/jilebi-plugins">https://github.com/jilebi-plugins</a>.
				I may flesh out jilebi more and provide the remote capability as a service in the future where you don't need to download jilebi to use it. 
			</>
		),
	},
	{
		question: "Why not use Docker?",
		answer: (
			<>
				Docker is a pretty good choice, it grants isolation and you can configure permissions and envs within a docker compose file.
				I felt containers take up more system resources, and can limit how many MCPs you can run at a time.
			</>
		),
	},
	{
		question: "I don't want to use jilebi, but like the idea of pluggable MCPs. What are my options?",
		answer: (
			<>
				When I was looking for a solution, I came across <a href="https://github.com/tuananh/hyper-mcp">hyper-mcp</a> which runs MCPs as
				wasm modules plugged in via extism. You can also try <a href="https://github.com/microsoft/wassette">Wassette</a> which Microsoft
				released while I was creating Jilebi. Both are great options if you want to avoid using Docker or full MCP servers. They both are open
				source.
			</>
		),
	},
	{
		question: "How do I get started with Jilebi?",
		answer: (
			<>
				Getting started is easy! Download Jilebi for your platform from our{' '}
				<a href="/docs/download">download page</a>, install it, and start adding plugins with{' '}
				<code>jilebi plugins add [plugin-name]</code>. Check out our{' '}
				<a href="/docs/hosts">quick start guide</a> for detailed setup instructions.
			</>
		),
	},
];

function FAQAccordion({ question, answer, isOpen, onToggle }: FAQItem & { isOpen: boolean; onToggle: () => void }) {
	return (
		<div className={styles.faqItem}>
			<button
				className={clsx(styles.faqQuestion, { [styles.faqQuestionOpen]: isOpen })}
				onClick={onToggle}
				aria-expanded={isOpen}
			>
				<span className={styles.faqQuestionText}>{question}</span>
				<span className={clsx(styles.faqIcon, { [styles.faqIconOpen]: isOpen })}>
					<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
						<path d="M8 4a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4z" />
					</svg>
				</span>
			</button>
			<div className={clsx(styles.faqAnswer, { [styles.faqAnswerOpen]: isOpen })}>
				<div className={styles.faqAnswerContent}>
					{answer}
				</div>
			</div>
		</div>
	);
}

export default function FAQ(): ReactNode {
	const [openItems, setOpenItems] = useState<Set<number>>(new Set());

	const toggleItem = (index: number) => {
		const newOpenItems = new Set(openItems);
		if (newOpenItems.has(index)) {
			newOpenItems.delete(index);
		} else {
			newOpenItems.add(index);
		}
		setOpenItems(newOpenItems);
	};

	return (
		<section className={styles.faq}>
			<div className="container">
				<div className={styles.faqHeader}>
					<Heading as="h2" className={styles.faqTitle}>
						Frequently Asked Questions
					</Heading>
					<p className={styles.faqSubtitle}>
						Get answers to common questions about Jilebi and how it works
					</p>
				</div>
				<div className={styles.faqList}>
					{FAQList.map((faqItem, index) => (
						<FAQAccordion
							key={index}
							question={faqItem.question}
							answer={faqItem.answer}
							isOpen={openItems.has(index)}
							onToggle={() => toggleItem(index)}
						/>
					))}
				</div>
			</div>
		</section>
	);
}
