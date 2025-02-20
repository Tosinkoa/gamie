## SpecStory

## Reducing Numbers to Twenty (20/02/2025, 20:41:00)

_**User**_

I did like to reduce the total available numbers/shuffled numbers to 20

---

_**Assistant**_

To reduce the total available numbers to 20 in your `GameBoard.svelte` component, you need to ensure that the `availableNumbers` array is initialized or updated to contain only 20 numbers. This can be done by slicing the array or by setting it directly if you have a specific set of numbers in mind.

Here's how you can modify the code:

1. **Initialize `availableNumbers` with 20 numbers**: If you want to start with a specific set of 20 numbers, you can directly assign them to the `availableNumbers` array.

2. **Slice the `availableNumbers` array**: If the numbers are generated or shuffled elsewhere, you can slice the array to ensure it only contains 20 numbers.

Let's proceed with the first option, assuming you want to initialize `availableNumbers` with a specific set of 20 numbers. I will update the `availableNumbers` initialization in the script section of your `GameBoard.svelte` file.

---

_****_

```diff
    import Icon from "@iconify/svelte";
  
-   export let availableNumbers: number[] = [];
+   export let availableNumbers: number[] = Array.from({ length: 20 }, (_, i) => i + 1);
    export let isCurrentTurn: boolean = false;
    export let timeRemaining: number = 60;
```

---

_****_

I have updated the `availableNumbers` array to initialize with 20 numbers, ranging from 1 to 20. If you need to customize these numbers further or have any other requirements, feel free to let me know!

---



---

