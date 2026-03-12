<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import DefaultWorkspace from "@/components/workspace/DefaultWorkspace/DefaultWorkspace.vue";
import RecentProjects from "@/components/workspace/RecentProjects/RecentProjects.vue";
import CreateProject from "@/components/workspace/CreateProject/CreateProject.vue";
import type { Pages } from "@/shared/types";

const pageStack = ref<Array<Pages>>([
    {
        name: "RecentProjects",
        meta: {},
    },
]);

const page = computed({
    get: () => pageStack.value.slice(-1)[0],
    set: (page: Pages) => {
        pageStack.value.push(page);
    },
});

onMounted(async () => {
    try {
        // get the path that was set during binary run
        const path = await invoke<string | null>("get_initial_project_path");

        if (path && path.trim()) {
            // check if the selected folder has a nativedoctor.json project file
            const hasConfig = await invoke<boolean>(
                "project_has_nativedoctor",
                {
                    path: path.trim(),
                },
            );

            // if it doesnt, create it
            if (!hasConfig) {
                await invoke("write_nativedoctor", {
                    path: path.trim(),
                    payload: {
                        name: "",
                        description: "",
                        files: [],
                    },
                });
            }

            // go to page
            page.value = {
                name: "DefaultWorkspace",
                meta: {
                    projectPath: path.trim(),
                },
            };

            // uppdate recent project
            await invoke("add_recent_project", {
                path: path.trim(),
                name: null,
            });
        }
    } catch (_) {
        // No initial path or error; stay in Recent Projects
    }
});
</script>

<template>
    <div class="h-full w-full overflow-auto">
        <CreateProject v-if="page.name === 'CreateProject'" />
        <DefaultWorkspace
            v-else-if="page.name === 'DefaultWorkspace'"
            :project-path="page.meta.projectPath"
        />
        <RecentProjects v-else />
    </div>
</template>
