<script setup lang="ts">
import { computed } from "vue";
import type { HttpResource } from "@/shared/types/resources";
import { Globe } from "lucide-vue-next";
import { Badge } from "@/components/ui/badge";

const props = defineProps<{
    resource: HttpResource;
}>();

defineEmits<{
    (e: "select", id: string): void;
}>();

const methodColor = computed(() => {
    switch (props.resource.method) {
        case "GET":
            return "text-green-600 bg-green-500/10";
        case "POST":
            return "text-blue-600 bg-blue-500/10";
        case "PUT":
            return "text-orange-600 bg-orange-500/10";
        case "PATCH":
            return "text-yellow-600 bg-yellow-500/10";
        case "DELETE":
            return "text-red-600 bg-red-500/10";
        default:
            return "text-muted-foreground bg-muted";
    }
});
</script>

<template>
    <button
        class="flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-sm text-sidebar-foreground hover:bg-sidebar-accent hover:text-sidebar-accent-foreground"
        @click="$emit('select', resource.id)"
    >
        <Globe class="size-4 shrink-0 text-muted-foreground" />
        <Badge
            variant="outline"
            :class="['px-1.5 py-0 text-[10px] font-semibold', methodColor]"
        >
            {{ resource.method }}
        </Badge>
        <span class="truncate">{{ resource.name || "Untitled request" }}</span>
    </button>
</template>
