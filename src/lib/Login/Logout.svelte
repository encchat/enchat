<script lang="ts">
import { isAuthenticated  } from "src/store";

import { supabaseClient } from "src/supabase";
const onClick = async () => {
    supabaseClient.auth.signOut()
    isAuthenticated.set(false)
}
</script>

<button on:click={onClick}>Logout</button>