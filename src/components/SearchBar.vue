<script setup lang="ts">
import { onMounted } from "vue";
import Icon from "../Icon.vue";

const props = defineProps<{
  modelValue: string;
}>();

const emit = defineEmits<{
  (event: "update:modelValue", text: string): void;
}>();

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

function bubbleInputEvent(event: Event) {
  const target = event.target as HTMLInputElement;
  emit("update:modelValue", target.value);
}
</script>

<template>
  <div class="border-b-1 border-zinc-700 flex gap-4 p-2">
    <Icon name="Search" class="w-7 h-7" />
    <input
      id="search-bar"
      class="flex-1 bg-transparent text-white w-full h-7"
      :value="modelValue"
      @input="bubbleInputEvent"
      autofocus
    />
  </div>
</template>
