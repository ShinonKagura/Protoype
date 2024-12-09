# Smart Transfer

Ein modernes Dateitransfer-Tool mit Plugin-System für verschiedene Kompressionsmethoden.

## Features

- 🔌 Modulares Plugin-System
- 🗜️ Mehrere Kompressions-Plugins:
  - ZIP (Standard-Kompression)
  - 7-Zip (LZMA2 Hochkompression)
  - Zstandard (Schnelle Kompression)
- 📦 TAR+Kompression Unterstützung
- 🚀 Asynchrone Dateiübertragung
- 🔒 Sichere Fehlerbehandlung

## Installation

### Voraussetzungen

- Rust (neueste stabile Version)
- 7-Zip (für 7-Zip Plugin)

### Build

```bash
cd rust/src-tauri
cargo build --release
```

## Plugin-System

### Verfügbare Plugins

1. **ZIP Plugin** (`zip.rs`)
   - Standard ZIP-Kompression
   - Einfach und weit verbreitet

2. **7-Zip Plugin** (`sevenz.rs`)
   - LZMA2 Hochkompression
   - Externe 7-Zip Integration

3. **Zstandard Plugin** (`zstd.rs`)
   - Schnelle Kompression
   - TAR+ZSTD Unterstützung

### Eigene Plugins entwickeln

1. Plugin-Template kopieren
2. Traits implementieren:
   - `Plugin` (Basis)
   - `CompressionPlugin` (für Kompression)
3. In `plugins/mod.rs` registrieren
4. Testen und dokumentieren

Siehe `plugin-template/README.md` für Details.

## Dokumentation

- `docs/PROJECT_STRUCTURE.md`: Projektstruktur
- `docs/PLUGIN_SYSTEM.md`: Plugin-System Details
- `docs/DEVELOPMENT_STATE.md`: Entwicklungsstand

## Entwicklung

### Setup

1. Repository klonen
2. Dependencies installieren
3. Build ausführen

### Tests

```bash
cargo test
```

### Neue Features

1. Issue erstellen
2. Branch erstellen
3. Implementieren
4. Tests schreiben
5. PR erstellen

## Lizenz

MIT
