import { currentMonitor } from '@tauri-apps/api/window';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { LogicalSize, LogicalPosition } from '@tauri-apps/api/dpi';
import { invoke } from '@tauri-apps/api/core';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { getMatches } from '@tauri-apps/plugin-cli';
const appWindow = getCurrentWebviewWindow();
// @ts-ignore
var out: HTMLElement = document.getElementById('results');
// @ts-ignore
var recognize_button: HTMLElement = document.getElementById('recognize_button');

async function recognizeText() {
    // @ts-ignore
    var image_data = await mycan.toDataURL().split('base64,')[1];
    // @ts-ignore
    await invoke('recognize_text', { 
        base64Image: image_data, isDarkTheme: is_dark_theme 
    }).then((response) => { 
        // @ts-ignore
        displayRecognizedText(response.replace(/(?:\r\n|\r|\n|\t)/g, ' ').replace(/(?:\s\s+)/g, ' ').trim(), null); 
    }).catch((err) => { 
        displayRecognizedText("", err) 
    });
    recognize_button.style.fontWeight = "normal";
}

function displayRecognizedText(text: any, err: any) {
    if(err) {
        out.innerHTML = '<div class="errorMessage">' + err + '</div>'
    } else {
        out.innerHTML = '<div class="selectWordItem" onclick="choseWord(\'' + text + '\')">' + text + '</div>';
    }
}

var offset = 20;
var voffset = 100;
// var bottom_offset = 40;
var use_clipboard = false;
var is_dark_theme = false;

// @ts-ignore
var mycan: HTMLElement = document.getElementById('can');
mycan.setAttribute('width', String(window.outerWidth  - offset));
mycan.setAttribute('height', String(window.outerHeight  - voffset));
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
    can = new handwriting.Canvas(mycan, is_dark_theme);
    if(is_dark_theme) can.setStrokeColor("white");
    //Set line width shown on the canvas element (default: 3)
    can.setLineWidth(5);
    window.onresize = () => { 
        mycan.setAttribute('width', String(window.outerWidth - offset));
        mycan.setAttribute('height', String(window.outerHeight - voffset));
        // @ts-ignore
        can.height = window.outerHeight - voffset;
        // @ts-ignore
        can.width = window.outerWidth - offset;
        // @ts-ignore
        if(is_dark_theme) can.setStrokeColor("white");
    };
    if (args.args.automode.value == true) {
        recognize_button.innerHTML = "";
        can.setMouseUpCallBack(() => recognizeText());
    } else {
        can.setMouseUpCallBack(() => { recognize_button.style.fontWeight = "bold" });
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
            if(await appWindow.isFocused()) invoke('alt_tab'); 
        });
        document.addEventListener("touchend", async () => { 
            if(await appWindow.isFocused()) invoke('alt_tab'); 
        });
        if(await appWindow.isFocused()) invoke('alt_tab'); // workaround to return focus to previous active window
    }
})();

function erase() {
    // @ts-ignore
    can.erase();
    out.innerHTML = "";
    recognize_button.style.fontWeight = "normal";
}

async function choseWord(word: String) {
    var in_focus = await appWindow.isFocused();
    if(use_clipboard == true) await writeText(String(word));
    await invoke('write_text', { 
        text: word, inFocus: in_focus, useClipboard: use_clipboard 
    }).then(() => { erase(); }).catch((err) => { displayRecognizedText("", err); });
}

export {
    erase,
    choseWord,
    recognizeText
}