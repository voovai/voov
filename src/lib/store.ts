import { readable, writable } from "svelte/store";
import type { MessageData } from "./types";
export const currentPage = writable("Home");
//export const baseURL = readable("http://127.0.0.1:7001")
export const baseURL = readable("https://PageChat.gavi1.repl.co")
export const sessionId = writable("");
export const currentUser = writable(null);
export const currentErr = writable({});
export const messages = writable<Array<MessageData>>([]);
export const url = writable("");
export const message = writable("");
export const content = writable("")
export const currentType = writable("URL")