## Core Principles

Das Plugin-System von "HeavensByte - Smart Transfer" basiert auf folgenden Kernprinzipien:

1. **Plugin-Unabhängigkeit**

   - Jedes Plugin ist eigenständig und unabhängig
   - Keine Abhängigkeiten zu Core-Dateien
   - Einfaches Hinzufügen und Entfernen
   - Alle Plugin-spezifische Logik bleibt im Plugin

2. **Plattformunabhängigkeit**

   - Verwendung plattformunabhängiger Bibliotheken
   - Vermeidung von Shell-Befehlen - oder zumindest Plattform Spezifische Shell / Terminal Befehle die für jedes System selbst erkannt werden zB. Linux, MacOS, Windows, iOS und entsprechend ausgeführt werden.
   - Unterstützung für Windows, Linux, macOS, iOS
   - Ein Ausführbares Programm muss erstellt werden. Beispiel: \*.exe in Windows. Derzeit wird ja alles über ein Terminal gestartet. User NOT Friendly. Natürlich erst bei Release.

3. **Clean Architecture**

   - Trait-basiertes Plugin-System (Was genau bedeutet Trait?)
   - Klare Trennung von Frontend und Backend
   - Typsichere Implementierung

4. **Sicherheit**
   - Sichere Dateiverarbeitung
   - Validierung von Benutzereingaben
   - Ressourcen-Cleanup

## System-Komponenten

### 1. Plugin-API (`plugin_api/`)

#### Base Plugin (`base.rs`)

```rust
pub trait Plugin {
    fn metadata(&self) -> PluginMetadata;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct PluginMetadata {
    pub name: String, // Name des Plugins
    pub version: String, // Version des Plugins
    pub author: String, // Wer ist der Author
    pub description: String, // Was macht dieses Plugin
    pub category: String, // Wie ist das Plugin zu Kategorisieren zB. Chat, Pack Programm, Core Plugin, Transfer usw.
    pub subCategory: String, // SubCategory Beispiel: WindowsTuning-> Registry
    pub menuItem: String, // Wenn es ein Menü einitem Beinhaltet sonst Leer
}
```

### 1. Plugin Manager (`core/plugin_manager.rs`)

Der Plugin-Manager ist verantwortlich für:

- Registrierung von Plugins
- Plugin-Lebenszyklus
- Fehlerbehandlung
- Plugin-Zugriff

```rust
pub struct PluginManager {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self;
    pub fn register_plugin(&mut self, plugin: Box<dyn Plugin>) -> Result<(), PluginError>;
    pub fn get_plugin(&self, name: &str) -> Option<&Box<dyn Plugin>>;
    pub fn list_plugins(&self) -> Vec<PluginMetadata>;
    pub fn register_default_plugins(&mut self) -> Result<(), PluginError>;
}
```

## Plugin-Entwicklung

### 1. Plugin-Template

Ein neues Plugin sollte das Template unter `plugins/template/` als Basis verwenden:

```rust
pub struct MyPlugin {
    metadata: PluginMetadata,
}

impl Plugin for MyPlugin {
    fn metadata(&self) -> PluginMetadata {
        self.metadata.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
```

### 2. Plugin-Integration

1. **Backend**

   - Plugin-Trait implementieren
   - Fehlerbehandlung einbauen
   - Tests schreiben

2. **Frontend**
   - UI-Komponenten erstellen
   - Plugin-Hooks implementieren
   - Tauri-Commands definieren

## Fehlerbehandlung

```rust
pub enum PluginError {
    IoError(std::io::Error),
    //CompressionError(String), // Woher weiß das Plugin das es ein Compressions error geben kann? Chats, Filebrowser Plugins etc. haben so etwas nicht
    ValidationError(String),
    Other(String),
}

impl From<std::io::Error> for PluginError {
    fn from(error: std::io::Error) -> Self {
        PluginError::IoError(error)
    }
}
```

## Best Practices

1. **Plugin-Entwicklung**

   - Template als Basis verwenden
   - Unabhängigkeit wahren
   - Fehler sauber behandeln
   - Tests schreiben

2. **Sicherheit**

   - Eingaben validieren
   - Ressourcen aufräumen
   - Fehler loggen
   - Berechtigungen prüfen

3. **Performance**
   - Asynchrone Operationen nutzen
   - Ressourcen effizient nutzen
   - Speicher-Management beachten

## Roadmap

### 1. Kurzfristig

- Plugin Hot-Reloading
- Bessere Fehlerbehandlung
- Plugin-Konfiguration

### 2. Mittelfristig

- Plugin-Abhängigkeiten
- Versionierung
- Plugin-Marketplace

### 3. Langfristig

- Plugin-Sandboxing
- Plugin-Updates
- Plugin-Statistiken
