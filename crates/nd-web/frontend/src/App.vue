<script setup lang="ts">
import { onMounted } from "vue";
import { useAppModel } from "@/composables/useAppModel";
import AppLayout from "@/components/layout/AppLayout.vue";
import WorkspaceSidebar from "@/components/sidebar/WorkspaceSidebar.vue";
import EditorTabBar from "@/components/editor/EditorTabBar.vue";
import RequestWorkspace from "@/components/request/RequestWorkspace.vue";
import ScriptWorkspace from "@/components/script/ScriptWorkspace.vue";

const app = useAppModel();

onMounted(() => {
    void app.loadWorkspace();
});
</script>

<template>
    <AppLayout>
        <template #sidebar>
            <WorkspaceSidebar
                :workspace="app.workspace"
                :load-err="app.loadErr"
                :active-id="app.activeId"
                @open-file="app.openFile"
            />
        </template>

        <div
            v-if="!app.activeTab"
            class="text-muted-foreground flex flex-1 items-center justify-center text-sm"
        >
            Select a request or script from the sidebar
        </div>

        <template v-else>
            <EditorTabBar
                :tabs="app.tabs"
                :active-id="app.activeId"
                @select="(id: string) => (app.activeId = id)"
                @close="app.closeTab"
            />
            <RequestWorkspace
                v-if="app.activeTab.kind === 'request'"
                :app="app"
            />
            <ScriptWorkspace v-else :app="app" />
        </template>
    </AppLayout>
</template>
