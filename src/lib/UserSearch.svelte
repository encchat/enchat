<script lang="ts">
import type { User } from "@supabase/supabase-js";
import { chatCounter } from "src/store";

import { supabaseClient } from "src/supabase";
import { showError } from "src/toasts";
import { startChat } from "./Chat/chat";
import { currentChat } from "./Chat/chatStore";

export let user: User;
let search: string = ""
const searchForUser = async () => {
    try {
        const {data, error} = await supabaseClient.rpc('get_user_by_username', {name: search})
        if (data.length == 0) return;
        const newChat = await startChat(data[0].id, user.id)
        currentChat.set({
            chatId: newChat.id,
            chatAvatarUrl: data[0].avatar_url,
            chatNickname: data[0].username
        })
        chatCounter.set($chatCounter + 1)
    }
    catch (err) {
        showError(err.message)
    }
}
</script>
<form class="flex overflow-hidden max-w-[100%]" on:submit|preventDefault={searchForUser}>
    <svg on:click={searchForUser} xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5 absolute translate-y-4 left-4 origin-top-right">
        <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z" />
      </svg>
    <input bind:value={search} placeholder="Search (exact)" class="rounded-[10px] mx-2 my-2 py-2 pl-7 px-2 w-0 flex-1"/>
</form>
