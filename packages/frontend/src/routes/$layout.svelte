<script lang="ts">
  import './_global.postcss';
  import { page } from '$app/stores';
  import Header from '$lib/Header.svelte';
  import { onDestroy } from 'svelte';
  import Footer from '$lib/Footer.svelte';

  let current_path;

  // pass to header, I should probably just move this to the header comp
  const unsubscribe = page.subscribe(({ path }) => {
    current_path = path;
  });

  onDestroy(unsubscribe);
</script>

<main class="full-w-header flex flex-col">
  <Header path={current_path} />
  <div
    class="max-w-sm sm:max-w-2xl md:max-w-6xl lg:max-w-7xl mx-auto flex-1 w-full mt-4"
  >
    <slot />
  </div>
</main>
<Footer />
