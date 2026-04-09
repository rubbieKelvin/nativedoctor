<script setup lang="ts">
import type { WorkspaceSnapshot } from "@/api";
import { useEditorStore } from "@/stores/editor";
import { useWorkspaceStore } from "@/stores/workspace";
import {
    Collapsible,
    CollapsibleContent,
    CollapsibleTrigger,
} from "@/components/ui/collapsible";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
    Sidebar,
    SidebarContent,
    SidebarFooter,
    SidebarHeader,
    SidebarInput,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
    SidebarSeparator,
} from "@/components/ui/sidebar";
import { cn } from "@/lib/utils";
import {
    ChevronRight,
    ChevronsUpDown,
    FileText,
    Folder,
    FolderOpen,
    LayoutDashboard,
    RefreshCw,
    Search,
    Terminal,
} from "lucide-vue-next";
import { storeToRefs } from "pinia";
import { computed, ref } from "vue";

const VITE_NATIVEDOCTOR_VERSION = import.meta.env.VITE_NATIVEDOCTOR_VERSION;

type WorkspaceFileKind = "request" | "script";

type MergedFolderGroup = {
    root_index: number;
    root_label: string;
    entries: { path: string; name: string; kind: WorkspaceFileKind }[];
};

function mergeWorkspaceByRoot(ws: WorkspaceSnapshot): MergedFolderGroup[] {
    const map = new Map<number, MergedFolderGroup>();
    for (const g of ws.requests) {
        map.set(g.root_index, {
            root_index: g.root_index,
            root_label: g.root_label,
            entries: g.entries.map((e) => ({ ...e, kind: "request" as const })),
        });
    }
    for (const g of ws.scripts) {
        const scriptEntries = g.entries.map((e) => ({
            ...e,
            kind: "script" as const,
        }));
        const existing = map.get(g.root_index);
        if (existing) {
            existing.entries.push(...scriptEntries);
        } else {
            map.set(g.root_index, {
                root_index: g.root_index,
                root_label: g.root_label,
                entries: scriptEntries,
            });
        }
    }
    return ws.roots
        .map((r) => map.get(r.index))
        .filter((g): g is MergedFolderGroup => g != null);
}

function sortEntries(
    entries: MergedFolderGroup["entries"],
): MergedFolderGroup["entries"] {
    return [...entries].sort((a, b) =>
        a.name.localeCompare(b.name, undefined, { sensitivity: "base" }),
    );
}

function filterMergedFolders(
    folders: MergedFolderGroup[],
    qRaw: string,
): MergedFolderGroup[] {
    const q = qRaw.trim().toLowerCase();
    const withSorted = folders.map((f) => ({
        ...f,
        entries: sortEntries(f.entries),
    }));
    if (!q) return withSorted.filter((f) => f.entries.length > 0);
    return withSorted
        .map((f) => ({
            ...f,
            entries: f.entries.filter(
                (e) =>
                    e.name.toLowerCase().includes(q) ||
                    e.path.toLowerCase().includes(q),
            ),
        }))
        .filter((f) => f.entries.length > 0);
}

const workspaceStore = useWorkspaceStore();
const editor = useEditorStore();
const { workspace, loadErr } = storeToRefs(workspaceStore);
const { activeId } = storeToRefs(editor);

const multiRoot = computed(() => (workspace.value?.roots.length ?? 0) > 1);

const roots = computed(() => workspace.value?.roots ?? []);

const workspaceSubtitle = computed(() => {
    const r = roots.value;
    if (r.length === 0) return "Loading workspace…";
    if (r.length === 1) return r[0].label;
    return `${r.length} roots`;
});

const searchQuery = ref("");

const mergedFolders = computed((): MergedFolderGroup[] => {
    const ws = workspace.value;
    if (!ws) return [];
    return mergeWorkspaceByRoot(ws);
});

const filteredFolders = computed(() =>
    filterMergedFolders(mergedFolders.value, searchQuery.value),
);

const totalWorkspaceFiles = computed(() =>
    mergedFolders.value.reduce((n, f) => n + f.entries.length, 0),
);

const skippedCount = computed(
    () => workspace.value?.skipped_requests?.length ?? 0,
);

/** Single-root flat list (after search). */
const singleRootEntries = computed(() => filteredFolders.value[0]?.entries ?? []);

function goLibrary() {
    editor.activeId = null;
}
</script>

