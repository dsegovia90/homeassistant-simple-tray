<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { ref, onMounted } from "vue";

interface BooleanEntity {
  id: string;
  friendly_name: string;
  state: string;
}
const entities = ref<BooleanEntity[]>([]);
const checkedEntities = ref<Map<string, BooleanEntity>>(new Map());

const getBooleanEntities = async () => {};

const addToList = async (entity: BooleanEntity, e: Event) => {
  const target = e.target as HTMLInputElement;
  try {
    await invoke("save_entity_to_store", { entity, save: target.checked });
    console.log(`Entity ${entity.id} added to store`);
  } catch (error) {
    console.error("Failed to save entity:", error);
  }
};

onMounted(async () => {
  const loadedEntities = await invoke<BooleanEntity[]>(
    "load_entities_from_store",
  );
  console.log(loadedEntities);
  const booleanEntities = await invoke<BooleanEntity[]>("get_switch_entities");

  entities.value = booleanEntities;
  loadedEntities.forEach((entity) => {
    checkedEntities.value.set(entity.id, entity);
  });
});
</script>

<template>
  <div class="p-6 max-w-md mx-auto">
    <h2 class="text-2xl font-bold text-gray-800 mb-6">Configuration</h2>

    <button @click="getBooleanEntities">Get Boolean Entities</button>

    <div v-if="entities.length > 0" class="mt-6">
      <h3 class="text-lg font-semibold text-gray-700 mb-4">
        Available Entities
      </h3>
      <ul class="space-y-2">
        <li
          v-for="entity in entities"
          :key="entity.id"
          class="flex items-center justify-between p-3 bg-gray-50 rounded-lg hover:bg-gray-100"
        >
          <div class="flex-1">
            <p class="font-medium text-gray-800">{{ entity.friendly_name }}</p>
            <p class="text-sm text-gray-500">{{ entity.id }}</p>
          </div>
          <input
            type="checkbox"
            @change="(e) => addToList(entity, e)"
            :checked="checkedEntities.has(entity.id)"
            class="h-5 w-5 text-blue-600 rounded focus:ring-blue-500"
          />
        </li>
      </ul>
    </div>
  </div>
</template>
