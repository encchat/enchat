<script lang="ts">
import { dialog } from "@tauri-apps/api";

import { createEventDispatcher } from "svelte";


export let selectedFiles: string[] = []

const selectFile = async () => {
    selectedFiles = (await dialog.open({
        title: "Select files to upload",
        multiple: true,
        directory: false
    })) as string[]
}
</script>

<svg on:click={selectFile} xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
    <path stroke-linecap="round" stroke-linejoin="round" d="M18.375 12.739l-7.693 7.693a4.5 4.5 0 01-6.364-6.364l10.94-10.94A3 3 0 1119.5 7.372L8.552 18.32m.009-.01l-.01.01m5.699-9.941l-7.81 7.81a1.5 1.5 0 002.112 2.13" />
  </svg>