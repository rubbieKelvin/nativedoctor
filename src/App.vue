<script setup lang="ts">
import DefaultWorkspace from "@/components/workspace/DefaultWorkspace/DefaultWorkspace.vue";
import { ref } from "vue";

// the directory we want to pull our files from
const workingDirectory = ref<string | null>();
</script>

<template>
    <DefaultWorkspace />
</template>
