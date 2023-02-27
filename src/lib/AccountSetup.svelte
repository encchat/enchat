<script lang="ts">
import type User from "@supabase/supabase-js";
import { supabaseClient } from "src/supabase";
import { onMount } from "svelte";
import Avatar from "./Avatar.svelte";
import AvatarUpload from "./AvatarUpload.svelte";
export let user: User.User
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
</script>

<form on:submit|preventDefault={updateProfile} class="flex flex-col p-5">
    {#key avatar_url}
        <Avatar avatarUrl={avatar_url}/>
    {/key}
    <AvatarUpload on:upload={onUploaded}/>
    <h3>Username</h3>
    <input class="mb-5 rounded-[4px] border p-3 hover:outline-none focus:outline-none hover:border-blue-500" type="text" bind:value={username} placeholder={"Email"}/>
    <button type="submit">Save</button>
</form>