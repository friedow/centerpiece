<script setup lang="ts">
import { ref } from 'vue';
import { open } from '@tauri-apps/api/shell';
import { appWindow } from "@tauri-apps/api/window";

export interface IListItem {
    title: string;
    action: {
        keys: string[];
        text: string;
        command: string[];
        open: string;
    };
}

const props = defineProps<{
    listItem: IListItem
}>()

const isActive = ref(false);

function activate() {
    isActive.value = true;
}

function deactivate() {
    isActive.value = false;
}

async function executeAction() {
    if (props.listItem.action.open !== "") {
        appWindow.hide();
        await open(props.listItem.action.open);
    }
}

defineExpose({
  activate,
  deactivate,
  executeAction,
})
</script>

<template>
    <li class="flex justify-between items-center py-1.5 px-4"
        :class="{ 'bg-zinc-800': isActive }">
        <span class="text-sm">{{ listItem.title }}</span>
        <div class="text-xs flex gap-1 items-center" :class="{
            visible: isActive,
            invisible: !isActive,
        }">
            <div v-for="key in listItem.action.keys" :key="key" class="
                border-1
                rounded-sm
                w-3.5
                h-3.5
                flex
                justify-center
                items-center
                pt-0.5
              ">
                {{ key }}
            </div>
            <span>{{ listItem.action.text }}</span>
        </div>
    </li>
</template>