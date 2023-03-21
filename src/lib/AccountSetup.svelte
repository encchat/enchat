<script lang="ts">
import type User from "@supabase/supabase-js";
import { isAuthenticated } from "src/store";
import { supabaseClient } from "src/supabase";
import { onMount } from "svelte";
import Avatar from "./Avatar.svelte";
import AvatarUpload from "./AvatarUpload.svelte";
import { currentChat } from "./Chat/chatStore";

export let user: User;
let username: string | null = null
let avatar_url: string | null = null

const getProfile = async () => {
    const {data, error, status} = await supabaseClient.from('profiles')
        .select('username, avatar_url')
        .eq('id', user.id)
        .single()
    if (error) return
    username = data.username
    avatar_url = data.avatar_url.length ? data.avatar_url : null
}

onMount(async () => {
    await getProfile()
})


const updateProfile = async () => {
    const {error} = await supabaseClient.from('profiles')
        .update({
            username
        }).eq('id', user.id)

}

const onUploaded = (uploaded_url: CustomEvent<string>) => avatar_url = uploaded_url.detail

const logout = async () => {
    isAuthenticated.set(false)
    currentChat.set(null)
    await supabaseClient.auth.signOut().then(s => console.debug('looged of'))
}
 
let isEditing = false
</script>

<div class="bg-neutral-800 py-2 flex font-sans px-2">
    <div class="avatar">
        <div class="absolute w-6 h-7 text-transparent bg-transparent hover:text-white hover:bg-black hover:bg-opacity-50">
            <AvatarUpload user={user} on:uploaded={onUploaded}>
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-7 h-7">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M9 8.25H7.5a2.25 2.25 0 00-2.25 2.25v9a2.25 2.25 0 002.25 2.25h9a2.25 2.25 0 002.25-2.25v-9a2.25 2.25 0 00-2.25-2.25H15m0-3l-3-3m0 0l-3 3m3-3V15" />
                  </svg>
            </AvatarUpload>
        </div>
        <Avatar avatarUrl={avatar_url}/>
    </div>
    <div class="flex px-2 text-clip gap-2 overflow-hidden justify-evenly flex-1">
        {#if isEditing}
            <form on:submit={updateProfile} class="flex overflow-auto">
                <input class="text-clip overflow-hidden" autofocus type="text" bind:value={username} on:blur={() => isEditing = false}/>
                
                <svg title="Cancel" on:click={() => isEditing = false} xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6 cursor-grab text-white stroke-2 self-center">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                </svg>
            </form>
        {:else}
            <div class="text-lg text-white text-ellipsis ml-2 overflow-hidden flex-1">{username}</div>
            <svg title="Change username" on:click={() => isEditing = true} xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-6 h-6 text-white stroke-2 self-center cursor-grab">
                <path stroke-linecap="round" stroke-linejoin="round" d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L6.832 19.82a4.5 4.5 0 01-1.897 1.13l-2.685.8.8-2.685a4.5 4.5 0 011.13-1.897L16.863 4.487zm0 0L19.5 7.125" />
            </svg>
        {/if}
        <svg on:click={logout} title="Logout" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6 text-white stroke-2 self-center ml-1 cursor-grab">
            <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 9V5.25A2.25 2.25 0 0013.5 3h-6a2.25 2.25 0 00-2.25 2.25v13.5A2.25 2.25 0 007.5 21h6a2.25 2.25 0 002.25-2.25V15M12 9l-3 3m0 0l3 3m-3-3h12.75" />
          </svg>
    </div>
</div>