<script lang="ts">
    import { onMount } from "svelte";

    let meals: string[] = [];
    let loading = true;
    let error: string | null = null;

    onMount(async () => {
        try {

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
{:else if meals.length === 0}
    <p>No meals found.</p>
{:else}
    <div style="display: flex; flex-wrap: wrap; gap: 8px;">
        {#each meals as meal}
            <p>{meal}</p>
        {/each}
    </div>
{/if}
