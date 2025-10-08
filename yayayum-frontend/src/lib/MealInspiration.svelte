<script lang="ts">
    import { onMount } from "svelte";
    import { DishesService, type Dish } from "./api";

    let dishes: Dish[] = [];
    let loading = true;
    let error: string | null = null;

    onMount(async () => {
        try {
            dishes = await DishesService.getDishes();
        } catch (err) {
            error = (err as Error).message;
        } finally {
            loading = false;
        }
    });
</script>

{#if loading}
    <p>Loading meals...</p>
{:else if error}
    <p style="color: red;">Error: {error}</p>
{:else if dishes.length === 0}
    <p>No meals found.</p>
{:else}
    <div style="display: flex; flex-wrap: wrap; gap: 8px;">
        {#each dishes as dish}
            <p>{dish.description}</p>
        {/each}
    </div>
{/if}
