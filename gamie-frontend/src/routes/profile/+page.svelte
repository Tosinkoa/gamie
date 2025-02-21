<script lang="ts">
  import Icon from "@iconify/svelte";
  import { onMount } from "svelte";
  import AuthGuard from "$lib/components/AuthGuard.svelte";
  import { userStore } from "$lib/stores/user";

  // Settings form data
  let username = $userStore.user?.username || "";
  let email = $userStore.user?.email || "";
  let oldPassword = "";
  let verificationToken = "";
  let newPassword = "";
  let confirmPassword = "";
  let selectedFile: File | null = null;
  let previewUrl = $userStore.user?.profile_picture || "";
  let passwordStage: "old" | "token" | "new" = "old";
  let showSuccessMessage = false;
  let successMessage = "";
  let mounted = false;

  onMount(() => {
    mounted = true;
  });

  $: currentUser = $userStore.user;

  function handleFileSelect(event: Event) {
    const input = event.target as HTMLInputElement;
    if (input.files && input.files[0]) {
      selectedFile = input.files[0];
      previewUrl = URL.createObjectURL(input.files[0]);
    }
  }

  function handlePasswordSubmit() {
    if (passwordStage === "old") {
      // Simulate password verification
      if (oldPassword === "password123") {
        passwordStage = "token";
        // Simulate sending email
        showSuccessMessage = true;
        successMessage = "Verification code sent to your email!";
        setTimeout(() => (showSuccessMessage = false), 3000);
      }
    } else if (passwordStage === "token") {
      // Simulate token verification
      if (verificationToken === "123456") {
        passwordStage = "new";
      }
    } else if (passwordStage === "new") {
      // Simulate password update
      if (newPassword === confirmPassword) {
        passwordStage = "old";
        oldPassword = "";
        verificationToken = "";
        newPassword = "";
        confirmPassword = "";
        showSuccessMessage = true;
        successMessage = "Password updated successfully!";
        setTimeout(() => (showSuccessMessage = false), 3000);
      }
    }
  }

  function handleEmailUpdate() {
    // Simulate email update
    showSuccessMessage = true;
    successMessage = "Verification email sent to new address!";
    setTimeout(() => (showSuccessMessage = false), 3000);
  }

  function handleProfileUpdate() {
    // Simulate profile update
    showSuccessMessage = true;
    successMessage = "Profile updated successfully!";
    setTimeout(() => (showSuccessMessage = false), 3000);
  }
</script>

