<script lang="ts">
import { onMount } from 'svelte';
import { isAuthenticated, login, user, logout } from './auth';

onMount(async () => {
  await login()
})
</script>

<main class="container">
  {#if $isAuthenticated}
   <h1> Hello {$user.name} </h1>
   <button on:click={logout}>Logout</button>
  {/if}
</main>