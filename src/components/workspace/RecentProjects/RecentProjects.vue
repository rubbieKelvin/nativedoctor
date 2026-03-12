<script setup lang="ts">
import { ref, inject, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { Button } from "@/components/ui/button";

const setCurrentProject =
    inject<(path: string | null) => void>("setCurrentProject");
const setShowCreateProject = inject<(show: boolean) => void>(
    "setShowCreateProject",
);

interface RecentProject {
    path: string;
    name: string | null;
    opened_at: number;
}

const recentProjects = ref<RecentProject[]>([]);
const loading = ref(true);

onMounted(async () => {
    try {
        const list = await invoke<RecentProject[]>("get_recent_projects");
        recentProjects.value = list ?? [];
    } catch (e) {
        recentProjects.value = [];
        console.log(e);
    } finally {
        loading.value = false;
    }
});

async function handleOpenProject() {
    const selected = await open({
        multiple: false,
        filters: [{ name: "Project", extensions: ["json"] }],
    });

    if (!selected || typeof selected !== "string") return;

    try {
        const root = await invoke<string>("get_project_root_from_config_path", {
            configPath: selected,
        });
        const hasConfig = await invoke<boolean>("project_has_nativedoctor", {
            path: root,
        });
        if (!hasConfig) return;
        await invoke("add_recent_project", { path: root, name: null });
        setCurrentProject?.(root);
    } catch (_) {
        // ignore
    }
}

function handleOpenRecent(path: string, name: string | null) {
    setCurrentProject?.(path);
    invoke("add_recent_project", { path, name });
}

function handleCreateProject() {
    setShowCreateProject?.(true);
}
</script>

<template>
    <div
        class="flex h-full w-full flex-col items-center justify-center gap-6 p-8"
    >
        <h1 class="text-2xl font-semibold">Recent Projects</h1>
        <div v-if="loading" class="text-muted-foreground text-sm">Loading…</div>
        <div
            v-else-if="recentProjects.length > 0"
            class="flex w-full max-w-md flex-col gap-2"
        >
            <button
                v-for="proj in recentProjects"
                :key="proj.path"
                type="button"
                class="flex flex-col rounded-md border border-border bg-card px-4 py-3 text-left transition-colors hover:bg-accent/50"
                @click="handleOpenRecent(proj.path, proj.name)"
            >
                <span class="font-medium">{{
                    proj.name || "Unnamed project"
                }}</span>
                <span class="text-muted-foreground truncate text-sm">{{
                    proj.path
                }}</span>
            </button>
        </div>
        <p v-else class="text-muted-foreground text-sm">No recent projects</p>
        <div class="flex gap-2">
            <Button @click="handleOpenProject"> Open project </Button>
            <Button variant="outline" @click="handleCreateProject">
                Create project
            </Button>
        </div>
    </div>
</template>
