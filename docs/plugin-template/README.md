# Smart Transfer Plugin Template

## Übersicht

Dieses Template dient als Ausgangspunkt für die Entwicklung neuer Plugins für Smart Transfer.

## Plugin-Struktur

```rust
use std::path::PathBuf;
use crate::plugin_api::base::{Plugin, PluginConfig, PluginType, PluginError};
use crate::plugin_api::compression::{CompressionPlugin, CompressionOptions};

pub struct MyPlugin {
    config: PluginConfig,
    initialized: bool,
}

impl MyPlugin {
    pub fn new() -> Self {
        let config = PluginConfig {
            name: "Mein Plugin".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "Plugin-Beschreibung".to_string(),
            category: "Plugin-Category".to_string(), // Wie ist das Plugin zu Kategorisieren zB. Chat, Pack Programm, Core Plugin, Transfer usw.
            subCategory: "Plugin-subCategory".to_string(), // SubCategory Beispiel: WindowsTuning-> Registry
            menuItem: "Plugin-MenuItem".to_string(), // Wenn es ein Menü einitem Beinhaltet sonst Leer
            plugin_type: PluginType::Compression,
        };

        Self {
            config,
            initialized: false,
        }
    }
}

impl Plugin for MyPlugin {
    fn get_config(&self) -> &PluginConfig {
        &self.config
    }

    fn initialize(&mut self) -> Result<(), PluginError> {
        if self.initialized {
            return Ok(());
        }
        self.initialized = true;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), PluginError> {
        if !self.initialized {
            return Ok(());
        }
        self.initialized = false;
        Ok(())
    }
}

impl CompressionPlugin for MyPlugin {
    fn compress(&self, files: &[PathBuf], output: PathBuf, options: &CompressionOptions)
        -> Result<(), PluginError> {
        // Implementiere Kompression
        Ok(())
    }

    fn decompress(&self, archive: PathBuf, output_dir: PathBuf)
        -> Result<(), PluginError> {
        // Implementiere Dekompression
        Ok(())
    }
}
```

## Integration

1. Erstelle eine neue Datei in `src/plugins/` (z.B. `myplugin.rs`)
2. Implementiere die erforderlichen Traits
3. Registriere das Plugin in `src/plugins/mod.rs`:
   ```rust
   pub mod myplugin;
   ```

## Plugin-API

### PluginConfig

```rust
pub struct PluginConfig {
    pub name: String,        // Plugin-Name
    pub version: String,     // Version
    pub description: String, // Beschreibung
    pub plugin_type: PluginType, // Plugin-Typ (Compression)
}
```

### CompressionOptions

```rust
pub struct CompressionOptions {
    pub level: u32,  // Kompressionslevel (0-9)
}
```

### Fehlerbehandlung

```rust
pub enum PluginError {
    InitError(String),      // Initialisierungsfehler
    ExecutionError(String), // Ausführungsfehler
}
```

## Best Practices

1. **Fehlerbehandlung**

   - Verwende `PluginError` für alle Fehler
   - Gib aussagekräftige Fehlermeldungen zurück

2. **Ressourcen-Management**

   - Initialisiere Ressourcen in `initialize()`
   - Räume in `shutdown()` auf
   - Prüfe `initialized` Status

3. **Konfiguration**

   - Setze sinnvolle Standardwerte
   - Dokumentiere Optionen
   - Validiere Eingaben

4. **Dokumentation**
   - Dokumentiere öffentliche Funktionen
   - Beschreibe Fehlerfälle
   - Füge Beispiele hinzu

## Beispiele

Siehe existierende Plugins:

- `zip.rs`: Einfache ZIP-Kompression
- `sevenz.rs`: Externe Programm-Integration
- `zstd.rs`: Fortgeschrittene Funktionen (TAR+ZSTD)
