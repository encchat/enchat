<script lang="ts">
import AccountSetup from './lib/AccountSetup.svelte';
import Chat from './lib/Chat/Chat.svelte';
import KeyManager from './lib/KeyManager.svelte';
import Login from './lib/Login/Login.svelte';
import Logut from './lib/Login/Logout.svelte';
import UserSearch from './lib/UserSearch.svelte';
import {isAuthenticated, user} from './store'
</script>

<main class="w-screen h-screen">
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
        <KeyManager user={currentUser}/>
        <Chat user={currentUser} chatId="4178b2a9-f4d2-4d89-9cd1-20ce67a472d6"/>
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

  main {
    background: url('background.svg');
    background-size: cover;
    background-repeat: no-repeat;
  }
  :root {
    @font-face {
      font-family: 'Inter';
      src: url('Inter.ttf');
      font-weigth: 400 500 600;
    }
  }
</style>