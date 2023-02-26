import { writable } from "svelte/store";
import { supabaseClient } from "./supabase";
import { asyncable } from 'svelte-asyncable'

export const isAuthenticated = asyncable(async () => {
    const {error} = await supabaseClient.auth.getUser()
    return !error
})

export const user = asyncable(async () => {
    const {data} = await supabaseClient.auth.getUser()
    return data.user
}, undefined, [isAuthenticated])