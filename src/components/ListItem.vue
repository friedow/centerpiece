<script setup lang="ts">
import { ref } from 'vue';
import { Command } from '@tauri-apps/api/shell';
import { appWindow } from "@tauri-apps/api/window";
import { computed } from '@vue/reactivity';

export interface IListItem {
    title: string;
    action: {
        text: string;
        keys: string[];
        command: {
            program: string;
            args: string[];
        };
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

const hasAction = computed(() => props.listItem.action.command);

async function executeAction() {
    if (!hasAction.value) return;
    appWindow.hide();

    const command = new Command(props.listItem.action.command.program, props.listItem.action.command.args);
    command.execute();
}

defineExpose({
  activate,
  deactivate,
  hasAction,
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