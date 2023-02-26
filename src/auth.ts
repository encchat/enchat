import { invoke } from "@tauri-apps/api";
import { derived, get, writable } from "svelte/store";
import {getClient, Body, type Client} from '@tauri-apps/api/http'
import jwtDecode from "jwt-decode";

export const isAuthenticated = writable(false)

interface TokenStore {
    refresh_token: string;
    id_token: string;
    access_token: string
}
interface User {
    nickname: string;
    sub: string;
    name: string;
}
export const tokens = writable<TokenStore | null>(null)
export const user = derived<typeof tokens, User | null>(tokens, (tokens) => {
    if (!tokens) return null
    return jwtDecode<User>(tokens.id_token)
}, null)

const authConfig = {
    domain: import.meta.env.VITE_DOMAIN,
    clientid: import.meta.env.VITE_CLIENT_ID,
    redirectUrl: import.meta.env.VITE_REDIRECT_URI
}

function getAuthenticationUrl() {
    return `https://${authConfig.domain}/authorize?scope=openid profile offline_access&response_type=code&client_id=${authConfig.clientid}&redirect_uri=${authConfig.redirectUrl}`
}

interface ExchangeResponse {
    id_token: string;
    refresh_token?: string;
    access_token: string;
}

function exchangeToken(client: Client, token: string, refreshToken?: boolean) {
    const body = Body.json({
            grant_type: refreshToken ? 'refresh_token' : 'authorization_code',
            client_id: authConfig.clientid,
            code: token,
            refresh_token: token,
            redirect_uri: authConfig.redirectUrl
        })
    return client.post<ExchangeResponse>(`https://${authConfig.domain}/oauth/token`, body)
}

export function createClient() {
}

export async function login() {
    if (get(tokens) != null) return
    const client = await getClient()
    let refreshToken: string | null = await invoke('get_refresh_token')
    if (refreshToken) {
        // TODO: Logout if invalid
        const {data} = await exchangeToken(client, refreshToken, true)
        tokens.set({
            refresh_token: refreshToken,
            ...data
        })
        if (data.refresh_token) 
            await invoke('set_refresh_token', {refreshToken: data.refresh_token})

    }
    else {
        const url: string = await invoke('login', {authUrl: getAuthenticationUrl()})
        const {data} = await exchangeToken(client,
            url.substring(url.indexOf('code=')+'code='.length))
        tokens.set({
            refresh_token: refreshToken,
            ...data
        })
        await invoke('set_refresh_token', {refreshToken: data.refresh_token})
    }
    isAuthenticated.set(true)
}

function logout() {
    
}