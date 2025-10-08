<script lang="ts">
    import { onMount } from "svelte";
    import { DishesService, type Dish } from "./api";
    import DishListItem from "./DishListItem.svelte";

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
    <div class="dish-list">
        {#if dishes.length > 0}
            {#each dishes as dish (dish.id)}
                <DishListItem {dish} />
            {/each}
        {:else}
            <p>Inga rätter tillgängliga just nu 🍶</p>
        {/if}
    </div>
{/if}

<style>
    .dish-list {
        max-width: 600px;
        margin: 0 auto;
        padding: 2rem 1rem;
    }
</style>
