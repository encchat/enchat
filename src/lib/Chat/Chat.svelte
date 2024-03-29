<script lang="ts">
import type { User } from "@supabase/supabase-js";
import { invoke} from "@tauri-apps/api";
import { supabaseClient } from "src/supabase";
import { showError } from "src/toasts";
import { onMount } from "svelte";
import Avatar from "../Avatar.svelte";
import type { ChangeStatusFunction } from "./Attachment/attachements";
import AttachmentList from "./Attachment/AttachmentList.svelte";
import Uploader from "./Attachment/Uploader.svelte";
import Uploading from './Attachment/Uploading.svelte'
import { decryptMessage, getMessages, initialReceiver, initialSender, isInitialReceiver, isInitialSender, sendMessage, type DecryptedMessage, type MessageEntry } from "./chat";

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

const newMessageArrived = async (message: any) => {
    console.log(message)
    if (message.new.sender_id == user.id) return
    const decrypted = await decryptMessage($currentChat.chatId, message.new, user.id)
    decryptedMessages = [ ...decryptedMessages, decrypted]
    jumpTo()
    setupPagination()
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
    try {
        await invoke('reenter_chat', {chatId: chat.chatId})
        await fetchMessages(chat.chatId)
    } catch (e) {
        showError(e.message)
    }
    jumpTo()
    setupPagination()
}
let subscription
onMount(() => {
    console.log('mount')
    const chatId = $currentChat?.chatId
    if (!chatId) return
    subscription = supabaseClient.channel('table-db-changes')
        .on('postgres_changes', {
            event: 'UPDATE',
            schema: 'public',
            table: 'chat-message',
            filter: `chat_id=eq.${chatId}`,
        }, newMessageArrived)
        .subscribe()
    console.log(subscription)
})

const send = async () => {
    try {
        const chatId = $currentChat.chatId
        if (await isInitialSender(chatId)) {
            await initialSender(chatId, user.id)
        }
        const res = await sendMessage(chatId, message, user.id, selectedFiles, changeStatus)
        console.log(res)
        decryptedMessages = [ ...decryptedMessages, {
            text: message,
            id: res.data[0].id
        }]
        jumpTo()
        setupPagination()
        selectedFiles = []
        message = ""
    } catch (e) {
        showError(e.message)
    }
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

let selectedFiles: string[]
let changeStatus: ChangeStatusFunction = () => ({})

</script>

{#if $currentChat}
{#await changeChat($currentChat)}
    <p>Loading chat</p>
{:then _}
    <div class="flex flex-col overflow-y-scroll basis-11/12 scrollbar-thin scrollbar-thumb-action scrollbar-track-you" bind:this={container}>
        {#each decryptedMessages as item}
            <div data-index={item.id}  class={`items-end ${item.received ?  '' : 'justify-end'} flex mx-2 my-5`}>
                {#if item.received}
                    <div class="h-full origin-top-right " style="transform: translate(0.5rem, -0.5rem);">
                        <Avatar avatarUrl={$currentChat.chatAvatarUrl}/>
                    </div>
                {/if}
                <div class="py-2 {item.received ? "bg-action" : "bg-you"} pr-3 px-2 rounded-[10px] text-white break-all max-w-[70%]">
                    {item.text}
                    <AttachmentList localMessageId={item.localId} messageId={item.id} receiving={item.received}/>
                </div>
            </div>
        {/each}
    </div>
    <form on:submit|preventDefault={send} class="px-6 flex pb-2">
        <input type="text" bind:value={message} placeholder="Type a message" class="bg-silver border-2 border-neutral-500 flex-1 px-2 py-1 rounded-lg">
        <Uploader bind:selectedFiles/>
        <button type="submit" class="pl-2 pb-2 text-neutral-500">
            <svg style="transform: rotate(-45deg);" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8 stroke-2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M6 12L3.269 3.126A59.768 59.768 0 0121.485 12 59.77 59.77 0 013.27 20.876L5.999 12zm0 0h7.5" />
            </svg>
        </button>
    </form>
    <Uploading bind:selectedFiles bind:changeStatus/>
{/await}
{:else}
    <h1>Your chat will be visible here</h1>
{/if}

