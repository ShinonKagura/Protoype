References:
[text](PLUGIN_SYSTEM.md)
[text](PLUGIN_TEMPLATE.md)
[text](Plugin-Compression.md)

## Core Principles

1. **Plugin Independence**

   - Each plugin must be self-contained and independent of core files
   - Plugins should be easily added or removed without modifying core code
   - Use the plugin template as a starting point
   - All plugin-specific logic must stay within the plugin

2. **Platform Independence**

   - All plugins must work across Windows, Linux, macOS and iOS
   - Use cross-platform libraries and APIs
   - Avoid platform-specific code

3. **Clean Architecture**

   - Follow the template pattern strictly
   - Maintain separation of concerns
   - Use proper error handling and type safety
   - Keep plugin logic isolated from core system

4. **Frontend Integration**

   - Each plugin can provide its own React / Tauri components
   - Follow modern UI/UX principles
   - Support both light and dark themes
   - Keep frontend components modular and independent

5. **Security**
   - Validate all inputs
   - Handle resources safely
   - Follow least privilege principle
   - Clean up resources properly

## Creating a New Plugin

### 1. Backend Implementation

Start by copying the template from `plugins/template/`:

```rust
use crate::plugin_api::base::{Plugin, PluginError};
use crate::plugin_api::types::{PluginMetadata};
// use crate::plugin_api::compression::CompressionPlugin; // Nicht jedes Plugin hat eine Compression zB. Chats, Filebrowser, WindowsTuning Tools.
use std::path::PathBuf;

pub struct MyPlugin {
    metadata: PluginMetadata,
}

impl MyPlugin {
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata {
                name: String::from("my_plugin"),
                version: String::from("0.1.0"),
                author: String::from("Your Name"),
                description: String::from("My awesome plugin"),
                category: String::from("Plugin Kategorie"), // Category
                subcategory: String::from("Plugin subKategorie"), // Category
                menuItem: String::from("Wenn es ein MenüItem gibt sonst leer"), // MenüItem gibt sonst leer
            },
        }
    }
}

impl Plugin for MyPlugin {
    fn metadata(&self) -> PluginMetadata {
        self.metadata.clone()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

//Note: Nicht jedes Plugin hat eine Compression
impl CompressionPlugin for MyPlugin {
    fn compress(
        &self,
        input_files: &[PathBuf],
        output_file: &PathBuf,
        options: &CompressionOptions,
    ) -> Result<(), PluginError> {
        // Implement compression logic
        Ok(())
    }

    fn decompress(
        &self,
        input_file: &PathBuf,
        output_dir: &PathBuf,
        overwrite: bool,
    ) -> Result<(), PluginError> {
        // Implement decompression logic
        Ok(())
    }
}
```

### 2. Error Handling

Use the `PluginError` enum for error handling:

```rust
pub enum PluginError {
    IoError(std::io::Error),
    CompressionError(String),
    ValidationError(String),
    Other(String),
}

// Example error handling
fn process_file(path: &PathBuf) -> Result<(), PluginError> {
    std::fs::read(path).map_err(PluginError::IoError)?;
    // Process file
    Ok(())
}
```

### 3. Frontend Integration

Create a React / Tauri component for your plugin:

```typescript
import React from "react";
import { invoke } from "@tauri-apps/api/tauri";

interface MyPluginProps {
  // Define your props
}

export const MyPluginComponent: React.FC<MyPluginProps> = (props) => {
  const handleCompress = async () => {
    try {
      await invoke("compress_files", {
        pluginName: "my_plugin",
        inputFiles: ["file1.txt", "file2.txt"],
        outputFile: "output.zip",
      });
    } catch (error) {
      console.error("Compression failed:", error);
    }
  };

  return (
    <div>
      <h2>My Plugin</h2>
      <button onClick={handleCompress}>Compress Files</button>
    </div>
  );
};
```

## Best Practices

### 1. Code Organization

- Keep all plugin files in a dedicated directory
- Use clear and consistent naming
- Follow Rust and React / Tauri best practices
- Document your code thoroughly

### 2. Error Handling

- Use appropriate error types
- Provide meaningful error messages
- Handle all possible error cases
- Clean up resources in error cases

### 3. Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression() {
        let plugin = MyPlugin::new();
        let result = plugin.compress(
            &[PathBuf::from("test.txt")],
            &PathBuf::from("test.zip"),
            &CompressionOptions::default(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_decompression() {
        let plugin = MyPlugin::new();
        let result = plugin.decompress(
            &PathBuf::from("test.zip"),
            &PathBuf::from("output"),
            true,
        );
        assert!(result.is_ok());
    }
}
```

### 4. Performance

- Use asynchronous operations where appropriate
- Handle large files efficiently
- Implement progress reporting
- Cache results when possible

### 5. Security

- Validate all user inputs
- Use safe file operations
- Handle permissions correctly
- Clean up temporary files

## Examples

### 1. ZIP Plugin

See `plugins/zip/mod.rs` for a basic compression plugin example.

### 2. 7-Zip Plugin

See `plugins/sevenz/mod.rs` for a more advanced plugin example.

## Debugging

1. **Logging**

   - Use the logging system
   - Add debug information
   - Track resource usage

2. **Error Cases**
   - Test error conditions
   - Verify error messages
   - Check resource cleanup

## Publishing

1. **Documentation**

   - Update README
   - Document API changes
   - Include examples

2. **Testing**

   - Run all tests
   - Cross-platform testing
   - Performance testing

3. **Review**
   - Code review
   - Security review
   - Documentation review
