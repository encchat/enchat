<script lang="ts">
import type { User } from "@supabase/supabase-js";

import { invoke } from "@tauri-apps/api/tauri";
import { supabaseClient } from "src/supabase";
import { onMount } from "svelte";

export let user: User.User

const MAX_KEYS = 10

interface Keybundle {
    identity: string;
    onetime: string[];
    prekey: string;
    signature: string;
}

const uploadKeys = (keys: string[], user_id: string) => {
    const promise = keys.map(x => supabaseClient.from('onetime-key').insert({
        user: user.id,
        key: x
    }))
    return Promise.all(promise)
}

const setupKeys = async () => {
    console.debug('Creating keys')
    const keybundle = await invoke<Keybundle>('generate_keys')
    await supabaseClient.from('identity-key').insert({
        id: user.id,
        key: keybundle.identity
    })
    await supabaseClient.from('prekey').insert({
        id: user.id,
        key: keybundle.prekey,
        signature: keybundle.signature
    })
    await uploadKeys(keybundle.onetime, user.id)
    console.debug('Uploaded keys')
}


const hasStoredKeys = async (): Promise<boolean> => {
    const {data} = await supabaseClient.from('identity-key').select().eq('id', user.id)
    return data?.length > 0
}

const getNumberOfKeysToGenerate = async (): Promise<number> => {
    const remainingKeys = await supabaseClient.from('onetime-key').select('*', {count: 'exact'}).eq('user', user.id)
    return MAX_KEYS - remainingKeys.count
}
const generateOnetimeKeys = async (n: number) =>     }{
    console.debug(`Refreshing ${n} onetime keys`)
    const keys = await invoke<string[]>('request_onetime_keys', {keys: n})
    await uploadKeys(keys, user.id)
    console.debug(`Uploaded ${n} onetime keys`)
}

onMount(async () => {
    if (!await hasStoredKeys())
        return await setupKeys()
    const keys = await getNumberOfKeysToGenerate()
    if (keys > 0)
        await generateOnetimeKeys(keys)
})

</script>