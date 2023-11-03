import { writeText } from '@tauri-apps/api/clipboard';
import { Command } from "@tauri-apps/api/shell";
const pasteword = new Command("xdotool", ['key', "--delay", "100", 'alt+Tab', 'ctrl+v']);
// @ts-ignore
var out: HTMLElement = document.getElementById('results')

function recognize() {
    if(can.trace.length > 0) can.recognize()
}

function displayRecognizedWords(data: any, err: any) {
    if(err) {
        out.innerHTML = '<div v-else class="errorMessage">' + err.message + '</div>'
    } else {
        out.innerHTML = ""
        for(var word of data) {
        out.innerHTML = out.innerHTML + '<div class="selectWordItem" onclick="choseWord(\'' + word + '\')">' + word + '</div>';
        }
    }
}

var offset = 20;
// @ts-ignore
var mycan: HTMLElement = document.getElementById('can');
mycan.setAttribute('width', String(window.outerWidth  - offset));
window.onresize = () => { mycan.setAttribute('width', String(window.outerWidth - 20)) };

// @ts-ignore
var can = new handwriting.Canvas(mycan);
can.setCallBack((data: any, err: any) => displayRecognizedWords(data, err));
can.setMouseUpCallBack(() => recognize());
//Set line width shown on the canvas element (default: 3)
can.setLineWidth(3);
//Set options
can.setOptions({
    language: 'zh-CN',
    numOfReturn: 5
});

function erase() {
    can.erase();
    out.innerHTML = "";
}

async function choseWord(word: String) {
    await writeText(String(word));
    pasteword.spawn();
    erase();
}

export {
    erase,
    choseWord
}