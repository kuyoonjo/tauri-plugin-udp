import { EventCallback, Options, UnlistenFn } from "@tauri-apps/api/event";

export interface Payload {
  id: string;
  addr: string;
  data: number[];
}

declare module "@tauri-apps/api/event" {
  function listen(event: 'plugin://udp', handler: EventCallback<Payload>, options?: Options): Promise<UnlistenFn>;
}

