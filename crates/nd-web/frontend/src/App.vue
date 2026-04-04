<script setup lang="ts">
import { onMounted } from "vue";
import { useAppModel } from "@/composables/useAppModel";
import AppLayout from "@/components/layout/AppLayout.vue";
import AppSideBar from "@/components/sidebar/AppSideBar.vue";
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
            <AppSideBar
                :workspace="app.workspace"
                :load-err="app.loadErr"
                :active-id="app.activeId"
                @open-file="app.openFile"
            />
        </template>

        <div
            v-if="!app.activeTab"
            class="text-muted-foreground flex min-h-0 flex-1 items-center justify-center p-8 text-sm"
        >
            Select a request or script from the sidebar
        </div>

        <div
            v-else
            class="flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden"
        >
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
        </div>
    </AppLayout>
</template>
