import { invoke } from '@tauri-apps/api/core';
import { EventCallback, Options, listen as _listen } from '@tauri-apps/api/event';
import { Buffer } from 'buffer';

/**
 * 
 * @param id A unique ID
 * @param bindAt e.g. 0.0.0.0:8080
 * @param broadcast default to false
 */
export async function bind(id: string, bindAt: string, broadcast = false) {
  await invoke('plugin:udp|bind', {
    id, bindAt, broadcast,
  });
}

/**
 * 
 * @param id A unique ID
 */
export async function unbind(id: string) {
  await invoke('plugin:udp|unbind', {
    id
  });
}

/**
 * 
 * @param id A unique ID
 * @param target e.g. 255.255.255.255:8080
 * @param message A string or a uint8 array
 */
export async function send(id: string, target: string, message: string | number[]) {
  await invoke('plugin:udp|send', {
    id,
    target,
    message: typeof message === 'string' ? Array.from(Buffer.from(message)) : message,
  });
}

export interface Payload {
  id: string;
  addr: string;
  data: number[];
}

export function listen(handler: EventCallback<Payload>, options?: Options) {
  return _listen('plugin://udp', handler, options);
}
