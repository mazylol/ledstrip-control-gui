import { invoke } from '@tauri-apps/api/tauri';

export async function on() {
    await invoke('on');
}

export async function off() {
    await invoke('off');
}

export async function brightness(brightness: string) {
    await invoke('brightness', { brightness: brightness });
}

export async function color(color: string) {
    await invoke('color', { color: color });
}