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
    await invoke('recognize_text', {base64Image: image_data}).then((response) => { displayRecognizedText(response.replace(/(?:\r\n|\r|\n|\t)/g, ' ').replace(/(?:\s\s+)/g, ' ').trim(), null); }).catch((err) => { displayRecognizedText("", err) });
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
var bottom_offset = 40;
var use_clipboard = false;
(async () => {
    var args = await getMatches();
    if (args.args.automode.value == true) {
        recognize_button.innerHTML = "";
        can.setMouseUpCallBack(() => recognizeText());
    } else {
        can.setMouseUpCallBack(() => { recognize_button.style.fontWeight = "bold" });
    }
    const monitor = await currentMonitor();
    if (monitor) {
        await appWindow.setSize(new LogicalSize(monitor.size.width, 300));
        await appWindow.setPosition(new LogicalPosition(monitor.position.x, monitor.position.y + monitor.size.height - window.outerHeight - bottom_offset));
    }
    if(args.args["not-return-focus"].value == false) {
        document.addEventListener("mouseup", () => { invoke('alt_tab'); });
        document.addEventListener("touchend", () => { invoke('alt_tab'); });
        if(await appWindow.isFocused()) invoke('alt_tab');
    }
    use_clipboard = Boolean(args.args["use_clipboard"].value);
})();
// @ts-ignore
var mycan: HTMLElement = document.getElementById('can');
mycan.setAttribute('width', String(window.outerWidth  - offset));
mycan.setAttribute('height', String(window.outerHeight  - voffset));
window.onresize = () => { 
    mycan.setAttribute('width', String(window.outerWidth - offset));
    mycan.setAttribute('height', String(window.outerHeight - voffset));
    can.height = window.outerHeight - voffset;
    can.width = window.outerWidth - offset;
};

// @ts-ignore
var can = new handwriting.Canvas(mycan);
//Set line width shown on the canvas element (default: 3)
can.setLineWidth(5);

function erase() {
    can.erase();
    out.innerHTML = "";
    recognize_button.style.fontWeight = "normal";
}

async function choseWord(word: String) {
    var in_focus = await appWindow.isFocused();
    if(use_clipboard == true) await writeText(String(word));
    await invoke('write_text', { text: word, inFocus: in_focus, useClipboard: use_clipboard }).then(() => { erase(); }).catch((err) => { displayRecognizedText("", err); });
}

export {
    erase,
    choseWord,
    recognizeText
}