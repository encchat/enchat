<script lang="ts">
import { supabaseClient } from "src/supabase";


let search: string = ""
const searchForUser = async () => {
}
</script>

<input bind:value={search} on:input={searchForUser} placeholder="Name or email">