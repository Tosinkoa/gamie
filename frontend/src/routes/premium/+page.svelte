<script lang="ts">
  import Icon from "@iconify/svelte";
  import { onMount } from "svelte";

  interface PremiumFeature {
    icon: string;
    title: string;
    description: string;
  }

  interface PricingTier {
    name: string;
    price: number;
    period: string;
    description: string;
    features: string[];
    highlighted?: boolean;
    icon: string;
  }

  const premiumFeatures: PremiumFeature[] = [
    {
      icon: "ph:crown-simple",
      title: "Exclusive Games",
      description: "Access to premium-only game modes and special tournaments",
    },
    {
      icon: "ph:trophy",
      title: "Advanced Statistics",
      description: "Detailed performance analytics and gameplay insights",
    },
    {
      icon: "ph:paint-brush",
      title: "Custom Themes",
      description: "Personalize your gaming experience with unique themes",
    },
    {
      icon: "ph:medal",
      title: "Priority Matchmaking",
      description: "Find matches faster with priority queue access",
    },
    {
      icon: "ph:chat-circle",
      title: "Premium Support",
      description: "24/7 dedicated support for premium members",
    },
    {
      icon: "ph:gift",
      title: "Monthly Rewards",
      description: "Exclusive rewards and bonuses every month",
    },
  ];

  const pricingTiers: PricingTier[] = [
    {
      name: "Basic",
      price: 0,
      period: "forever",
      description: "Perfect for casual players",
      icon: "ph:game-controller",
      features: [
        "Access to basic games",
        "Standard matchmaking",
        "Basic statistics",
        "Community support",
      ],
    },
    {
      name: "Premium",
      price: 9.99,
      period: "month",
      description: "Most popular for enthusiasts",
      icon: "ph:crown-simple",
      highlighted: true,
      features: [
        "All Basic features",
        "Exclusive game modes",
        "Priority matchmaking",
        "Advanced statistics",
        "Custom themes",
        "Premium support",
      ],
    },
    {
      name: "Pro",
      price: 19.99,
      period: "month",
      description: "For serious competitors",
      icon: "ph:trophy",
      features: [
        "All Premium features",
        "Tournament priority",
        "Personal coach access",
        "Strategy analysis tools",
        "Team management tools",
        "Custom tournaments",
        "API access",
      ],
    },
  ];

  let selectedPeriod: "month" | "year" = "month";
  let mounted = false;

  onMount(() => {
    mounted = true;
  });

  $: getAnnualPrice = (monthlyPrice: number) => {
    return selectedPeriod === "year" ? monthlyPrice * 10 : monthlyPrice;
  };
</script>

