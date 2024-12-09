import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import type { Plugin } from './types';

export function usePlugins() {
  const [plugins, setPlugins] = useState<Plugin[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function loadPlugins() {
      try {
        const loadedPlugins = await invoke<Plugin[]>('list_plugins');
        setPlugins(loadedPlugins);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to load plugins');
      } finally {
        setLoading(false);
      }
    }

    loadPlugins();
  }, []);

  return { plugins, loading, error };
}

export function usePlugin(name: string) {
  const [plugin, setPlugin] = useState<Plugin | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function loadPlugin() {
      try {
        const loadedPlugin = await invoke<Plugin>('get_plugin', { name });
        setPlugin(loadedPlugin);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to load plugin');
      } finally {
        setLoading(false);
      }
    }

    loadPlugin();
  }, [name]);

  return { plugin, loading, error };
}
