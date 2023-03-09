<script lang="ts">
import type { User } from "@supabase/supabase-js";
import { invoke } from "@tauri-apps/api";

import { IdentityKey, OnetimeKey, populateKey, Prekey } from "src/Keys";

import { supabaseClient } from "src/supabase";
import { onMount } from "svelte";



export let chatId: string;
export let user: User.User


const isInitial = async () => {
    const messages = await supabaseClient.from('chat-message').select('*', {count: "estimated"}).eq('chat_id', chatId)
    return messages.count === 0
}

const initialChat = async () => {
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
const initialReceivedMessage = async () => {
    const {data} = await supabaseClient.from('chat-message').select('*', { count: 'estimated'})
        .eq('chat_id', chatId)
        .eq('')
}
const sendMessage = async () => {
    if (await isInitial())
        await initialChat()
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
    console.log(res)
}

let message: string = ""

</script>

<form on:submit|preventDefault={sendMessage}>
Wiadomosc: <input type="text" bind:value={message}>
<button type="submit">Wyslij</button>
</form>