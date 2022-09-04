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

const plugins = reactive([new ApplicationsPlugin()]);

const itemGroups = computed((): IItemGroup[] => {
  activateFirstListItem();
  return plugins
    .map(plugin => plugin.getItemGroup())
    .filter(itemGroup => !!itemGroup) as IItemGroup[]
});


onMounted(() => {
  registerGlobalShortcut();
  initializePlugins();
});

async function registerGlobalShortcut() {
  if (await isRegistered("Super+Space")) unregister("Super+Space");

  register("Super+Space", async () => {
    if (await appWindow.isVisible()) appWindow.hide();
    else appWindow.show();
  });
}

function initializePlugins() {
  plugins.forEach(plugin => plugin.initialize());
}


const searchString = ref("");

// start: handling of active list items //

const itemGroupRefs: Ref<InstanceType<typeof ItemGroup>[]> = ref([]);
const activeListItemIndex = ref(0);
const isNoResultsTextVisible = ref(false);
const activeItem = computed(() => allListItems()[activeListItemIndex.value]);

function allListItems(): InstanceType<typeof ListItem>[] {
  return itemGroupRefs.value.flatMap(itemGroupRef => itemGroupRef.getListItemRefs().value);
}

function resetActiveListItem() {
  allListItems().forEach(listItem => listItem.deactivate());
}

async function activateFirstListItem() {
  resetActiveListItem();
  await nextTick();

  if (allListItems().length === 0) {
    isNoResultsTextVisible.value = true;
    return;
  };

  isNoResultsTextVisible.value = false;
  activeListItemIndex.value = 0
  activeItem.value.activate();
}

function activatePreviousListItem() {
  if ((activeListItemIndex.value - 1) < 0) return;

  resetActiveListItem();
  activeListItemIndex.value--;
  activeItem.value.activate();
}

function activateNextListItem() {
  if ((activeListItemIndex.value + 1) >= allListItems().length) return;

  resetActiveListItem();
  activeListItemIndex.value++;
  activeItem.value.activate();
}



function executeActiveListItemAction() {
  if(!activeItem.value.hasAction) return;
  activeItem.value.executeAction();
  searchString.value = "";
}

// end: handling of active list items //

</script>

<!-- ⎇⌘⌃⇧⌥ -->

<template>
  <main class="bg-zinc-900 text-white font-mono flex flex-col max-h-full px-5 pt-3">
    <SearchBar v-model="searchString" @keydown.up="activatePreviousListItem" @keydown.down="activateNextListItem"
      @keydown.enter="executeActiveListItemAction" @update:model-value="activateFirstListItem" />
    <ul class="pointer-events-none overflow-y-auto pb-3">
      <ItemGroup v-for="(itemGroup, itemGroupIndex) in itemGroups" :key="itemGroupIndex" :item-group="itemGroup"
        :search-string="searchString" ref="itemGroupRefs" />

      <ListItem v-if="isNoResultsTextVisible" :list-item="{
        title: `No results for: ${searchString}`,
        action: {
          keys: [],
          text: '',
          open: '',
          command: [],
        }
      }" class="pt-5 text-zinc-400" />
    </ul>
  </main>
</template>
