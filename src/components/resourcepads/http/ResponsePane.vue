<script setup lang="ts">
import { Badge } from "@/components/ui/badge"
import { ScrollArea } from "@/components/ui/scroll-area"

defineProps<{
  status?: number
  headers?: [string, string][]
  body?: string
  durationMs?: number
  error?: string
}>()

function statusVariant(status: number): "default" | "secondary" | "destructive" | "outline" {
  if (status >= 200 && status < 300) return "default"
  if (status >= 400) return "destructive"
  return "secondary"
}
</script>

<template>
  <div class="rounded-md border border-border bg-muted/20">
    <div class="flex flex-wrap items-center gap-2 border-b border-border px-3 py-2">
      <Badge
        v-if="status != null"
        :variant="statusVariant(status)"
      >
        {{ status }}
      </Badge>
      <span
        v-if="durationMs != null"
        class="text-muted-foreground text-sm"
      >
        {{ durationMs }} ms
      </span>
      <span
        v-if="error"
        class="text-destructive text-sm"
      >
        {{ error }}
      </span>
      <span
        v-if="status == null && !error"
        class="text-muted-foreground text-sm"
      >
        No response yet
      </span>
    </div>
    <div
      v-if="(headers?.length || body != null || error) && !error"
      class="flex flex-1 flex-col gap-2 p-3"
    >
      <div
        v-if="headers?.length"
        class="space-y-1"
      >
        <p class="text-muted-foreground text-xs font-medium uppercase">
          Response headers
        </p>
        <ul class="space-y-0.5 font-mono text-xs">
          <li
            v-for="([name, val], i) in headers"
            :key="i"
            class="flex gap-2 break-all"
          >
            <span class="shrink-0 text-muted-foreground">{{ name }}:</span>
            <span>{{ val }}</span>
          </li>
        </ul>
      </div>
      <div
        v-if="body != null"
        class="flex-1 space-y-1"
      >
        <p class="text-muted-foreground text-xs font-medium uppercase">
          Body
        </p>
        <ScrollArea class="h-[200px] w-full rounded border border-border font-mono text-xs">
          <pre class="whitespace-pre-wrap break-words p-2">{{ body }}</pre>
        </ScrollArea>
      </div>
    </div>
    <div
      v-else-if="error"
      class="p-3"
    >
      <p class="text-destructive text-sm">
        {{ error }}
      </p>
    </div>
  </div>
</template>
