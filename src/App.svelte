<script lang="ts">
import AccountSetup from './lib/AccountSetup.svelte';
import Login from './lib/Login.svelte';
import Logut from './lib/Logout.svelte';
import UserSearch from './lib/UserSearch.svelte';
import {isAuthenticated, user} from './store'
</script>

<main>
  {#await $isAuthenticated}
    <p>...</p>
  {:then isAuthenticated}
    {#if isAuthenticated}
      <UserSearch/>
      <Logut/>
      {#await $user}
        <p>...</p>
      {:then currentUser}
        <AccountSetup user={currentUser}/>
      {/await}
    {:else}
      <Login/>
    {/if}
    {:catch}
    <p>Supabase error</p>
  {/await}
</main>

<style>
  @tailwind base;
  @tailwind components;
  @tailwind utilities;
</style>