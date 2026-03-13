<script setup lang="ts">
import type { ScrollAreaRootProps } from "reka-ui"
import type { HTMLAttributes } from "vue"
import { ref } from "vue"
import { reactiveOmit } from "@vueuse/core"
import {
  ScrollAreaCorner,
  ScrollAreaRoot,
  ScrollAreaViewport,
} from "reka-ui"
import { cn } from "@/lib/utils"
import ScrollBar from "./ScrollBar.vue"

const props = withDefaults(
  defineProps<
    ScrollAreaRootProps & {
      class?: HTMLAttributes["class"]
      orientation?: "vertical" | "horizontal"
    }
  >(),
  { orientation: "vertical" },
)

const delegatedProps = reactiveOmit(props, "class", "orientation")

const viewportRef = ref<InstanceType<typeof ScrollAreaViewport> | null>(null)

function getViewportElement(): HTMLElement | null {
  const el = viewportRef.value?.viewportElement
  return (el && typeof el === "object" && "value" in el ? (el as { value: HTMLElement }).value : el) ?? null
}

defineExpose({ getViewportElement })

const emit = defineEmits<{ scroll: [event: Event] }>()
</script>

<template>
  <ScrollAreaRoot
    data-slot="scroll-area"
    v-bind="delegatedProps"
    :class="cn('relative', props.class)"
  >
    <ScrollAreaViewport
      ref="viewportRef"
      data-slot="scroll-area-viewport"
      class="focus-visible:ring-ring/50 size-full rounded-[inherit] transition-[color,box-shadow] outline-none focus-visible:ring-[3px] focus-visible:outline-1"
      @scroll="emit('scroll', $event)"
    >
      <slot />
    </ScrollAreaViewport>
    <ScrollBar :orientation="orientation" />
    <ScrollAreaCorner />
  </ScrollAreaRoot>
</template>
