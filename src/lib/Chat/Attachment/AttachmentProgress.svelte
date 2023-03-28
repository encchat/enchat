<script lang="ts">
import { AttachmentStatus } from "./attachements";

export let progress: AttachmentStatus 

const progressPercentage = () => {
    switch (progress) {
        case AttachmentStatus.Encrypting, AttachmentStatus.Downloading:
            return 10
        case AttachmentStatus.Uploading, AttachmentStatus.Decrypting:
            return 50
        case AttachmentStatus.Done:
            return 100
        default:
            return 0
    }
}

const getProgressLabel = () => {
    // Don't set string in Enum value, I think it would take more space and we will use a lot!
    switch (progress) {
        case AttachmentStatus.Decrypting:
            return "Decrypting"
        case AttachmentStatus.Downloading:
            return "Downloading"
        case AttachmentStatus.Done:
            return "Done"
        case AttachmentStatus.Encrypting:
            return "Encrypting"
        case AttachmentStatus.Uploading:
            return "Uploading"
        default:
            return "Waiting"
    }
}
$: console.debug(progress)
</script>
{#if progress != 3}
        <div class="flex w-full h-4 bg-gray-200 rounded-full overflow-hidden dark:bg-gray-700">
            {#key progress}
            <div class="flex flex-col justify-center overflow-hidden bg-blue-500 text-xs text-white text-center" role="progressbar" style="width: {progressPercentage()}%"
                aria-valuenow={progressPercentage()} aria-valuemin="0" aria-valuemax="100">{getProgressLabel()}</div>
            {/key}
        </div>
{/if}