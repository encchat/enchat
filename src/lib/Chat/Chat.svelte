<script lang="ts">
import type { User } from "@supabase/supabase-js";
import { invoke } from "@tauri-apps/api";
import { getMessages, initialReceiver, initialSender, isInitialReceiver, sendMessage, type DecryptedMessage } from "./chat";

import { currentChat, type Chat } from "./chatStore";

export let user: User



let decryptedMessages: DecryptedMessage[] = []


let message: string = ""

const jumpTo = (id?: number) => {
    requestAnimationFrame(() => {
        const element = document.querySelector(`[data-index="${id ?? decryptedMessages[decryptedMessages.length - 1].id}"]`)
        console.log(element)
        if (element) element.scrollIntoView()
    })
}

const fetchMessages = async (chatId: string) => {
    const skip = decryptedMessages.length
    console.log('Fetching, skipped: ' + skip)
    let found = 0;
    for await (const message of getMessages(chatId, user.id, skip)) {
        console.log(message)
        decryptedMessages.push(message)
        found++;
    }
    if (found > 0) {
        decryptedMessages.sort((a, b) => a.id - b.id)
        decryptedMessages = decryptedMessages
    }
}

const changeChat = async (chat: Chat | null) => {
    if (!chat) return

    if (!await invoke('reenter_chat', {chatId: chat.chatId})) {
        if (await isInitialReceiver(chat.chatId))
            await initialReceiver(chat.chatId, user.id)
        else await initialSender(chat.chatId, user.id)
            
    }
    await fetchMessages(chat.chatId)
    jumpTo()
    setupPagination()
}


const send = async () => {
    const res = await sendMessage($currentChat.chatId, message, user.id)
    decryptedMessages = [{
        text: message,
        id: res.data[0].id
    }, ...decryptedMessages]
    console.log(decryptedMessages)
    jumpTo()
    setupPagination()
}

currentChat.subscribe(changeChat)

let observer: IntersectionObserver;
let container: HTMLElement;

const setupPagination = () => {
    requestAnimationFrame(() => {
        const options = {
            root: container,
            rootMargin: '0px',
            threshold: 1.0
        }
        observer = new IntersectionObserver(lastItemOnVisible, options)
        const lastItem = document.querySelector(`[data-index="${decryptedMessages[0].id}"]`)
        observer.observe(lastItem)

    })
}

const lastItemOnVisible = (entries: IntersectionObserverEntry[]) => {
    console.log(entries)
    if (entries[0].isIntersecting) {
        const currentLastId = decryptedMessages[0].id
        console.log('last item is visible')
        fetchMessages($currentChat.chatId).then(_ => jumpTo(currentLastId))
    }
}

</script>

{#if $currentChat}
    <form on:submit|preventDefault={send}>
        Wiadomosc: <input type="text" bind:value={message}>
        <button type="submit">Wyslij</button>
    </form>
    <div class="flex flex-col overflow-scroll" bind:this={container}>
        {#each decryptedMessages as item}
            <div data-index={item.id}  class={`items-end ${item.received ?  '' : 'justify-end'} flex`}>
                <div class="py-2">
                    {item.text}
                </div>
            </div>
        {/each}
    </div>
{:else}
    <h1>Your chat will be visible here</h1>
{/if}

