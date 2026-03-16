# RustForge Terminal (TUI)

The TUI crate (`rust-finance/crates/tui`) represents the core visual interface of the RustForge trading platform. Engineered using `ratatui` and `crossterm`, it provides an ultra low-latency, multi-column dashboard mirroring professional desktop terminals like Bloomberg and Reuters Eikon.

It achieves true decoupling by acting solely as a subscriber to the `event_bus`, consuming asynchronous `BotEvent` streams without blocking its render loop.

## Architecture & State Management

The TUI maintains a continuous paint cycle targeting 60 frames per second. The state is governed by the `App` struct (`app.rs`), which models:
* Interactive charts with internal Viewport tracking
* Deep Order Book data
* Active multi-exchange connection statuses
* AI Agent intelligence events (Dexter & Mirofish)
* Live portfolio positioning

### Key Modules

*   **`main.rs`**: Handles terminal initialization (entering alternate screens, enabling raw mode), spawns the TCP event consumption thread, deserializes events, and orchestrates the massive 3-column layout rendering pass.
*   **`app.rs`**: The unified state container. Contains methods to mutate the state safely from incoming network events and provides interactive placeholders for TUI actions.
*   **`event_handler.rs`**: Manages all keyboard input routing, dispatching keystrokes directly to the `App` state mutators for chart panning, zooming, and dialog management.
*   **`widgets/`**: Contains specialized, highly complex drawing code.
    *   **`chart_widget.rs`**: A custom-built Bloomberg-style charting engine. Features algorithmic gradient filling using braille characters, lower volume histograms, time-range cycling (1D, 1W, 1Y), horizontal panning, scaling, and real-time moving average implementations.

## Controls & Keybindings

The terminal relies on intuitive keyboard bindings to navigate the data streams and execute trades instantly.

### Global Nav
*   `q` or `Ctrl+C`: Quit the application
*   `Esc`: Dismiss active dialogs/alerts

### Interactive Chart
*   `t`: Cycle time ranges (1D -> 1W -> 1M -> 6M -> 1Y -> ALL)
*   `+` or `Ctrl+Up`: Zoom In
*   `-` or `Ctrl+Down`: Zoom Out
*   `Shift+Right` or `Shift+L`: Pan Right (Into the future/most recent)
*   `Shift+Left`  or `Shift+H`: Pan Left (Into history)

### Trading Primitives
*   `b`: Open Buy Modal (Starts institutional bracket configuration)
*   `s`: Open Sell Modal (Starts institutional bracket configuration)
*   `c`: Cancel pending order selected in OMS
*   `C` (Shift+C): Cancel ALL pending orders globally
*   `z`: Halve the currently selected open position (MKT order)
*   `Z` (Shift+Z): Close the currently selected entire position (MKT order)
*   `Enter`: Confirm dialog/submission

### AI Engines
*   `x`: Request Dexter Analyst fundamental analysis
*   `m`: Trigger MiroFish 5,000-agent swarm intelligence probability simulation
*   `a`: Toggle full Auto-Trade executing mode
*   `p`: Cycle AI confidence threshold limits (e.g. Medium > High > Extreme)

## Theming Constraints

The UI utilizes a strict hex color palette implemented via `Color::Rgb` to guarantee aesthetic continuity regardless of the end user's terminal color profile:

*   **Background**: `10, 12, 15` (Jet Black)
*   **Borders/Grid**: `30, 37, 48` (Slate Muted)
*   **Bulls/Greens**: `74, 222, 128` (Neon Green)
*   **Bears/Reds**: `248, 113, 113` (Soft Crimson)
*   **Information**: `96, 165, 250` (Sky Blue)
*   **Text Primary**: `226, 232, 240` (Slate-100)
