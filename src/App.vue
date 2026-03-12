<script setup lang="ts">
import { ref, provide, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import DefaultWorkspace from "@/components/workspace/DefaultWorkspace/DefaultWorkspace.vue";
import RecentProjects from "@/components/workspace/RecentProjects/RecentProjects.vue";
import CreateProject from "@/components/workspace/CreateProject/CreateProject.vue";

const currentProjectPath = ref<string | null>(null);
const showCreateProject = ref(false);

function setCurrentProject(path: string | null) {
    currentProjectPath.value = path;
}

function setShowCreateProject(show: boolean) {
    showCreateProject.value = show;
}

provide("setCurrentProject", setCurrentProject);
provide("setShowCreateProject", setShowCreateProject);

onMounted(async () => {
    try {
        const path = await invoke<string | null>("get_initial_project_path");

        if (path && path.trim()) {
            const hasConfig = await invoke<boolean>(
                "project_has_nativedoctor",
                {
                    path: path.trim(),
                },
            );
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
            currentProjectPath.value = path.trim();
            await invoke("add_recent_project", {
                path: path.trim(),
                name: null,
            });
        }
    } catch (_) {
        // No initial path or error; show Recent Projects
    }
});
</script>

<template>
    <div class="h-full w-full overflow-auto">
        <CreateProject v-if="showCreateProject" />
        <DefaultWorkspace
            v-else-if="currentProjectPath"
            :project-path="currentProjectPath"
        />
        <RecentProjects v-else />
    </div>
</template>
