<script setup lang="ts">
import SideBar from "./SideBar.vue";
import HttpResourcePad from "@/components/resourcepads/http/HttpResourcePad.vue";

defineOptions({ name: "DefaultWorkspace" });

defineProps<{
  projectPath?: string;
}>();
</script>

<template>
    <div class="h-full w-full flex flex-row">
        <SideBar />
        <HttpResourcePad />
    </div>
</template>
