# Handwriting-keyboard-for-Linux.

This is program written for Linux desktop environment.

To recognize handwritten pattern program uses OCR engine.

At the moment program supports 2 OCR engines, which is: Tesseract OCR and PaddleOCR.

To send the keyboard input program uses xdotool or ydotool.

You can find compiled .deb and .AppImage packages in releases page.

This is the instruction for version 2, instruction for version 1 is located at v1 branch of this repository.

## How to use the program

0) Launch the program with or without command line options 
1) write text in the canvas by using mouse or stylus (on graphical tablet) 
2) press recognize button 
3) press to recognized text, programm will type this text or copy to clipboard and paste by triggering ctrl+V (or shift+ctrl+V) keypress

Note: before using the programm you need to install [dependencies](#installing-dependencies).

If programs window is in focus, before sending keyboard input program will trigger alt+Tab keypress to return focus to previous active window and only then send the input (this does not applied when '--skip-taskbar' option is set).

## Command line options 

```
Usage: handwriting-keyboard-t [OPTIONS]

Options:
      --use-paddle-ocr...
          Use PaddleOCR to recognize handwriting pattern. By default program uses Tesseract OCR.

  -l, --lang <lang>
          Language used to recognize handwriting pattern. Value depends on OCR engine. 
          Default value is 'chi_all' for Tesseract OCR and 'ch' for PaddleOCR.

      --tessdata-dir <tessdata-dir>
          A directory with *.traineddata files for Tesseract OCR engine. Tesseract OCR specefic.

  -a, --automode...
          Automatically send recognize text request to OCR engine after every stroke.

      --use-tmp-file...
          Save canvas as temporary image file and send path of this file to OCR engine. 
          By default program sends image data to OCR engine via stdin.

      --use-ydotool...
          Use ydotool to send keyboard input. By default program uses xdotool.

      --use-clipboard...
          Copy text to clipboard and paste it via triggering ctrl+V (or shift+ctrl+V) 
          kyepress to paste the text. 
          By default program will try to type text (ydotool only supports typing latin characters).

      --use-shift...
          Trigger shift+ctrl+V kyepress to paste text from clipboard. By default program uses ctrl+V. 
          Only applyed when '--use-clipboard' option is set.

      --return-focus...
          Program will return focus to previous window by triggering alt+Tab kyepress 
          every time when program gains focus (after mouseup event inside the window). 
          Will not work if option '--skip-taskbar' is set.

      --fly-to-bottom...
          At launch program window will fly to the bottom of the screen and resize to screen width.

      --skip-taskbar...
          Program window will skip taskbar.

      --dark-theme...
          Use dark theme. Change colors of the application to dark theme.

      --debug...
          Output some debug info in console.

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

Example of using command line options:

```
handwriting-keyboard-t --tessdata-dir=/home/user/ --lang=chi_sim -a
```

In this case (above), to recognize hand written pattern programm will use Tesseract OCR (as default OCR engine) with training data from folder "**/home/user/**" and language "**chi_sim**" (Chinese simplified), particularly the file "**/home/user/chi_sim.traineddata**". Also in this case the programm will automatically send request to tesseract-ocr after every stroke, because it was launched with "**-a**" parameter.

## Installing dependencies

1) Install preferred OCR engine Tesseract OCR or/and PaddleOCR.
    - in debian based linux system you can install Tesseract OCR from repository:
        ```
        sudo apt install tesseract-ocr
        ```
        for other linux distributions you can find instructions in their github repository: https://github.com/tesseract-ocr/tesseract?tab=readme-ov-file#installing-tesseract 
    - to install PaddleOCR, I would recommend to:
        1) set up conda envirenment by [this instruction](https://paddlepaddle.github.io/PaddleOCR/main/en/ppocr/environment.html), because PaddleOCR's python dependencies might conflict with existing python environment
        2) then activate this conda environment and install PaddleOCR:
            ```
            conda activate paddle_env
            
            python -m pip install paddlepaddle==3.0.0rc1 -i https://www.paddlepaddle.org.cn/packages/stable/cpu/
            
            pip install "paddleocr>=2.0.1"
            ```
            For different type of settings and instructions visit PaddleOCR's official website https://paddlepaddle.github.io/PaddleOCR/main/en/index.html

2) Install xdotool or/and ydotool.
    - in debian based linux system you can install xdotool from repository:
        ```
        sudo apt install xdotool
        ```
        for other linux distributions you can find instructions in their github repository: https://github.com/jordansissel/xdotool?tab=readme-ov-file#installation
    - to install ydotool, I would recommend:
        1) compile ydotool from source code in their repository: https://github.com/ReimuNotMoe/ydotool?tab=readme-ov-file#build
        2) then copy **ydotool** and **ydotoold** binaries into **~/bin** folder 

## Notes about dependencies

1) If you use the program with Tesseract OCR, I would recommend to install tesseract 4 (instead of tesseract 5). Because the results is the most accurate when using with tesseract 4 (at least for recognition of text (writing) in Chinese language).

2) If you use the  with Tesseract OCR, you also need to download model data for tesseract-ocr and copy [.traineddata](https://github.com/tesseract-ocr/tessdata) files to data folder of tesseract-ocr (for example for tesseract-ocr 4.0 it would be this folder **/usr/share/tesseract-ocr/4.00/tessdata/**). Or alternatively you can put these files in watever folder you like and run program with `--tessdata-dir` cli parameter and point to the folder where model data files are located.

3) By default program uses Tesseract OCR with language set as **chi_all**, *.traineddata files for which you can download by [this link](https://github.com/gumblex/tessdata_chi).

4) PaddleOCR is significantly more accurate than Tesseract OCR, at least in regognizing chinese characters, however it is also slower at least on my hardware.

5) If you use the program with PaddleOCR. PaddleOCR downloads model data at first use, then it can be used offline. List of available languages can be found by this [link](https://paddlepaddle.github.io/PaddleOCR/main/en/ppocr/blog/multi_languages.html#5-support-languages-and-abbreviations).

6) If you use the program with PaddleOCR and it is installed in conda environment, you neet to activete conda environment first, and then launch this program.

7) As for keyboard input:
    - **xdotool** - only supports X11 desktop environment 
    - **ydotool** - works in X11 and Wayland desktop environment ydotool can type only latin characters and **ydotoold** process should be running (in background or in separate terminal) in order to **ydotool** to work
    - instead of typing program can copy the text to clipboard and paste by trigerring ***ctrl+V*** (or ***shift+ctrl+V***)

## Some technical details

Program written by using tauri framework https://tauri.app/

The script from https://github.com/ChenYuHo/handwriting.js is used to make a writing canvas.

To recognize handwritten pattern program uses [Tesseract OCR](https://github.com/tesseract-ocr/tesseract) or [PaddleOCR](https://paddlepaddle.github.io/PaddleOCR/main/en/index.html).

To send keyboard input program uses [xdotool](https://github.com/jordansissel/xdotool) or [ydotool](https://github.com/ReimuNotMoe/ydotool).

In order to run from code or compile the programm: You need to install [Node.js 20](https://nodejs.org/en) or newer version and [Rust](https://www.rust-lang.org/) as well.

Install Node.js dependencies: `npm install`

Run program in development environment: `npm run tauri dev`

Run program in development environment with cli (command line) oprions: `npm run tauri dev -- -- -- cli_options`

Compile the programm: `npm run tauri build`

Older version of this program using Google API instead of tesseract-ocr is available by this link: [https://github.com/BigIskander/Handwriting-keyboard-for-Linux.](https://github.com/BigIskander/Handwriting-keyboard-for-Linux.)

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
