<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Input from "@/components/ui/input/Input.vue";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from "@/components/ui/dialog";
import { useCurrentProject, useCurrentProjectActions } from "@/store/project";

const searchQuery = ref("");
const dialogOpen = ref(false);
const newResourceName = ref("");
const creating = ref(false);

const currentProject = useCurrentProject();
const { discoverResources } = useCurrentProjectActions();

const filteredFiles = computed(() => {
    const q = searchQuery.value.toLowerCase().trim();
    const list = currentProject.resourceFiles.value ?? [];
    if (!q) return list;
    return list.filter((f) => f.toLowerCase().includes(q));
});

async function handleCreate() {
    const path = currentProject.path;
    if (!path?.value) return;
    const name = newResourceName.value.trim() || "New request";
    creating.value = true;
    try {
        await invoke("create_http_resource", {
            projectPath: path.value,
            name,
        });
        await discoverResources();
        newResourceName.value = "";
        dialogOpen.value = false;
    } catch (e) {
        console.error(e);
    } finally {
        creating.value = false;
    }
}
</script>

<template>
    <div class="flex h-full flex-col border-r border-sidebar-border bg-sidebar">
        <div class="flex shrink-0 flex-col gap-2 p-2">
            <Input
                v-model="searchQuery"
                placeholder="Search resources"
                class="h-8"
            />
            <Dialog v-model:open="dialogOpen">
                <DialogTrigger as-child>
                    <Button variant="outline" size="sm" class="w-full">
                        Add resource
                    </Button>
                </DialogTrigger>
                <DialogContent>
                    <DialogHeader>
                        <DialogTitle>Add HTTP request</DialogTitle>
                        <DialogDescription>
                            Create a new HTTP request resource in this project.
                        </DialogDescription>
                    </DialogHeader>
                    <div class="grid gap-2 py-2">
                        <label class="text-sm font-medium" for="resource-name">
                            Name
                        </label>
                        <Input
                            id="resource-name"
                            v-model="newResourceName"
                            placeholder="e.g. Get users"
                        />
                    </div>
                    <DialogFooter>
                        <Button variant="outline" @click="dialogOpen = false">
                            Cancel
                        </Button>
                        <Button :disabled="creating" @click="handleCreate">
                            {{ creating ? "Creating…" : "Create" }}
                        </Button>
                    </DialogFooter>
                </DialogContent>
            </Dialog>
        </div>
        <ScrollArea class="flex-1 px-2">
            <div
                v-if="currentProject.loadError"
                class="py-2 text-sm text-destructive"
            >
                {{ currentProject.loadError }}
            </div>
            <div
                v-else-if="filteredFiles.length === 0"
                class="py-4 text-center text-sm text-muted-foreground"
            >
                {{ searchQuery.trim() ? "No matches" : "No resources yet" }}
            </div>
            <ul v-else class="space-y-0.5 pb-2">
                <li
                    v-for="file in filteredFiles"
                    :key="file"
                    class="rounded-md px-2 py-1.5 text-sm text-sidebar-foreground hover:bg-sidebar-accent hover:text-sidebar-accent-foreground"
                >
                    {{ file }}
                </li>
            </ul>
        </ScrollArea>
    </div>
</template>
