import { currentMonitor } from '@tauri-apps/api/window';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { LogicalSize, LogicalPosition } from '@tauri-apps/api/dpi';
import { invoke } from '@tauri-apps/api/core';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { getMatches } from '@tauri-apps/plugin-cli';
// @ts-ignore
import { autoCorrect } from "./stroke_autocorrect/strokeAutocorrect.js";
const appWindow = getCurrentWebviewWindow();
// @ts-ignore
var out: HTMLElement = document.getElementById('results');
// @ts-ignore
var recognize_button: HTMLElement = document.getElementById('recognize_button');
var recognize_button_link: HTMLElement = recognize_button.getElementsByTagName('a')[0];
// @ts-ignore
var isAutocorrectElement: HTMLElement = document.getElementById('isAutocorrect');
// @ts-ignore
var isPunctuationElement: HTMLElement = document.getElementById('isPunctuation');
// @ts-ignore
var isUndoElement: HTMLElement = document.getElementById('isUndo');
var isRecognizing = false;

function recognizing_style(is_recognizing: Boolean = true) {
    if(is_recognizing) {
        document.body.style.cursor = "wait";
        // @ts-ignore
        mycan.style.cursor = "wait";
        recognize_button.style.fontWeight = "normal";
        if(isAutorecognize)
            recognize_button.innerHTML = "<em class=\"internalLinkGreen\">Recognizing...</em>";
        else
            recognize_button_link.innerText = "Recognizing...";
    } else {
        document.body.style.cursor = "default";
        // @ts-ignore
        mycan.style.cursor = "crosshair";
        if(isAutorecognize)
            recognize_button.innerHTML = "";
        else
            recognize_button_link.innerText = "Recognize.";
    }
}

async function recognizeText() {
    if(!isRecognizing) {
        isRecognizing = true;
        if(isAutorecognize) isCanvasChanged = false;
        recognizing_style(true);
        // prepare image
        // @ts-ignore
        hidden_can.setAttribute("width", mycan.width);
        // @ts-ignore
        hidden_can.setAttribute("height", mycan.height);
        // @ts-ignore
        var tempContext = hidden_can.getContext("2d");
        if(is_dark_theme) {
            tempContext.fillStyle = "black";
        } else {
            tempContext.fillStyle = "white";
        }
        // @ts-ignore
        tempContext.fillRect(0, 0, mycan.width, mycan.height);
        // @ts-ignore
        tempContext.drawImage(mycan, 0, 0);
        // @ts-ignore
        var image_data = await hidden_can.toDataURL().split('base64,')[1];
        // send recognize request and get result
        // @ts-ignore
        await invoke('recognize_text', { 
            base64Image: image_data, isDarkTheme: is_dark_theme 
        }).then((response) => { 
            // @ts-ignore
            displayRecognizedText(response.replace(/(?:\r\n|\r|\n|\t)/g, ' ').replace(/(?:\s\s+)/g, ' ').trim(), null); 
            recognizing_style(false);
        }).catch((err) => { 
            displayRecognizedText("", err);
            recognizing_style(false);
        });
        isRecognizing = false;
        // run recognizeText again if canvas is changed 
        if(isAutorecognize && isCanvasChanged) recognizeText();
    }
}

function displayRecognizedText(text: any, err: any) {
    if(err) {
        out.innerHTML = '<div class="errorMessage">' + 
                             err.replaceAll("<", "&lt;").replaceAll(">", "&gt;") + 
                        '</div>'
    } else {
        if(text == "")
            out.innerHTML = '';
        else {
            out.innerHTML = '<div class="selectWordItem" onclick="choseWord(this.innerText)">' +
                                 text.replaceAll("<", "&lt;").replaceAll(">", "&gt;") + 
                            '</div>';
        } 
    }
}

var offset = 20;
var voffset = 100;
// var bottom_offset = 40;
var use_clipboard = false;
var is_dark_theme = false;
var show_grid = false;
var isAutorecognize = false;
var isCanvasChanged = false;

// @ts-ignore
var mycan: HTMLElement = document.getElementById('can');
mycan.setAttribute('width', String(window.outerWidth  - offset));
mycan.setAttribute('height', String(window.outerHeight  - voffset));
// @ts-ignore
var mycan_wrap: HTMLElement = document.getElementById('can_wrapper');
// @ts-ignore
var hidden_can: HTMLElement = document.getElementById('hiddenCanvas');
// @ts-ignore
var can;

