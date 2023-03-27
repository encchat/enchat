<script lang="ts">
import { supabaseClient } from "src/supabase";

import type { PreviewAttachment } from "./attachements";
import Attachment from "./Attachment.svelte";


export let messageId: number;
export let receiving: boolean;
export let localMessageId: number;
let attachments: PreviewAttachment[] = []

const getAttachments = async (messageId) => {
    const {data, error} = await supabaseClient.from('chat-message-attachment')
        .select('id, info->>filename')
        .eq('message', messageId)
    attachments = data
}

$: getAttachments(messageId)
</script>

{#each attachments as attachment}
    <Attachment localMessageId={localMessageId} realName={attachment.filename} id={attachment.id} receiving={receiving} messageId={messageId}/>
{/each}
