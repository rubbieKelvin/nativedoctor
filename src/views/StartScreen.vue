<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

async function create_project() {
  let res = await invoke("create_project");
}
</script>

<template>
  <div class="h-full flex items-center justify-center">
    <div>
      <h1 class="text-3xl">Native Doctor</h1>
      <div class="flex gap-4">
        <button class="px-4 flex-grow py-2 bg-gray-200 hover:bg-gray-400">
          Open
        </button>
        <button
          class="px-4 flex-grow py-2 bg-gray-200 hover:bg-gray-400"
          @click="create_project"
        >
          Create
        </button>
      </div>
    </div>
  </div>
</template>
