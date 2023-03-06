import { invoke } from '@tauri-apps/api';
import { supabaseClient } from './supabase';

const MAX_KEYS = 10


/**
 * Class used for handling public keys
 */
export abstract class Key {
    private _key: String | null = null;

    public get key() { return this._key }
    
    abstract shouldGenerate(userId: string): Promise<boolean>;
    abstract generate(userId: string): Promise<string>;
    abstract fetch(userId: string, callerId: string): Promise<string | null>

    public async populate(userId: string): Promise<void> {
        const {data: {user}} = await supabaseClient.auth.getUser()
        const key = await ((user.id === userId && await this.shouldGenerate(userId)) ? this.generate(userId) : this.fetch(userId, user.id))
        // TODO: Handle nulls
        this._key = key
    }
}


/**
 * Public identity key
 */
export class IdentityKey extends Key {
    async shouldGenerate(userId: string): Promise<boolean> {
        const {error, data} = await supabaseClient.from('identity-key').select('id').eq('id', userId).single()
        return !!error
    }
    async generate(userId: string): Promise<string> {
        console.debug('Generating new identity key')
        const key = await invoke<string>('request_identity_key')
        await supabaseClient.from('identity-key').insert({
            id: userId,
            key
        })
        console.debug('Done generating identity key')
        return key
    }
    async fetch(userId: string, callerId: string): Promise<string | null> {
        console.debug('Getting identity key')
        const {data} = await supabaseClient.from('identity-key').select('key').eq('id', userId).single()
        return data.key
    }
}

interface PrekeySerialized {
    prekey: string;
    signature: string;
}

export class Prekey extends Key {
    async shouldGenerate(userId: string): Promise<boolean> {
        const {error, data} = await supabaseClient.from('prekey').select('id, created_at').eq('id', userId).single()
        // TODO: Add rotation
        return !!error
    }
    async generate(userId: string): Promise<string> {
        console.debug('Generating new prekey')
        const {prekey, signature} = await invoke<PrekeySerialized>('request_prekey')
        await supabaseClient.from('prekey').insert({
            id: userId,
            key: prekey,
            signature
        })
        console.debug('Done generating prekey')
        return prekey
        
    }
    async fetch(userId: string, callerId: string): Promise<string | null> {
        console.debug('Getting prekey')
        const {data} = await supabaseClient.from('prekey').select('key').eq('id', userId).single()
        return data.key
    }
}

interface OnetimeKeys {
    id: number;
    key: String 
}
export class OnetimeKey extends Key {
    private _keysToGenerate: number = 0

    async shouldGenerate(userId: string): Promise<boolean> {
        const remainingKeys = await supabaseClient.from('onetime-key')
            .select('*', {count: 'exact'})
            .eq('user', userId)
            .filter('used', 'is', 'null')
        this._keysToGenerate = MAX_KEYS - remainingKeys.count
        return this._keysToGenerate > 0
    }
    async generate(userId: string): Promise<string> {
        console.debug(`Generating ${this._keysToGenerate} onetime keys`)
        const {data} = await supabaseClient.from('onetime-key')
            .select('local_id')
            .eq('user', userId)
            .order('local_id', { ascending: false } )
            .limit(1)
        const lastKey = data?.length > 0 ? data[0].local_id : 0
        const keys = await (await invoke<string[]>('request_onetime_keys', {keys: this._keysToGenerate, lastKey: lastKey}))
        const uploadPromise = keys.map((x, i) => supabaseClient.from('onetime-key').insert({
            user: userId,
            key: x,
            local_id: lastKey + i + 1
        }))
        await Promise.all(uploadPromise)
        console.debug('Done generating onetime keys')
        return keys[0]
    }
    async fetch(userId: string, callerId: string): Promise<string | null> {
        if (callerId == userId) return null
        console.debug(`Fetching onetime key for user ${userId}`)
        const {data, error} = await supabaseClient.rpc('get_onetime_key', {user_id: userId});
        return data
    }  
}
export const populateKey = async <K extends Key>(userId: string, type: { new(): K; }): Promise<K>  =>  {
    const key = new type()
    await key.populate(userId)
    return key
}