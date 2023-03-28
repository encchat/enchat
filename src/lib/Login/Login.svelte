<script lang="ts">
import { isAuthenticated } from "src/store";

import { supabaseClient } from "src/supabase";
import { showError, showSuccess } from "src/toasts";
import LabeledInput from "../Input/LabeledInput.svelte";

let email: string = ""
let password: string = ""
let isLogin: boolean = true

const login = async () => {
    console.log('start')
   const {error} =  await supabaseClient.auth.signInWithPassword({
        email,
        password
    })
    if (error)
        showError(error.message)
    else isAuthenticated.set(true)
    console.log('end')
}
const register = async () => {
    const {data, error} = await supabaseClient.auth.signUp({
        email,
        password
    })
    if (error)
        return showError(error.message)
    showSuccess("Verify your email and login again!")
    isLogin = true
}
const toggleLogin = () => {
    isLogin = !isLogin
}

const onSubmit = async () => {
    return isLogin ? await login() : await register()
}
</script>

<div class="flex items-center justify-center h-screen font-sans">
    <form class="min-w-fit flex-col text-white" on:submit|preventDefault={onSubmit}>
        <div class="mb-8 flex text-xl">
            <h1>{isLogin ? "Welcome!" : "Join the enchat"}</h1>
        </div>
        <div class="flex flex-col text-sm rounded-md">
            <LabeledInput label="Email:" bind:value={email}/>
            <LabeledInput label="Password:" bind:value={password} type="password"/>
        </div>
        {#if !isLogin}
            <button class="rounded-[15px] bg-action w-full border-none shadow-md mt-10 px-5 py-2 hover:border-blue-500" type="submit">Register</button>
            <p on:click={toggleLogin} class="text-sm pt-4">Already have an account?<span class="underline text-link pl-1"> Log in</span></p>
        {:else}
            <button class="rounded-[15px] bg-action w-full border-none shadow-md mt-10 px-5 py-2 hover:border-blue-500" type="submit">Login</button>
            <p on:click={toggleLogin} class="text-sm pt-4">Don't have an account? <span class="underline text-link pl-1"> Sign up</span></p>
        {/if}
    </form>
</div>