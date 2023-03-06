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

const intialMessage = async () => {
    console.log(chatId)
    const {data, error} = await supabaseClient.from('chat-party')
        .select('user')
        .eq('chat', chatId)
        .neq('user', user.id)
        .single()
    console.log(data)
    console.log(error)
    const receiverIdentity = await populateKey(data.user, IdentityKey);
    const receiverPrekey = await populateKey(data.user, Prekey)
    const receiverOnetime = await populateKey(data.user, OnetimeKey)
    const res = await invoke('calculate_psk', {
        receiverPrekey: receiverPrekey.key,
        receiverIdentity: receiverIdentity.key,
        receiverOnetime: receiverOnetime.key
    })
    console.log(res)
}
const sendMessage = async () => {
    if (await isInitial())
        await intialMessage()
}

let message: string = ""

</script>

<form on:submit|preventDefault={sendMessage}>
Wiadomosc: <input type="text" bind:value={message}>
<button type="submit">Wyslij</button>
</form>