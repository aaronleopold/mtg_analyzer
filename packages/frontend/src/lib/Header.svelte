<script lang="ts">
  import ThemeToggle from './ThemeToggle.svelte';

  export let path;

  // TODO: add logged in vs logged out state
</script>

<nav class="h-12 flex justify-between items-center w-full">
  <div class="flex items-center space-x-4">
    <a
      href="/"
      class="text-gray-900 hover:text-gray-700 dark:text-gray-50 dark:hover:text-gray-200"
    >
      <h2 class="text-2xl font-extrabold leading-7 sm:text-3xl">
        MTG Analyzer
      </h2></a
    >
  </div>
  <div class="flex space-x-6 text-gray-900 dark:text-gray-50">
    <ThemeToggle />
    <a href="/login" class="hover:text-gray-700 dark:hover:text-gray-200">
      Login
    </a>
    <a href="/register" class="hover:text-gray-700 dark:hover:text-gray-200">
      Register
    </a>
  </div>
</nav>
