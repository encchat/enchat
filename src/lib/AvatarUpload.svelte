<script lang="ts">
import type { User } from "@supabase/supabase-js";

import { supabaseClient } from "src/supabase";
import { createEventDispatcher } from "svelte";
import { get } from "svelte/store";

export let user: User;
let files: FileList;
let uploading = false
const dispatch = createEventDispatcher()
const uploadAvatar = async () => {
    uploading = true
    if (!files || files.length === 0)
        throw new Error('No image selected')
    if (!user) return
    const file = files[0]
    const {data, error} = await supabaseClient.storage.from('avatars').upload(user.id, file, {upsert: true})
    if (error) return console.error(error)
    await supabaseClient.from('profiles').update({
        avatar_url: data.path
    }).eq('id', user.id)
    dispatch("upload", data.path)
    uploading = false
}
const showUpload = () => {
    document.getElementById('avatar-upload').click()
}
</script>

<div>
    <button on:click={showUpload}>
        <slot></slot>
    </button>
    <input class="hidden" id="avatar-upload" name="avatar" type="file" accept="image/*" bind:files on:change={uploadAvatar} disabled={uploading}/>
</div>