(async () => {
    var args = await getMatches();
    is_dark_theme = Boolean(args.args["dark-theme"].value);
    // switch to dark theme
    if(is_dark_theme) {
        document.body.className = 'dark';
        document.body.style.backgroundColor = "black";
        document.body.style.color = "white";
    }
    // setting up writing canvas
    // @ts-ignore
    can = new handwriting.Canvas(mycan);
    can.setFillStyle("transparent");
    if(is_dark_theme) {
        can.setStrokeColor("white");
    } else {
        can.setStrokeColor("black");
    }
    //Set line width shown on the canvas element (default: 3)
    can.setLineWidth(5);
    // undo ?
    if(Boolean(args.args["allow-undo"].value)) {
        can.set_Undo_Redo(true, false);
        isUndoElement.style.visibility = "visible";
    }
    // add autocorrect capability
    if(Boolean(args.args["stroke-autocorrect"].value)) {
        can.setAutocorrect(true, autoCorrect);
        isAutocorrectElement.style.visibility = "visible";
    }
    // add common Chinese punctuation
    if(Boolean(args.args["common-punctuation"].value)) {
        isPunctuationElement.style.visibility = "visible";
    }
    // change canvas size if needed
    if(Boolean(args.args["stroke-autocorrect"].value) || Boolean(args.args["common-punctuation"].value)) {
        voffset = 110;
        mycan.setAttribute('height', String(window.outerHeight  - voffset));
        // @ts-ignore
        can.height = window.outerHeight - voffset;
        can.setFillStyle("transparent");
        if(is_dark_theme) {
            can.setStrokeColor("white");
        } else {
            can.setStrokeColor("black");
        }
    }
    // set background grid
    show_grid = Boolean(args.args["show-grid"].value);
    if(show_grid) {
        mycan_wrap.className = "can_wrapper";
        // @ts-ignore
        mycan_wrap.style.backgroundSize = mycan.height + "px " + mycan.height + "px";
    }
    // ...
    window.onresize = () => { 
        mycan.setAttribute('width', String(window.outerWidth - offset));
        mycan.setAttribute('height', String(window.outerHeight - voffset));
        // @ts-ignore
        if(show_grid) mycan_wrap.style.backgroundSize = mycan.height + "px " + mycan.height + "px";
        // @ts-ignore
        can.height = window.outerHeight - voffset;
        // @ts-ignore
        can.width = window.outerWidth - offset;
        // @ts-ignore
        can.setFillStyle("transparent");
        // @ts-ignore
        if(is_dark_theme) {
            // @ts-ignore
            can.setStrokeColor("white");
        } else {
            // @ts-ignore
            can.setStrokeColor("black");
        }
    };
    isAutorecognize = Boolean(args.args.automode.value);
    if (isAutorecognize) {
        recognize_button.innerHTML = "";
        can.setMouseUpCallBack(() => { isCanvasChanged = true; recognizeText(); });
    } else {
        can.setMouseUpCallBack(() => { recognize_button.style.fontWeight = "bold"; });
    }
    // change window size and position on launch
    const monitor = await currentMonitor();
    if (monitor) {
        // appWindow.center(); // this function not working
        if(args.args["fly-to-bottom"].value == true) {
            appWindow.setSize(new LogicalSize(monitor.size.width, 300));
            appWindow.setPosition(new LogicalPosition(
                monitor.position.x, monitor.position.y + monitor.size.height - window.outerHeight // - bottom_offset
            ));
        }
    }
    use_clipboard = Boolean(args.args["use-clipboard"].value);
    if(args.args["return-focus"].value == true && args.args["skip-taskbar"].value == false) {
        document.addEventListener("mouseup", async () => { 
            setTimeout(async () => { if(await appWindow.isFocused()) invoke('alt_tab'); }, 100); 
        });
        document.addEventListener("touchend", async () => { 
            setTimeout(async () => { if(await appWindow.isFocused()) invoke('alt_tab'); }, 100); 
        });
        // if(await appWindow.isFocused()) invoke('alt_tab'); // workaround to return focus to previous active window
    }
})();

function erase() {
    if(!isRecognizing) {
        if(isAutorecognize) isCanvasChanged = false;
        // @ts-ignore
        can.erase();
        out.innerHTML = "";
        recognize_button.style.fontWeight = "normal";
    }
}

function undo() {
    // @ts-ignore
    can.undo();
    // @ts-ignore
    if(isAutorecognize) {
        // @ts-ignore
        if(can.step.length != 0) { isCanvasChanged = true; recognizeText(); }
        else { isCanvasChanged = false; out.innerHTML = "";  }
    } else {
        // @ts-ignore
        if(!isRecognizing && can.step.length == 0) recognize_button.style.fontWeight = "normal";
        else recognize_button.style.fontWeight = "bold";
    }
}

async function choseWord(word: String, is_erase: Boolean = true) {
    if(!isRecognizing) {
        if(use_clipboard == true) await writeText(String(word));
        await invoke('write_text', { 
            text: word, useClipboard: use_clipboard 
        }).then(() => { 
            if(is_erase) erase(); 
        }).catch((err) => { displayRecognizedText("", err); });
    }
}

function setAutocorrect(value: boolean) {
    // @ts-ignore
    can.setAutocorrect(value);
}

export {
    erase,
    choseWord,
    recognizeText,
    setAutocorrect,
    undo
}