<template>
    <Sidebar
        collapsible="none"
        class="border-sidebar-border font-sans border-r"
    >
        <SidebarHeader class="gap-3 border-sidebar-border/60 border-b pb-3">
            <DropdownMenu>
                <DropdownMenuTrigger as-child>
                    <button
                        type="button"
                        :class="
                            cn(
                                'hover:bg-sidebar-accent/80 flex w-full items-center gap-2 rounded-lg border border-sidebar-border bg-sidebar-accent/25 px-2 py-2 text-left transition-colors',
                                'outline-none focus-visible:ring-2 focus-visible:ring-sidebar-ring',
                            )
                        "
                    >
                        <div
                            class="bg-sidebar-primary text-sidebar-primary-foreground flex size-8 shrink-0 items-center justify-center rounded-md font-semibold"
                        >
                            N
                        </div>
                        <div class="min-w-0 flex-1">
                            <div class="truncate font-medium">NativeDoctor</div>
                            <div class="text-sidebar-foreground/60 truncate">
                                {{ workspaceSubtitle }}
                            </div>
                        </div>
                        <ChevronsUpDown
                            class="text-sidebar-foreground/50 size-4 shrink-0"
                            aria-hidden="true"
                        />
                    </button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="start" class="w-64" side="bottom">
                    <DropdownMenuLabel class="font-normal">
                        Workspace roots
                    </DropdownMenuLabel>
                    <template v-if="roots.length === 0">
                        <DropdownMenuItem
                            class="text-sidebar-foreground/60"
                            disabled
                        >
                            No roots loaded yet
                        </DropdownMenuItem>
                    </template>
                    <DropdownMenuItem
                        v-for="r in roots"
                        :key="r.index"
                        class="hover:bg-accent/50 cursor-default flex-col items-start gap-0.5 py-2"
                        @select.prevent
                    >
                        <span class="font-medium">{{ r.label }}</span>
                        <span
                            class="text-sidebar-foreground/55 w-full truncate font-mono text-xs"
                            :title="r.path"
                            >{{ r.path }}</span
                        >
                    </DropdownMenuItem>
                </DropdownMenuContent>
            </DropdownMenu>

            <div
                class="border-sidebar-border bg-sidebar-accent/15 flex items-center gap-2 rounded-lg border px-2 py-1.5"
            >
                <Search
                    class="text-sidebar-foreground/45 size-3.5 shrink-0"
                    aria-hidden="true"
                />
                <SidebarInput
                    v-model="searchQuery"
                    class="placeholder:text-sidebar-foreground/40 min-w-0 flex-1 border-0 bg-transparent font-sans shadow-none focus-visible:ring-0"
                    placeholder="Search workspace…"
                    type="search"
                    autocomplete="off"
                    spellcheck="false"
                />
            </div>

            <SidebarMenu class="gap-0.5">
                <SidebarMenuItem>
                    <SidebarMenuButton
                        :is-active="activeId === null"
                        class="h-9 rounded-md font-normal"
                        tooltip="Browse without a file open"
                        @click="goLibrary"
                    >
                        <LayoutDashboard class="size-4 shrink-0" />
                        <span>Library</span>
                    </SidebarMenuButton>
                </SidebarMenuItem>
            </SidebarMenu>
        </SidebarHeader>

        <SidebarContent
            class="flex min-h-0 flex-1 flex-col gap-0 overflow-hidden py-2"
        >
            <div v-if="loadErr" class="text-destructive px-2">
                {{ loadErr }}
            </div>

            <div
                v-else-if="skippedCount > 0"
                class="text-sidebar-foreground/80 mx-2 mb-2 rounded-md border border-amber-500/25 bg-amber-500/10 px-2 py-1.5 leading-snug"
                :title="
                    workspace?.skipped_requests
                        ?.map((s) => s.path)
                        .join('\n') ?? ''
                "
            >
                {{ skippedCount }}
                request file(s) could not be loaded (see server logs).
            </div>

            <ScrollArea class="min-h-0 flex-1">
                <div class="space-y-1 px-2 pb-2">
                    <!-- Multiple roots: one collapsible per folder -->
                    <template v-if="multiRoot">
                        <Collapsible
                            v-for="folder in filteredFolders"
                            :key="'folder-' + folder.root_index"
                            v-slot="{ open }"
                            :default-open="true"
                        >
                            <div class="flex flex-col gap-0.5">
                                <CollapsibleTrigger
                                    :class="
                                        cn(
                                            'hover:bg-sidebar-accent/80 flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-left outline-none',
                                            'focus-visible:ring-2 focus-visible:ring-sidebar-ring',
                                        )
                                    "
                                >
                                    <ChevronRight
                                        :class="
                                            cn(
                                                'text-sidebar-foreground/60 size-3.5 shrink-0 transition-transform duration-200',
                                                open && 'rotate-90',
                                            )
                                        "
                                        aria-hidden="true"
                                    />
                                    <Folder
                                        class="text-sidebar-foreground/70 size-3.5 shrink-0"
                                        aria-hidden="true"
                                    />
                                    <span
                                        class="text-sidebar-foreground min-w-0 flex-1 truncate font-medium"
                                        :title="folder.root_label"
                                        >{{ folder.root_label }}</span
                                    >
                                    <span
                                        class="text-sidebar-foreground/45 bg-sidebar-accent/40 shrink-0 rounded px-1.5 py-0 font-mono tabular-nums"
                                        >{{ folder.entries.length }}</span
                                    >
                                </CollapsibleTrigger>
                                <CollapsibleContent>
                                    <SidebarMenu
                                        class="ml-1 border-sidebar-border border-l pl-2"
                                    >
                                        <SidebarMenuItem
                                            v-for="e in folder.entries"
                                            :key="e.path"
                                        >
                                            <SidebarMenuButton
                                                :is-active="activeId === e.path"
                                                :tooltip="e.path"
                                                class="h-8 gap-2 font-normal"
                                                @click="
                                                    editor.openFile(
                                                        e.path,
                                                        e.kind,
                                                        e.name,
                                                    )
                                                "
                                            >
                                                <FileText
                                                    v-if="e.kind === 'request'"
                                                    class="text-sidebar-foreground/60 size-3.5 shrink-0"
                                                    aria-hidden="true"
                                                />
                                                <Terminal
                                                    v-else
                                                    class="text-sidebar-foreground/60 size-3.5 shrink-0"
                                                    aria-hidden="true"
                                                />
                                                <span class="truncate">{{
                                                    e.name
                                                }}</span>
                                            </SidebarMenuButton>
                                        </SidebarMenuItem>
                                    </SidebarMenu>
                                </CollapsibleContent>
                            </div>
                        </Collapsible>
                    </template>

                    <!-- Single root: flat list -->
                    <template v-else>
                        <SidebarMenu class="ml-0">
                            <template
                                v-if="
                                    totalWorkspaceFiles === 0 && !loadErr
                                "
                            >
                                <div
                                    class="text-sidebar-foreground/45 flex flex-col items-center gap-2 py-6 text-center"
                                >
                                    <FolderOpen
                                        class="size-8 opacity-40"
                                        aria-hidden="true"
                                    />
                                    <p class="text-[11px]">
                                        No request or script files in workspace
                                    </p>
                                </div>
                            </template>
                            <template
                                v-else-if="singleRootEntries.length === 0"
                            >
                                <p
                                    class="text-sidebar-foreground/45 px-2 py-3 text-center text-[11px]"
                                >
                                    No matches
                                </p>
                            </template>
                            <SidebarMenuItem
                                v-for="e in singleRootEntries"
                                :key="e.path"
                            >
                                <SidebarMenuButton
                                    :is-active="activeId === e.path"
                                    :tooltip="e.path"
                                    class="h-8 gap-2 font-normal"
                                    @click="
                                        editor.openFile(
                                            e.path,
                                            e.kind,
                                            e.name,
                                        )
                                    "
                                >
                                    <FileText
                                        v-if="e.kind === 'request'"
                                        class="text-sidebar-foreground/60 size-3.5 shrink-0"
                                        aria-hidden="true"
                                    />
                                    <Terminal
                                        v-else
                                        class="text-sidebar-foreground/60 size-3.5 shrink-0"
                                        aria-hidden="true"
                                    />
                                    <span class="truncate">{{ e.name }}</span>
                                </SidebarMenuButton>
                            </SidebarMenuItem>
                        </SidebarMenu>
                    </template>

                    <!-- Multi-root: nothing to show -->
                    <template v-if="multiRoot && filteredFolders.length === 0">
                        <div
                            v-if="totalWorkspaceFiles === 0 && !loadErr"
                            class="text-sidebar-foreground/45 flex flex-col items-center gap-2 py-6 text-center"
                        >
                            <FolderOpen
                                class="size-8 opacity-40"
                                aria-hidden="true"
                            />
                            <p class="text-[11px]">
                                No request or script files in workspace
                            </p>
                        </div>
                        <p
                            v-else-if="totalWorkspaceFiles > 0"
                            class="text-sidebar-foreground/45 px-2 py-3 text-center text-[11px]"
                        >
                            No matches
                        </p>
                    </template>
                </div>
            </ScrollArea>
        </SidebarContent>

        <SidebarFooter class="border-sidebar-border/60 mt-auto border-t pt-2">
            <SidebarMenu>
                <SidebarMenuItem>
                    <SidebarMenuButton
                        class="text-sidebar-foreground/80 h-8 font-normal"
                        tooltip="Reload workspace from disk"
                        @click="workspaceStore.loadWorkspace()"
                    >
                        <RefreshCw class="size-3.5 shrink-0" />
                        <span>Refresh workspace</span>
                    </SidebarMenuButton>
                </SidebarMenuItem>
            </SidebarMenu>
            <SidebarSeparator class="bg-sidebar-border/60" />
            <p class="text-sidebar-foreground/45 px-2 pb-1 font-mono">
                v{{ VITE_NATIVEDOCTOR_VERSION }}
            </p>
        </SidebarFooter>
    </Sidebar>
</template>
