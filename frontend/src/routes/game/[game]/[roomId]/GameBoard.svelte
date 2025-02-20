<!-- GameBoard.svelte -->
<script lang="ts">
  import { spring } from "svelte/motion";
  import { flip } from "svelte/animate";
  import { fade, fly, scale } from "svelte/transition";
  import Icon from "@iconify/svelte";

  export let availableNumbers: number[] = [];
  export let isCurrentTurn: boolean = false;
  export let timeRemaining: number = 60;
  export let players: Array<{
    id: string;
    username: string;
    avatar: string;
    score: number;
  }> = [];

  // State for drag and drop
  let draggedNumber: number | null = null;
  let draggedIndex: number | null = null;
  let hoveredBox: number | null = null;
  let focusedBox: number | null = null;
  let isDragging = false;

  // Initialize empty boxes
  let boxes: (number | null)[] = Array(10).fill(null);

  function handleDragStart(event: DragEvent, number: number, index: number) {
    if (!isCurrentTurn) return;
    draggedNumber = number;
    draggedIndex = index;
    isDragging = true;
  }

  function handleDragOver(event: DragEvent, boxIndex: number) {
    event.preventDefault();
    hoveredBox = boxIndex;
  }

  function handleDrop(event: DragEvent, boxIndex: number) {
    event.preventDefault();
    if (!isCurrentTurn || draggedNumber === null) return;

    placeNumber(draggedNumber, boxIndex);
    isDragging = false;
  }

  function handleDragEnd() {
    hoveredBox = null;
    draggedNumber = null;
    draggedIndex = null;
    isDragging = false;
  }

  function handleKeyDown(event: KeyboardEvent, number: number, index: number) {
    if (!isCurrentTurn) return;

    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      if (draggedNumber === null) {
        // Start drag
        draggedNumber = number;
        draggedIndex = index;
      } else if (focusedBox !== null) {
        // Complete drag
        placeNumber(draggedNumber, focusedBox);
      }
    }

    if (event.key === "Escape" && draggedNumber !== null) {
      draggedNumber = null;
      draggedIndex = null;
      focusedBox = null;
    }
  }

  function placeNumber(number: number, boxIndex: number) {
    boxes[boxIndex] = number;
    availableNumbers = availableNumbers.filter((_, i) => i !== draggedIndex);

    hoveredBox = null;
    draggedNumber = null;
    draggedIndex = null;
    focusedBox = null;
  }

  function getBoxStyles(box: number | null, index: number) {
    const baseStyles = "h-16 flex items-center justify-center rounded-xl border-2";
    const hoverStyles =
      hoveredBox === index && isCurrentTurn
        ? "border-blue-500 bg-blue-500/20"
        : "border-gray-700";
    const focusStyles = focusedBox === index ? "ring-2 ring-blue-500" : "";
    const disabledStyles = !isCurrentTurn ? "opacity-50" : "";

    return `${baseStyles} ${hoverStyles} ${focusStyles} ${disabledStyles}`;
  }

  // Progress bar spring animation
  const progress = spring(100);
  $: progress.set((timeRemaining / 60) * 100);
</script>

<div
  class="game-board flex flex-col h-full gap-6 p-4"
  role="application"
  aria-label="Number arrangement game"
