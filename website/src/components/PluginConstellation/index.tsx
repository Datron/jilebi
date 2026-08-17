import { type ReactNode, useState } from "react";
import Link from "@docusaurus/Link";
import useDocusaurusContext from "@docusaurus/useDocusaurusContext";
import Heading from "@theme/Heading";
import styles from "./PluginConstellation.module.css";

function CopyButton({ text }: { text: string }) {
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    await navigator.clipboard.writeText(text);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <button
      className={styles.copyButton}
      onClick={handleCopy}
      title="Copy to clipboard"
    >
      {copied ? (
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          strokeWidth="2"
          strokeLinecap="round"
          strokeLinejoin="round"
        >
          <polyline points="20 6 9 17 4 12"></polyline>
        </svg>
      ) : (
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          strokeWidth="2"
          strokeLinecap="round"
          strokeLinejoin="round"
        >
          <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
          <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
        </svg>
      )}
    </button>
  );
}

function InstallCommands() {
  const linuxCommand = "curl -fsSL https://mcp.jilebi.ai/install.sh | bash";
  const windowsCommand = "irm https://mcp.jilebi.ai/install.ps1 | iex";

  return (
    <div className={styles.installSection}>
      <div className={styles.installCommand}>
        <span className={styles.platformLabel}>Linux / macOS</span>
        <div className={styles.commandWrapper}>
          <code className={styles.commandText}>{linuxCommand}</code>
          <CopyButton text={linuxCommand} />
        </div>
      </div>
      <div className={styles.installCommand}>
        <span className={styles.platformLabel}>Windows (PowerShell)</span>
        <div className={styles.commandWrapper}>
          <code className={styles.commandText}>{windowsCommand}</code>
          <CopyButton text={windowsCommand} />
        </div>
      </div>
    </div>
  );
}

type Plugin = {
  id: string;
  name: string;
  icon: string;
  number_of_downloads: string;
};

const plugins: Plugin[] = [
  {
    id: "filesystem",
    name: "Filesystem",
    icon: require("@site/static/img/folder-open-line.png").default,
    number_of_downloads: "100+",
  },
  {
    id: "cloudflare",
    name: "Cloudflare",
    icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/cloudflare/cloudflare-original.svg",
    number_of_downloads: "100+",
  },
  {
    id: "fetch",
    name: "fetch",
    icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/chrome/chrome-original.svg",
    number_of_downloads: "100+",
  },
  {
    id: "context7",
    name: "Context7",
    icon: "https://context7.com/context7-icon-green.png",
    number_of_downloads: "100+",
  },
  {
    id: "time",
    name: "Time Utils",
    icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/javascript/javascript-original.svg",
    number_of_downloads: "100+",
  },
  {
    id: "github",
    name: "Github",
    icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/github/github-original.svg",
    number_of_downloads: "100+",
  },
  {
    id: "grafana",
    name: "grafana",
    icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/grafana/grafana-original.svg",
    number_of_downloads: "100+",
  },
  {
    id: "memory",
    name: "Memory",
    icon: require("@site/static/img/ram-2-fill.png").default,
    number_of_downloads: "100+",
  },
  {
    id: "sequential-thinking",
    name: "Sequential Thinking",
    icon: require("@site/static/img/brain-line.png").default,
    number_of_downloads: "100+",
  },
  {
    id: "rust-docs",
    name: "Rust Docs",
    icon: "https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/rust/rust-original.svg",
    number_of_downloads: "100+",
  },
  {
    id: "met-museum",
    name: "Met Museum",
    icon: require("@site/static/img/the_met.png").default,
    number_of_downloads: "100+",
  },
  {
    id: "anilist",
    name: "Anilist",
    icon: "https://anilist.co/img/icons/icon.svg",
    number_of_downloads: "100+",
  },
  {
    id: "blender",
    name: "Blender",
    icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/blender/blender-original.svg",
    number_of_downloads: "Coming soon",
  },
  {
    id: "Playwright",
    name: "Playwright",
    icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/playwright/playwright-original.svg",
    number_of_downloads: "Coming soon",
  },
  {
    id: "aws-docs",
    name: "AWS Docs",
    icon: "https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/amazonwebservices/amazonwebservices-plain-wordmark.svg",
    number_of_downloads: "Coming soon",
  },
  {
    id: "exa-search",
    name: "Exa Search",
    icon: require("@site/static/img/exa.png").default,
    number_of_downloads: "Coming soon",
  },
  {
    id: "elastic-search",
    name: "Elastic Search",
    icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/elasticsearch/elasticsearch-original.svg",
    number_of_downloads: "Coming soon",
  },
  {
    id: "whatsapp",
    name: "WhatsApp",
    icon: "https://static.whatsapp.net/rsrc.php/yZ/r/JvsnINJ2CZv.svg",
    number_of_downloads: "Coming soon",
  },
  {
    id: "jira",
    name: "JIRA",
    icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/jira/jira-original.svg",
    number_of_downloads: "Coming soon",
  },
  {
    id: "Figma",
    name: "Figma",
    icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/figma/figma-original.svg",
    number_of_downloads: "Coming soon",
  },
  {
    id: "Azure",
    name: "Azure",
    icon: "https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/azure/azure-original.svg",
    number_of_downloads: "Coming soon",
  },
  {
    id: "GCP",
    name: "GCP",
    icon: "https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/googlecloud/googlecloud-original.svg",
    number_of_downloads: "Coming soon",
  },
];

function PluginCard({
  plugin,
  position,
}: {
  plugin: Plugin;
  position: { x: number; y: number };
}) {
  return (
    <div
      className={styles.pluginCard}
      style={
        {
          "--x": `${position.x}px`,
          "--y": `${position.y}px`,
        } as React.CSSProperties
      }
    >
      <div className={styles.cardContent}>
        <div className={styles.pluginIcon}>
          <img
            src={plugin.icon}
            alt={plugin.name}
            className={styles.pluginIconImg}
          />
        </div>
        <div className={styles.pluginInfo}>
          <h4 className={styles.pluginName}>{plugin.name}</h4>
          <p className={styles.pluginDownloads}>{plugin.number_of_downloads}</p>
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
    { x: -360, y: -240 },
    { x: -180, y: -240 },
    { x: 0, y: -240 },
    { x: 180, y: -240 },
    { x: 360, y: -240 },
    // Upper middle row
    { x: -420, y: -120 },
    { x: -240, y: -120 },
    { x: 240, y: -120 },
    { x: 420, y: -120 },
    // Middle row (around CPU)
    { x: -420, y: 0 },
    { x: -240, y: 0 },
    { x: 240, y: 0 },
    { x: 420, y: 0 },
    // Lower middle row
    { x: -420, y: 120 },
    { x: -240, y: 120 },
    { x: 240, y: 120 },
    { x: 420, y: 120 },
    // Bottom row
    { x: -360, y: 240 },
    { x: -180, y: 240 },
    { x: 0, y: 240 },
    { x: 180, y: 240 },
    { x: 360, y: 240 },
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
                <PluginCard
                  key={plugin.id}
                  plugin={plugin}
                  position={position}
                />
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
          <InstallCommands />
          <div className={styles.buttons}>
            <Link
              className="button button--secondary button--lg"
              to="/docs/download"
            >
              Download Jilebi
            </Link>
            <Link
              className="button button--outline button--secondary button--lg"
              to="/plugins"
            >
              Find Plugins
            </Link>
          </div>
        </div>
      </div>
    </header>
  );
}
