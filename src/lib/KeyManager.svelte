<script lang="ts">
import { IdentityKey, OnetimeKey, populateKey, Prekey } from "src/Keys";
import { onMount } from "svelte";

export let user: User.User


onMount(async () => {
    await populateKey(user.id, IdentityKey)
    await populateKey(user.id, Prekey)
    await populateKey(user.id, OnetimeKey)
})

</script>
