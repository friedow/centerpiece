<script setup lang="ts">
import { appWindow } from "@tauri-apps/api/window";
import { onMounted } from "vue";

const props = defineProps<{
  modelValue: string
}>()

const emit = defineEmits<{
  (event: 'update:modelValue', text: string): void
}>()

onMounted(() => {
  trapFocusInSearchbar();
});

function trapFocusInSearchbar() {
  const searchBar = document.getElementById("search-bar");

  if (!searchBar) return;
  searchBar.addEventListener("focusout", () => {
    setTimeout(() => {
      searchBar.focus();
    }, 0);
  });
}
</script>

<template>
  <div class="border-b-1 border-zinc-700 relative">
    <font-awesome-icon
      icon="fa-solid fa-magnifying-glass"
      class="
        absolute
        mt-3
        ml-4
        text-base
        mb-0.5
        pointer-events-none
        text-zinc-500
      "
    />
    <input
      id="search-bar"
      class="flex-1 py-2 pl-11 pr-4 bg-transparent text-white w-full"
      :value="modelValue"
      @keydown.esc="appWindow.hide"
      @input="$emit('update:modelValue', $event.target.value)"
      autofocus
    />
  </div>
</template>