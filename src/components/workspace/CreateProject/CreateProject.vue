<script setup lang="ts">
import { ref, inject } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";

const setCurrentProject = inject<(path: string | null) => void>("setCurrentProject");
const setShowCreateProject = inject<(show: boolean) => void>("setShowCreateProject");

const name = ref("");
const description = ref("");
const folderPath = ref("");
const error = ref("");
const creating = ref(false);

async function handleBrowse() {
  const selected = await open({
    directory: true,
    multiple: false,
  });
  if (selected && typeof selected === "string") {
    folderPath.value = selected;
  }
}

async function handleCreate() {
  const path = folderPath.value.trim();
  const projectName = name.value.trim() || "Unnamed project";
  if (!path) {
    error.value = "Choose a folder";
    return;
  }
  error.value = "";
  creating.value = true;
  try {
    const result = await invoke<string>("create_project", {
      folderPath: path,
      name: projectName,
      description: description.value.trim(),
    });
    await invoke("add_recent_project", { path: result, name: projectName });
    setCurrentProject?.(result);
    setShowCreateProject?.(false);
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e);
  } finally {
    creating.value = false;
  }
}

function handleBack() {
  setShowCreateProject?.(false);
}
</script>

<template>
  <div class="flex h-full w-full flex-col gap-6 p-8">
    <div class="flex items-center gap-2">
      <Button
        variant="ghost"
        size="sm"
        @click="handleBack"
      >
        Back
      </Button>
      <h1 class="text-2xl font-semibold">Create project</h1>
    </div>
    <form
      class="flex max-w-md flex-col gap-4"
      @submit.prevent="handleCreate"
    >
      <div class="flex flex-col gap-2">
        <label
          for="name"
          class="text-sm font-medium"
        >
          Project name
        </label>
        <Input
          id="name"
          v-model="name"
          placeholder="My API project"
        />
      </div>
      <div class="flex flex-col gap-2">
        <label
          for="description"
          class="text-sm font-medium"
        >
          Description
        </label>
        <Textarea
          id="description"
          v-model="description"
          placeholder="Optional description"
          class="min-h-[80px]"
        />
      </div>
      <div class="flex flex-col gap-2">
        <label
          for="folder"
          class="text-sm font-medium"
        >
          Folder
        </label>
        <div class="flex gap-2">
          <Input
            id="folder"
            v-model="folderPath"
            placeholder="Choose a folder"
            readonly
            class="flex-1"
          />
          <Button
            type="button"
            variant="outline"
            @click="handleBrowse"
          >
            Browse
          </Button>
        </div>
      </div>
      <p
        v-if="error"
        class="text-destructive text-sm"
      >
        {{ error }}
      </p>
      <Button
        type="submit"
        :disabled="creating || !folderPath.trim()"
      >
        {{ creating ? "Creating…" : "Create project" }}
      </Button>
    </form>
  </div>
</template>
