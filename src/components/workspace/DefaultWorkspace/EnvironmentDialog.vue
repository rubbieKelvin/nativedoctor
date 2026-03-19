<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Button } from "@/components/ui/button";
import { RefreshCw } from "lucide-vue-next";
import { useCurrentProject, useCurrentProjectActions } from "@/store/project";

const props = defineProps<{
  open: boolean;
}>();

const emit = defineEmits<{
  (e: "update:open", value: boolean): void;
}>();

const { path: projectPath, config } = useCurrentProject();
const { updateProjectConfig } = useCurrentProjectActions();

const envVars = ref<Record<string, string>>({});
const loadError = ref<string | null>(null);
const loading = ref(false);

const envSources = computed(() => config.value?.envSources ?? []);
const selectedEnvName = computed(() => config.value?.selectedEnv ?? null);

const selectedSource = computed(() => {
  const name = selectedEnvName.value;
  if (!name) return envSources.value[0] ?? null;
  return envSources.value.find((s) => s.name === name) ?? envSources.value[0] ?? null;
});

const envEntries = computed(() =>
  Object.entries(envVars.value).filter(([k]) => k.trim()),
);

async function loadEnv() {
  const path = projectPath.value;
  const source = selectedSource.value;
  if (!path || !source) {
    envVars.value = {};
    loadError.value = null;
    return;
  }
  loading.value = true;
  loadError.value = null;
  try {
    envVars.value = await invoke<Record<string, string>>("load_env_file", {
      projectPath: path,
      relativePath: source.path,
    });
  } catch (e) {
    loadError.value = e instanceof Error ? e.message : String(e);
    envVars.value = {};
  } finally {
    loading.value = false;
  }
}

async function onSelectEnv(name: string) {
  await updateProjectConfig({ selectedEnv: name });
  await loadEnv();
}

async function onRefresh() {
  await loadEnv();
}

watch(
  () => [props.open, selectedSource.value] as const,
  async ([open, source]) => {
    if (open && source) await loadEnv();
  },
  { immediate: true },
);
</script>

<template>
  <Dialog :open="open" @update:open="emit('update:open', $event)">
    <DialogContent class="sm:max-w-xl">
      <DialogHeader>
        <DialogTitle>Environment</DialogTitle>
      </DialogHeader>
      <div v-if="!projectPath" class="text-muted-foreground text-sm">
        No project open
      </div>
      <template v-else>
        <div v-if="envSources.length === 0" class="text-muted-foreground text-sm">
          No environment sources in this project. Add envSources to nativedoctor.json.
        </div>
        <template v-else>
          <div class="flex items-center gap-2">
            <span class="text-muted-foreground shrink-0 text-sm">Environment</span>
            <Select
              :model-value="selectedEnvName ?? selectedSource?.name ?? ''"
              @update:model-value="onSelectEnv"
            >
              <SelectTrigger class="w-[200px]">
                <SelectValue placeholder="Select environment" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem
                  v-for="s in envSources"
                  :key="s.name"
                  :value="s.name"
                >
                  {{ s.name }}
                </SelectItem>
              </SelectContent>
            </Select>
            <Button
              variant="outline"
              size="icon"
              :disabled="!selectedSource || loading"
              @click="onRefresh"
            >
              <RefreshCw
                :class="['size-4', loading && 'animate-spin']"
              />
              <span class="sr-only">Refresh</span>
            </Button>
          </div>
          <div v-if="loadError" class="text-destructive text-sm">
            {{ loadError }}
          </div>
          <div
            v-if="selectedSource"
            class="rounded-md border border-border overflow-hidden"
          >
            <table class="w-full text-sm">
              <tbody>
                <tr
                  v-for="[key, value] in envEntries"
                  :key="key"
                  class="border-b border-border last:border-b-0"
                >
                  <td
                    class="w-[40%] border-r border-border bg-muted/30 px-3 py-2 font-medium"
                  >
                    {{ key }}
                  </td>
                  <td class="px-3 py-2 text-muted-foreground">{{ value }}</td>
                </tr>
              </tbody>
            </table>
            <p
              v-if="envEntries.length === 0 && !loadError"
              class="text-muted-foreground px-3 py-4 text-sm"
            >
              No variables
            </p>
          </div>
        </template>
      </template>
    </DialogContent>
  </Dialog>
</template>
