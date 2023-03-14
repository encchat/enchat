<script lang="ts">
import AccountSetup from './lib/AccountSetup.svelte';
import Chat from './lib/Chat/Chat.svelte';
import ChatList from './lib/ChatList/ChatList.svelte';
import Dashboard from './lib/Dashboard.svelte';
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
      {#await $user}
        <p>...</p>
      {:then currentUser}
        <KeyManager user={currentUser}/>
        <Dashboard user={currentUser}/>
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