<script lang="ts">
import type { User } from "@supabase/supabase-js";
import Chat from "./Chat/Chat.svelte";
import ChatInfo from "./Chat/ChatInfo.svelte";
import ChatList from "./ChatList/ChatList.svelte";
import Logout from "./Login/Logout.svelte";
import UserSearch from "./UserSearch.svelte";
export let user: User;
</script>

<div class="flex w-full h-full">
    <div class="flex flex-col w-1/4 border-4 border-solid border-separator border-collapse">
        <UserSearch/>
        <Logout/>
        <ChatList user={user}/>
    </div>
    <div class="flex flex-col w-3/4 border-4 border-solid border-separator border-collapse">
        <ChatInfo/>
        <Chat user={user}/>
    </div>
</div>