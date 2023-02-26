import { invoke } from "@tauri-apps/api";
import { derived, get, writable } from "svelte/store";
import {getClient, Body, type Client} from '@tauri-apps/api/http'
import jwtDecode from "jwt-decode";


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
export const isAuthenticated = derived<typeof tokens, boolean>(tokens, (tokens) => tokens !== null)
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
    return `https://${authConfig.domain}/authorizee?scope=openid profile offline_access&response_type=code&client_id=${authConfig.clientid}&redirect_uri=${authConfig.redirectUrl}`
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

export async function login() {
    if (get(tokens) != null) return
    console.debug('Logging in')
    const client = await getClient()
    let refreshToken: string | null = await invoke('get_refresh_token')
    if (refreshToken) {
        try {
            const {data} = await exchangeToken(client, refreshToken, true)
            console.debug('Token refreshed')
            tokens.set({
                refresh_token: refreshToken,
                ...data
            })
            // Is this path possible? I know refresh token may rotate
            if (data.refresh_token) {
                console.debug('Received new refresh token')
                await invoke('set_refresh_token', {refreshToken: data.refresh_token})
            }
        } catch {
            console.debug('Invalid refresh token, logging out')
            await logout()
            await login()
        }

    }
    else {
        const url: string = await invoke('login', {authUrl: getAuthenticationUrl()})
        console.debug('Received auth code from auth0')
        const {data} = await exchangeToken(client,
            url.substring(url.indexOf('code=')+'code='.length))
        console.debug('Exchanged code for tokens')
        tokens.set({
            refresh_token: refreshToken,
            ...data
        })
        await invoke('set_refresh_token', {refreshToken: data.refresh_token})
        console.debug('Saved refresh token')
    }
}

async function revokeRefreshToken(client: Client, token: string) {
    const body = Body.json({
        client_id: authConfig.clientid,
        token
    })
    return client.post(`https://${authConfig.domain}/oauth/revoke`, body)
}

export async function logout() {
    const client = await getClient()
    let refreshToken: string | null = await invoke('get_refresh_token')
    if (!refreshToken) return;
    await revokeRefreshToken(client, refreshToken).catch(console.error)
    console.debug('Refresh token revoked')
    await invoke('logout').then(_ => console.debug('Logged out'))
    tokens.set(null)
}