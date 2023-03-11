<script lang="ts">
import type { User } from "@supabase/supabase-js";
import { invoke } from "@tauri-apps/api";

import { IdentityKey, OnetimeKey, populateKey, Prekey } from "src/Keys";

import { supabaseClient } from "src/supabase";
import { onMount } from "svelte";



export let chatId: string;
export let user: User.User


const isInitialReceiver = async () => {
    const messages = await supabaseClient.from('chat-message').select('*', {count: "estimated"}).eq('chat_id', chatId)
    console.log(messages)
    return messages.count > 0
}
const initialSender = async () => {
    const {data, error} = await supabaseClient.from('chat-party')
        .select('user')
        .eq('chat', chatId)
        .neq('user', user.id)
        .limit(1)
        .single()
    const receiverIdentity = await populateKey(data.user, IdentityKey);
    const receiverPrekey = await populateKey(data.user, Prekey)
    const receiverOnetime = await populateKey(data.user, OnetimeKey)
    const res = await invoke('enter_chat', {
        chatId,
        receiverKeys: {
            receiver_prekey: receiverPrekey.key,
            receiver_identity: receiverIdentity.key,
            receiver_onetime: receiverOnetime.key,
            receiver_onetime_id: receiverOnetime.id,
            receiver_prekey_id: 1
        }
    })

}

const initialReceiver = async () => {
    const firstMessage = await supabaseClient.from('chat-message').select('*').eq('chat_id', chatId).order('created_at', { ascending: true }).limit(1)
    console.log(firstMessage)
    const {data, error} = await supabaseClient.from('chat-party')
        .select('user')
        .eq('chat', chatId)
        .neq('user', user.id)
        .limit(1)
        .single()
    const senderIdentity = await populateKey(data.user, IdentityKey);
    console.log(senderIdentity)
    const res = await invoke('enter_chat', {
        chatId,
        senderIdentity: senderIdentity.key,
        receivedMessage: JSON.parse(firstMessage.data[0].content)
    })
}

interface MessageEntry {
    id: string;
    content: string,
    sender_id: string;
}

interface DecryptedMessage {
    text: string
}

const decryptMessages = async (message: MessageEntry): Promise<DecryptedMessage> => {
    try {
        const parsed = JSON.parse(message.content)
        let decryptedBytes = await invoke<Array<number>>('try_decrypt', {
            chatId,
            received: message.sender_id != user.id,
            message: parsed,
        })
        if (!decryptedBytes && message.sender_id != user.id)
            decryptedBytes = await invoke<Array<number>>('receive', {
                chatId,
                message: parsed,
            })
        console.log(decryptedBytes)
        const text = new TextDecoder().decode(Uint8Array.from(decryptedBytes))
        console.log(text)
        return {
            text
        }
    } catch (err) {
        console.error(err)
        return {
            text: 'Decryption failed'
        }
    }
}
const getMessages = async () => {
    const messages =  await supabaseClient.from('chat-message')
        .select('sender_id, content, id')
        .eq('chat_id', chatId)
        .order("created_at", {ascending: true});
    for (const message of messages.data) {
        decryptMessages(message)
    }
}
const sendMessage = async () => {
    const res = await invoke('send', {
        chatId,
        message
    })
    console.log(JSON.stringify(res))
    await supabaseClient.from('chat-message').insert({
        chat_id: chatId,
        sender_id: user.id,
        content: JSON.stringify(res)
    })
}

let message: string = ""

onMount(async () => {
    if (!await invoke('reenter_chat', {chatId: chatId})) {
        if (await isInitialReceiver())
            await initialReceiver()
        else await initialSender()
            
    }
    getMessages()
})

</script>

<form on:submit|preventDefault={sendMessage}>
Wiadomosc: <input type="text" bind:value={message}>
<button type="submit">Wyslij</button>
</form>