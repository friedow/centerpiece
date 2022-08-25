<script setup lang="ts">
import { appWindow } from "@tauri-apps/api/window";
import {
  register,
  isRegistered,
  unregister,
} from "@tauri-apps/api/globalShortcut";
import { ref, onMounted } from "vue";

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
      keys: ["↵"],
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

const plugins = [
  {
    name: "Apps",
    icon: "rocket",
    items: listItems,
  },
  {
    name: "Open Windows",
    icon: "rocket",
    items: listItems,
  },
];

const activeListItemIndex = ref(0);

function decreaseActiveListItemIndex() {
  if (activeListItemIndex.value <= 0) return;
  activeListItemIndex.value--;
}

function increaseActiveListItemIndex() {
  if (activeListItemIndex.value >= listItems.length - 1) return;
  activeListItemIndex.value++;
}

onMounted(() => {
  registerGlobalShortcut();
  trapFocusInSearchbar();
});

async function registerGlobalShortcut() {
  if (await isRegistered("Super+Space")) unregister("Super+Space");

  register("Super+Space", async () => {
    if (await appWindow.isVisible()) appWindow.hide();
    else appWindow.show();
  });
}

function trapFocusInSearchbar() {
  const searchBar = document.getElementById("search-bar");

  if (!searchBar) return;
  searchBar.addEventListener("focusout", () => {
    setTimeout(() => {
      searchBar.focus();
    }, 0);
  });
}

const searchString = ref("");

function filteredListItems() {
  return listItems.filter((listItem) => {
    return listItem.title
      .toLowerCase()
      .includes(searchString.value.toLowerCase());
  });
}
</script>

<!-- ⎇⌘⌃⇧⌥ -->

<template>
  <main class="bg-zinc-900 text-white">
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
        @keydown.up="decreaseActiveListItemIndex"
        @keydown.down="increaseActiveListItemIndex"
        @keydown.esc="appWindow.hide"
        v-model="searchString"
        autofocus
      />
    </div>
    <ul class="pointer-events-none">
      <li
        v-for="(listItem, listItemIndex) in filteredListItems()"
        :key="listItemIndex"
        class="flex justify-between items-center py-1.5 px-4"
        :class="{ 'bg-zinc-800': activeListItemIndex === listItemIndex }"
      >
        <span>{{ listItem.title }}</span>
        <div
          class="text-xs flex gap-1 items-center"
          :class="{
            visible: activeListItemIndex === listItemIndex,
            invisible: activeListItemIndex !== listItemIndex,
          }"
        >
          <div
            v-for="key in listItem.action.keys"
            :key="key"
            class="
              border-1
              rounded-sm
              w-3.5
              h-3.5
              flex
              justify-center
              items-center
            "
          >
            {{ key }}
          </div>
          <span>{{ listItem.action.text }}</span>
        </div>
      </li>
    </ul>
  </main>
</template>
