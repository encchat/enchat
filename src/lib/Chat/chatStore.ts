import { writable } from "svelte/store";

export interface Chat {
    chatId: string;
    chatNickname: string;
    chatAvatarUrl: string;
}

export const currentChat = writable<Chat | null>(null);