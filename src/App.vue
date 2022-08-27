<script setup lang="ts">
import { appWindow } from "@tauri-apps/api/window";
import {
  register,
  isRegistered,
  unregister,
} from "@tauri-apps/api/globalShortcut";
import { ref, onMounted, Ref, nextTick, computed } from "vue";
import SearchBar from "./components/SearchBar.vue";
import ItemGroup from "./components/ItemGroup.vue";
import ListItem from "./components/ListItem.vue";

interface IPlugin {
  name: string;
  items: IListItem[];
}

interface IListItem {
  title: string;
  action: {
    keys: string[];
    text: string;
  };
}

const listItems = [
  {
    title: "Alacritty",
    action: {
      keys: ["↵"],
      text: "open",
    },
  },
  {
    title: "Brave",
    action: {
      keys: ["⌘"],
      text: "open",
    },
  },
  {
    title: "VS Code",
    action: {
      keys: ["↵"],
      text: "open",
    },
  },
  {
    title: "XFCE Settings",
    action: {
      keys: ["↵"],
      text: "open",
    },
  },
  {
    title: "Firefox",
    action: {
      keys: ["↵"],
      text: "open",
    },
  },
];

const itemGroups = [
  {
    name: "Apps",
    icon: "rocket",
    items: listItems,
  },
  {
    name: "Open Windows",
    icon: "window-maximize",
    items: listItems,
  },
  {
    name: "Open Windows",
    icon: "window-maximize",
    items: listItems,
  },
  {
    name: "Open Windows",
    icon: "window-maximize",
    items: listItems,
  },
];

onMounted(() => {
  registerGlobalShortcut();
  activateFirstListItem();
});

async function registerGlobalShortcut() {
  if (await isRegistered("Super+Space")) unregister("Super+Space");

  register("Super+Space", async () => {
    if (await appWindow.isVisible()) appWindow.hide();
    else appWindow.show();
  });
}


const searchString = ref("");

// start: handling of active list items //

const itemGroupRefs: Ref<InstanceType<typeof ItemGroup>[]> = ref([]);
const activeListItemIndex = ref(0);

function allListItems(): InstanceType<typeof ListItem>[] {
  return itemGroupRefs.value.flatMap(itemGroupRef => itemGroupRef.getListItemRefs().value);
}

function resetActiveListItem() {
  allListItems().forEach(listItem => listItem.deactivate());
}

async function activateFirstListItem() {
  resetActiveListItem();
  await nextTick();

  if (allListItems().length === 0) return;

  activeListItemIndex.value = 0
  allListItems()[activeListItemIndex.value].activate();
}

function activatePreviousListItem() {
  if ((activeListItemIndex.value - 1) < 0) return;

  resetActiveListItem();
  activeListItemIndex.value--;
  allListItems()[activeListItemIndex.value].activate();
}

function activateNextListItem() {
  if ((activeListItemIndex.value + 1) >= allListItems().length) return;

  resetActiveListItem();
  activeListItemIndex.value++;
  allListItems()[activeListItemIndex.value].activate();
}

// end: handling of active list items //

</script>

<!-- ⎇⌘⌃⇧⌥ -->

<template>
  <main class="bg-zinc-900 text-white font-mono flex flex-col max-h-full px-5 pt-3">
    <SearchBar v-model="searchString" @keydown.up="activatePreviousListItem" @keydown.down="activateNextListItem"
      @update:model-value="activateFirstListItem" />
    <ul class="pointer-events-none overflow-y-auto pb-3">
      <ItemGroup v-for="(itemGroup, itemGroupIndex) in itemGroups" :key="itemGroupIndex" :item-group="itemGroup"
        :search-string="searchString" ref="itemGroupRefs" />
    </ul>
  </main>
</template>
