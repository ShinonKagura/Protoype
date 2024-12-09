export interface PluginMetadata {
  name: string;
  description: string;
  version: string;
  author: string;
  pluginType: 'Compression' | 'Transfer';
}

export interface Plugin {
  metadata: PluginMetadata;
  config: {
    name: string;
    description: string;
    version: string;
    pluginType: 'Compression' | 'Transfer';
  };
}

export interface CompressionOptions {
  mode: 'Binary' | 'Preserve';
  splitSize?: number;
}
