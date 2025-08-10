# Chronodoro
![Static Badge](https://img.shields.io/badge/Rust-Chronodoro-blue)


A fast, portable, and cross‑platform Pomodoro timer built with **Rust** + **Egui**. Designed to run without installation — just copy the executable to your machine or USB drive and go.

---

## Features

* **Linux Support**: Runs on most Linux distributions
* **Portable**: No installer needed — configuration is stored alongside the binary.
* **Customizable**: Set focus, short break, and long break durations.
* **Auto‑start**: Optionally start the next phase automatically.
* **Session tracking**: Automatically switch to long breaks after a set number of focus sessions.
* **In progress**: Tray and notification support, cross-platform releases for Windows 10/11, and macOS (Intel & ARM) in progress! 
---

## Download & Run

### Linux

1. Download the latest `chronodoro-linux` artifact from the [Releases](../../releases) page.
2. Extract the archive.
3. Make it executable:

   ```bash
   chmod +x chronodoro
   ```
4. Run it:

   ```bash
   ./chronodoro
   ```

### Windows / macOS - In Work

> **Tip:** Keep `pomodoro_config.json` in the same folder/directory as the executable to preserve settings.

---

## ⚙️ Configuration

`pomodoro_config.json` (auto‑created if missing):

```json
{
  "focus_minutes": 25,
  "short_break_minutes": 5,
  "long_break_minutes": 15,
  "sessions_until_long_break": 4,
  "auto_start_next": false
}
```

Edit this file with your preferred durations and behavior.

---

## 🛠 Build from Source

Requirements:

* Rust (stable)
* System dependencies (Linux):

  ```bash
  sudo apt-get update
  sudo apt-get install -y pkg-config libx11-dev libasound2-dev \
    libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
  ```

Build:

```bash
RUSTFLAGS="-C target-cpu=x86-64-v2" cargo build --release
```

The binary will be in `target/release/chronodoro`.

---

## Development

I use **GitHub Actions** for CI/CD:

* **Linux** builds run on `ubuntu-latest` with portable CPU flags.
* Optional builds for Windows/macOS with cross‑compilation or runners in work.
* Artifacts are zipped and uploaded for quick download.

---

## License

MIT License — See [LICENSE](LICENSE-MIT) for details.

APACHE License - See [LICENSE](LICENSE-APACHE) for details

Derived from dependencies - if you see a discrepancy, let me know! I am not trying to steal anything. 

## Stats
![GitHub commit activity](https://img.shields.io/github/commit-activity/t/enachtigall245/Chronodoro-rs)
