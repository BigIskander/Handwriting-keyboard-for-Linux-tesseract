# Handwriting-keyboard-for-Linux.

This is programm written for Linux X11 desktop environment.

To recognize handwritten pattern program uses tesseract-ocr.

You can find compiled .deb package in releases page.

# How to use

1) Install dependencies:

<code>sudo apt install xdotool</code>

<code>sudo apt install tesseract-ocr</code>

2) Install programm (you can find compiled <b>.deb</b> package in releases page)

3) Download training data for tesseract-ocr and copy training data files to data folder of tesseract-ocr (for example for tesseract-ocr 4.0 it would be ths folder <b>/usr/share/tesseract-ocr/4.00/tessdata/</b>).

Or alternatively you can put these files in watever folder you like and run program with <code>--tessdata-dir</code> cli parameter and point to the folder where training data files are located.

By default program uses language <b>chi_all</b>, which you can download from this source [https://github.com/gumblex/tessdata_chi](https://github.com/gumblex/tessdata_chi), or you can select desired language by running program with cli parameter <code>--lang</code> and set language.

4) Launch the program with or without cli parameters <code>handwriting-keyboard-t</code> and just use it.

# CLI (command line interface) parameters

<code>--lang</code> or <code>-l</code> - language used to recognize handwritten pattern.

<code>--tessdata-dir</code> - custom folder where is located the training data (for tesseract-ocr) used to recognize handwriting pattern.

<code>--automode</code> or <code>-a</code> - programm will send request to tesseract-ocr automatically after every stroke.

Example:

<code>handwriting-keyboard-t --tessdata-dir=/home/user/ --lang=chi_sim -a</code>

In this case (above), to recognize hand written pattern programm will use training data from folder "<b>/home/user/</b>" and language "<b>chi_sim</b>" (Chinese simplified), particularly the file "<b>/home/user/chi_sim.traineddata</b>". And also automatically send data to tesseract-ocr after every stroke, because program was launched with "<b>-a</b>" parameter.

# Some technical details

Programm written by using tauri framework https://tauri.app/

The script from https://github.com/ChenYuHo/handwriting.js is used to make a writing canvas.

To recognize handwritten pattern program uses [tesseract-ocr](https://github.com/tesseract-ocr/tesseract).

In order to run from code or compile the programm: You need to install [Node.js 18](https://nodejs.org/en) or newer version and [Rust](https://www.rust-lang.org/) as well.

Install Node.js dependencies: <code>npm istall</code>

Run program in development environment: <code>npm run tauri dev</code>

Run program in development environment with cli (command line) parameters: <code>npm run tauri dev -- -- -- cli_parameters</code>

Compile the programm: <code>npm run tauri build</code>

Older version of this program using Google API instead of tesseract-ocr is available by this link: [https://github.com/BigIskander/Handwriting-keyboard-for-Linux.](https://github.com/BigIskander/Handwriting-keyboard-for-Linux.)

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
