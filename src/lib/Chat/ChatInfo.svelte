<script lang="ts">
import { currentChat } from "./chatStore";



</script>

<div class="border-b-4 border-collapse border-separator font-sans text-white">
    <div class="py-2 px-4">{$currentChat?.chatNickname ?? "Open a chat"}</div>
</div>