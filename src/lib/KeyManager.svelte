<script lang="ts">
import type { User } from "@supabase/supabase-js";

import { invoke } from "@tauri-apps/api";

import { IdentityKey, OnetimeKey, populateKey, Prekey } from "src/Keys";
import { onMount } from "svelte";

export let user: User


onMount(async () => {
    await invoke('login', {userId: user.id})
    await populateKey(user.id, IdentityKey)
    await populateKey(user.id, Prekey)
    await populateKey(user.id, OnetimeKey)
})

</script>
