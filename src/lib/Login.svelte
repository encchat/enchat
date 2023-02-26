<script lang="ts">
import { isAuthenticated } from "src/store";

import { supabaseClient } from "src/supabase";

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
    <div class="min-w-fit flex-col border bg-white px-6 py-14 shadow-md rounded-[4px]">
        <div class="mb-8 flex justify-center">
            <h1>Enchat</h1>
        </div>
            <div class="flex justify-center text-md mb-2">
                {#if errorMessage}
                    <div class="text-red-500">{errorMessage}</div>
                {:else if registered}
                    <div>Verify your email and login again</div>
                {/if}
            </div>
        <div class="flex flex-col text-sm rounded-md">
            <input class="mb-5 rounded-[4px] border p-3 hover:outline-none focus:outline-none hover:border-blue-500" type="text" bind:value={email} placeholder={"Email"}/>
            <input class="mb-5 rounded-[4px] border p-3 hover:outline-none focus:outline-none hover:border-blue-500" type="password" bind:value={password} placeholder={"Password"}/>
        </div>
        <div class="flex text-sm rounded-md justify-evenly">
            <button class="rounded-md border shadow-md px-5 py-2 hover:border-blue-500" on:click={login}>Login</button>
            <button class="rounded-md border shadow-md px-5 py-2 hover:border-blue-500" on:click={register}>Register</button>
        </div>
    </div>
</div>