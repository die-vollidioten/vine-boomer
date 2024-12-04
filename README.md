# Vine Boomer 🔊

A fun desktop app that randomly plays the iconic Vine Boom sound effect while you're using your computer. Perfect for streamers, content creators, or anyone who enjoys a good meme!

## Features
- 🎵 Random Vine Boom sounds
- ⏰ Customizable intervals
- 🎲 Rare special effects
- 📊 Statistics tracking
- 🚀 Auto-start with system
- 🎯 System tray integration

## Download

Get the latest version for your operating system:
- [Windows](https://github.com/die-vollidioten/vine-boomer/releases/latest)
- [macOS](https://github.com/die-vollidioten/vine-boomer/releases/latest)
- [Linux](https://github.com/die-vollidioten/vine-boomer/releases/latest)

## Website

Visit our [website](https://die-vollidioten.de/vine-boomer) for more information and updates.

## Building from Source

### Prerequisites

Before building, ensure you have:
- [Node.js](https://nodejs.org/) (Latest LTS version recommended)
- [Rust](https://rustup.rs/) and Cargo (via rustup)

### Build Steps

1. Clone the repository:

```bash
git clone https://github.com/die-vollidioten/vine-boomer
cd vine-boomer
```

2. Install dependencies:

```bash
npm install --force  # --force is required due to React 19/Next.js 15 dependency conflicts
```

3. Build the application:

```bash
npm run tauri build
```

### Note About Code Signing

When building, you'll receive a warning about code signing. This is normal! The warning appears because the application isn't signed with a certificate. 

For personal use, you can safely ignore this warning. The application will still work, but your OS might show security warnings when running it for the first time.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Support

Having issues? Check out our:
- [FAQ](https://die-vollidioten.de/vine-boomer/faq)
- [Discord Server](https://discord.gg/your-discord)
- [GitHub Issues](https://github.com/die-vollidioten/vine-boomer/issues)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Credits

- Original Vine Boom Sound Effect
- Built with [Tauri](https://tauri.app/), [Next.js](https://nextjs.org/), and [Rust](https://www.rust-lang.org/)
- Created by [die-vollidioten](https://die-vollidioten.de)
