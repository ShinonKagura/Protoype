import React from 'react';
import { usePlugins } from './hooks';
import { Plugin } from './types';

interface PluginItemProps {
  plugin: Plugin;
  onSelect: (plugin: Plugin) => void;
}

const PluginItem: React.FC<PluginItemProps> = ({ plugin, onSelect }) => {
  return (
    <div 
      className="p-4 border rounded-lg hover:bg-gray-50 cursor-pointer"
      onClick={() => onSelect(plugin)}
    >
      <h3 className="text-lg font-semibold">{plugin.metadata.name}</h3>
      <p className="text-gray-600">{plugin.metadata.description}</p>
      <div className="mt-2 text-sm text-gray-500">
        <span>Version: {plugin.metadata.version}</span>
        <span className="mx-2">•</span>
        <span>By: {plugin.metadata.author}</span>
      </div>
    </div>
  );
};

interface PluginListProps {
  onPluginSelect: (plugin: Plugin) => void;
}

export const PluginList: React.FC<PluginListProps> = ({ onPluginSelect }) => {
  const { plugins, loading, error } = usePlugins();

  if (loading) {
    return <div className="text-center p-4">Loading plugins...</div>;
  }

  if (error) {
    return (
      <div className="text-center p-4 text-red-500">
        Error loading plugins: {error}
      </div>
    );
  }

  if (!plugins.length) {
    return (
      <div className="text-center p-4 text-gray-500">
        No plugins found. Please install some plugins to get started.
      </div>
    );
  }

  return (
    <div className="space-y-4">
      {plugins.map((plugin) => (
        <PluginItem
          key={plugin.metadata.name}
          plugin={plugin}
          onSelect={onPluginSelect}
        />
      ))}
    </div>
  );
};
