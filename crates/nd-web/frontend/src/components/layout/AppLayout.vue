<template>
    <div class="flex h-full min-h-0 font-mono text-xs">
        <slot name="sidebar" />
        <div class="flex min-w-0 flex-1 flex-col bg-muted/20">
            <slot />
        </div>
    </div>
</template>
