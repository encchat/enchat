import { invoke } from '@tauri-apps/api';
import { binary_to_base58 } from 'base58-js'
import { supabaseClient } from './supabase';

export function decode_base58(encoded: string): Uint8Array {
    return binary_to_base58(encoded)
}

/**
 * Class used for handling public keys
 */
export abstract class Key {
    private _key: Uint8Array;
    public get getKey() { return this._key }
    
    abstract shouldGenerate(userId: string): Promise<boolean>;
    abstract generate(userId: string): Promise<string>;
    abstract fetch(userId: string): Promise<string>

    public async populate(userId: string): Promise<void> {
        const {data: {user}} = await supabaseClient.auth.getUser()
        const key = await ((user.id === userId && await this.shouldGenerate(userId)) ? this.generate(userId) : this.fetch(userId))
        // TODO: Handle nulls
        this._key = decode_base58(key)
    }
}

/**
 * Public identity key
 */
export class IdentityKey extends Key {
    async shouldGenerate(userId: string): Promise<boolean> {
        const {error, data} = await supabaseClient.from('identity-key').select('id').eq('id', userId).single()
        console.log(data)
        return !!error
    }
    async generate(userId: string): Promise<string> {
        const key = await invoke<string>('request_identity_key')
        await supabaseClient.from('identity-key').insert({
            id: userId,
            key
        })
        return key
    }
    async fetch(userId: string): Promise<string> {
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
        console.log(data)
        // TODO: Add rotation
        return !!error
    }
    async generate(userId: string): Promise<string> {
        const {prekey, signature} = await invoke<PrekeySerialized>('request_prekey')
        await supabaseClient.from('prekey').insert({
            id: userId,
            key: prekey,
            signature
        })
        return prekey
        
    }
    async fetch(userId: string): Promise<string> {
        // TODO: Verify the signature
        const {data} = await supabaseClient.from('prekey').select('key').eq('id', userId).single()
        return data.key
    }
}

