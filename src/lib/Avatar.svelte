<script lang="ts">
import { supabaseClient } from "src/supabase";
import defaultAvatar from '/public/tauri.svg'
export let avatarUrl: string | null = null;
let url: string | null = null
const downloadImage = async (path: string) => {
    console.log('download')
    const {data, error} = await supabaseClient.storage.from('avatars').download(path)
    if (error) return
    url = URL.createObjectURL(data)
}
$:
    if (avatarUrl) downloadImage(avatarUrl)
</script>

<img src={url ?? defaultAvatar} class="rounded-full h-7 w-7 border border-transparent  object-fill" alt="user avatar">
