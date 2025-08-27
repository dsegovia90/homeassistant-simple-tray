<script setup lang="ts">
import { ref } from "vue";
import SettingsForm from "./components/SettingsForm.vue";
import Config from "./components/Config.vue";

const activeTab = ref<"settings" | "config">("settings");
</script>

<template>
  <div class="flex flex-col h-screen bg-gray-50">
    <!-- Main content area -->
    <main class="flex-1 overflow-y-auto">
      <div v-if="activeTab === 'settings'">
        <SettingsForm />
      </div>
      <div v-else-if="activeTab === 'config'">
        <Config />
      </div>
    </main>

    <!-- Bottom tab navigation -->
    <nav class="bg-white border-t border-gray-200 px-4 py-2">
      <div class="flex justify-around">
        <!-- Settings Tab -->
        <button
          @click="activeTab = 'settings'"
          :class="[
            'flex flex-col items-center py-2 px-4 rounded-lg transition-colors duration-200',
            activeTab === 'settings'
              ? 'text-blue-600 bg-blue-50'
              : 'text-gray-600 hover:text-gray-800 hover:bg-gray-50',
          ]"
        >
          <svg
            class="w-6 h-6 mb-1"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
            />
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
            />
          </svg>
          <span class="text-xs font-medium">Settings</span>
        </button>

        <!-- Config Tab -->
        <button
          @click="activeTab = 'config'"
          :class="[
            'flex flex-col items-center py-2 px-4 rounded-lg transition-colors duration-200',
            activeTab === 'config'
              ? 'text-blue-600 bg-blue-50'
              : 'text-gray-600 hover:text-gray-800 hover:bg-gray-50',
          ]"
        >
          <svg
            class="w-6 h-6 mb-1"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4"
            />
          </svg>
          <span class="text-xs font-medium">Config</span>
        </button>
      </div>
    </nav>
  </div>
</template>

<style scoped>
/* Additional styles if needed */
</style>
