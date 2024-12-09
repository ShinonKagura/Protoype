## Template Location

Das offizielle Plugin-Template befindet sich unter:

```
rust/src-tauri/src/plugins/template/mod.rs
```

Dieses Template dient als Basis für alle neuen Plugins und implementiert bereits alle notwendigen Traits und Strukturen.

## Overview

Das Plugin-Template ist die Basis für alle Smart Transfer Plugins. Es implementiert die grundlegenden Traits und Strukturen, die für ein funktionierendes Plugin benötigt werden. Mit diesem Template können Sie schnell neue Plugins erstellen, die automatisch vom System erkannt werden.

## Template-Struktur

Ein Plugin basierend auf dem Template besteht aus folgenden Komponenten:

### 1. Basis-Struktur

```rust
pub struct MyPlugin {
    metadata: PluginMetadata,
}

impl MyPlugin {
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata {
                name: String::from("my_plugin"),      // Eindeutiger Plugin-Name
                version: String::from("0.1.0"),       // Semantic Versioning
                author: String::from("Your Name"),    // Autor des Plugins
                description: String::from("Plugin Description"), // Beschreibung
                category: String::from("Plugin Kategorie"), // Category
                subcategory: String::from("Plugin subKategorie"), // Category
                menuItem: String::from("Wenn es ein MenüItem gibt sonst leer"), // MenüItem gibt sonst leer
            },
        }
    }
}
```

### 2. Plugin-Trait Implementation

```rust
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
```

### 3. Spezifische Trait Implementation (z.B. CompressionPlugin)

```rust
impl CompressionPlugin for MyPlugin {
    fn compress(
        &self,
        input_files: &[PathBuf],
        output_file: &PathBuf,
        options: &CompressionOptions,
    ) -> Result<(), PluginError> {
        // Plugin-spezifische Implementierung
        Ok(())
    }

    fn decompress(
        &self,
        input_file: &PathBuf,
        output_dir: &PathBuf,
        overwrite: bool,
    ) -> Result<(), PluginError> {
        // Plugin-spezifische Implementierung
        Ok(())
    }
}
```

## Schnellstart: Neues Plugin erstellen

1. **Plugin-Verzeichnis erstellen**

   ```
   plugins/
   └── winrar/              # Ihr Plugin-Name
       ├── mod.rs           # Hauptimplementierung
       └── tests/           # Tests (optional)
   ```

2. **Minimale Implementation (mod.rs)**

   ```rust
   use crate::plugin_api::base::{Plugin, PluginError};
   use crate::plugin_api::types::{PluginMetadata, CompressionOptions};
   use crate::plugin_api::compression::CompressionPlugin;
   use std::path::PathBuf;

   pub struct WinRarPlugin {
       metadata: PluginMetadata,
   }

   impl WinRarPlugin {
       pub fn new() -> Self {
           Self {
               metadata: PluginMetadata {
                   name: String::from("winrar"),
                   version: String::from("0.1.0"),
                   author: String::from("Your Name"),
                   description: String::from("WinRAR compression plugin"),
                   category: String::from("Plugin Kategorie"), // Category
                   subcategory: String::from("Plugin subKategorie"), // Category
                   menuItem: String::from("Wenn es ein MenüItem gibt sonst leer"), // MenüItem gibt sonst leer
               },
           }
       }
   }

   impl Plugin for WinRarPlugin {
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

   impl CompressionPlugin for WinRarPlugin {
       // Minimale Implementation für Kompression
       fn compress(
           &self,
           input_files: &[PathBuf],
           output_file: &PathBuf,
           options: &CompressionOptions,
       ) -> Result<(), PluginError> {
           Ok(()) // Dummy-Implementation
       }

       // Minimale Implementation für Dekompression
       fn decompress(
           &self,
           input_file: &PathBuf,
           output_dir: &PathBuf,
           overwrite: bool,
       ) -> Result<(), PluginError> {
           Ok(()) // Dummy-Implementation
       }
   }
   ```

## Automatische Plugin-Erkennung

Das Plugin-System erkennt automatisch alle Plugins, die:

1. Das `Plugin`-Trait implementieren
2. Eine gültige `PluginMetadata`-Struktur bereitstellen
3. Im `plugins/`-Verzeichnis liegen
4. Eine `new()`-Funktion zur Instanziierung bereitstellen

## Plugin-Arten

Das Template unterstützt verschiedene Plugin-Arten durch spezifische Traits:

1. **CompressionPlugin**

   - Für Archive und Kompression (ZIP, RAR, 7Z, etc.)
   - Implementiert `compress` und `decompress`

2. **TransferPlugin** (zukünftig)

   - Für Dateitransfer-Protokolle
   - Implementiert `send` und `receive`

3. **ProcessingPlugin** (zukünftig)
   - Für Dateiverarbeitung
   - Implementiert `process`

## Best Practices

1. **Namensgebung**

   - Eindeutige Plugin-Namen wählen
   - Konsistente Versionierung verwenden
   - Aussagekräftige Beschreibungen

2. **Fehlerbehandlung** // Diese Fehlerbehandlung betrifft nur die Kompression. Chats, Filebrowser etc. haben in der Regel kein Compression die benötigen andere Fehlerbehandlungen. Unklar welche art von Fehlerbehandlun hier erwartet wird!

   ```rust
    fn compress(...) -> Result<(), PluginError> {
       if !input_files.is_empty() {
           Ok(())
       } else {
           Err(PluginError::ValidationError("No input files".to_string()))
       }
   }
   ```

3. **Ressourcen-Management**

   - Dateien ordnungsgemäß schließen
   - Temporäre Dateien aufräumen
   - Speicher effizient nutzen

4. **Dokumentation**
   - Öffentliche Funktionen dokumentieren
   - Beispiele bereitstellen
   - Einschränkungen dokumentieren

## Beispiele

### WinRAR Plugin (Minimal)

```rust
// plugins/winrar/mod.rs
pub struct WinRarPlugin {
    metadata: PluginMetadata,
}

impl Plugin for WinRarPlugin {
    // ... (wie oben gezeigt)
}

impl CompressionPlugin for WinRarPlugin {
    // ... (wie oben gezeigt)
}
```

### 7-Zip Plugin (Minimal)

```rust
// plugins/sevenzip/mod.rs
pub struct SevenZipPlugin {
    metadata: PluginMetadata,
}

impl Plugin for SevenZipPlugin {
    // ... (wie oben gezeigt)
}

impl CompressionPlugin for SevenZipPlugin {
    // ... (wie oben gezeigt)
}
```

## Tests

Jedes Plugin sollte grundlegende Tests enthalten:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_metadata() {
        let plugin = WinRarPlugin::new();
        let metadata = plugin.metadata();
        assert_eq!(metadata.name, "winrar");
        assert!(!metadata.version.is_empty());
    }

    #[test]
    fn test_basic_compression() {
        let plugin = WinRarPlugin::new();
        let result = plugin.compress(
            &[PathBuf::from("test.txt")],
            &PathBuf::from("test.rar"),
            &CompressionOptions::default(),
        );
        assert!(result.is_ok());
    }
}
```

## Fazit

Mit diesem Template können Sie schnell neue Plugins erstellen, die vom Smart Transfer System automatisch erkannt werden. Das Template stellt sicher, dass Ihr Plugin:

1. Alle notwendigen Traits implementiert
2. Die Core-Prinzipien einhält
3. Vom System erkannt wird
4. Erweiterbar und wartbar ist

Für die eigentliche Plugin-Logik müssen Sie nur die spezifischen Trait-Methoden implementieren - der Rest wird vom Template bereitgestellt.
