import { writable } from "svelte/store";

export const currentChatId = writable<string | null>(null);