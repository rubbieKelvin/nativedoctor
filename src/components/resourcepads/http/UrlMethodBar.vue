<script setup lang="ts">
import { computed } from "vue";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select";
import { HTTP_METHODS, type HttpMethodType } from "@/shared/constants/http";

const url = defineModel<string>("url");
const method = defineModel<HttpMethodType>("method", {
    default: "GET",
});

defineProps<{
    loading?: boolean;
}>();

const emit = defineEmits<{
    (e: "send"): void;
}>();

function onSend() {
    emit("send");
}

function methodColor(m: string): string {
    switch (m) {
        case "GET":
            return "text-green-600 dark:text-green-400";
        case "POST":
            return "text-blue-600 dark:text-blue-400";
        case "PUT":
            return "text-orange-600 dark:text-orange-400";
        case "PATCH":
            return "text-yellow-600 dark:text-yellow-400";
        case "DELETE":
            return "text-red-600 dark:text-red-400";
        case "HEAD":
        case "OPTIONS":
            return "text-muted-foreground";
        default:
            return "text-muted-foreground";
    }
}

const triggerMethodColor = computed(() =>
    methodColor((method as any) ?? "GET"),
);
</script>

<template>
    <div class="flex flex-wrap items-center border-b border-border">
        <Select
            :model-value="method"
            @update:model-value="
                (v) => {
                    method = (v as HttpMethodType) ?? 'GET';
                }
            "
        >
            <SelectTrigger
                :class="[
                    'w-26 rounded-none border-0 text-xs font-medium',
                    triggerMethodColor,
                ]"
            >
                <SelectValue placeholder="Method" />
            </SelectTrigger>
            <SelectContent>
                <SelectItem
                    v-for="m in HTTP_METHODS"
                    :key="m"
                    :value="m"
                    :class="methodColor(m)"
                >
                    <span class="text-xs">{{ m }}</span>
                </SelectItem>
            </SelectContent>
        </Select>
        <Input
            v-model="url"
            type="url"
            placeholder="https://api.example.com/..."
            class="min-w-50 flex-1 rounded-none border-0"
        />
        <Button
            class="border-0 rounded-none"
            :disabled="loading || !url?.trim()"
            @click="onSend"
        >
            {{ loading ? "Sending…" : "Send" }}
        </Button>
    </div>
</template>
