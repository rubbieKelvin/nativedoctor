<script setup lang="ts">
import type { AppModel } from "@/composables/useAppModel";
import RequestToolbar from "@/components/request/RequestToolbar.vue";
import RequestEditorTabs from "@/components/request/RequestEditorTabs.vue";
import ResponsePanel from "@/components/response/ResponsePanel.vue";

defineProps<{
    app: AppModel;
}>();
</script>

<template>
    <div class="flex min-h-0 min-w-0 flex-1 flex-col">
        <RequestToolbar :app="app" />
        <RequestEditorTabs :app="app" />
        <ResponsePanel :app="app" />
    </div>
</template>
