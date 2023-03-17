<script lang="ts">
import { isAuthenticated } from "src/store";

import { supabaseClient } from "src/supabase";
import { currentChat } from "../Chat/chatStore";
const onClick = async () => {
    isAuthenticated.set(false)
    currentChat.set(null)
    await supabaseClient.auth.signOut().then(s => console.debug('looged of'))
}
</script>

<button on:click={onClick}>Logout</button>