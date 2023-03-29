<script lang="ts">
import Dashboard from './lib/Dashboard.svelte';
import KeyManager from './lib/KeyManager.svelte';
import Login from './lib/Login/Login.svelte';
import {isAuthenticated} from './store'
import { supabaseClient } from './supabase';
import { SvelteToast } from '@zerodevx/svelte-toast';
import backgroundPath from '../public/background.svg'

const getCurrentUser = async () => {
  const {data} = await supabaseClient.auth.getUser()
  console.log("Getting current user")
  console.log(data.user)
  return data.user
}
</script>

<SvelteToast/>  

<main class="w-screen h-screen" style="background-image: url({backgroundPath})">
  {#await $isAuthenticated}
    <p>...</p>
  {:then isAuthenticated}
    {#if isAuthenticated}
      {#await getCurrentUser()}
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
    background-size: cover;
    background-repeat: no-repeat;
    overflow: hidden;
  }
  :root {
    @font-face {
      font-family: 'Inter';
      src: url('Inter.ttf');
      font-weight: 400 500 600;
    }
  }
  :global(.toast-error) {
    --toastBackground: #C73E1D;
    --toastColor: #fff;
  }
</style>