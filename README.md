<div align="center">
  <pre>
╔════════════════════════════════════╗
║                                    ║
║  ███████╗███████╗████████╗        ║
║  ╚══███╔╝╚══███╔╝╚══███╔╝        ║
║    ███╔╝   ███╔╝   ███╔╝         ║
║   ███╔╝   ███╔╝   ███╔╝          ║
║  ███████╗███████╗███████╗         ║
║  ╚══════╝╚══════╝╚══════╝         ║
║                                    ║
║  Zeta Toolchain Installer · v0.1.0║
╚════════════════════════════════════╝
  </pre>
</div>

<div align="center">
  <h1>⚡ zippy</h1>
  <p><strong>Install Zeta in one command.</strong></p>
  <p>
    <a href="https://github.com/murphsicles/zippy/releases"><img src="https://img.shields.io/github/v/release/murphsicles/zippy?color=EAB308&label=version&style=flat-square" alt="Version"></a>
    <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-EAB308?style=flat-square" alt="License"></a>
    <a href="https://z-lang.org"><img src="https://img.shields.io/badge/z-lang.org-0D1117?style=flat-square&color=0D1117" alt="Website"></a>
    <a href="https://zorbs.io/docs"><img src="https://img.shields.io/badge/docs-zorbs.io-EAB308?style=flat-square" alt="Docs"></a>
  </p>
</div>

---

## 🚀 One-liner

```bash
curl -sSf https://raw.githubusercontent.com/murphsicles/zippy/main/install.sh | sh
zippy install
```

That's it. No package managers, no dependencies, no Python, no Node.

---

## 📦 Commands

| Command | What it does |
|---------|-------------|
| `zippy install` | Download & install the latest Zeta compiler |
| `zippy update` | Upgrade to a newer version |
| `zippy status` | Check what's installed and where |
| `zippy list` | Show all installed Zeta versions |
| `zippy default v1.0.18` | Pin a specific version |
| `zippy uninstall` | Clean removal, undo everything |
| `zippy doctor` | Diagnose and fix installation issues |

---

## 🎨 Terminal Experience

Zippy uses the official Zeta color palette:

```
      Background: #0D1117  (deep black)
      Surface:    #151b23  (dark grey)
      Text:       #D8DEE9  (light grey)
      Accent:     #EAB308  (amber gold)
```

When you run `zippy install`, you get:

```
╔══════════════════════════════════════╗
║         ███████╗███████╗            ║
║         ╚══███╔╝╚══███╔╝            ║
║           ███╔╝   ███╔╝             ║
║          ███╔╝   ███╔╝              ║
║         ███████╗███████╗            ║
║         ╚══════╝╚══════╝            ║
║           Zeta Installer             ║
╚══════════════════════════════════════╝

  ▶ Detecting platform... ✓ Linux x86_64
  ▶ Creating directories... ✓ ~/.zeta/
  ▶ Downloading zetac... ████████████ 18.1 MB
  ▶ Verifying... ✓ zetac is ready
  ▶ Configuring shell... ✓ ~/.zshrc

  ╔════════════════════════════════════╗
  ║  ✓ Zeta v1.0.18 installed!        ║
  ║                                    ║
  ║    zetac --version                 ║
  ║    zetac --zorb search nour        ║
  ║                                    ║
  ║    z-lang.org  ·  zorbs.io/docs   ║
  ╚════════════════════════════════════╝
```

---

## 🔧 How it works

1. **Bootstrap** — `install.sh` downloads the zippy binary from GitHub releases
2. **Install** — `zippy install` detects your platform, downloads the right `zetac` build
3. **Verify** — checks the binary is valid and executable
4. **Configure** — adds `~/.zeta/bin` to your shell PATH (bash, zsh, or fish)
5. **Done** — run `zetac` immediately

Zippy is a static musl binary with zero runtime dependencies. No OpenSSL, no libc quirks, no system package manager required.

---

## 🌐 Roadmap

- [x] Linux x86_64 support
- [ ] Linux ARM64
- [ ] macOS (Intel + Apple Silicon)
- [ ] Windows (x86_64)
- [ ] Self-update (`zippy self-update`)
- [ ] Nightly/stable channels
- [ ] LLVM dependency management
- [ ] Port to pure Zeta after self-hosting milestone

---

## 🏗️ Building from source

```bash
git clone https://github.com/murphsicles/zippy.git
cd zippy
cargo build --release
./target/release/zippy install
```

Requires Rust 1.75+.

---

## 📄 License

MIT — see [LICENSE](LICENSE).

Built by [The Zeta Foundation Dark Factory](https://z-lang.org).

---

<div align="center">
  <sub>⚡ First principles. Final language.</sub>
</div>