<div class="min-h-screen bg-gradient-to-br from-gray-800 to-black">
  <!-- Hero Section -->
  <div class="relative overflow-hidden">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pt-20 pb-16 text-center">
      <h1
        class="text-4xl sm:text-5xl lg:text-6xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-blue-400 to-purple-500 mb-6"
      >
        Elevate Your Gaming Experience
      </h1>
      <p class="text-xl text-gray-300 max-w-2xl mx-auto mb-8">
        Unlock exclusive features, join premium tournaments, and take your gaming to the next
        level
      </p>
      <div class="flex justify-center space-x-4 mb-12">
        <button
          class="px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors duration-300 flex items-center"
        >
          <Icon icon="ph:crown-simple" class="w-5 h-5 mr-2" />
          Get Premium Now
        </button>
        <button
          class="px-6 py-3 bg-gray-700 hover:bg-gray-600 text-white rounded-lg transition-colors duration-300 flex items-center"
        >
          <Icon icon="ph:info" class="w-5 h-5 mr-2" />
          Learn More
        </button>
      </div>
    </div>
  </div>

  <!-- Features Grid -->
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
    <h2 class="text-3xl font-bold text-white text-center mb-12">Premium Features</h2>
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
      {#each premiumFeatures as feature, index}
        <div
          class="bg-gray-800 rounded-xl p-6 transform hover:scale-105 transition-all duration-300 {mounted
            ? 'animate-fade-in'
            : ''}"
          style="animation-delay: {index * 100}ms"
        >
          <div class="flex items-center mb-4">
            <div class="w-12 h-12 bg-blue-500/20 rounded-lg flex items-center justify-center">
              <Icon icon={feature.icon} class="w-6 h-6 text-blue-400" />
            </div>
            <h3 class="text-xl font-semibold text-white ml-4">{feature.title}</h3>
          </div>
          <p class="text-gray-400">{feature.description}</p>
        </div>
      {/each}
    </div>
  </div>

  <!-- Pricing Section -->
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
    <h2 class="text-3xl font-bold text-white text-center mb-6">Choose Your Plan</h2>
    <p class="text-gray-400 text-center mb-12 max-w-2xl mx-auto">
      Select the perfect plan for your gaming journey. Upgrade or downgrade anytime.
    </p>

    <!-- Billing Toggle -->
    <div class="flex justify-center items-center space-x-4 mb-12">
      <button
        class="px-4 py-2 rounded-lg {selectedPeriod === 'month'
          ? 'bg-blue-600 text-white'
          : 'bg-gray-800 text-gray-400'}"
        on:click={() => (selectedPeriod = "month")}
      >
        Monthly
      </button>
      <button
        class="px-4 py-2 rounded-lg {selectedPeriod === 'year'
          ? 'bg-blue-600 text-white'
          : 'bg-gray-800 text-gray-400'}"
        on:click={() => (selectedPeriod = "year")}
      >
        Yearly
        <span class="text-xs text-green-400 ml-1">Save 20%</span>
      </button>
    </div>

    <!-- Pricing Cards -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
      {#each pricingTiers as tier, index}
        <div
          class="relative {tier.highlighted ? 'transform -translate-y-4' : ''} {mounted
            ? 'animate-fade-in'
            : ''}"
          style="animation-delay: {index * 100}ms"
        >
          {#if tier.highlighted}
            <div class="absolute -top-4 left-0 right-0 flex justify-center">
              <span class="px-3 py-1 bg-blue-600 text-white text-sm rounded-full">
                Most Popular
              </span>
            </div>
          {/if}
          <div
            class="h-full bg-gray-800 rounded-xl p-8 flex flex-col {tier.highlighted
              ? 'ring-2 ring-blue-500'
              : ''}"
          >
            <div class="flex items-center justify-between mb-4">
              <h3 class="text-2xl font-bold text-white">{tier.name}</h3>
              <Icon icon={tier.icon} class="w-8 h-8 text-blue-400" />
            </div>
            <div class="mb-4">
              <span class="text-4xl font-bold text-white">${getAnnualPrice(tier.price)}</span>
              <span class="text-gray-400">/{tier.period}</span>
            </div>
            <p class="text-gray-400 mb-6">{tier.description}</p>
            <ul class="space-y-3 mb-8 flex-grow">
              {#each tier.features as feature}
                <li class="flex items-center text-gray-300">
                  <Icon icon="ph:check-circle" class="w-5 h-5 text-green-400 mr-2" />
                  {feature}
                </li>
              {/each}
            </ul>
            <button
              class="w-full py-3 rounded-lg {tier.highlighted
                ? 'bg-blue-600 hover:bg-blue-700'
                : 'bg-gray-700 hover:bg-gray-600'} text-white transition-colors duration-300"
            >
              {tier.price === 0 ? "Get Started" : "Subscribe Now"}
            </button>
          </div>
        </div>
      {/each}
    </div>
  </div>

  <!-- FAQ Section -->
  <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
    <h2 class="text-3xl font-bold text-white text-center mb-12">Frequently Asked Questions</h2>
    <div class="space-y-6">
      <div class="bg-gray-800 rounded-xl p-6">
        <h3 class="text-xl font-semibold text-white mb-2">Can I cancel anytime?</h3>
        <p class="text-gray-400">
          Yes, you can cancel your subscription at any time. No long-term commitments required.
        </p>
      </div>
      <div class="bg-gray-800 rounded-xl p-6">
        <h3 class="text-xl font-semibold text-white mb-2">
          What payment methods do you accept?
        </h3>
        <p class="text-gray-400">
          We accept all major credit cards, PayPal, and various local payment methods.
        </p>
      </div>
      <div class="bg-gray-800 rounded-xl p-6">
        <h3 class="text-xl font-semibold text-white mb-2">Can I switch plans later?</h3>
        <p class="text-gray-400">
          Absolutely! You can upgrade or downgrade your plan at any time from your account
          settings.
        </p>
      </div>
    </div>
  </div>
</div>

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
