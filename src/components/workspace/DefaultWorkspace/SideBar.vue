<script setup lang="ts">
import { ref, computed } from "vue";
import Input from "@/components/ui/input/Input.vue";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
    DropdownMenu,
    DropdownMenuTrigger,
    DropdownMenuContent,
    DropdownMenuItem,
} from "@/components/ui/dropdown-menu";
import { Plus, Folder, Globe, ListOrdered } from "lucide-vue-next";
import { useCurrentProjectActions } from "@/store/project";
import { useWorkspaceTabs } from "@/store/workspaceTabs";
import { ResourceItem } from "./ResourceItems";
import { sortedResources } from "@/shared/resources";

const searchQuery = ref("");

const store = useCurrentProjectActions();
const workspaceTabs = useWorkspaceTabs();

const rootResources = computed(() => {
    return store.resources.filter((r) => r.folderId === null);
});

const filteredResources = computed(() => {
    const q = searchQuery.value.toLowerCase().trim();
    if (!q) return rootResources.value;
    const res = store.resources.filter((r) => r.name.toLowerCase().includes(q));
    return sortedResources(res);
});

const isSearching = computed(() => searchQuery.value.trim().length > 0);

function handleCreateFolder() {
    store.createFolderResource();
}

function handleCreateHttp() {
    store.createHttpResource();
}

function handleCreateSequence() {
    store.createSequenceResource();
}

function handleSelectResource(id: string) {
    const resource = store.getResourceById(id);
    if (resource?.type === "folder") return;
    workspaceTabs.openTab(id);
}
</script>

<template>
    <div class="flex h-full flex-col border-sidebar-border bg-sidebar">
        <div class="flex shrink-0 gap-2 p-2">
            <Input
                v-model="searchQuery"
                placeholder="Search resources"
                class=""
            />
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

        <ScrollArea class="flex-1 px-2">
            <div v-if="store.loadError" class="py-2 text-sm text-destructive">
                {{ store.loadError }}
            </div>
            <div
                v-else-if="filteredResources.length === 0"
                class="py-4 text-center text-sm text-muted-foreground"
            >
                {{ searchQuery.trim() ? "No matches" : "No resources yet" }}
            </div>
            <ul v-else class="space-y-0.5 pb-2">
                <li v-for="resource in filteredResources" :key="resource.id">
                    <ResourceItem
                        :resource="resource"
                        :depth="isSearching ? 0 : undefined"
                        @select="handleSelectResource"
                    />
                </li>
            </ul>
        </ScrollArea>
    </div>
</template>
