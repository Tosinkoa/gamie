<script lang="ts">
  import Icon from "@iconify/svelte";
  import { page } from "$app/stores";

  let isOpen = false;

  const navItems = [
    { href: "/games", label: "Games", icon: "ph:game-controller" },
    { href: "/rooms", label: "Rooms", icon: "ph:door" },
    { href: "/premium", label: "Premium", icon: "ph:crown" },
    { href: "/leaderboard", label: "Leaderboard", icon: "ph:trophy" },
    { href: "/tournaments", label: "Tournaments", icon: "ph:flag" },
  ];

  $: currentPath = $page.url.pathname;

  function toggleMenu() {
    isOpen = !isOpen;
  }

  function closeMenu() {
    isOpen = false;
  }
</script>

<nav class="fixed top-0 left-0 right-0 z-50 bg-gray-900 border-b border-gray-800">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <div class="flex justify-between h-16">
      <!-- Logo section -->
      <div class="flex-shrink-0 flex items-center">
        <a href="/" class="flex items-center space-x-2">
          <Icon icon="ph:game-controller" class="h-8 w-8 text-blue-500" />
          <span class="text-white text-xl font-bold">GameConnect</span>
        </a>
      </div>

      <!-- Desktop navigation (lg screens and up) -->
      <div class="hidden lg:flex lg:items-center lg:space-x-4">
        {#each navItems as item}
          <a
            href={item.href}
            class="flex items-center px-3 py-2 rounded-md text-sm font-medium transition-colors duration-200 {currentPath ===
            item.href
              ? 'bg-gray-800 text-white'
              : 'text-gray-300 hover:bg-gray-700 hover:text-white'}"
          >
            <Icon icon={item.icon} class="h-5 w-5 mr-1.5" />
            {item.label}
          </a>
        {/each}
      </div>

      <!-- Profile section (lg screens and up) -->
      <div class="hidden lg:flex lg:items-center lg:space-x-4">
        <a
          href="/profile"
          class="flex items-center px-3 py-2 rounded-md text-sm font-medium text-gray-300 hover:bg-gray-700 hover:text-white transition-colors duration-200"
        >
          <Icon icon="ph:user-circle" class="h-6 w-6 mr-1" />
          <span>Profile</span>
        </a>
      </div>

      <!-- Menu button (below lg screens) -->
      <div class="flex items-center lg:hidden">
        <button
          type="button"
          class="inline-flex items-center justify-center p-2 rounded-md text-gray-400 hover:text-white hover:bg-gray-700 focus:outline-none"
          on:click={toggleMenu}
          aria-expanded={isOpen}
        >
          <span class="sr-only">{isOpen ? "Close menu" : "Open menu"}</span>
          <Icon icon={isOpen ? "ph:x" : "ph:list"} class="h-6 w-6" />
        </button>
      </div>
    </div>
  </div>

  <!-- Mobile/Tablet menu (below lg screens) -->
  <div
    class="lg:hidden {isOpen
      ? 'translate-x-0'
      : '-translate-x-full'} fixed top-16 left-0 right-0 bottom-0 bg-gray-900 transform transition-transform duration-300 ease-in-out overflow-y-auto"
  >
    <div class="px-2 pt-2 pb-3 space-y-1">
      {#each navItems as item}
        <a
          href={item.href}
          class="flex items-center px-3 py-3 rounded-md text-base font-medium transition-colors duration-200 {currentPath ===
          item.href
            ? 'bg-gray-800 text-white'
            : 'text-gray-300 hover:bg-gray-700 hover:text-white'}"
          on:click={closeMenu}
        >
          <Icon icon={item.icon} class="h-5 w-5 mr-2" />
          {item.label}
        </a>
      {/each}

      <!-- Mobile/Tablet profile section -->
      <a
        href="/profile"
        class="w-full flex items-center px-3 py-3 rounded-md text-base font-medium text-gray-300 hover:bg-gray-700 hover:text-white transition-colors duration-200"
        on:click={closeMenu}
      >
        <Icon icon="ph:user-circle" class="h-6 w-6 mr-2" />
        <span>Profile</span>
      </a>
    </div>
  </div>
</nav>

<!-- Spacer to prevent content from hiding under fixed navbar -->
<div class="h-16"></div>

<!-- Backdrop for mobile menu -->
{#if isOpen}
  <div class="lg:hidden fixed inset-0 bg-black bg-opacity-50 z-40" on:click={closeMenu}></div>
{/if}
