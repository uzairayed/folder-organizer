<template>
  <div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100">
    <div class="container mx-auto px-4 py-8">
      <!-- Header -->
      <div class="text-center mb-8">
        <h1 class="text-4xl font-bold text-gray-900 mb-2">
          📁 Folder Organizer
        </h1>
        <p class="text-lg text-gray-600">
          Automatically organize your files into categorized folders
        </p>
      </div>

      <!-- Main Card -->
      <div class="max-w-2xl mx-auto bg-white rounded-xl shadow-lg p-8">
        <!-- Folder Selection -->
        <div class="mb-6">
          <label class="block text-sm font-medium text-gray-700 mb-2">
            Select Folder to Organize
          </label>
          <div class="flex gap-3">
            <input
              v-model="selectedFolder"
              type="text"
              placeholder="Choose a folder to organize..."
              class="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              readonly
            />
            <button
              @click="selectFolder"
              class="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-colors"
            >
              Browse
            </button>
          </div>
        </div>

        <!-- Options -->
        <div class="mb-6 space-y-4">
          <div class="flex items-center">
            <input
              v-model="recursive"
              type="checkbox"
              id="recursive"
              class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
            />
            <label for="recursive" class="ml-2 text-sm text-gray-700">
              Include subdirectories (recursive)
            </label>
          </div>
          
          <div class="flex items-center">
            <input
              v-model="dryRun"
              type="checkbox"
              id="dryRun"
              class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
            />
            <label for="dryRun" class="ml-2 text-sm text-gray-700">
              Preview only (dry run)
            </label>
          </div>
        </div>

        <!-- Action Buttons -->
        <div class="flex gap-3 mb-6">
          <button
            @click="organizeFolder"
            :disabled="!selectedFolder || isProcessing"
            class="flex-1 px-6 py-3 bg-green-600 text-white rounded-lg hover:bg-green-700 focus:ring-2 focus:ring-green-500 focus:ring-offset-2 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <span v-if="isProcessing" class="flex items-center justify-center">
              <span class="animate-spin mr-2">⏳</span>
              Processing...
            </span>
            <span v-else>
              {{ dryRun ? 'Preview Changes' : 'Organize Folder' }}
            </span>
          </button>
          
          <button
            @click="undoOrganization"
            :disabled="!selectedFolder || isProcessing"
            class="px-6 py-3 bg-orange-600 text-white rounded-lg hover:bg-orange-700 focus:ring-2 focus:ring-orange-500 focus:ring-offset-2 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            Undo Last
          </button>
        </div>

        <!-- Results -->
        <div v-if="result" class="mt-6 p-4 rounded-lg" :class="result.success ? 'bg-green-50 border border-green-200' : 'bg-red-50 border border-red-200'">
          <div class="flex items-start">
            <div class="flex-shrink-0">
              <span v-if="result.success" class="text-2xl">✅</span>
              <span v-else class="text-2xl">❌</span>
            </div>
            <div class="ml-3">
              <h3 class="text-sm font-medium" :class="result.success ? 'text-green-800' : 'text-red-800'">
                {{ result.success ? 'Success!' : 'Error' }}
              </h3>
              <div class="mt-2 text-sm" :class="result.success ? 'text-green-700' : 'text-red-700'">
                <p>{{ result.message }}</p>
                <div v-if="result.success && result.total_files > 0" class="mt-2 space-y-1">
                  <p>📊 Files processed: {{ result.total_files }}</p>
                  <p>📁 Categories created: {{ result.categories_created }}</p>
                  <p>⏱️ Duration: {{ (result.duration_ms / 1000).toFixed(2) }}s</p>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Categories Info -->
        <div class="mt-6 p-4 bg-gray-50 rounded-lg">
          <h3 class="text-sm font-medium text-gray-900 mb-3">File Categories</h3>
          <div class="grid grid-cols-2 gap-2 text-xs text-gray-600">
            <div>📷 Images (jpg, png, gif, etc.)</div>
            <div>🎥 Videos (mp4, avi, mov, etc.)</div>
            <div>📄 Documents (pdf, doc, txt, etc.)</div>
            <div>🎵 Audio (mp3, wav, flac, etc.)</div>
            <div>📦 Archives (zip, rar, 7z, etc.)</div>
            <div>💻 Code (py, js, java, etc.)</div>
            <div>📊 Spreadsheets (xls, csv, etc.)</div>
            <div>📋 Presentations (ppt, pptx, etc.)</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

export default {
  name: 'App',
  setup() {
    const selectedFolder = ref('')
    const recursive = ref(false)
    const dryRun = ref(true)
    const isProcessing = ref(false)
    const result = ref(null)

    const selectFolder = async () => {
      try {
        const folder = await invoke('select_folder')
        if (folder) {
          selectedFolder.value = folder
        }
      } catch (error) {
        console.error('Error selecting folder:', error)
      }
    }

    const organizeFolder = async () => {
      if (!selectedFolder.value) return
      
      isProcessing.value = true
      result.value = null
      
      try {
        const response = await invoke('organize_folder', {
          targetDir: selectedFolder.value,
          recursive: recursive.value,
          dryRun: dryRun.value
        })
        
        result.value = response
      } catch (error) {
        result.value = {
          success: false,
          message: error
        }
      } finally {
        isProcessing.value = false
      }
    }

    const undoOrganization = async () => {
      if (!selectedFolder.value) return
      
      isProcessing.value = true
      result.value = null
      
      try {
        const message = await invoke('undo_organization', {
          targetDir: selectedFolder.value
        })
        
        result.value = {
          success: true,
          message: message
        }
      } catch (error) {
        result.value = {
          success: false,
          message: error
        }
      } finally {
        isProcessing.value = false
      }
    }

    return {
      selectedFolder,
      recursive,
      dryRun,
      isProcessing,
      result,
      selectFolder,
      organizeFolder,
      undoOrganization
    }
  }
}
</script>

<style>
/* Add any custom styles here */
</style> 