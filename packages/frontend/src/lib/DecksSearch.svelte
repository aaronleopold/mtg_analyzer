<script>
  import Dropdown from './ui/Dropdown.svelte';
  import Input from './ui/Input.svelte';

  let activeFilter = 'date_created';
  let format = 'any';
  let deckSource = 'all';
</script>

<div class="border-b border-gray-200 dark:border-dark-500">
  <nav class="mb-2 flex justify-between items-center" aria-label="Tabs">
    <div class="flex space-x-8">
      <span
        class="cursor-pointer border-transparent text-gray-800 dark:text-dark-200 hover:text-gray-700 hover:border-gray-300 py-4 px-1 border-b-2 font-medium text-sm  -mb-2"
        class:activeDeck={deckSource === 'all'}
        on:click={() => (deckSource = 'all')}
      >
        <span>All Decks</span>
      </span>

      <span
        class="cursor-pointer border-transparent text-gray-800 dark:text-dark-200 hover:text-gray-700 hover:border-gray-300 py-4 px-1 border-b-2 font-medium text-sm  -mb-2"
        class:activeDeck={deckSource === 'favorites'}
        on:click={() => (deckSource = 'favorites')}
      >
        <span>Favorites</span>
      </span>
    </div>

    <div class="flex space-x-6">
      <!-- TODO: change this to a select component -->
      <Dropdown label="Order By">
        <div slot="options">
          <span
            class="cursor-pointer block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 hover:text-gray-900"
            class:activeFilter={activeFilter === 'date_created'}
            on:click={() => (activeFilter = 'date_created')}>Date Created</span
          >
        </div>
      </Dropdown>

      <Dropdown label="Format">
        <div slot="options">
          <span
            class="cursor-pointer block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 hover:text-gray-900"
            class:activeFilter={format === 'any'}
            on:click={() => (format = 'any')}>Any</span
          >
        </div>
      </Dropdown>

      <Input />
    </div>
  </nav>
</div>

<style lang="postcss">
  .activeFilter {
    @apply border-indigo-500 text-indigo-600;
  }

  .activeDeck {
    @apply border-indigo-500 text-indigo-600;
  }
</style>
