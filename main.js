const { app, BrowserWindow, screen, Menu } = require('electron')
app.allowRendererProcessReuse = false;
const path = require('path');

let win = undefined
const createWindow = () => {
  let display = screen.getDisplayNearestPoint(screen.getCursorScreenPoint())
  let windowHeight = 300
  let windowOffset = 40
  win = new BrowserWindow({
    y: display.bounds.y + display.bounds.height - windowHeight - windowOffset,
    x: display.bounds.x,
    width: display.bounds.width,
    height: windowHeight,
    minWidth: 900,
    minHeight: 300,
    maxHeight: 300,
    maximizable: false,
    webPreferences: {
      devTools: false,
      sandbox: false,
      preload: path.join(__dirname, 'preload.js')
    }
  })

  win.on('maximize', () => {
    win.unmaximize()
  });
  
  win.loadFile('index.html')
}

app.whenReady().then(() => {
  createWindow()
})

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') app.quit()
})

// No Menu
const template = []
const menu = Menu.buildFromTemplate(template)
Menu.setApplicationMenu(menu)