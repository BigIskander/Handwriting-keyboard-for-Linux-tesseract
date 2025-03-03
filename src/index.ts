import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { invoke } from '@tauri-apps/api/core';

const appWindow = getCurrentWebviewWindow();
invoke('open_keyboard_window');
appWindow.close();