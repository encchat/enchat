<script lang="ts">
import { isAuthenticated } from "src/store";

import { supabaseClient } from "src/supabase";
import LabeledInput from "../Input/LabeledInput.svelte";

let email: string = ""
let password: string = ""
let errorMessage: string | null = null
let registered: boolean = false
const login = async () => {
   const {error} =  await supabaseClient.auth.signInWithPassword({
        email,
        password
    })
    if (error)
        errorMessage = error.message
    else isAuthenticated.set(true)
}
const register = async () => {
    const {data, error} = await supabaseClient.auth.signUp({
        email,
        password
    })
    if (error)
        errorMessage = error.message
    else registered = !!data.user.identities.length
}
</script>

<div class="flex items-center justify-center h-screen">
    <div class="min-w-fit flex-col text-white">
        <div class="mb-8 flex text-xl">
            <h1>{!registered ? "Welcome!" : "Join the enchat"}</h1>
        </div>
            <div class="flex justify-center text-md mb-2">
                {#if errorMessage}
                    <div class="text-red-500">{errorMessage}</div>
                {:else if registered}
                    <div>Verify your email and login again</div>
                {/if}
            </div>
        <div class="flex flex-col text-sm rounded-md">
            <LabeledInput label="Email" bind:value={email}/>
            <LabeledInput label="Password" bind:value={password} type="password"/>
        </div>
        <div class="flex text-sm rounded-md justify-evenly">
            <button class="rounded-md border shadow-md px-5 py-2 hover:border-blue-500" on:click={login}>Login</button>
            <button class="rounded-md border shadow-md px-5 py-2 hover:border-blue-500" on:click={register}>Register</button>
        </div>
    </div>
</div>