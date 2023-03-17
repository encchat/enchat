import { supabaseClient } from "./supabase";
import { asyncable } from 'svelte-asyncable'

export const isAuthenticated = asyncable(async () => {
    const {error} = await supabaseClient.auth.getUser()
    return !error
})
