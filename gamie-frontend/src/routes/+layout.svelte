<script lang="ts">
  import "../app.css";
  import Navbar from "$lib/components/Navbar.svelte";
  import { page } from "$app/stores";
  let { children } = $props();

  const isGameRoute = $derived($page.url.pathname.startsWith("/game/"));
</script>

<div class="min-h-[100dvh] flex flex-col bg-gradient-to-br from-gray-800 to-black">
  {#if !isGameRoute}
    <Navbar />
  {/if}
  <main
    class="flex-1 {!isGameRoute ? '' : 'h-[100dvh]'} bg-gradient-to-br from-gray-800 to-black"
  >
    {@render children()}
  </main>
</div>
