<script lang="ts">
import { user } from "src/store";
import { supabaseClient } from "src/supabase";
import { createEventDispatcher } from "svelte";
import { get } from "svelte/store";

let files: FileList;
let uploading = false
const dispatch = createEventDispatcher()
const uploadAvatar = async () => {
    uploading = true
    const currentUser = await get(user)
    if (!files || files.length === 0)
        throw new Error('No image selected')
    if (!currentUser) return
    const file = files[0]
    const {data, error} = await supabaseClient.storage.from('avatars').upload(currentUser.id, file, {upsert: true})
    if (error) return console.error(error)
    await supabaseClient.from('profiles').update({
        avatar_url: data.path
    }).eq('id', currentUser.id)
    dispatch("upload", data.path)
    uploading = false
}
const showUpload = () => {
    document.getElementById('avatar-upload').click()
}
</script>

<div>
    <button on:click={showUpload}>
        {uploading ? "Uploading..." : "Upload"}
    </button>
    <input class="hidden" id="avatar-upload" name="avatar" type="file" accept="image/*" bind:files on:change={uploadAvatar} disabled={uploading}/>
</div>
