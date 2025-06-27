<script setup lang="ts">
import { useAppState } from "./state/app";
import StartScreen from "./views/StartScreen.vue";

let allow_global_context_menu = true;
const { screen } = useAppState();

const handleRootContextMenu = (e: MouseEvent) => {
  if (!allow_global_context_menu) {
    e.preventDefault();
  }
};
</script>

<template>
  <div id="app-inner" class="h-full" @contextmenu="handleRootContextMenu">
    <StartScreen v-if="screen === 'start'" />
  </div>
</template>
