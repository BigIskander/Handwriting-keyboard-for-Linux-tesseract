# Experimenting with tesseract

This is experimental, unfinished and not working version of program. For stable version go to main branch.

Install tesseract: <code>sudo apt install tesseract-ocr</code>

Training data to recognize chinese language [https://github.com/gumblex/tessdata_chi](https://github.com/gumblex/tessdata_chi)

Testing offline capabilities...

# Handwriting-keyboard-for-Linux.

This is programm written for Linux X11 desktop environment.

Currently supports only Chinese language.

You can find compiled .deb package in releases page.

In order to use the programm xdotool package should be installed: <code>sudo apt install xdotool</code>

# Some technical details

Programm written by using tauri framework https://tauri.app/

Program based on script from https://github.com/ChenYuHo/handwriting.js

To recognize handwritten pattern program uses Google API.

You can change language of recognition by editing this line <code>language: 'zh-CN',</code> in /src/main.ts file.

In order to run from code or compile the programm: You need to install [Node.js 18](https://nodejs.org/en) or newer version and [Rust](https://www.rust-lang.org/) as well.

Install Node.js dependencies: <code>npm istall</code>

Run program in development environment: <code>npm run tauri dev</code>

Compile the programm: <code>npm run tauri build</code>

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
