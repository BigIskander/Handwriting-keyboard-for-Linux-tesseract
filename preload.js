const { contextBridge } = require('electron')
const { keyboard, Key, clipboard } = require('@nut-tree/nut-js');

contextBridge.exposeInMainWorld('writeit', {
  pasteWord: async (word) => {
    await clipboard.setContent(word);
    await keyboard.pressKey(Key.LeftAlt, Key.Tab);
    await keyboard.releaseKey(Key.LeftAlt, Key.Tab);
    await keyboard.pressKey(Key.LeftControl, Key.V);
    await keyboard.releaseKey(Key.LeftControl, Key.V);
    await keyboard.pressKey(Key.LeftAlt, Key.Tab);
    await keyboard.releaseKey(Key.LeftAlt, Key.Tab);
  } 
})