<script setup lang="ts">
import { Button } from "@/components/ui/button";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuLabel,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { AlertCircle, CheckCheck } from "lucide-vue-next";

defineProps<{
    loadFailures: Array<{ fileName: string; error?: string }>;
}>();
</script>

<template>
    <div>
        <!-- Load failures -->
        <div
            class="flex shrink-0 items-center gap-1.5 border-t border-sidebar-border py-0.5 px-2"
        >
            <DropdownMenu>
                <DropdownMenuTrigger as-child>
                    <Button
                        size="icon"
                        variant="ghost"
                        class="size-7"
                        :class="
                            loadFailures.length
                                ? 'text-destructive hover:text-destructive'
                                : ''
                        "
                        title="Show load errors"
                    >
                        <AlertCircle v-if="loadFailures?.length > 0" />
                        <CheckCheck v-else class="size-3.5" />
                    </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent
                    align="start"
                    side="top"
                    class="max-h-60 max-w-xs"
                >
                    <DropdownMenuLabel class="text-xs font-normal">
                        <template v-if="loadFailures.length">
                            {{ loadFailures.length }}
                            {{ loadFailures.length === 1 ? "file" : "files" }}
                            could not be loaded
                        </template>
                        <template v-else>
                            All files loaded succesfully
                        </template>
                    </DropdownMenuLabel>
                    <template v-if="loadFailures.length">
                        <DropdownMenuSeparator />
                        <div
                            class="max-h-40 space-y-1 overflow-y-auto px-2 py-1"
                        >
                            <div
                                v-for="(f, i) in loadFailures"
                                :key="i"
                                class="rounded-sm px-2 py-1.5 text-xs"
                            >
                                <p
                                    class="truncate font-medium text-foreground"
                                    :title="f.fileName"
                                >
                                    {{ f.fileName }}
                                </p>
                                <p
                                    v-if="f.error"
                                    class="mt-0.5 truncate text-muted-foreground"
                                    :title="f.error"
                                >
                                    {{ f.error }}
                                </p>
                            </div>
                        </div>
                    </template>
                </DropdownMenuContent>
            </DropdownMenu>
        </div>
    </div>
</template>
