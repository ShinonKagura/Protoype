import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import { appWindow } from '@tauri-apps/api/window';
import { 
  MantineProvider,
  createTheme,
  Button, 
  Group, 
  Paper, 
  Text, 
  Select, 
  Container,
  Radio,
  Switch,
  NumberInput,
  Stack,
  Divider,
  Modal,
  List,
  ActionIcon,
  useMantineColorScheme,
} from '@mantine/core';
import { Notifications, notifications } from '@mantine/notifications';
import { IconSun, IconMoon, IconPower } from '@tabler/icons-react';
import '@mantine/core/styles.css';
import '@mantine/notifications/styles.css';

interface PluginInfo {
  name: string;
  version: string;
}

interface CompressionOptions {
  mode: 'Binary' | 'Text';
  split_size: number | null;
}

const theme = createTheme({
  primaryColor: 'blue',
});

function App() {
  const { colorScheme, toggleColorScheme } = useMantineColorScheme();
  const [selectedFiles, setSelectedFiles] = useState<string[]>([]);
  const [selectedPlugin, setSelectedPlugin] = useState<string | null>(null);
  const [plugins, setPlugins] = useState<PluginInfo[]>([]);
  const [options, setOptions] = useState<CompressionOptions>({
    mode: 'Binary',
    split_size: null
  });
  const [splitEnabled, setSplitEnabled] = useState(false);
  const [outputDirectory, setOutputDirectory] = useState<string | null>(null);
  const [existingFiles, setExistingFiles] = useState<string[]>([]);
  const [showOverwriteDialog, setShowOverwriteDialog] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  useEffect(() => {
    async function loadPlugins() {
      try {
        const pluginList = await invoke<PluginInfo[]>('list_plugins');
        console.log('Loaded plugins:', pluginList);
        setPlugins(pluginList);
      } catch (e) {
        console.error('Failed to load plugins:', e);
        setErrorMessage(`Failed to load plugins: ${e}`);
      }
    }
    loadPlugins();
  }, []);

  const handleFileSelect = async () => {
    try {
      const selected = await open({
        multiple: true,
        directory: false,
      });
      
      if (Array.isArray(selected)) {
        setSelectedFiles(selected);
      } else if (selected) {
        setSelectedFiles([selected]);
      }
    } catch (e) {
      console.error('File selection error:', e);
      setErrorMessage(`Failed to select files: ${e}`);
    }
  };

  const handleDecompress = async () => {
    if (!selectedFiles || selectedFiles.length === 0 || !outputDirectory) {
      return;
    }
    
    // Check if file is an archive before attempting decompression
    const file = selectedFiles[0];
    const isArchive = file.toLowerCase().match(/\.(zip|7z|gz|rar|tar|zst|zstd)$/i);
    
    if (!isArchive) {
      notifications.show({
        title: 'Error',
        message: `The selected file "${file}" is not a compressed archive. Please select a compressed file to decompress.`,
        color: 'red'
      });
      return;
    }

    try {
      console.log('Decompressing files:', {
        files: selectedFiles,
        outputDir: outputDirectory
      });

      await invoke('decompress_files', {
        files: selectedFiles,
        outputDir: outputDirectory,
        overwrite: false
      });

      notifications.show({
        title: 'Success',
        message: 'Files decompressed successfully',
        color: 'green'
      });
    } catch (e) {
      if (String(e).includes('FileExists')) {
        setExistingFiles(JSON.parse(String(e).split('FileExists:')[1]));
        setShowOverwriteDialog(true);
      } else {
        console.error('Decompression error:', e);
        notifications.show({
          title: 'Error',
          message: `Decompression failed: ${e}`,
          color: 'red'
        });
      }
    }
  };

  const handleOverwriteConfirm = async () => {
    try {
      await invoke('decompress_files', {
        files: selectedFiles,
        outputDir: outputDirectory,
        overwrite: true
      });
      
      notifications.show({
        title: 'Success',
        message: 'Files decompressed successfully',
        color: 'green'
      });
      
      setShowOverwriteDialog(false);
    } catch (e) {
      console.error('Decompression error:', e);
      notifications.show({
        title: 'Error',
        message: `Decompression failed: ${e}`,
        color: 'red'
      });
    }
  };

  const handleOverwriteCancel = () => {
    setShowOverwriteDialog(false);
  };

  const handleOutputDirectorySelect = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: 'Select destination folder for decompressed files',
      });

      if (typeof selected === 'string') {
        setOutputDirectory(selected);
      }
    } catch (error) {
      console.error('Error selecting output directory:', error);
      setErrorMessage(String(error));
    }
  };

  const handleCompress = async () => {
    if (!selectedFiles || selectedFiles.length === 0 || !outputDirectory || !selectedPlugin) {
      return;
    }

    try {
      // Generate output filename based on input files
      let outputFileName = selectedFiles.length === 1 
        ? selectedFiles[0].split(/[\\/]/).pop()?.split('.')[0] || 'compressed'
        : 'compressed';
      
      // Ensure .zip extension
      const outputPath = `${outputDirectory}/${outputFileName}.${selectedPlugin}`;

      console.log('Compressing files:', {
        files: selectedFiles,
        plugin: selectedPlugin,
        outputPath: outputPath,
        options: options
      });

      await invoke('compress_files', {
        files: selectedFiles,
        plugin: selectedPlugin,
        outputDir: outputPath,
        options: {
          mode: options.mode,
          split_size: options.split_size || 0
        }
      });

      notifications.show({
        title: 'Success',
        message: `Files compressed successfully to ${outputPath}`,
        color: 'green'
      });
    } catch (e) {
      console.error('Compression error:', e);
      const errorMsg = String(e);
      
      if (errorMsg.includes('Archive already exists')) {
        notifications.show({
          title: 'Warning',
          message: 'An archive with this name already exists. Please remove it first or choose a different output location.',
          color: 'yellow'
        });
      } else {
        notifications.show({
          title: 'Error',
          message: `Compression failed: ${e}`,
          color: 'red'
        });
      }
    }
  };

  const handleExit = async () => {
    await appWindow.close();
  };

  return (
    <MantineProvider theme={theme}>
      <Notifications />
      <Container size="sm" p="md">
        <Paper shadow="xs" p="md">
          <Group justify="space-between" mb="md">
            <ActionIcon
              variant="default"
              onClick={() => toggleColorScheme()}
              size={30}
            >
              {colorScheme === 'dark' ? (
                <IconSun size={16} />
              ) : (
                <IconMoon size={16} />
              )}
            </ActionIcon>
            <ActionIcon
              variant="default"
              onClick={handleExit}
              size={30}
              color="red"
            >
              <IconPower size={16} />
            </ActionIcon>
          </Group>
          <Stack>
            {errorMessage && (
              <Text color="red" size="sm">
                {errorMessage}
              </Text>
            )}

            {/* Plugin Selection */}
            <Select
              label="Select Compression Format"
              placeholder="Choose a format"
              value={selectedPlugin}
              onChange={setSelectedPlugin}
              data={plugins.map(p => ({ value: p.name, label: p.name }))}
            />

            <Divider />

            {/* Compression Options */}
            <Radio.Group
              label="Compression Mode"
              value={options.mode}
              onChange={(value) => setOptions({ ...options, mode: value as 'Binary' | 'Text' })}
            >
              <Group mt="xs">
                <Radio value="Binary" label="Binary Mode" />
                <Radio value="Text" label="Text Mode" />
              </Group>
            </Radio.Group>

            <Group align="flex-end">
              <Switch 
                label="Enable File Splitting"
                checked={splitEnabled}
                onChange={(event) => setSplitEnabled(event.currentTarget.checked)}
              />
              {splitEnabled && (
                <NumberInput
                  label="Split Size (MB)"
                  value={options.split_size !== null ? Math.floor(Number(options.split_size) / (1024 * 1024)) : 0}
                  onChange={(value) => setOptions(prev => ({ 
                    ...prev, 
                    split_size: typeof value === 'number' ? value * 1024 * 1024 : null
                  }))}
                  min={1}
                  max={2048}
                  step={1}
                />
              )}
            </Group>

            <Divider />

            {/* File Selection */}
            <Group justify="center" mt="xl">
              <Button onClick={handleFileSelect}>
                Select Files
              </Button>
              <Button onClick={handleOutputDirectorySelect}>
                Select Output Directory
              </Button>
              <Button 
                onClick={handleCompress}
                disabled={!selectedFiles || selectedFiles.length === 0 || !selectedPlugin || !outputDirectory}
              >
                Compress
              </Button>
              <Button 
                onClick={handleDecompress}
                disabled={!selectedFiles || selectedFiles.length === 0 || !outputDirectory}
              >
                Decompress
              </Button>
            </Group>

            {/* Action Buttons */}
          </Stack>
        </Paper>
      </Container>

      {/* Overwrite Dialog */}
      <Modal
        opened={showOverwriteDialog}
        onClose={handleOverwriteCancel}
        title="Files Already Exist"
      >
        <Text>The following files already exist:</Text>
        <List>
          {existingFiles.map((file, index) => (
            <List.Item key={index}>{file}</List.Item>
          ))}
        </List>
        <Text mt="md">Do you want to overwrite these files?</Text>
        <Group mt="md">
          <Button onClick={handleOverwriteConfirm} color="red">Overwrite</Button>
          <Button onClick={handleOverwriteCancel} variant="outline">Cancel</Button>
        </Group>
      </Modal>

      {/* Error Dialog */}
      <Modal
        opened={!!errorMessage}
        onClose={() => setErrorMessage(null)}
        title="Error"
      >
        <Stack>
          <Text>{errorMessage}</Text>
          <Button onClick={() => setErrorMessage(null)}>OK</Button>
        </Stack>
      </Modal>
    </MantineProvider>
  );
}

export default App;
