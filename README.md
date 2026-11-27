# LazyFlow

**A project-aware terminal tool for running everyday development commands without having to remember them.**

LazyFlow is a Rust-based command-line application designed to reduce repetitive typing when switching between software projects. Instead of memorizing build, run, and test commands for each technology stack, developers can open LazyFlow in a project folder, choose a task, and run it.

## Tech Stack

- **Rust** — application logic
- **Ratatui** — terminal user interface (planned)
- **Crossterm** — keyboard and terminal events (planned)
- **Serde / serde_json** — reading Node.js project scripts (planned)
- **Cargo** — Rust build and dependency management

## Installation

### Prerequisites

- Rust and Cargo, installed through [rustup](https://rustup.rs/)
- Git

Confirm Rust is installed:

```bash
rustc --version
cargo --version
```

### Build from source

Clone your LazyFlow repository:

```bash
git clone https://github.com/RenzDureza/lazyflow.git
cd LazyFlow
cargo build
```

Run the development version:

```bash
cargo run
```

After a release build, the executable will be in `target/release/`:

```bash
cargo build --release
```

> The `LazyFlow` command will only work globally after you install the binary or add it to your `PATH`. Once the application is ready, you can install it locally from the project folder using `cargo install --path .`.

## Usage

1. Open your terminal in a Rust or Node.js project directory.
2. Start LazyFlow using `lazyflow` (if installed), or run it from source with `cargo run` inside the LazyFlow repository.
3. Use the **Up/Down** arrow keys to select a task.
4. Press **Enter** to execute the selected task, or **Q** to exit.

## Project Structure

```text
LazyFlow/
├── src/
│   ├── main.rs       # Application entry point
│   ├── project.rs    # Project detection
│   ├── task.rs       # Task discovery and definitions
│   ├── runner.rs     # Command execution
│   └── ui.rs         # Terminal interface
├── tests/            # Integration tests
├── Cargo.toml
├── Cargo.lock
└── README.md
```

## Development Roadmap

- [x] **Phase 1 — Foundations:** Set up the Rust project; detect Rust and Node.js projects.
- [ ] **Phase 2 — Task discovery:** Create task definitions and parse `package.json` scripts.
- [ ] **Phase 3 — Command execution:** Run selected tasks and handle errors safely.
- [ ] **Phase 4 — Terminal UI:** Add a keyboard-driven interface with Ratatui.
- [ ] **Phase 5 — Testing and release:** Add tests, run formatting and lint checks, and prepare documentation.

### Future Improvements

- [ ] **Custom workflows:** Define shortcuts and multi-step tasks in a `.LazyFlow.toml` file.
- [ ] **More language support:** Detect Python, Go, and other project types.
- [ ] **Monorepo support:** Select among multiple applications or stacks in one repository.
- [ ] **Live output viewer:** View logs and process output without leaving the interface.
- [ ] **Command history:** Re-run recent tasks and inspect their exit status.
- [ ] **Git integration:** Access common repository actions from the same interface.
- [ ] **User-defined task favorites:** Pin frequently used commands.
