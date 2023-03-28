<script lang="ts">
import { AttachmentStatus, type ChangeStatusFunction } from "./attachements";
import AttachmentProgress from "./AttachmentProgress.svelte";
export let selectedFiles: string[] = []

let progress: Record<number, AttachmentStatus> = {}

export const changeStatus: ChangeStatusFunction = (index: number, status: AttachmentStatus) => {
    console.debug(index, status)
    progress[index] = status
    progress = progress
}

$:
 selectedFiles?.forEach((_file, index) => {
    if (!progress[index]) {
        progress[index] = AttachmentStatus.Waiting
        progress = progress
    }
})
$: if (selectedFiles.length === 0) progress = {}

const remove = (index: number) => {
    selectedFiles.splice(index, 1)
    selectedFiles = selectedFiles
}
</script>

<div class="flex flex-col max-h-[20vh] overflow-scroll">
    {#each selectedFiles as file, index}
        <div class="flex flex-col justify-between bg-neutral-800">
            <div class="flex flex-row">
                <div class="text-sm text-gray-500 flex-1 p-1">{file}</div>
                {#if progress[index] == AttachmentStatus.Waiting}
                <div class="bg-red-400 text-white shrink-0 m-auto" on:click|capture={() => remove(index)}>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </div>
                {/if}
            </div>
            <AttachmentProgress progress={progress[index]}/>
        </div>
    {/each}
</div>