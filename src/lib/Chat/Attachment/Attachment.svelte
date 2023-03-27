<script lang="ts">
import { dialog, invoke } from "@tauri-apps/api";
import { Attachment, type FileInfo } from "./attachements";
import { currentChat } from '../chatStore';

export let realName: string;
export let id: string;
export let receiving: boolean;
export let messageId: number;
export let localMessageId: number;

const onClick = async () => {
    const tempName = realName + '.tmp'
    const downloadPath = await dialog.save({defaultPath: realName})

    const attachement = new Attachment(receiving, tempName, downloadPath, messageId, $currentChat.chatId)
    await attachement.download(id)
    await attachement.decrypt(localMessageId)
    console.log('Downloaded')
    
}
</script>

<div class="flex shadow-md p-3" on:click={onClick}>
    <div>DOWNLOAD</div>
    <div>{realName}</div>
</div>
