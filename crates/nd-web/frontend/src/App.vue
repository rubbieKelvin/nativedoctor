<script setup lang="ts">
import { onMounted } from "vue";
import { storeToRefs } from "pinia";
import { useEditorStore } from "@/stores/editor";
import { useWorkspaceStore } from "@/stores/workspace";
import AppLayout from "@/components/layout/AppLayout.vue";
import AppSideBar from "@/components/sidebar/AppSideBar.vue";
import EditorTabBar from "@/components/editor/EditorTabBar.vue";
import RequestWorkspace from "@/components/request/RequestWorkspace.vue";
import ScriptWorkspace from "@/components/script/ScriptWorkspace.vue";

const workspace = useWorkspaceStore();
const editor = useEditorStore();
const { activeTab } = storeToRefs(editor);

onMounted(() => {
    void workspace.loadWorkspace();
});
</script>

<template>
    <AppLayout>
        <template #sidebar>
            <AppSideBar />
        </template>

        <div
            v-if="!activeTab"
            class="text-muted-foreground flex min-h-0 flex-1 items-center justify-center p-8 text-sm"
        >
            Select a request or script from the sidebar
        </div>

        <div
            v-else
            class="flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden"
        >
            <EditorTabBar />
            <RequestWorkspace v-if="activeTab.kind === 'request'" />
            <ScriptWorkspace v-else />
        </div>
    </AppLayout>
</template>
