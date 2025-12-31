import React, { useEffect, useState, useMemo, type ReactNode } from "react";
import Link from "@docusaurus/Link";
import styles from "./PluginDirectory.module.css";

const API_BASE_URL = "https://jilebi.ai";
// const API_BASE_URL = "http://localhost:8787";

type PluginListItem = {
  name: string;
  version: string;
  download_count: number;
};

function PluginCard({ plugin }: { plugin: PluginListItem }): ReactNode {
  return (
    <Link
      to={`/plugins/detail?name=${encodeURIComponent(plugin.name)}`}
      className={styles.cardLink}
    >
      <div className={styles.pluginCard}>
        <div className={styles.cardHeader}>
          <div className={styles.pluginIcon}>
            <span className={styles.pluginIconText}>
              {plugin.name.charAt(0).toUpperCase()}
            </span>
          </div>
          <div className={styles.pluginMeta}>
            <h3 className={styles.pluginName}>{plugin.name}</h3>
            <span className={styles.pluginVersion}>v{plugin.version}</span>
          </div>
        </div>
        <div className={styles.cardFooter}>
          <div className={styles.downloadCount}>
            <svg
              className={styles.downloadIcon}
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
            >
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
              <polyline points="7 10 12 15 17 10" />
              <line x1="12" y1="15" x2="12" y2="3" />
            </svg>
            <span>{plugin.download_count} downloads</span>
          </div>
        </div>
      </div>
    </Link>
  );
}

function LoadingSkeleton(): ReactNode {
  return (
    <div className={styles.grid}>
      {[...Array(6)].map((_, i) => (
        <div key={i} className={styles.skeletonCard}>
          <div className={styles.skeletonHeader}>
            <div className={styles.skeletonIcon}></div>
            <div className={styles.skeletonMeta}>
              <div className={styles.skeletonTitle}></div>
              <div className={styles.skeletonVersion}></div>
            </div>
          </div>
          <div className={styles.skeletonFooter}></div>
        </div>
      ))}
    </div>
  );
}

function SearchBar({
  value,
  onChange,
}: {
  value: string;
  onChange: (value: string) => void;
}): ReactNode {
  return (
    <div className={styles.searchContainer}>
      <svg
        className={styles.searchIcon}
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
      >
        <circle cx="11" cy="11" r="8" />
        <path d="M21 21l-4.35-4.35" />
      </svg>
      <input
        type="text"
        className={styles.searchInput}
        placeholder="Search plugins..."
        value={value}
        onChange={(e) => onChange(e.target.value)}
      />
      {value && (
        <button
          className={styles.searchClear}
          onClick={() => onChange("")}
          aria-label="Clear search"
        >
          ×
        </button>
      )}
    </div>
  );
}

export default function PluginDirectory(): ReactNode {
  const [plugins, setPlugins] = useState<PluginListItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState("");

  useEffect(() => {
    async function fetchPlugins() {
      try {
        const response = await fetch(`${API_BASE_URL}/api/plugins`);
        if (!response.ok) {
          throw new Error("Failed to fetch plugins");
        }
        const data = await response.json();
        setPlugins(data);
      } catch (err) {
        setError(err instanceof Error ? err.message : "An error occurred");
      } finally {
        setLoading(false);
      }
    }

    fetchPlugins();
  }, []);

  // Sort by download count (descending) and filter by search query
  const filteredAndSortedPlugins = useMemo(() => {
    let result = [...plugins];

    // Sort by download count (descending)
    result.sort((a, b) => b.download_count - a.download_count);

    // Filter by search query
    if (searchQuery.trim()) {
      const query = searchQuery.toLowerCase().trim();
      result = result.filter((plugin) =>
        plugin.name.toLowerCase().includes(query),
      );
    }

    return result;
  }, [plugins, searchQuery]);

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <h1 className={styles.title}>Plugin Directory</h1>
        <p className={styles.subtitle}>
          Explore the Jilebi plugin ecosystem. Find and install plugins to
          extend your MCP runtime.
        </p>
        <a
          href="https://github.com/datron/jilebi-plugins"
          target="_blank"
          rel="noopener noreferrer"
          className={styles.githubLink}
        >
          <svg
            className={styles.githubIcon}
            viewBox="0 0 24 24"
            fill="currentColor"
          >
            <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" />
          </svg>
          View on GitHub
        </a>
      </div>

      {!loading && !error && (
        <SearchBar value={searchQuery} onChange={setSearchQuery} />
      )}

      {loading && <LoadingSkeleton />}

      {error && (
        <div className={styles.errorContainer}>
          <div className={styles.errorIcon}>⚠️</div>
          <h3 className={styles.errorTitle}>Failed to load plugins</h3>
          <p className={styles.errorMessage}>{error}</p>
          <button
            className={styles.retryButton}
            onClick={() => window.location.reload()}
          >
            Retry
          </button>
        </div>
      )}

      {!loading && !error && (
        <>
          <div className={styles.resultsInfo}>
            <span>
              {filteredAndSortedPlugins.length === plugins.length
                ? `${plugins.length} plugins available`
                : `${filteredAndSortedPlugins.length} of ${plugins.length} plugins`}
            </span>
          </div>
          {filteredAndSortedPlugins.length > 0 ? (
            <div className={styles.grid}>
              {filteredAndSortedPlugins.map((plugin) => (
                <PluginCard key={plugin.name} plugin={plugin} />
              ))}
            </div>
          ) : (
            <div className={styles.noResults}>
              <p className={styles.noResultsText}>
                No plugins found matching "{searchQuery}"
              </p>
              <button
                className={styles.clearSearchButton}
                onClick={() => setSearchQuery("")}
              >
                Clear search
              </button>
            </div>
          )}
        </>
      )}
    </div>
  );
}
