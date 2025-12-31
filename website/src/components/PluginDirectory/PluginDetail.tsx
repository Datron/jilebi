import React, { useEffect, useState, type ReactNode } from "react";
import Link from "@docusaurus/Link";
import styles from "./PluginDetail.module.css";

const API_BASE_URL = "https://jilebi.ai";

type InputSchema = {
  type: string;
  required?: string[];
  properties?: Record<
    string,
    {
      type: string;
      description?: string;
    }
  >;
};

type Permissions = {
  hosts?: string[];
  envs?: string[];
  secrets?: string[];
};

type Tool = {
  name: string;
  description: string;
  function?: string;
  annotations?: {
    destructive_hint?: boolean;
    idempotent_hint?: boolean;
    open_world_hint?: boolean;
    read_only_hint?: boolean;
    title?: string;
  };
  input_schema?: InputSchema;
  permissions?: Permissions;
};

type PromptArgument = {
  name: string;
  description?: string;
  required?: boolean;
};

type PromptMessage = {
  content: string;
  role: string;
  type: string;
};

type Prompt = {
  name: string;
  description?: string;
  arguments?: PromptArgument[];
  messages?: PromptMessage[];
};

type Resource = {
  name: string;
  description?: string;
  uri?: string;
  mimeType?: string;
  permissions?: Permissions;
};

// Top-level env variable definition (different from permissions.envs)
type EnvDefinition = {
  default?: string;
  schema?: {
    type?: string;
  };
};

// Top-level secret definition (different from permissions.secrets)
type SecretDefinition = {
  schema?: {
    type?: string;
  };
};

type PluginDetail = {
  name: string;
  version: string;
  creator?: string;
  contact?: string;
  homepage?: string;
  downloads: number;
  tools?: Record<string, Tool>;
  prompts?: Record<string, Prompt>;
  resources?: Record<string, Resource>;
  permissions?: Permissions;
  env?: Record<string, EnvDefinition>;
  secrets?: Record<string, SecretDefinition>;
};

function PermissionsBadges({
  permissions,
}: {
  permissions?: Permissions;
}): ReactNode {
  if (!permissions) return null;

  const hasHosts = permissions.hosts && permissions.hosts.length > 0;
  const hasEnvs = permissions.envs && permissions.envs.length > 0;
  const hasSecrets = permissions.secrets && permissions.secrets.length > 0;

  if (!hasHosts && !hasEnvs && !hasSecrets) return null;

  return (
    <div className={styles.permissionsInline}>
      {hasHosts && (
        <span
          className={styles.permBadge}
          title={permissions.hosts!.join(", ")}
        >
          🌐 {permissions.hosts!.length} host
          {permissions.hosts!.length > 1 ? "s" : ""}
        </span>
      )}
      {hasEnvs && (
        <span className={styles.permBadge} title={permissions.envs!.join(", ")}>
          📋 {permissions.envs!.length} env
          {permissions.envs!.length > 1 ? "s" : ""}
        </span>
      )}
      {hasSecrets && (
        <span
          className={`${styles.permBadge} ${styles.permBadgeSecret}`}
          title={permissions.secrets!.join(", ")}
        >
          🔐 {permissions.secrets!.length} secret
          {permissions.secrets!.length > 1 ? "s" : ""}
        </span>
      )}
    </div>
  );
}

