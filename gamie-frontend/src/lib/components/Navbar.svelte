<script lang="ts">
  import Icon from "@iconify/svelte";
  import { page } from "$app/stores";
  import { userStore } from "$lib/stores/user";
  import { authService } from "$lib/services/auth";
  import { goto } from "$app/navigation";

  let isOpen = false;
  let isProfileOpen = false;

  const navItems = [
    { href: "/games", label: "Games", icon: "ph:game-controller" },
    { href: "/rooms", label: "Rooms", icon: "ph:door" },
    { href: "/premium", label: "Premium", icon: "ph:crown" },
    { href: "/leaderboard", label: "Leaderboard", icon: "ph:trophy" },
    { href: "/tournaments", label: "Tournaments", icon: "ph:flag" },
    { href: "/feedback", label: "Feedback", icon: "ph:megaphone-simple" },
  ];

  $: currentPath = $page.url.pathname;
  $: isAuthenticated = !!$userStore.user;

  function toggleMenu() {
    isOpen = !isOpen;
  }

  function closeMenu() {
    isOpen = false;
  }

  function toggleProfile() {
    isProfileOpen = !isProfileOpen;
  }

  function closeProfile() {
    isProfileOpen = false;
  }

  async function handleLogout() {
    try {
      await authService.logout();
      closeProfile();
      closeMenu();
      goto("/login");
    } catch (error) {
      console.error("Logout failed:", error);
      // Force logout even if the server request fails
      userStore.logout();
      goto("/login");
    }
  }
</script>

