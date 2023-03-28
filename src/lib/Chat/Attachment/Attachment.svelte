<script lang="ts">
import { dialog, invoke } from "@tauri-apps/api";
import { Attachment, AttachmentStatus, type FileInfo } from "./attachements";
import { currentChat } from '../chatStore';
import AttachmentProgress from "./AttachmentProgress.svelte";

export let realName: string;
export let id: string;
export let receiving: boolean;
export let messageId: number;
export let localMessageId: number;

let progress: AttachmentStatus = AttachmentStatus.Waiting

const onClick = async () => {
    const tempName = realName + '.tmp'
    const downloadPath = await dialog.save({defaultPath: realName})
    const attachement = new Attachment(receiving, tempName, downloadPath, messageId, $currentChat.chatId)
    progress = AttachmentStatus.Downloading
    await attachement.download(id)
    progress = AttachmentStatus.Decrypting
    await attachement.decrypt(localMessageId)
    progress = AttachmentStatus.Done
    console.log('Downloaded')
    
}
</script>

<div class="shadow-md p-3 text-sm" on:click={onClick}>
    <div class="flex">
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6 py-1">
            <path stroke-linecap="round" stroke-linejoin="round" d="M18.375 12.739l-7.693 7.693a4.5 4.5 0 01-6.364-6.364l10.94-10.94A3 3 0 1119.5 7.372L8.552 18.32m.009-.01l-.01.01m5.699-9.941l-7.81 7.81a1.5 1.5 0 002.112 2.13" />
        </svg>
        <div>{realName}</div>
    </div>
    {#if progress != AttachmentStatus.Waiting}
        <AttachmentProgress progress={progress}/>
    {/if}
</div>
