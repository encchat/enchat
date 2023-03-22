import { supabaseClient } from "./supabase";
import { asyncable } from 'svelte-asyncable'
import { writable } from "svelte/store";

export const isAuthenticated = asyncable(async () => {
    const {error} = await supabaseClient.auth.getUser()
    return !error
})

export const chatCounter = writable(0)