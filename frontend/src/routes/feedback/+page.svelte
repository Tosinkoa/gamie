<script lang="ts">
  import Icon from "@iconify/svelte";

  type FeedbackType = "bug" | "feature";

  let feedbackData = {
    type: "bug" as FeedbackType,
    title: "",
    description: "",
    email: "", // Optional: for follow-up
    priority: "medium", // Optional: low, medium, high
    screenshots: [] as File[], // Optional: for bug reports (now used for attachments)
  };

  const handleSubmit = async () => {
    // TODO: Implement feedback submission logic
    console.log("Feedback submitted:", feedbackData);
  };
</script>

<div class="min-h-screen bg-gradient-to-br from-gray-800 to-black p-4">
  <div class="max-w-2xl mx-auto bg-gray-800 rounded-xl p-8">
    <h2 class="text-3xl font-bold text-white mb-6">Send Feedback</h2>

    <form on:submit|preventDefault={handleSubmit} class="space-y-6">
      <!-- Email -->
      <div>
        <label class="block text-gray-300 mb-2">Email</label>
        <input
          type="email"
          bind:value={feedbackData.email}
          class="w-full bg-gray-700 text-white rounded-lg p-3"
          placeholder="For follow-up communications"
        />
      </div>

      <!-- Feedback Type -->
      <div>
        <label class="block text-gray-300 mb-2">Feedback Type</label>
        <select
          bind:value={feedbackData.type}
          class="w-full bg-gray-700 text-white rounded-lg p-3"
        >
          <option value="bug">Report Bug/Issue</option>
          <option value="feature">Request New Feature</option>
        </select>
      </div>

      <!-- Title -->
      <div>
        <label class="block text-gray-300 mb-2">Title</label>
        <input
          type="text"
          bind:value={feedbackData.title}
          class="w-full bg-gray-700 text-white rounded-lg p-3"
          placeholder="Brief summary of your feedback"
          required
        />
      </div>

      <!-- Description -->
      <div>
        <label class="block text-gray-300 mb-2">Description</label>
        <textarea
          bind:value={feedbackData.description}
          class="w-full bg-gray-700 text-white rounded-lg p-3 min-h-[150px]"
          placeholder="Please provide detailed information..."
          required
        ></textarea>
      </div>

      <!-- Attachment (for bugs) -->
      {#if feedbackData.type === "bug"}
        <div>
          <label for="attachment" class="block text-gray-300 mb-2">Attachment (optional)</label
          >
          <input
            id="attachment"
            type="file"
            multiple
            on:change={(e) => {
              const target = e.target as HTMLInputElement;
              feedbackData.screenshots = Array.from(target.files || []);
            }}
            class="block w-full text-sm text-gray-500
                   file:mr-4 file:py-2 file:px-4 file:rounded-lg
                   file:border-0 file:text-sm file:font-semibold
                   file:bg-blue-600 file:text-white hover:file:bg-blue-700"
          />
        </div>
      {/if}

      <!-- Submit Button -->
      <button
        type="submit"
        class="w-full bg-blue-600 hover:bg-blue-700 text-white py-3 rounded-lg transition-colors duration-300"
      >
        Submit Feedback
      </button>
    </form>
  </div>
</div>
