<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getVersion } from "@tauri-apps/api/app";

interface ApiStatusResponse {
  status: "Online" | "Offline";
  message: string;
}

interface Settings {
  app_url: string;
  token: string;
}

const token = ref("");
const appUrl = ref("");
const apiStatus = ref<ApiStatusResponse>();
const isChecking = ref(false);
const version = ref("");

const statusColor = computed(() => {
  if (!apiStatus.value) return "";
  return apiStatus.value.status === "Online"
    ? "text-green-600"
    : "text-red-600";
});

const statusBgColor = computed(() => {
  if (!apiStatus.value) return "";
  return apiStatus.value.status === "Online" ? "bg-green-100" : "bg-red-100";
});

const statusIcon = computed(() => {
  if (!apiStatus.value) return "";
  return apiStatus.value.status === "Online" ? "✓" : "✗";
});

async function check() {
  if (!appUrl.value || !token.value) {
    apiStatus.value = undefined;
    return;
  }

  isChecking.value = true;
  try {
    apiStatus.value = await invoke<ApiStatusResponse>("check_api_status", {
      appUrl: appUrl.value,
      token: token.value,
    });
  } catch (error) {
    apiStatus.value = error as ApiStatusResponse;
  } finally {
    isChecking.value = false;
  }
}

// Load saved settings when component mounts
onMounted(async () => {
  try {
    const savedSettings = await invoke<Settings | null>("load_settings");
    if (savedSettings) {
      appUrl.value = savedSettings.app_url;
      token.value = savedSettings.token;
      // Check API status with loaded settings
      if (appUrl.value && token.value) {
        await check();
      }
    }
  } catch (error) {
    console.error("Failed to load settings:", error);
  }

  version.value = await getVersion();
});
</script>

<template>
  <main class="container mx-auto p-6 max-w-md">
    <div class="bg-white p-6">
      <h2 class="text-2xl font-bold text-gray-800 mb-6">
        Home Assistant Settings
      </h2>

      <form @submit.prevent class="space-y-4">
        <div>
          <label for="url" class="block text-sm font-medium text-gray-700 mb-1">
            Home Assistant URL
          </label>
          <input
            id="url"
            v-model="appUrl"
            @input="check"
            type="url"
            placeholder="http://homeassistant.local:8123"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition duration-200"
          />
        </div>

        <div>
          <label
            for="token"
            class="block text-sm font-medium text-gray-700 mb-1"
          >
            Access Token
          </label>
          <input
            id="token"
            v-model="token"
            @input="check"
            type="password"
            placeholder="Enter your long-lived access token"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition duration-200"
          />
        </div>

        <!-- Status Display -->
        <div v-if="isChecking" class="mt-4 p-4 bg-gray-100 rounded-md">
          <div class="flex items-center justify-center">
            <svg
              class="animate-spin h-5 w-5 mr-2 text-gray-600"
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
            >
              <circle
                class="opacity-25"
                cx="12"
                cy="12"
                r="10"
                stroke="currentColor"
                stroke-width="4"
              ></circle>
              <path
                class="opacity-75"
                fill="currentColor"
                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
              ></path>
            </svg>
            <span class="text-gray-600">Checking connection...</span>
          </div>
        </div>

        <div
          v-else-if="apiStatus"
          class="mt-4 p-4 rounded-md transition-all duration-300"
          :class="statusBgColor"
        >
          <div class="flex items-start">
            <span class="text-2xl mr-3" :class="statusColor">{{
              statusIcon
            }}</span>
            <div class="flex-1">
              <h3 class="font-semibold text-lg" :class="statusColor">
                {{ apiStatus.status }}
              </h3>
              <p class="text-sm text-gray-600 mt-1">{{ apiStatus.message }}</p>
            </div>
          </div>
        </div>

        <!-- Instructions -->
        <div
          v-if="apiStatus?.status !== 'Online'"
          class="mt-6 p-4 bg-blue-50 rounded-md"
        >
          <h4 class="text-sm font-semibold text-blue-900 mb-1">
            How to get your access token:
          </h4>
          <ol class="text-sm text-blue-700 space-y-1 list-decimal list-inside">
            <li>Go to your Home Assistant instance</li>
            <li>Click on your profile</li>
            <li>Scroll down to "Long-Lived Access Tokens"</li>
            <li>Create a new token and copy it here</li>
          </ol>
        </div>
      </form>
    </div>
    <p class="text-center text-sm text-gray-600 mt-4">version: {{ version }}</p>
  </main>
</template>

<style scoped>
/* Additional styles if needed */
</style>