>
  <!-- Timer and Turn Indicator -->
  <div class="flex flex-col gap-2">
    <!-- Player Scores -->
    <div class="flex items-center justify-center gap-4" role="list" aria-label="Player scores">
      {#each players as player}
        <div class="flex items-center gap-2 bg-gray-800/50 rounded-lg p-2" role="listitem">
          <img src={player.avatar} alt="" class="w-8 h-8 rounded-full" />
          <div class="flex flex-col">
            <span class="text-sm text-white">{player.username}</span>
            <span class="text-xs text-green-400">{player.score} pts</span>
          </div>
        </div>
      {/each}
    </div>

    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <div
          class="text-lg font-bold {isCurrentTurn ? 'text-green-400' : 'text-gray-400'}"
          role="status"
        >
          {isCurrentTurn ? "Your Turn!" : "Waiting..."}
        </div>
      </div>
      <div class="flex items-center gap-2" role="timer" aria-label="Time remaining">
        <Icon icon="ph:clock" class="w-5 h-5 text-yellow-500" aria-hidden="true" />
        <div class="text-lg font-bold text-yellow-500">{timeRemaining}s</div>
      </div>
    </div>
  </div>

  <!-- Progress Bar -->
  <div
    class="h-2 bg-gray-700 rounded-full overflow-hidden mb-4"
    role="progressbar"
    aria-valuenow={timeRemaining}
    aria-valuemin="0"
    aria-valuemax="60"
  >
    <div
      class="h-full bg-yellow-500 transition-all duration-300 rounded-full"
      style="width: {$progress}%"
    ></div>
  </div>

  <!-- Number Arrangement Boxes -->
  <div class="flex-1 flex flex-col">
    <div
      class="max-w-3xl w-full mx-auto grid grid-cols-5 gap-3 mb-4"
      role="grid"
      aria-label="Number arrangement grid"
    >
      {#each boxes as box, i}
        <div
          role="gridcell"
          tabindex={isCurrentTurn ? 0 : -1}
          class="h-16 md:h-20 flex items-center justify-center rounded-xl border-2
                 {hoveredBox === i && isCurrentTurn
            ? 'border-blue-500 bg-blue-500/20'
            : 'border-gray-700'}
                 {focusedBox === i ? 'ring-2 ring-blue-500' : ''}
                 {!isCurrentTurn ? 'opacity-60' : ''}"
          on:dragover={(e) => handleDragOver(e, i)}
          on:drop={(e) => handleDrop(e, i)}
          on:focus={() => (focusedBox = i)}
          on:blur={() => (focusedBox = null)}
          aria-label={box ? `Box ${i + 1} contains ${box}` : `Empty box ${i + 1}`}
        >
          {#if box !== null}
            <div
              class="text-2xl md:text-3xl font-bold text-white bg-green-600 w-full h-full rounded-lg
                     flex items-center justify-center shadow-lg"
              in:fly={{ y: -20, duration: 200 }}
              out:scale={{ duration: 200 }}
              role="presentation"
            >
              {box}
            </div>
          {/if}
        </div>
      {/each}
    </div>

    <!-- Available Numbers -->
    <div
      class="max-w-4xl w-full mx-auto grid grid-cols-6 gap-2 md:gap-4 mb-4"
      role="list"
      aria-label="Available numbers"
    >
      {#each availableNumbers as number, i (number)}
        <div
          role="button"
          tabindex="0"
          class="h-[3.1rem] md:h-16 w-full bg-blue-600 rounded-md flex items-center justify-center
                 cursor-move hover:bg-blue-700 transition-all duration-200
                 {isDragging && draggedNumber === number ? 'opacity-50' : ''}
                 {!isCurrentTurn ? 'opacity-60 cursor-not-allowed' : ''}"
          draggable={true}
          on:dragstart={(e) => handleDragStart(e, number, i)}
          on:dragend={handleDragEnd}
          on:keydown={(e) => handleKeyDown(e, number, i)}
          aria-label={`Number ${number}${draggedNumber === number ? " (being dragged)" : ""}`}
        >
          <span class="text-base md:text-xl font-bold text-white">{number}</span>
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .game-board {
    min-height: 400px;
  }

  /* Prevent drag ghost image */
  [draggable] {
    -webkit-user-drag: element;
    user-select: none;
    cursor: move;
    touch-action: none;
  }

  /* Smooth transitions for drag and drop */
  button {
    transform-origin: center;
    will-change: transform, opacity;
  }

  /* Hover effect for drop targets */
  [role="gridcell"]:not(:has(div)):hover {
    background-color: rgba(59, 130, 246, 0.1);
    transition: background-color 0.2s ease;
  }
</style>
