<script setup lang="ts">
import { appWindow } from "@tauri-apps/api/window";
import {
  register,
  isRegistered,
  unregister,
} from "@tauri-apps/api/globalShortcut";
import { ref, onMounted, Ref, nextTick, reactive, computed } from "vue";
import SearchBar from "./components/SearchBar.vue";
import ItemGroup, { IItemGroup } from "./components/ItemGroup.vue";
import ListItem from "./components/ListItem.vue";
import ApplicationsPlugin from "./plugins/applications";
import WindowsPlugin from "./plugins/windows";

const plugins = reactive([new WindowsPlugin(), new ApplicationsPlugin()]);

const itemGroups = computed((): IItemGroup[] => {
  return plugins
    .map((plugin) => plugin.getItemGroup())
    .filter((itemGroup) => !!itemGroup) as IItemGroup[];
});

onMounted(async () => {
  await initializePlugins();
  await activateFirstListItem();
});

async function initializePlugins() {
  const initializePluginPromises = plugins.map((plugin) => plugin.initialize());
  await Promise.all(initializePluginPromises);
}

const searchString = ref("");

// start: handling of active list items //

const itemGroupRefs: Ref<InstanceType<typeof ItemGroup>[]> = ref([]);
const activeListItemIndex = ref(0);
const isNoResultsTextVisible = ref(false);

function allListItems(): InstanceType<typeof ListItem>[] {
  return itemGroupRefs.value.flatMap(
    (itemGroupRef) => itemGroupRef.getListItemRefs().value
  );
}

function activeItem() {
  return allListItems()[activeListItemIndex.value];
}

function resetActiveListItem() {
  allListItems().forEach((listItem) => listItem.deactivate());
}

async function activateFirstListItem() {
  resetActiveListItem();
  await nextTick();

  if (allListItems().length === 0) {
    isNoResultsTextVisible.value = true;
    return;
  }

  isNoResultsTextVisible.value = false;
  activeListItemIndex.value = 0;
  await nextTick();
  activeItem().activate();
}

function activatePreviousListItem() {
  if (activeListItemIndex.value - 1 < 0) return;

  resetActiveListItem();
  activeListItemIndex.value--;
  activeItem().activate();
}

function activateNextListItem() {
  if (activeListItemIndex.value + 1 >= allListItems().length) return;

  resetActiveListItem();
  activeListItemIndex.value++;
  activeItem().activate();
}

function executeActiveListItemAction() {
  if (!activeItem().hasAction) return;
  activeItem().executeAction();
  searchString.value = "";
}

// end: handling of active list items //
</script>

<!-- ⎇⌘⌃⇧⌥ -->

<template>
  <main
    class="bg-zinc-900 text-white font-mono flex flex-col max-h-full px-5 pt-3"
  >
    <SearchBar
      v-model="searchString"
      @keydown.up="activatePreviousListItem"
      @keydown.down="activateNextListItem"
      @keydown.enter="executeActiveListItemAction"
      @update:model-value="activateFirstListItem"
    />
    <ul class="pointer-events-none overflow-y-auto pb-3">
      <ItemGroup
        v-for="(itemGroup, itemGroupIndex) in itemGroups"
        :key="itemGroupIndex"
        :item-group="itemGroup"
        :search-string="searchString"
        ref="itemGroupRefs"
      />

      <ListItem
        v-if="isNoResultsTextVisible"
        :list-item="{
          title: `No results for: ${searchString}`,
          actions: [],
        }"
        class="pt-5 text-zinc-400"
      />
    </ul>
  </main>
</template>
