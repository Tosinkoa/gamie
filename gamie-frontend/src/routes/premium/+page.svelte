<script lang="ts">
  import Icon from "@iconify/svelte";
  import { onMount } from "svelte";

  interface PricingTier {
    name: string;
    price: number;
    period: string;
    description: string;
    features: string[];
    highlighted?: boolean;
    icon: string;
  }

  const pricingTiers: PricingTier[] = [
    {
      name: "Basic",
      price: 0,
      period: "forever",
      description: "Get started with essential features",
      icon: "ph:game-controller",
      features: [
        "Play unlimited games",
        "Join public tournaments",
        "Basic player stats",
        "Global leaderboard access",
      ],
    },
    {
      name: "Premium",
      price: 9.99,
      period: "month",
      description: "Enhanced gaming experience",
      icon: "ph:crown-simple",
      highlighted: true,
      features: [
        "All Basic features",
        "Ad-free experience",
        "Exclusive tournaments",
        "Detailed match analytics",
        "Custom profile themes",
        "24/7 priority support",
      ],
    },
    {
      name: "Pro",
      price: 14.99,
      period: "month",
      description: "For competitive players",
      icon: "ph:trophy",
      features: [
        "All Premium features",
        "Early access to new features",
        "Professional coaching sessions",
        "Advanced performance analytics",
        "Private tournament hosting",
        "Team management tools",
        "Developer API access",
      ],
    },
  ];

  let selectedPeriod: "month" | "year" = "month";
  let mounted = false;

  onMount(() => {
    mounted = true;
  });

  $: getAnnualPrice = (monthlyPrice: number) => {
    if (selectedPeriod === "year") {
      return (monthlyPrice * 0.8).toFixed(2);
    }
    return monthlyPrice.toFixed(2);
  };
</script>

<div class="min-h-screen bg-gradient-to-br from-gray-800 to-black py-8 sm:py-12">
  <!-- Pricing Section -->
  <div class="container max-w-6xl mx-auto px-4">
    <!-- Header Section -->
    <div class="text-center max-w-2xl mx-auto mb-8 sm:mb-12">
      <h2 class="text-2xl sm:text-3xl font-bold text-white mb-3">Choose Your Plan</h2>
      <p class="text-gray-400 text-sm sm:text-base">
        Select the perfect plan for your gaming journey. Upgrade or downgrade anytime.
      </p>
    </div>

    <!-- Billing Toggle -->
    <div class="flex justify-center items-center space-x-4 mb-8">
      <div class="inline-flex bg-gray-900 p-1 rounded-lg">
        <button
          class="px-4 py-2 text-sm rounded-md transition-colors duration-200 {selectedPeriod ===
          'month'
            ? 'bg-blue-600 text-white'
            : 'text-gray-400 hover:text-white'}"
          on:click={() => (selectedPeriod = "month")}
        >
          Monthly
        </button>
        <button
          class="px-4 py-2 text-sm rounded-md transition-colors duration-200 {selectedPeriod ===
          'year'
            ? 'bg-blue-600 text-white'
            : 'text-gray-400 hover:text-white'}"
          on:click={() => (selectedPeriod = "year")}
        >
          Yearly
          <span class="text-xs text-green-400 ml-1">-20%</span>
        </button>
      </div>
    </div>

    <!-- Pricing Cards -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 lg:gap-6 max-w-5xl mx-auto">
      {#each pricingTiers as tier, index}
        <div
          class="relative {tier.highlighted ? 'transform md:-translate-y-4' : ''} {mounted
            ? 'animate-fade-in'
            : ''}"
          style="animation-delay: {index * 100}ms"
        >
          {#if tier.highlighted}
            <div class="absolute -top-3 left-0 right-0 flex justify-center">
              <span
                class="px-3 py-1 bg-gradient-to-r z-10 from-blue-600 to-blue-500 text-white text-xs font-medium rounded-full shadow-lg"
              >
                Most Popular
              </span>
            </div>
          {/if}
          <div
            class="h-full bg-gray-800/80 backdrop-blur-sm rounded-xl shadow-xl p-6 flex flex-col {tier.highlighted
              ? 'ring-2 ring-blue-500 bg-gray-800/90'
              : ''}"
          >
            <!-- Card Header -->
            <div class="flex items-center justify-between mb-4">
              <div>
                <h3 class="text-lg sm:text-xl font-bold text-white mb-1">{tier.name}</h3>
                <p class="text-gray-400 text-sm">{tier.description}</p>
              </div>
              <div class="bg-gray-700/50 p-2 rounded-lg">
                <Icon icon={tier.icon} class="w-6 h-6 text-blue-400" />
              </div>
            </div>

            <!-- Pricing -->
            <div class="mb-6">
              <div class="flex items-baseline">
                <span class="text-3xl sm:text-4xl font-bold text-white"
                  >${getAnnualPrice(tier.price)}</span
                >
                <span class="text-gray-400 text-sm ml-2">/{tier.period}</span>
              </div>
            </div>

            <!-- Features -->
            <ul class="space-y-3 mb-6 flex-grow">
              {#each tier.features as feature}
                <li class="flex items-start text-gray-300 text-sm">
                  <Icon
                    icon="ph:check-circle"
                    class="w-5 h-5 text-green-400 mr-3 flex-shrink-0 mt-0.5"
                  />
                  <span class="leading-tight">{feature}</span>
                </li>
              {/each}
            </ul>

            <!-- Action Button -->
            <button
              class="w-full py-2.5 px-4 rounded-lg text-sm font-medium transition-all duration-200 {tier.highlighted
                ? 'bg-blue-600 hover:bg-blue-700 shadow-lg shadow-blue-500/25'
                : 'bg-gray-700 hover:bg-gray-600'} text-white"
            >
              {tier.price === 0 ? "Get Started" : "Subscribe Now"}
            </button>
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .animate-fade-in {
    animation: fadeIn 0.6s cubic-bezier(0.16, 1, 0.3, 1) forwards;
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
