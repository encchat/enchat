<script lang="ts">
import { onMount } from 'svelte';
import { createClient, isAuthenticated, login } from './auth';
import type { Auth0Client } from "@auth0/auth0-spa-js";

let authClient: Auth0Client;
onMount(async () => {
  await login()
})
</script>

<main class="container">
</main>