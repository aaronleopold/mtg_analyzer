<script lang="ts">
  import ThemeToggle from './ThemeToggle.svelte';

  export let path;

  // TODO: add logged in vs logged out state
  let loggedIn = false;
</script>

<nav class="h-16 w-full shadow-sm bg-white dark:bg-dark-800">
  <div
    class="flex justify-between items-center max-w-sm sm:max-w-2xl md:max-w-6xl lg:max-w-7xl mx-auto flex-1 w-full mt-4"
  >
    <a
      href="/"
      class="text-gray-900 hover:text-gray-700 dark:text-gray-50 dark:hover:text-gray-200"
    >
      <h2 class="text-2xl font-extrabold leading-7 sm:text-3xl">
        MTG Analyzer
      </h2></a
    >

    <div class="flex space-x-6 text-gray-600 dark:text-gray-50 font-medium">
      <ThemeToggle />
      <a
        href="/decks"
        class="hover:text-gray-700 dark:hover:text-gray-200"
        class:active={path === '/decks'}
      >
        Browse
      </a>
      {#if loggedIn}
        <a
          href="/profile"
          class="hover:text-gray-700 dark:hover:text-gray-200"
          class:active={path === '/profile'}
        >
          Profile
        </a>
      {:else}
        <a
          href="/login"
          class="hover:text-gray-700 dark:hover:text-gray-200"
          class:active={path === '/login'}
        >
          Login
        </a>
        <a
          href="/register"
          class="hover:text-gray-700 dark:hover:text-gray-200"
          class:active={path === '/register'}
        >
          Register
        </a>
      {/if}
    </div>
  </div>
</nav>

<style lang="postcss">
  .active {
    /* the dark: is not working?? probably bug with svelte kit */
    @apply font-semibold dark:text-dark-200 text-gray-900;
  }
</style>
