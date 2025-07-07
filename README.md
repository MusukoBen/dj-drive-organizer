# 🎵 DJ Drive Organizer

Eine moderne DJ-Drive-Management-Anwendung, entwickelt mit **Rust + Yew** und **Tauri**.

## ✨ Features

### 🎛️ Track-Management
- **Track-Tabelle** mit Status-Indikatoren (✓ Synced, ⚠ Warning, ✗ Error)
- **Metadaten-Anzeige**: Title, Artist, BPM, Genre, Duration
- **Partition-Zuordnung** mit farbigen Indikatoren
- **Echtzeit-Suche** und Filter-Funktionen

### 💾 Laufwerk-Management
- **Multi-Drive-Support**: DJ-Main, Backup, External Drives
- **Speicherplatz-Überwachung** mit visuellen Indikatoren
- **Verbindungsstatus** für alle Laufwerke
- **Add Drive** und **Partition Manager** Funktionen

### 🔄 Sync-Funktionalitäten
- **Quick Sync** für schnelle Synchronisation
- **Health Check** für Datenintegrität
- **Sync Profiles** für automatisierte Workflows
- **Status-Tracking** für alle Sync-Operationen

### 🎨 Modernes UI
- **Dark Theme** mit professionellem Design
- **Responsive Layout** für verschiedene Bildschirmgrößen
- **Tailwind CSS** für konsistentes Styling
- **Smooth Animations** und Übergänge

## 🚀 Schnellstart

### Voraussetzungen
- Rust (neueste stabile Version)
- Node.js und npm
- Trunk für Yew-Builds

### Installation

1. **Repository klonen**
   ```bash
   git clone <repository-url>
   cd dj-drive-organizer
   ```

2. **Dependencies installieren**
   ```bash
   # Rust Dependencies
   cargo build
   
   # Frontend Dependencies (falls erforderlich)
   cd frontend
   cargo build
   ```

3. **Trunk installieren** (falls nicht vorhanden)
   ```bash
   cargo install trunk
   ```

### 🏃‍♂️ Entwicklung starten

#### Voraussetzungen
Bevor du mit der Entwicklung beginnst, stelle sicher, dass folgende Tools installiert sind:

```bash
# Rust WASM-Target installieren
rustup target add wasm32-unknown-unknown

# Trunk installieren (falls nicht vorhanden)
cargo install trunk
```

#### Entwicklungsworkflow

**Option 1: Fullstack-Entwicklung (empfohlen)**
```bash
# Startet Backend + Frontend mit Hot Reload
cargo tauri dev
```

**Option 2: Nur Frontend-Entwicklung**
```bash
# Nur Frontend mit Hot Reload
trunk serve --open

# Oder mit spezifischem Port
trunk serve --port 8080 --open
```

### 🔨 Build für Produktion

```bash
# Frontend build
trunk build --release

# Tauri App build (falls konfiguriert)
cargo tauri build
```

## 📁 Projektstruktur

```
dj-drive-organizer/
├── frontend/src/
│   ├── components.rs    # Haupt-App-Komponente
│   ├── pages.rs         # UI-Seiten (MainPage, Sidebar, Header)
│   ├── services.rs      # Business-Logik und State-Management
│   ├── types.rs         # Datenstrukturen und Enums
│   ├── tauri_api.rs     # Tauri-Backend-Integration
│   └── lib.rs           # Entry Point
├── src-tauri/           # Tauri Backend
├── index.html           # HTML Template mit Tailwind CSS
└── README.md
```

## 🧠 Architektur

### State Management
- **Zentraler AppState** mit `Rc<RefCell<T>>`
- **Context API** für State-Sharing zwischen Komponenten
- **Service Layer** für Business-Logik-Trennung

### Komponenten-Architektur
- **Smart-Dumb-Pattern**: Logik vs. Darstellung getrennt
- **Modulare Struktur**: Wiederverwendbare Komponenten
- **Props-basierte Kommunikation** mit TypeScript-ähnlicher Typsicherheit

### Design Patterns
- **Observer Pattern** für Reaktivität
- **Command Pattern** für Event-Handling
- **Builder Pattern** für komplexe State-Initialisierung

## 🎯 Roadmap

### Phase 1: Core Features ✅
- [x] Basis-UI mit Track-Tabelle
- [x] Laufwerk-Management
- [x] Such- und Filter-Funktionen
- [x] Status-Indikatoren

### Phase 2: Advanced Features 🚧
- [ ] Tauri-Backend-Integration
- [ ] Echte Dateisystem-Scans
- [ ] Playlist-Management
- [ ] Sync-Profile-Konfiguration

### Phase 3: Pro Features 📋
- [ ] Audio-Metadaten-Extraktion
- [ ] BPM-Analyse
- [ ] Duplicate-Detection
- [ ] Backup-Automation

## 🛠️ Entwicklung

### Code-Style
Folgt den **Windsurf Cascade Rules** für Rust + Yew:
- Modulare, kontextorientierte Struktur
- Strikte Trennung von View/Logic/Data
- Idiomatische Yew-Hooks und Patterns
- Typ-Sicherheit mit `Result` und `Option`

### Testing
```bash
# Unit Tests
cargo test

# Frontend Tests
cd frontend && cargo test
```

### Debugging
- Browser DevTools für Frontend-Debugging
- `log::info!()` für strukturiertes Logging
- Debug-Komponenten für State-Inspektion

## 📄 Lizenz

MIT License - siehe [LICENSE](LICENSE) für Details.

## 🤝 Contributing

1. Fork das Repository
2. Erstelle einen Feature-Branch (`git checkout -b feature/amazing-feature`)
3. Commit deine Changes (`git commit -m 'Add amazing feature'`)
4. Push zum Branch (`git push origin feature/amazing-feature`)
5. Öffne einen Pull Request

---

**Entwickelt mit ❤️ und Rust 🦀**
