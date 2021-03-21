<script lang="ts">
  /** @type { number } */
  export let status;
  /** @type { Error } */
  export let error;

  console.log(error);
</script>

<h1
  class="text-xl tracking-loose font-extrabold text-gray-900 dark:text-dark-200 uppercase"
>
  woah there
</h1>

<h3 class="text-lg tracking-loose font-medium text-gray-900 dark:text-dark-200">
  I just got wind of an error that returned a status of {status}
</h3>

<div class="pt-3">
  {#if status === 404}
    <p>The page you tried to request unfortunately does not exist :(</p>
  {:else if status === 500}
    <p>
      Seems like something went wrong with the server! Please check back
      shortly!
    </p>
  {:else}
    <p>I'm not quite sure what happened there!</p>
  {/if}
</div>
