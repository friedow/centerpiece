<script setup lang="ts">
import { ref, Ref } from 'vue';
import ListItem, { IListItem } from './ListItem.vue';

export interface IItemGroup {
  name: string;
  icon: string;
  items: IListItem[];
}

const props = defineProps<{
  itemGroup: IItemGroup,
  searchString: string,
}>()

function filteredListItems(): IListItem[] {
  return props.itemGroup.items.filter((listItem) => {
    return listItem.title
      .toLowerCase()
      .includes(props.searchString.toLowerCase());
  });
}

const listItemRefs: Ref<InstanceType<typeof ListItem>[]> = ref([]);

function getListItemRefs(): Ref<InstanceType<typeof ListItem>[]> {
  return listItemRefs;
}

defineExpose({
  getListItemRefs,
})
</script>

<template>
  <li v-if="filteredListItems().length > 0" class="
            px-4 pt-4 pb-1.5
            border-t-1 border-zinc-700 first:border-t-0
            font-bold font-sans
          " style="font-size: 0.6rem; line-height: 0.75rem;">
    <font-awesome-icon :icon="`fa-solid fa-${itemGroup.icon}`" class="text-zinc-300 mr-1.5" />
    <span>
      {{ itemGroup.name }}
    </span>
  </li>
  <ListItem v-for="(listItem, listItemIndex) in filteredListItems()" :key="listItemIndex" :list-item="listItem"
    ref="listItemRefs" />
</template>