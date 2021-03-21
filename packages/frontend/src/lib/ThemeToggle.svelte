<script>
  // TODO: make store
  let theme = 'light';

  try {
    theme = localStorage.theme;
  } catch (e) {
    // ignore — could be SSR, or e.g. Firefox with restrictive permissions
  }

  const toggle = () => {
    const { classList } = document.querySelector('html');
    classList.remove(theme);
    theme = theme === 'light' ? 'dark' : 'light';
    classList.add(theme);
    try {
      localStorage.theme = theme;
    } catch (e) {
      // ignore
    }
  };
</script>

<button type="button" aria-pressed="false" on:click={toggle}>
  {#if theme === 'dark'}
    <span title="Toggle Light Theme"
      ><svg
        class="w-5 h-5"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
        xmlns="http://www.w3.org/2000/svg"
        ><path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"
        /></svg
      ></span
    >
  {:else}
    <span title="Toggle Dark Theme"
      ><svg
        class="w-5 h-5"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
        xmlns="http://www.w3.org/2000/svg"
        ><path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"
        /></svg
      ></span
    >
  {/if}
</button>
