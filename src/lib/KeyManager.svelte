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

interface OnetimeKey {
    key: string;
    id: number;
}

const uploadKeys = (keys: OnetimeKey[], user_id: string) => {
    const promise = keys.map(x => supabaseClient.from('onetime-key').insert({
        user: user.id,
        key: x.key,
        local_id: x.id
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
    await uploadKeys(keybundle.onetime.map((x,i) => ({key: x, id: i})), user.id)
    console.debug('Uploaded keys')
}


const hasStoredKeys = async (): Promise<boolean> => {
    const {data} = await supabaseClient.from('identity-key').select().eq('id', user.id)
    return data?.length > 0
}

const getNumberOfKeysToGenerate = async (): Promise<number> => {
    const remainingKeys = await supabaseClient.from('onetime-key')
        .select('*', {count: 'exact'})
        .eq('user', user.id)
        .filter('used', 'is', 'null')
    if (remainingKeys.error) return 0
    return MAX_KEYS - remainingKeys.count
}
const generateOnetimeKeys = async (n: number) => {
    const {data} = await supabaseClient.from('onetime-key')
        .select('local_id')
        .eq('user', user.id)
        .order('local_id', { ascending: false } )
        .limit(1)
    const lastKey = data?.length > 0 ? data[0].local_id : 0
    console.debug(`Refreshing ${n} onetime keys`)
    const keys = await invoke<string[]>('request_onetime_keys', {keys: n, lastKey: lastKey})
    await uploadKeys(keys.map((x,i) => ({key: x, id: i + 1 + lastKey}) ), user.id)
    console.debug(`Uploaded ${n} onetime keys`)
}

const removeUsedKeys = async () => {
    return supabaseClient.from('onetime-key').delete().eq('used', true).eq('user', user.id)
}

onMount(async () => {
    if (!await hasStoredKeys())
        return await setupKeys()
    const keys = await getNumberOfKeysToGenerate()
    if (keys > 0)
        await generateOnetimeKeys(keys)
    await removeUsedKeys()
})

</script>