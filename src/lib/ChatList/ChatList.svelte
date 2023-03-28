<script lang="ts">
import { supabaseClient } from "src/supabase";
import Avatar from "../Avatar.svelte";
import { currentChat, type Chat } from "../Chat/chatStore";
import { chatCounter } from '../../store'
import { onMount } from "svelte";
export let user: User.User;

const getFirstMemberOfChat = async (chatId: string) => {
    const firstMember = await supabaseClient.from('chat-party')
        .select('user')
        .eq('chat', chatId)
        .neq('user', user.id)
        .limit(1);
    return firstMember.data[0].user;
}
const getProfileOfChatMember = async (userId: string) => {
    const profile = await supabaseClient.from('profiles')
        .select('avatar_url, username')
        .eq('id', userId)
        .limit(1);
    return profile.data[0];
}
const getAllChats = async () => {
    const allChats = await supabaseClient.from('chat-party')
        .select('chat')
        .eq('user', user.id);
    return allChats.data
}

const getAndMapChats = async () => {
    console.log('mapping')
    const allChats = await getAllChats();
    const chats: Chat[] = [];
    for (const chat of allChats) {
        const chatId = chat.chat;
        const firstMemberId = await getFirstMemberOfChat(chatId);
        const profile = await getProfileOfChatMember(firstMemberId);
        chats.push({
            chatId,
            chatNickname: profile.username,
            chatAvatarUrl: profile.avatar_url
        });
    }
    console.log(chats)
    return chats;
}

const enterChat = (chat: Chat) => {
    console.log('Setting the chat')
    currentChat.set(chat)
}
let subscription;
onMount(() => {
    subscription = supabaseClient.channel('table-db-changes')
        .on('postgres_changes', {
            event: 'INSERT',
            schema: 'public',
            table: 'chat-party',
        }, () => chatCounter.set($chatCounter + 1))
        .subscribe((status) => console.log(status))
})

</script>

<div class="flex flex-col font-sans w-full text-white basis-11/12">
    <div class="text-xl w-full text-center mb-3">Chat list</div>
    <div>
        {#key  $chatCounter}
            {#await getAndMapChats()}
                Loading chats
            {:then chats} 
                {#each chats as chat}
                    <div class={`flex mx-5 px-1 items-center gap-2 py-1 ${chat.chatId == $currentChat?.chatId && 'border-l-2 border-neutral-400 bg-currentIndicator'}`}
                        on:click={() => enterChat(chat)}>
                        <Avatar avatarUrl={chat.chatAvatarUrl} />
                        <div class="overflow-hidden text-ellipsis">{chat.chatNickname}</div>
                    </div>
                {/each}
            {/await}
        {/key}
    </div>
</div>