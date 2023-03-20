<script lang="ts">
import type { User } from "@supabase/supabase-js";
import { invoke } from "@tauri-apps/api";
import Avatar from "../Avatar.svelte";
import { getMessages, initialReceiver, initialSender, isInitialReceiver, sendMessage, type DecryptedMessage } from "./chat";

import { currentChat, type Chat } from "./chatStore";

export let user: User



let decryptedMessages: DecryptedMessage[] = []


let message: string = ""

// when svelte finishes rendering, jump to the message with given id, or the latest one
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
    debugger
    console.log('Chat change')
    console.log(user)
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
    decryptedMessages = [ ...decryptedMessages, {
        text: message,
        id: res.data[0].id
    },]
    jumpTo()
    setupPagination()
}

let observer: IntersectionObserver;
let container: HTMLElement;

// listen for the message with the least id (as they are in descending order) to be visible
// we will fetch more messages then
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

// Request more messages when last message becomes visible
const lastItemOnVisible = (entries: IntersectionObserverEntry[]) => {
    if (entries[0].isIntersecting) {
        const currentLastId = decryptedMessages[0].id
        fetchMessages($currentChat.chatId).then(_ => jumpTo(currentLastId))
    }
}

</script>

{#if $currentChat}
{#await changeChat($currentChat)}
    <p>Loading chat</p>
{:then _}
    <div class="flex flex-col overflow-scroll basis-11/12" bind:this={container}>
        {#each decryptedMessages as item}
            <div data-index={item.id}  class={`items-end ${item.received ?  '' : 'justify-end'} flex mx-2 my-5`}>
                {#if item.received}
                    <div class="h-full origin-top-right" style="transform: translate(0.5rem, -0.5rem);">
                        <Avatar avatarUrl={$currentChat.chatAvatarUrl}/>
                    </div>
                {/if}
                <div class="py-2 {item.received ? "bg-action" : "bg-you"} pr-3 px-2 rounded-[10px] text-white">
                    {item.text}
                </div>
            </div>
        {/each}
    </div>
    <form on:submit|preventDefault={send} class="px-6 flex pb-2">
        <input type="text" bind:value={message} placeholder="Type a message" class="bg-silver border-2 border-neutral-500 flex-1 px-2 py-1 rounded-lg">
        <button type="submit" class="pl-2 pb-2 text-neutral-500">
            <svg style="transform: rotate(-45deg);" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8 stroke-2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M6 12L3.269 3.126A59.768 59.768 0 0121.485 12 59.77 59.77 0 013.27 20.876L5.999 12zm0 0h7.5" />
            </svg>
        </button>
    </form>
{/await}
{:else}
    <h1>Your chat will be visible here</h1>
{/if}

