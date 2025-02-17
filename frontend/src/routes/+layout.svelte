<script lang="ts">
  import "../app.css";
  import Navbar from "$lib/components/Navbar.svelte";
  let { children } = $props();
</script>

<div class="min-h-[100dvh] flex flex-col">
  <Navbar />
  <main class="flex-1">
    {@render children()}
  </main>
</div>
