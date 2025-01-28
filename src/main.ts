import { currentMonitor } from '@tauri-apps/api/window';
import { appWindow, LogicalSize, LogicalPosition } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/tauri';
import { writeText } from '@tauri-apps/api/clipboard';
import { getMatches } from '@tauri-apps/api/cli'
// @ts-ignore
var out: HTMLElement = document.getElementById('results');
// @ts-ignore
var recognize_button: HTMLElement = document.getElementById('recognize_button');

async function recognizeText() {
    // @ts-ignore
    var image_data = await mycan.toDataURL().split('base64,')[1];
    // @ts-ignore
    await invoke('recognize_text', {base64Image: image_data}).then((response) => { displayRecognizedText(response.replace(/(?:\r\n|\r|\n|\t)/g, ' ').replace(/(?:\s\s+)/g, ' ').trim().slice(0, -1), null); }).catch((err) => { displayRecognizedText("", err) });
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
    await writeText(String(word));
    await invoke('paste_text').then(() => { erase(); }).catch((err) => { displayRecognizedText("", err); });
}

export {
    erase,
    choseWord,
    recognizeText
}