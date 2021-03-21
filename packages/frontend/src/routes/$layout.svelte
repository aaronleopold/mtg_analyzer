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

<main
  class="full-w-header pt-6 lg:pt-12 px-4 xs:px-0 max-w-sm sm:max-w-2xl md:max-w-3xl lg:max-w-5xl mx-auto"
>
  <Header path={current_path} />
  <div class="pt-6 flex-1">
    <slot />
  </div>
</main>
<Footer />
