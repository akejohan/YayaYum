<script lang="ts">
    import { DietaryRestriction } from "../api";
    import { createEventDispatcher } from 'svelte';

    export let selectedRestrictions: DietaryRestriction[] = [];
    export let title: string = "Dietary Restrictions:";
    
    const dispatch = createEventDispatcher();

    function toggleRestriction(restriction: DietaryRestriction) {
        let newRestrictions: DietaryRestriction[];
        
        if (selectedRestrictions.includes(restriction)) {
            newRestrictions = selectedRestrictions.filter(r => r !== restriction);
        } else {
            newRestrictions = [...selectedRestrictions, restriction];
        }
        
        dispatch('change', newRestrictions);
    }
</script>

<div class="dietary-restrictions">
    <p class="title">{title}</p>
    <div class="restrictions-grid">
        {#each Object.values(DietaryRestriction) as restriction}
            <label class="checkbox-label">
                <input 
                    type="checkbox" 
                    checked={selectedRestrictions.includes(restriction)}
                    on:change={() => toggleRestriction(restriction)}
                />
                <span class="restriction-name">{restriction}</span>
            </label>
        {/each}
    </div>
</div>

<style>
    .dietary-restrictions {
        border: 1px solid #ccc;
        border-radius: 4px;
        padding: 12px;
    }

    .title {
        margin: 0 0 8px 0;
        font-weight: bold;
        font-size: 0.9em;
    }

    .restrictions-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
        gap: 6px;
    }

    .checkbox-label {
        display: flex;
        align-items: center;
        gap: 8px;
        cursor: pointer;
        padding: 4px;
        border-radius: 4px;
        transition: background-color 0.2s;
    }

    .checkbox-label:hover {
        background-color: #f8f9fa;
    }

    .restriction-name {
        font-size: 0.9em;
        user-select: none;
    }

    input[type="checkbox"] {
        margin: 0;
    }
</style>