function PermissionsDetails({
  permissions,
}: {
  permissions?: Permissions;
}): ReactNode {
  if (!permissions) return null;

  const hasHosts = permissions.hosts && permissions.hosts.length > 0;
  const hasEnvs = permissions.envs && permissions.envs.length > 0;
  const hasSecrets = permissions.secrets && permissions.secrets.length > 0;

  if (!hasHosts && !hasEnvs && !hasSecrets) return null;

  return (
    <div className={styles.permissionsDetails}>
      {hasHosts && (
        <div className={styles.permGroup}>
          <span className={styles.permLabel}>Hosts:</span>
          <div className={styles.permValues}>
            {permissions.hosts!.map((host) => (
              <code key={host} className={styles.permValue}>
                {host}
              </code>
            ))}
          </div>
        </div>
      )}
      {hasEnvs && (
        <div className={styles.permGroup}>
          <span className={styles.permLabel}>Envs:</span>
          <div className={styles.permValues}>
            {permissions.envs!.map((env) => (
              <code key={env} className={styles.permValue}>
                {env}
              </code>
            ))}
          </div>
        </div>
      )}
      {hasSecrets && (
        <div className={styles.permGroup}>
          <span className={styles.permLabel}>Secrets:</span>
          <div className={styles.permValues}>
            {permissions.secrets!.map((secret) => (
              <code
                key={secret}
                className={`${styles.permValue} ${styles.permValueSecret}`}
              >
                {secret}
              </code>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}

function ToolCard({ tool }: { tool: Tool }): ReactNode {
  const [expanded, setExpanded] = useState(false);

  return (
    <div className={styles.itemCard}>
      <div className={styles.itemHeader} onClick={() => setExpanded(!expanded)}>
        <div className={styles.itemTitleRow}>
          <span className={styles.itemIcon}>🔧</span>
          <h4 className={styles.itemName}>{tool.name}</h4>
          {tool.annotations?.read_only_hint && (
            <span className={styles.badge}>RO</span>
          )}
          {tool.annotations?.destructive_hint && (
            <span className={`${styles.badge} ${styles.badgeWarning}`}>⚠</span>
          )}
          <PermissionsBadges permissions={tool.permissions} />
        </div>
        <span className={styles.expandIcon}>{expanded ? "−" : "+"}</span>
      </div>
      <p className={styles.itemDescription}>{tool.description}</p>

      {expanded && (
        <div className={styles.itemDetails}>
          {tool.input_schema?.properties &&
            Object.keys(tool.input_schema.properties).length > 0 && (
              <div className={styles.schemaSection}>
                <h5 className={styles.schemaTitle}>Parameters</h5>
                <div className={styles.parameterList}>
                  {Object.entries(tool.input_schema.properties).map(
                    ([paramName, param]) => (
                      <div key={paramName} className={styles.parameterItem}>
                        <div className={styles.parameterHeader}>
                          <code className={styles.parameterName}>
                            {paramName}
                          </code>
                          <span className={styles.parameterType}>
                            {param.type}
                          </span>
                          {tool.input_schema?.required?.includes(paramName) && (
                            <span className={styles.requiredBadge}>*</span>
                          )}
                        </div>
                        {param.description && (
                          <p className={styles.parameterDescription}>
                            {param.description}
                          </p>
                        )}
                      </div>
                    ),
                  )}
                </div>
              </div>
            )}
          <PermissionsDetails permissions={tool.permissions} />
        </div>
      )}
    </div>
  );
}

function PromptCard({ prompt }: { prompt: Prompt }): ReactNode {
  const [expanded, setExpanded] = useState(false);

  return (
    <div className={styles.itemCard}>
      <div className={styles.itemHeader} onClick={() => setExpanded(!expanded)}>
        <div className={styles.itemTitleRow}>
          <span className={styles.itemIcon}>💬</span>
          <h4 className={styles.itemName}>{prompt.name}</h4>
        </div>
        <span className={styles.expandIcon}>{expanded ? "−" : "+"}</span>
      </div>
      {prompt.description && (
        <p className={styles.itemDescription}>{prompt.description}</p>
      )}

      {expanded && (
        <div className={styles.itemDetails}>
          {prompt.arguments && prompt.arguments.length > 0 && (
            <div className={styles.schemaSection}>
              <h5 className={styles.schemaTitle}>Arguments</h5>
              <div className={styles.parameterList}>
                {prompt.arguments.map((arg) => (
                  <div key={arg.name} className={styles.parameterItem}>
                    <div className={styles.parameterHeader}>
                      <code className={styles.parameterName}>{arg.name}</code>
                      {arg.required && (
                        <span className={styles.requiredBadge}>*</span>
                      )}
                    </div>
                    {arg.description && (
                      <p className={styles.parameterDescription}>
                        {arg.description}
                      </p>
                    )}
                  </div>
                ))}
              </div>
            </div>
          )}

          {prompt.messages && prompt.messages.length > 0 && (
            <div className={styles.messagesSection}>
              <h5 className={styles.schemaTitle}>Template</h5>
              {prompt.messages.map((msg, idx) => (
                <div key={idx} className={styles.messageItem}>
                  <span className={styles.messageRole}>{msg.role}</span>
                  <pre className={styles.messageContent}>{msg.content}</pre>
                </div>
              ))}
            </div>
          )}
        </div>
      )}
    </div>
  );
}

function ResourceCard({ resource }: { resource: Resource }): ReactNode {
  const [expanded, setExpanded] = useState(false);

  return (
    <div className={styles.itemCard}>
      <div className={styles.itemHeader} onClick={() => setExpanded(!expanded)}>
        <div className={styles.itemTitleRow}>
          <span className={styles.itemIcon}>📦</span>
          <h4 className={styles.itemName}>{resource.name}</h4>
          <PermissionsBadges permissions={resource.permissions} />
        </div>
        <span className={styles.expandIcon}>{expanded ? "−" : "+"}</span>
      </div>
      {resource.description && (
        <p className={styles.itemDescription}>{resource.description}</p>
      )}
      {expanded && (
        <div className={styles.itemDetails}>
          {resource.uri && (
            <div className={styles.detailRow}>
              <span className={styles.detailLabel}>URI:</span>
              <code className={styles.codeValue}>{resource.uri}</code>
            </div>
          )}
          {resource.mimeType && (
            <div className={styles.detailRow}>
              <span className={styles.detailLabel}>Type:</span>
              <code className={styles.codeValue}>{resource.mimeType}</code>
            </div>
          )}
          <PermissionsDetails permissions={resource.permissions} />
        </div>
      )}
    </div>
  );
}

function LoadingState(): ReactNode {
  return (
    <div className={styles.loadingContainer}>
      <div className={styles.spinner}></div>
      <p className={styles.loadingText}>Loading...</p>
    </div>
  );
}

function ConfigurationRequired({
  env,
  secrets,
}: {
  env?: Record<string, EnvDefinition>;
  secrets?: Record<string, SecretDefinition>;
}): ReactNode {
  const envEntries = env ? Object.entries(env) : [];
  const secretEntries = secrets ? Object.entries(secrets) : [];

  const hasEnvs = envEntries.length > 0;
  const hasSecrets = secretEntries.length > 0;

  if (!hasEnvs && !hasSecrets) return null;

  return (
    <section className={styles.configSection}>
      <div className={styles.configHeader}>
        <span className={styles.configIcon}>⚙️</span>
        <h2 className={styles.configTitle}>Configuration Required</h2>
      </div>
      <div className={styles.configContent}>
        {hasEnvs && (
          <div className={styles.configGroup}>
            <div className={styles.configGroupHeader}>
              <span>📋</span>
              <h4 className={styles.configGroupTitle}>Environment Variables</h4>
            </div>
            <p className={styles.configGroupDesc}>
              Set these environment variables in your Jilebi configuration:
            </p>
            <div className={styles.configListDetailed}>
              {envEntries.map(([envName, envDef]) => (
                <div key={envName} className={styles.configItemDetailed}>
                  <div className={styles.configItemHeader}>
                    <code className={styles.configItemName}>{envName}</code>
                    {envDef.schema?.type && (
                      <span className={styles.configItemType}>
                        {envDef.schema.type}
                      </span>
                    )}
                  </div>
                  {envDef.default !== undefined && (
                    <div className={styles.configItemDefault}>
                      <span className={styles.configItemDefaultLabel}>
                        Default:
                      </span>
                      <code className={styles.configItemDefaultValue}>
                        {envDef.default}
                      </code>
                    </div>
                  )}
                </div>
              ))}
            </div>
          </div>
        )}
        {hasSecrets && (
          <div className={styles.configGroup}>
            <div className={styles.configGroupHeader}>
              <span>🔐</span>
              <h4 className={styles.configGroupTitle}>Secrets</h4>
            </div>
            <p className={styles.configGroupDesc}>
              Add these secrets to your Jilebi secrets configuration:
            </p>
            <div className={styles.configListDetailed}>
              {secretEntries.map(([secretName, secretDef]) => (
                <div
                  key={secretName}
                  className={`${styles.configItemDetailed} ${styles.configItemDetailedSecret}`}
                >
                  <div className={styles.configItemHeader}>
                    <code
                      className={`${styles.configItemName} ${styles.configItemNameSecret}`}
                    >
                      {secretName}
                    </code>
                    {secretDef.schema?.type && (
                      <span className={styles.configItemType}>
                        {secretDef.schema.type}
                      </span>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>
    </section>
  );
}

function NetworkHosts({ hosts }: { hosts?: string[] }): ReactNode {
  if (!hosts || hosts.length === 0) return null;

  return (
    <section className={styles.section}>
      <div className={styles.sectionHeader}>
        <h2 className={styles.sectionTitle}>
          <span className={styles.sectionIcon}>🌐</span>
          Network Access
        </h2>
        <span className={styles.sectionCount}>{hosts.length}</span>
      </div>
      <div className={styles.hostsList}>
        {hosts.map((host) => (
          <code key={host} className={styles.hostItem}>
            {host}
          </code>
        ))}
      </div>
    </section>
  );
}

export default function PluginDetailView({
  pluginName,
}: {
  pluginName: string;
}): ReactNode {
  const [plugin, setPlugin] = useState<PluginDetail | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function fetchPlugin() {
      if (!pluginName) {
        setError("No plugin name provided");
        setLoading(false);
        return;
      }

      try {
        const response = await fetch(
          `${API_BASE_URL}/api/plugins/${pluginName}`,
        );
        if (!response.ok) {
          throw new Error("Plugin not found");
        }
        const data = await response.json();
        setPlugin(data);
      } catch (err) {
        setError(err instanceof Error ? err.message : "An error occurred");
      } finally {
        setLoading(false);
      }
    }

    fetchPlugin();
  }, [pluginName]);

  if (loading) {
    return <LoadingState />;
  }

  if (error || !plugin) {
    return (
      <div className={styles.container}>
        <div className={styles.errorContainer}>
          <div className={styles.errorIcon}>⚠️</div>
          <h3 className={styles.errorTitle}>Plugin not found</h3>
          <p className={styles.errorMessage}>
            {error || "The requested plugin does not exist."}
          </p>
          <Link to="/plugins" className={styles.backButton}>
            ← Back to Plugin Directory
          </Link>
        </div>
      </div>
    );
  }

  const toolsArray = plugin.tools ? Object.values(plugin.tools) : [];
  const promptsArray = plugin.prompts ? Object.values(plugin.prompts) : [];
  const resourcesArray = plugin.resources
    ? Object.values(plugin.resources)
    : [];

  // Aggregate all hosts from permissions for network access display
  const allHosts = new Set<string>();

  plugin.permissions?.hosts?.forEach((h) => allHosts.add(h));

  toolsArray.forEach((tool) => {
    tool.permissions?.hosts?.forEach((h) => allHosts.add(h));
  });

  resourcesArray.forEach((resource) => {
    resource.permissions?.hosts?.forEach((h) => allHosts.add(h));
  });

  const aggregatedHosts = allHosts.size > 0 ? Array.from(allHosts) : undefined;

  return (
    <div className={styles.container}>
      <Link to="/plugins" className={styles.backLink}>
        ← Back
      </Link>

      <div className={styles.header}>
        <div className={styles.headerContent}>
          <div className={styles.pluginIcon}>
            <span className={styles.pluginIconText}>
              {plugin.name.charAt(0).toUpperCase()}
            </span>
          </div>
          <div className={styles.headerInfo}>
            <h1 className={styles.pluginName}>{plugin.name}</h1>
            <div className={styles.metaRow}>
              <span className={styles.version}>v{plugin.version}</span>
              {plugin.creator && (
                <span className={styles.creator}>by {plugin.creator}</span>
              )}
              <span className={styles.downloads}>↓ {plugin.downloads}</span>
            </div>
          </div>
        </div>
      </div>

      <ConfigurationRequired env={plugin.env} secrets={plugin.secrets} />

      <NetworkHosts hosts={aggregatedHosts} />

      {toolsArray.length > 0 && (
        <section className={styles.section}>
          <div className={styles.sectionHeader}>
            <h2 className={styles.sectionTitle}>
              <span className={styles.sectionIcon}>🔧</span>
              Tools
            </h2>
            <span className={styles.sectionCount}>{toolsArray.length}</span>
          </div>
          <div className={styles.itemGrid}>
            {toolsArray.map((tool) => (
              <ToolCard key={tool.name} tool={tool} />
            ))}
          </div>
        </section>
      )}

      {promptsArray.length > 0 && (
        <section className={styles.section}>
          <div className={styles.sectionHeader}>
            <h2 className={styles.sectionTitle}>
              <span className={styles.sectionIcon}>💬</span>
              Prompts
            </h2>
            <span className={styles.sectionCount}>{promptsArray.length}</span>
          </div>
          <div className={styles.itemGrid}>
            {promptsArray.map((prompt) => (
              <PromptCard key={prompt.name} prompt={prompt} />
            ))}
          </div>
        </section>
      )}

      {resourcesArray.length > 0 && (
        <section className={styles.section}>
          <div className={styles.sectionHeader}>
            <h2 className={styles.sectionTitle}>
              <span className={styles.sectionIcon}>📦</span>
              Resources
            </h2>
            <span className={styles.sectionCount}>{resourcesArray.length}</span>
          </div>
          <div className={styles.itemGrid}>
            {resourcesArray.map((resource) => (
              <ResourceCard key={resource.name} resource={resource} />
            ))}
          </div>
        </section>
      )}

      {toolsArray.length === 0 &&
        promptsArray.length === 0 &&
        resourcesArray.length === 0 && (
          <div className={styles.emptyState}>
            <p className={styles.emptyText}>
              This plugin doesn't expose any tools, prompts, or resources.
            </p>
          </div>
        )}
    </div>
  );
}
