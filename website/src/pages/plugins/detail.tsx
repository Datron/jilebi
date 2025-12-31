import type { ReactNode } from "react";
import Layout from "@theme/Layout";
import { useLocation } from "@docusaurus/router";
import PluginDetailView from "@site/src/components/PluginDirectory/PluginDetail";

export default function PluginDetailPage(): ReactNode {
  const location = useLocation();
  // Extract plugin name from query params
  const searchParams = new URLSearchParams(location.search);
  const pluginName = searchParams.get("name") || "";

  return (
    <Layout
      title={`${pluginName || "Plugin"} - Plugin Details`}
      description={`View details, tools, prompts, and resources for the ${pluginName} Jilebi plugin.`}
    >
      <main>
        <PluginDetailView pluginName={pluginName} />
      </main>
    </Layout>
  );
}
