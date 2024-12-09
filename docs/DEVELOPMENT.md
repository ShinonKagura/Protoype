## Core Principles

1. **Platform Independence**

   - Code muss auf allen Plattformen funktionieren
   - Plattform-spezifischer Code in `platform/` Modulen
   - Entwicklungs- und Produktionspfade getrennt
   - Debug/Release-spezifisches Verhalten
   - Native Dialoge und Systemintegrationen über Tauri / React

2. **Plugin System**

   - Keine hardcoded Plugin-Referenzen
   - Dynamisches Laden zur Laufzeit
   - Plattform-unabhängige Plugin-API
   - Standardisierte Fehlerbehandlung
   - Dynamische UI-Anpassung basierend auf Plugin-Funktionalität

3. **Frontend Architecture**

   - React für UI-Komponenten
   - Tauri für native Funktionalität
   - Strikte Trennung von UI und Geschäftslogik
   - Plugin-basierte UI-Generierung
   - Responsive Design und Barrierefreiheit

4. **Entwicklung**
   - Klare Dokumentation
   - Plattform-spezifische Tests
   - Entwicklungsfreundliche Struktur
   - Aussagekräftige Logs
   - TypeScript für Type Safety

## Current State

### Core Components

1. **Plugin API** (`/rust/plugin_api/`)

   - Core trait definitions and types
   - Compression plugin interface
   - Plugin metadata and configuration
   - Status: ⚠️ Needs error type standardization

2. **Plugin System** (`/rust/src-tauri/src/core/`)

   - Plugin registry and management
   - Dynamic plugin loading
   - Status: ⚠️ Needs dynamic discovery implementation

3. **Compression Plugins** (`/rust/src-tauri/src/plugins/`)
   - ZIP Plugin: Basic implementation complete
   - 7Z Plugin: Basic implementation complete
   - ZSTD Plugin: Basic implementation complete
   - Status: Frontend integration not complete

### Frontend Implementation (`/frontend/`)

- React/TypeScript UI mit Tauri Integration
- Vite build system
- Mantine UI Framework
- Features:
  - Dark/Light Mode
  - Native Dateidialoge
  - Plugin-spezifische UI-Elemente
  - Responsive Design
  - Fortschrittsanzeigen
  - Fehlerbehandlung
- Status: ⚠️ Build-Prozess needs fixing

## Immediate Tasks

1. **Build System**

   - Fix Frontend build path in Tauri config
   - Implement production build pipeline
   - Add CI/CD workflows

2. **Plugin System Integration**

   - Implement plugin-specific UI components
   - Add plugin configuration validation
   - Improve error handling

3. **Documentation**
   - Update frontend development guide
   - Add UI component documentation
   - Document plugin UI integration

## Long-term Goals

### 1. Core System

- Asynchronous plugin operations
- Improved error reporting
- Cross-platform testing
- Performance benchmarking

### 2. Plugin Ecosystem

- Additional compression formats
- Plugin marketplace
- Version management
- Dependency handling

### 3. User Experience

- Intuitive interface
- Comprehensive documentation
- Multi-language support
- Theme customization

## Technical Debt

### 1. Code Quality

- [ ] Comprehensive test coverage
- [ ] Documentation updates
- [ ] Code style consistency
- [ ] Performance profiling

### 2. Architecture

- [ ] Plugin system refinement
- [ ] Error handling standardization
- [ ] Logging system improvements
- [ ] Configuration management

## Core Principles

1. **Platform Independence**

   - All code must work across Windows, Linux, and macOS
   - Use Tauri's platform-agnostic APIs when possible
   - Platform-specific code must be properly isolated in dedicated modules
   - Test thoroughly on all supported platforms

2. **Dynamic Plugin System**

   - No hardcoded plugin references
   - Plugins are loaded and registered at runtime
   - Each plugin defines its own UI components and backend logic
   - Clear plugin API contracts and documentation

3. **Clean Architecture**

   - Clear separation between frontend (React) and backend (Rust/Tauri)
   - Use of TypeScript for type safety in frontend
   - Modular component design
   - Dependency injection for better testability

4. **Modern UI/UX**
   - Responsive design that works on all screen sizes
   - Accessibility first approach (WCAG compliance)
   - Dark/Light mode support
   - Consistent error handling and user feedback

## Development Setup

1. **Prerequisites** // React? Da Tauri / React Hybrid?

   ```bash
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Install Node.js (v18 or later)
   # Windows: Download from https://nodejs.org/

   # Install Tauri CLI
   cargo install tauri-cli

   # Install frontend dependencies
   cd frontend
   npm install
   ```

2. **Development Workflow**

   ```bash
   # Start frontend development server
   cd frontend
   npm run dev

   # In another terminal, start Tauri development
   cd rust/src-tauri
   cargo tauri dev
   ```

## Project Structure

- `/frontend`: React + TypeScript frontend

  - Uses Vite for fast development
  - Mantine UI components
  - TypeScript for type safety
  - Plugin-specific UI components

- `/rust/src-tauri`: Rust backend
  - Plugin system implementation
  - File operations
  - System integration via Tauri
  - Cross-platform compatibility layer

## Immediate Tasks

1. **Frontend**

   - [ ] Complete plugin UI integration
   - [ ] Implement responsive layouts
   - [ ] Add accessibility features
   - [ ] Implement error boundaries

2. **Backend**

   - [ ] Finalize plugin loading system
   - [ ] Implement cross-platform file operations
   - [ ] Add comprehensive error handling
   - [ ] Complete platform-specific optimizations

3. **Documentation**
   - [ ] Update API documentation
   - [ ] Complete plugin development guide
   - [ ] Add architecture diagrams
   - [ ] Document build process

## Build Process

1. **Development Build**

   ```bash
   # Frontend
   cd frontend
   npm run dev

   # Backend
   cd rust/src-tauri
   cargo tauri dev
   ```

2. **Production Build**

   ```bash
   # Frontend
   cd frontend
   npm run build

   # Full application
   cd rust/src-tauri
   cargo tauri build
   ```

## Testing

1. **Frontend Tests**

   ```bash
   cd frontend
   npm test
   ```

2. **Backend Tests**
   ```bash
   cd rust/src-tauri
   cargo test
   ```

## Debugging

1. **Frontend**

   - Chrome DevTools for React debugging
   - Vite's hot module replacement
   - React Developer Tools extension

2. **Backend**
   - Rust debugging with VS Code
   - Tauri Developer Tools
   - Console logging

## Contributing

1. Fork the repository
2. Create a feature branch
3. Follow code style guidelines
4. Add tests for new features
5. Update documentation
6. Submit pull request

## Code Style

1. **Frontend**

   - Follow React best practices
   - Use TypeScript strictly
   - Follow ESLint configuration
   - Use Prettier for formatting

2. **Backend**
   - Follow Rust style guidelines
   - Use clippy for linting
   - Document public APIs
   - Write comprehensive tests