<nav class="fixed top-0 left-0 right-0 z-50 bg-gray-900 border-b border-gray-800">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <div class="flex justify-between h-16">
      <!-- Logo section -->
      <div class="flex-shrink-0 flex items-center">
        <a href="/" class="flex items-center space-x-2">
          <Icon icon="ph:game-controller" class="h-8 w-8 text-blue-500" />
        </a>
      </div>

      <!-- Desktop navigation (lg screens and up) -->
      <div class="hidden lg:flex lg:items-center xl:space-x-4">
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

      <!-- Profile/Auth section (lg screens and up) -->
      <div class="hidden lg:flex lg:items-center lg:space-x-4">
        {#if isAuthenticated}
          <div class="relative">
            <button
              on:click={toggleProfile}
              class="flex items-center space-x-3 px-3 py-2 rounded-full hover:bg-gray-700 transition-all duration-200 group"
            >
              <span class="text-sm font-medium text-gray-300 group-hover:text-white">
                {$userStore.user?.username}
              </span>
              <img
                src={$userStore.user?.profile_picture}
                alt="Profile"
                class="h-10 w-10 rounded-full object-cover ring-2 ring-gray-700 group-hover:ring-blue-500 transition-all duration-200"
              />
            </button>

            {#if isProfileOpen}
              <div
                class="absolute right-0 mt-2 w-48 rounded-md shadow-lg bg-gray-800 ring-1 ring-black ring-opacity-5 z-50"
              >
                <div class="py-1" role="menu">
                  <button
                    type="button"
                    on:click={async () => {
                      closeProfile();
                      await goto("/profile");
                    }}
                    class="flex items-center w-full px-4 py-2 cursor-pointer text-sm text-gray-300 hover:bg-gray-700 hover:text-white transition-colors duration-200"
                    role="menuitem"
                  >
                    <Icon icon="ph:user-circle" class="h-5 w-5 mr-2" />
                    Profile
                  </button>
                  <button
                    type="button"
                    on:click={async () => {
                      closeProfile();
                      await handleLogout();
                    }}
                    class="flex items-center w-full px-4 cursor-pointer py-2 text-sm text-gray-300 hover:bg-gray-700 hover:text-white transition-colors duration-200"
                    role="menuitem"
                  >
                    <Icon icon="ph:sign-out" class="h-5 w-5 mr-2" />
                    Logout
                  </button>
                </div>
              </div>

              <!-- Backdrop for profile dropdown -->
              <div
                class="fixed inset-0 z-40"
                on:click={closeProfile}
                on:keydown={(e) => e.key === "Escape" && closeProfile()}
                role="button"
                tabindex="0"
              ></div>
            {/if}
          </div>
        {:else}
          <div class="flex items-center space-x-3">
            <a
              href="/login"
              class="px-4 py-2 text-sm font-medium text-gray-300 hover:text-white transition-colors duration-200"
            >
              Login
            </a>
            <a
              href="/signup"
              class="px-4 py-2 whitespace-nowrap text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 transition-colors duration-200"
            >
              Sign Up
            </a>
          </div>
        {/if}
      </div>

      <!-- Menu button (below lg screens) -->
      <div class="flex items-center lg:hidden space-x-3">
        {#if !isAuthenticated}
          <a
            href="/login"
            class="inline-flex items-center justify-center px-4 py-2 rounded-md text-white bg-blue-600 hover:bg-blue-700 transition-colors duration-200"
          >
            <Icon icon="ph:sign-in" class="h-5 w-5 mr-1.5" />
            <span>Login</span>
          </a>
        {/if}
        <!-- Mobile Feedback Link -->
        <a
          href="/feedback"
          class="inline-flex items-center justify-center p-2 rounded-md text-gray-400 hover:text-white hover:bg-gray-700 focus:outline-none"
        >
          <Icon icon="ph:megaphone-simple" class="h-6 w-6" />
          <span class="sr-only">Feedback</span>
        </a>

        <!-- Menu button -->
        <button
          type="button"
          class="inline-flex items-center justify-center p-2 rounded-md text-gray-400 hover:text-white hover:bg-gray-700 focus:outline-none"
          on:click={toggleMenu}
          aria-expanded={isOpen}
        >
          <span class="sr-only">{isOpen ? "Close menu" : "Open menu"}</span>
          <Icon icon={isOpen ? "ph:x" : "ph:list"} class="h-6 w-6" />
        </button>

        {#if isAuthenticated}
          <button
            on:click={toggleProfile}
            class="inline-flex items-center justify-center rounded-full hover:bg-gray-700 transition-all duration-200 group"
          >
            <img
              src={$userStore.user?.profile_picture}
              alt="Profile"
              class="h-10 w-10 rounded-full object-cover ring-2 ring-gray-700 group-hover:ring-blue-500 transition-all duration-200"
            />
          </button>
        {/if}
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

      <!-- Mobile profile section -->
      {#if isAuthenticated}
        <a
          href="/profile"
          on:click|preventDefault={() => {
            closeMenu();
            goto("/profile");
          }}
          class="flex items-center px-3 py-3 rounded-md text-base font-medium text-gray-300 hover:bg-gray-700 hover:text-white transition-colors duration-200"
        >
          <Icon icon="ph:user-circle" class="h-6 w-6 mr-2" />
          <span>Profile</span>
        </a>
        <button
          on:click={() => {
            closeMenu();
            handleLogout();
          }}
          class="flex items-center w-full px-3 py-3 rounded-md text-base font-medium text-gray-300 hover:bg-gray-700 hover:text-white transition-colors duration-200"
        >
          <Icon icon="ph:sign-out" class="h-6 w-6 mr-2" />
          <span>Logout</span>
        </button>
      {:else}
        <!-- Auth buttons for mobile -->
        <div class="mt-4 p-4 bg-gray-800/50 rounded-lg">
          <a
            href="/login"
            class="flex items-center justify-center w-full px-4 py-3 mb-3 text-white bg-blue-600 rounded-lg hover:bg-blue-700 transition-colors duration-200"
            on:click={closeMenu}
          >
            <Icon icon="ph:sign-in" class="h-5 w-5 mr-2" />
            Login
          </a>
          <a
            href="/signup"
            class="flex items-center justify-center w-full px-4 py-3 text-gray-300 bg-gray-700 rounded-lg hover:bg-gray-600 hover:text-white transition-colors duration-200"
            on:click={closeMenu}
          >
            <Icon icon="ph:user-plus" class="h-5 w-5 mr-2" />
            Sign Up
          </a>
        </div>
      {/if}
    </div>
  </div>
</nav>

<!-- Spacer to prevent content from hiding under fixed navbar -->
<div class="h-16"></div>

<!-- Backdrop for mobile menu -->
{#if isOpen}
  <div
    class="lg:hidden fixed inset-0 bg-black bg-opacity-50 z-40"
    on:click={closeMenu}
    on:keydown={(e) => e.key === "Escape" && closeMenu()}
    role="button"
    tabindex="0"
  ></div>
{/if}
