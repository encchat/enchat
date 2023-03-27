import { invoke } from "@tauri-apps/api"
import { IdentityKey, OnetimeKey, populateKey, Prekey } from "src/Keys"
import { supabaseClient } from "src/supabase"
import { AttachementUpload } from "./Attachment/attachements";

interface MessageEntry {
    id: number;
    content: string,
    sender_id: string;
}

export interface DecryptedMessage {
    text: string
    id: number,
    received?: boolean,
    localId: number
}

export const isInitialReceiver = async (chatId: string) => {
    const messages = await supabaseClient.from('chat-message').select('*', {count: "estimated"}).eq('chat_id', chatId)
    console.log(messages)
    return messages.count > 0
}
export const initialSender = async (chatId: string, userId: string) => {
    const {data, error} = await supabaseClient.from('chat-party')
        .select('user')
        .eq('chat', chatId)
        .neq('user', userId)
        .limit(1)
        .single()
    const receiverIdentity = await populateKey(data.user, IdentityKey);
    const receiverPrekey = await populateKey(data.user, Prekey)
    const receiverOnetime = await populateKey(data.user, OnetimeKey)
    const res = await invoke('enter_chat', {
        chatId,
        receiverKeys: {
            receiver_prekey: receiverPrekey.key,
            receiver_identity: receiverIdentity.key,
            receiver_onetime: receiverOnetime.key,
            receiver_onetime_id: receiverOnetime.id,
            receiver_prekey_id: 1
        }
    })

}

export const initialReceiver = async (chatId: string, userId: string) => {
    console.debug('Initial receiver')
    const firstMessage = await supabaseClient.from('chat-message').select('*').eq('chat_id', chatId).order('created_at', { ascending: true }).limit(1).single()
    if (!firstMessage.data || !firstMessage.data.content) {
        throw new Error("Messages could not be retrieved")
    }
    console.log(firstMessage)
    const {data} = await supabaseClient.from('chat-party')
        .select('user')
        .eq('chat', chatId)
        .neq('user', userId)
        .limit(1)
        .single()
    if (!data?.user) {
        throw new Error("Sender identity not found");
    }
    console.log(data)
    const senderIdentity = await populateKey(data.user, IdentityKey);
    console.log(senderIdentity)
    await invoke('enter_chat', {
        chatId,
        senderIdentity: senderIdentity.key,
        receivedMessage: firstMessage.data.content
    })
}

export const decryptMessages = async (chatId: string, message: MessageEntry, userId: string): Promise<DecryptedMessage> => {
    try {
        const parsed = message.content
        const received = message.sender_id != userId
        let decryptedBytes = await invoke<Array<number>>('try_decrypt', {
            chatId,
            received,
            message: parsed,
        })
        if (!decryptedBytes && message.sender_id != userId)
            decryptedBytes = await invoke<Array<number>>('receive', {
                chatId,
                message: parsed,
            })
        const text = new TextDecoder().decode(Uint8Array.from(decryptedBytes))
        return {
            text,
            id: message.id,
            received,
            localId: parsed.header.id
        }
    } catch (err) {
        console.error(err)
        return {
            text: 'Decryption failed',
            id: message.id,
            received: message.sender_id != userId,
            localId: -1
        }
    }
}
export async function* getMessages (chatId: string, userId: string, skip: number = 0, limit: number = 15) {
    const messages =  await supabaseClient.from('chat-message')
        .select('sender_id, content, id')
        .eq('chat_id', chatId)
        .order("created_at", {ascending: false})
        .range(skip, skip + limit)
    console.log(messages)
    for (const message of messages.data ?? []) {
        yield await decryptMessages(chatId, message, userId)
    }
}

interface Message {
    ciphertext: string;
    header: {
        initial?: {
            ephemeral: Array<number>;
            onetime_key_id: number;
            prekey_id: number;
        };
        id: number;
        rachet_key: Array<number>;
        previous_receiver_length: number;
    }   
}
export const sendMessage = async (chatId: string, message: string, userId: string, selectedFiles: string[]) => {
    const mess = await invoke<Message>('send', {
        chatId,
        message
    })
    const res = await supabaseClient.from('chat-message').insert({
        chat_id: chatId,
        sender_id: userId,
        content: mess
    }).select('id')
    if (!res.data[0].id) return res
    for (const file of selectedFiles) {
        const upload = new AttachementUpload(file, mess.header.id, chatId)
        await upload.encrypt()
        await upload.upload(res.data[0].id)
    }
    return res
}

interface Response {
    id: string;
    isNew: boolean;
}
export const startChat = async (otherPartyId: string, currentUserId: string): Promise<Response> => {
    // We have RLS setup that restricts access to 3rd parties, so if there is no chat with the other party, it will return nothing
    const existingChat = await supabaseClient.from('chat-party').select('*').eq('user', otherPartyId).limit(1).single()
    if (existingChat.data) return {id: existingChat.data.chat, isNew: false}
    const {data} = await supabaseClient.rpc('create_chat', {
        user_id: currentUserId,
    })
    if (!data) throw new Error("Chat could not be created")
    const partyAddedd = await supabaseClient.from('chat-party').insert({ 
        chat: data,
        user: otherPartyId
    })
    if (partyAddedd.error) {
        // Cascade removal of the party
        await supabaseClient.from('chat').delete().eq('chat', data)
        throw new Error("Party couldn't be added to the chat")
    }
    return {id: data, isNew: true}
} 