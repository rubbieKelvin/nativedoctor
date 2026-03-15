<script setup lang="ts">
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
            <SelectTrigger class="w-28 rounded-none border-0">
                <SelectValue placeholder="Method" />
            </SelectTrigger>
            <SelectContent>
                <SelectItem v-for="m in HTTP_METHODS" :key="m" :value="m">
                    {{ m }}
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
