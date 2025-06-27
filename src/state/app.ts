import { defineStore, storeToRefs } from "pinia";
import { computed, ref } from "vue";

export type LoadedProject = {};
export type LoadedRequest = {};

const appState = defineStore("appState", () => {
  let project = ref<LoadedProject | null>(null);
  let requests = ref<Array<LoadedRequest>>([]);

  let screen = computed<"start" | "project">(() => {
    return project.value == null ? "start" : "project";
  });

  return { project, requests, screen };
});

export const useAppStateActions = () => appState();
export const useAppState = () => storeToRefs(appState());
