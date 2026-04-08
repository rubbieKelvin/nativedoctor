<script setup lang="ts">
import type { GroupedFiles } from "@/api";
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
    FolderOpen,
    LayoutDashboard,
    RefreshCw,
    Search,
    Terminal,
} from "lucide-vue-next";
import { storeToRefs } from "pinia";
import { computed, ref } from "vue";

const VITE_NATIVEDOCTOR_VERSION = import.meta.env.VITE_NATIVEDOCTOR_VERSION;

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

function filterGrouped(groups: GroupedFiles[], qRaw: string): GroupedFiles[] {
    const q = qRaw.trim().toLowerCase();
    if (!q) return groups;
    return groups
        .map((g) => ({
            ...g,
            entries: g.entries.filter(
                (e) =>
                    e.name.toLowerCase().includes(q) ||
                    e.path.toLowerCase().includes(q),
            ),
        }))
        .filter((g) => g.entries.length > 0);
}

const filteredRequests = computed(() =>
    filterGrouped(workspace.value?.requests ?? [], searchQuery.value),
);

const filteredScripts = computed(() =>
    filterGrouped(workspace.value?.scripts ?? [], searchQuery.value),
);

const requestTotal = computed(() =>
    (workspace.value?.requests ?? []).reduce((n, g) => n + g.entries.length, 0),
);

const scriptTotal = computed(() =>
    (workspace.value?.scripts ?? []).reduce((n, g) => n + g.entries.length, 0),
);

const skippedCount = computed(
    () => workspace.value?.skipped_requests?.length ?? 0,
);

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
                    placeholder="Search requests & scripts…"
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
                    <Collapsible v-slot="{ open }" :default-open="true">
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
                                <FileText
                                    class="text-sidebar-foreground/70 size-3.5 shrink-0"
                                    aria-hidden="true"
                                />
                                <span
                                    class="text-sidebar-foreground min-w-0 flex-1 font-medium"
                                    >Requests</span
                                >
                                <span
                                    class="text-sidebar-foreground/45 bg-sidebar-accent/40 rounded px-1.5 py-0 font-mono tabular-nums"
                                    >{{ requestTotal }}</span
                                >
                            </CollapsibleTrigger>
                            <CollapsibleContent>
                                <SidebarMenu
                                    class="ml-1 border-sidebar-border border-l pl-2"
                                >
                                    <template
                                        v-if="
                                            filteredRequests.length === 0 &&
                                            requestTotal === 0
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
                                                No request files found
                                            </p>
                                        </div>
                                    </template>
                                    <template
                                        v-else-if="
                                            filteredRequests.length === 0
                                        "
                                    >
                                        <p
                                            class="text-sidebar-foreground/45 px-2 py-3 text-center text-[11px]"
                                        >
                                            No matches
                                        </p>
                                    </template>
                                    <template
                                        v-for="g in filteredRequests"
                                        :key="'rq-' + g.root_index"
                                    >
                                        <SidebarMenuItem
                                            v-if="multiRoot"
                                            class="pointer-events-none"
                                        >
                                            <span
                                                class="text-sidebar-foreground/60 px-1 py-0.5 tracking-wide uppercase"
                                                >{{ g.root_label }}</span
                                            >
                                        </SidebarMenuItem>
                                        <SidebarMenuItem
                                            v-for="e in g.entries"
                                            :key="e.path"
                                        >
                                            <SidebarMenuButton
                                                :is-active="activeId === e.path"
                                                :tooltip="e.path"
                                                class="h-8 font-normal"
                                                @click="
                                                    editor.openFile(
                                                        e.path,
                                                        'request',
                                                        e.name,
                                                    )
                                                "
                                            >
                                                <span class="truncate">{{
                                                    e.name
                                                }}</span>
                                            </SidebarMenuButton>
                                        </SidebarMenuItem>
                                    </template>
                                </SidebarMenu>
                            </CollapsibleContent>
                        </div>
                    </Collapsible>

                    <Collapsible v-slot="{ open }" :default-open="true">
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
                                <Terminal
                                    class="text-sidebar-foreground/70 size-3.5 shrink-0"
                                    aria-hidden="true"
                                />
                                <span
                                    class="text-sidebar-foreground min-w-0 flex-1 font-medium"
                                    >Scripts</span
                                >
                                <span
                                    class="text-sidebar-foreground/45 bg-sidebar-accent/40 rounded px-1.5 py-0 font-mono tabular-nums"
                                    >{{ scriptTotal }}</span
                                >
                            </CollapsibleTrigger>
                            <CollapsibleContent>
                                <SidebarMenu
                                    class="ml-1 border-sidebar-border border-l pl-2"
                                >
                                    <template
                                        v-if="
                                            filteredScripts.length === 0 &&
                                            scriptTotal === 0
                                        "
                                    >
                                        <div
                                            class="text-sidebar-foreground/45 flex flex-col items-center gap-2 py-6 text-center"
                                        >
                                            <Terminal
                                                class="size-8 opacity-40"
                                                aria-hidden="true"
                                            />
                                            <p class="text-[11px]">
                                                No scripts found
                                            </p>
                                        </div>
                                    </template>
                                    <template
                                        v-else-if="filteredScripts.length === 0"
                                    >
                                        <p
                                            class="text-sidebar-foreground/45 px-2 py-3 text-center text-[11px]"
                                        >
                                            No matches
                                        </p>
                                    </template>
                                    <template
                                        v-for="g in filteredScripts"
                                        :key="'sc-' + g.root_index"
                                    >
                                        <SidebarMenuItem
                                            v-if="multiRoot"
                                            class="pointer-events-none"
                                        >
                                            <span
                                                class="text-sidebar-foreground/60 px-1 py-0.5 tracking-wide uppercase"
                                                >{{ g.root_label }}</span
                                            >
                                        </SidebarMenuItem>
                                        <SidebarMenuItem
                                            v-for="e in g.entries"
                                            :key="e.path"
                                        >
                                            <SidebarMenuButton
                                                :is-active="activeId === e.path"
                                                :tooltip="e.path"
                                                class="h-8 font-normal"
                                                @click="
                                                    editor.openFile(
                                                        e.path,
                                                        'script',
                                                        e.name,
                                                    )
                                                "
                                            >
                                                <span class="truncate">{{
                                                    e.name
                                                }}</span>
                                            </SidebarMenuButton>
                                        </SidebarMenuItem>
                                    </template>
                                </SidebarMenu>
                            </CollapsibleContent>
                        </div>
                    </Collapsible>
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
