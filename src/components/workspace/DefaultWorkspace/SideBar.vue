<script setup lang="ts">
import { ref, computed } from "vue";
import Input from "@/components/ui/input/Input.vue";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
    ContextMenu,
    ContextMenuContent,
    ContextMenuItem,
    ContextMenuTrigger,
} from "@/components/ui/context-menu";
import {
    DropdownMenu,
    DropdownMenuTrigger,
    DropdownMenuContent,
    DropdownMenuItem,
} from "@/components/ui/dropdown-menu";
import { Plus, Folder, Globe, ListOrdered, Save } from "lucide-vue-next";
import { useCurrentProject, useCurrentProjectActions } from "@/store/project";
import { useResources, useResourcesState } from "@/store/resources";
import { useWorkspaceTabsActions } from "@/store/tabs";
import StatusBar from "./StatusBar.vue";
import { ResourceItem } from "./ResourceItems";
import { sortedResources } from "@/shared/resources";

const searchQuery = ref("");

const projectStore = useCurrentProjectActions();
const { path: projectPath } = useCurrentProject();
const resourcesStore = useResources();
const { flattenedResources, loadFailures } = useResourcesState();
const { openTab } = useWorkspaceTabsActions();

const hasEditedResources = computed(() =>
    (flattenedResources.value ?? []).some((r) => r._editor_meta.changes_made),
);

async function handleSave() {
    const path = projectPath.value;
    if (!path) return;
    await resourcesStore.saveResources(path);
}

const rootResources = computed(() => {
    return resourcesStore.resources.filter((r) => r.folder_id === null);
});

const filteredResources = computed(() => {
    const q = searchQuery.value.toLowerCase().trim();
    if (!q) return rootResources.value;
    const all = flattenedResources.value ?? [];
    const res = all.filter((r) => r.name.toLowerCase().includes(q));
    return sortedResources(res);
});

const isSearching = computed(() => searchQuery.value.trim().length > 0);

function handleCreateFolder() {
    resourcesStore.createFolderResource();
}

function handleCreateHttp() {
    resourcesStore.createHttpResource();
}

function handleCreateSequence() {
    resourcesStore.createSequenceResource();
}

function handleSelectResource(id: string) {
    const resource = resourcesStore.getResourceById(id);
    if (resource?.type === "folder") return;
    openTab(id);
}
</script>

<template>
    <ContextMenu>
        <ContextMenuTrigger as-child>
            <div class="flex flex-col border-sidebar-border bg-sidebar grow">
                <div class="flex gap-2 p-2">
                    <Input
                        v-model="searchQuery"
                        placeholder="Search resources"
                        class=""
                    />
                    <Button
                        size="icon"
                        variant="outline"
                        title="Save"
                        :disabled="!projectPath || !hasEditedResources"
                        @click="handleSave"
                    >
                        <Save class="size-4" />
                    </Button>
                    <DropdownMenu>
                        <DropdownMenuTrigger as-child>
                            <Button size="icon" variant="outline">
                                <Plus class="size-4" />
                            </Button>
                        </DropdownMenuTrigger>
                        <DropdownMenuContent align="end">
                            <DropdownMenuItem @click="handleCreateFolder">
                                <Folder class="mr-2 size-4" />
                                Folder
                            </DropdownMenuItem>
                            <DropdownMenuItem @click="handleCreateHttp">
                                <Globe class="mr-2 size-4" />
                                HTTP
                            </DropdownMenuItem>
                            <DropdownMenuItem @click="handleCreateSequence">
                                <ListOrdered class="mr-2 size-4" />
                                Sequence
                            </DropdownMenuItem>
                        </DropdownMenuContent>
                    </DropdownMenu>
                </div>

                <ScrollArea class="h-0 grow px-2">
                    <div
                        v-if="projectStore.loadError"
                        class="py-2 text-sm text-destructive"
                    >
                        {{ projectStore.loadError }}
                    </div>
                    <div
                        v-else-if="filteredResources.length === 0"
                        class="py-4 text-center text-sm text-muted-foreground"
                    >
                        {{
                            searchQuery.trim()
                                ? "No matches"
                                : "No resources yet"
                        }}
                    </div>
                    <ul v-else class="space-y-0.5 pb-2">
                        <li
                            v-for="resource in filteredResources"
                            :key="resource.id"
                        >
                            <ResourceItem
                                :resource="resource"
                                :depth="isSearching ? 0 : undefined"
                                @select="handleSelectResource"
                            />
                        </li>
                    </ul>
                </ScrollArea>

                <StatusBar :load-failures="loadFailures ?? []" />
            </div>
        </ContextMenuTrigger>
        <ContextMenuContent class="w-48">
            <ContextMenuItem @click="handleCreateFolder">
                <Folder class="mr-2 size-4" />
                Folder
            </ContextMenuItem>
            <ContextMenuItem @click="handleCreateHttp">
                <Globe class="mr-2 size-4" />
                HTTP
            </ContextMenuItem>
            <ContextMenuItem @click="handleCreateSequence">
                <ListOrdered class="mr-2 size-4" />
                Sequence
            </ContextMenuItem>
        </ContextMenuContent>
    </ContextMenu>
</template>