<AuthGuard>
  <div class="min-h-screen bg-gradient-to-br from-gray-800 to-black px-4 py-8 sm:px-6 lg:px-8">
    <div class="max-w-7xl mx-auto">
      <!-- Profile Header -->
      <div class="relative bg-gray-900 rounded-2xl p-8 mb-8 overflow-hidden">
        <!-- Background Pattern -->
        <div class="absolute inset-0 opacity-10">
          <div class="absolute inset-0 bg-gradient-to-r from-blue-500 to-purple-500"></div>
        </div>

        <!-- Profile Content -->
        <div class="relative flex flex-col md:flex-row items-center md:items-start gap-8">
          <!-- Avatar Section -->
          <div class="relative">
            <div class="w-32 h-32 rounded-full overflow-hidden ring-4 ring-blue-500 shadow-xl">
              <img
                src={previewUrl}
                alt={currentUser?.username || "Profile"}
                class="w-full h-full object-cover"
              />
            </div>
          </div>

          <!-- User Info -->
          <div class="flex-grow text-center md:text-left">
            <div class="flex flex-col md:flex-row items-center gap-4 mb-4">
              <h1 class="text-3xl font-bold text-white">{currentUser?.username || "User"}</h1>
            </div>
          </div>
        </div>
      </div>

      <!-- Settings Form -->
      <div class="bg-gray-900 rounded-xl p-6 max-w-2xl mx-auto">
        {#if showSuccessMessage}
          <div
            class="bg-green-500/20 text-green-400 p-4 rounded-lg mb-6 animate-fade-in flex items-center gap-2"
          >
            <Icon icon="ph:check-circle" class="w-5 h-5" />
            {successMessage}
          </div>
        {/if}

        <div class="space-y-8">
          <!-- Avatar Upload -->
          <div class="flex flex-col items-center gap-4">
            <div class="relative">
              <img
                src={previewUrl}
                alt="Profile"
                class="w-32 h-32 rounded-full object-cover ring-4 ring-blue-500"
              />
              <label
                class="absolute bottom-0 right-0 w-8 h-8 bg-blue-600 rounded-full flex items-center justify-center cursor-pointer hover:bg-blue-700 transition-colors duration-200"
              >
                <Icon icon="ph:pencil" class="w-4 h-4 text-white" />
                <input
                  type="file"
                  accept="image/*"
                  class="hidden"
                  on:change={handleFileSelect}
                />
              </label>
            </div>
            <span class="text-sm text-gray-400">Click the pencil to change avatar</span>
          </div>

          <!-- Profile Section -->
          <div class="space-y-4">
            <h3 class="text-lg font-semibold text-white">Profile Information</h3>
            <div>
              <label for="username" class="block text-sm font-medium text-gray-400 mb-2"
                >Username</label
              >
              <input
                id="username"
                type="text"
                bind:value={username}
                class="w-full px-4 py-2 bg-gray-800 border border-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-white"
              />
            </div>
            <div class="flex justify-end">
              <button
                on:click={handleProfileUpdate}
                class="px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors duration-300 flex items-center gap-2"
              >
                <Icon icon="ph:check" class="w-5 h-5" />
                Save Changes
              </button>
            </div>
          </div>

          <!-- Email Section -->
          <div class="space-y-4 pt-4 border-t border-gray-800">
            <h3 class="text-lg font-semibold text-white">Email Settings</h3>
            <div>
              <div class="block text-sm font-medium text-gray-400 mb-2">Current Email</div>
              <div
                class="text-gray-300 px-4 py-2 bg-gray-800 rounded-lg border border-gray-700"
              >
                {email}
              </div>
            </div>
            <div>
              <label for="newEmail" class="block text-sm font-medium text-gray-400 mb-2"
                >New Email</label
              >
              <input
                id="newEmail"
                type="email"
                placeholder="Enter new email address"
                class="w-full px-4 py-2 bg-gray-800 border border-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-white"
              />
            </div>
            <div class="flex justify-end">
              <button
                on:click={handleEmailUpdate}
                class="px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors duration-300 flex items-center gap-2"
              >
                <Icon icon="ph:envelope" class="w-5 h-5" />
                Send Verification
              </button>
            </div>
          </div>

          <!-- Password Section -->
          <div class="space-y-4 pt-4 border-t border-gray-800">
            <h3 class="text-lg font-semibold text-white">Change Password</h3>
            {#if passwordStage === "old"}
              <div>
                <label for="oldPassword" class="block text-sm font-medium text-gray-400 mb-2">
                  Current Password
                </label>
                <input
                  id="oldPassword"
                  type="password"
                  bind:value={oldPassword}
                  class="w-full px-4 py-2 bg-gray-800 border border-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-white"
                  placeholder="Enter your current password"
                />
              </div>
            {:else if passwordStage === "token"}
              <div>
                <label
                  for="verificationToken"
                  class="block text-sm font-medium text-gray-400 mb-2"
                >
                  Verification Code
                </label>
                <input
                  id="verificationToken"
                  type="text"
                  bind:value={verificationToken}
                  class="w-full px-4 py-2 bg-gray-800 border border-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-white"
                  placeholder="Enter the code sent to your email"
                />
              </div>
            {:else}
              <div class="space-y-4">
                <div>
                  <label
                    for="newPassword"
                    class="block text-sm font-medium text-gray-400 mb-2"
                  >
                    New Password
                  </label>
                  <input
                    id="newPassword"
                    type="password"
                    bind:value={newPassword}
                    class="w-full px-4 py-2 bg-gray-800 border border-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-white"
                    placeholder="Enter new password"
                  />
                </div>
                <div>
                  <label
                    for="confirmPassword"
                    class="block text-sm font-medium text-gray-400 mb-2"
                  >
                    Confirm New Password
                  </label>
                  <input
                    id="confirmPassword"
                    type="password"
                    bind:value={confirmPassword}
                    class="w-full px-4 py-2 bg-gray-800 border border-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-white"
                    placeholder="Confirm new password"
                  />
                </div>
              </div>
            {/if}
            <div class="flex justify-end">
              <button
                on:click={handlePasswordSubmit}
                class="px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors duration-300 flex items-center gap-2"
              >
                <Icon icon="ph:arrow-right" class="w-5 h-5" />
                {passwordStage === "new" ? "Update Password" : "Continue"}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</AuthGuard>

<style>
  .animate-fade-in {
    animation: fadeIn 0.5s ease-out forwards;
    opacity: 0;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
