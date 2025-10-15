<script lang="ts">
    import { DishesService } from "../api/services/DishesService";
    import type { Dish, CreateDish } from "../api";
    import { createEventDispatcher } from 'svelte';
    import DishForm from './DishForm.svelte';

    export let dishes: Dish[] = [];
    
    const dispatch = createEventDispatcher();
    
    let editingDishId: number | null = null;
    let error: string | null = null;

    function startEdit(dish: Dish) {
        editingDishId = dish.id;
        error = null;
    }

    function cancelEdit() {
        editingDishId = null;
        error = null;
    }

    async function handleCreate(event: CustomEvent<CreateDish>) {
        try {
            const newDish = await DishesService.createDish(event.detail);
            dispatch('dishAdded', newDish);
            error = null;
        } catch (err) {
            error = (err as Error).message;
        }
    }

    async function handleSave(event: CustomEvent<{ id: number | undefined, data: CreateDish }>) {
        const { id, data } = event.detail;
        if (!id) return;
        
        try {
            const updatedDish = await DishesService.modifyDish(id, data);
            dispatch('dishUpdated', updatedDish);
            editingDishId = null;
            error = null;
        } catch (err) {
            error = (err as Error).message;
        }
    }

    async function confirmAndDelete(dishId: number) {
        const dish = dishes.find(d => d.id === dishId);
        const dishName = dish ? dish.name : `Dish ${dishId}`;
        
        if (confirm(`Are you sure you want to delete "${dishName}"?`)) {
            try {
                await DishesService.removeDish(dishId);
                dispatch('dishDeleted', dishId);
                error = null;
            } catch (err) {
                error = (err as Error).message;
            }
        }
    }

    $: editingDish = editingDishId ? dishes.find(d => d.id === editingDishId) : null;
</script>

<div class="dish-management">
    <div class="section-header">
        <h4>Add New Dish</h4>
    </div>
    
    <!-- Add new dish form -->
    <DishForm 
        on:create={handleCreate}
    />

    {#if error}
        <p class="error">{error}</p>
    {/if}

    <div class="section-header">
        <h4>Dish List</h4>
    </div>
    <div class="dish-list">
        {#each dishes as dish}
            {#if editingDishId === dish.id}
                <!-- Edit form -->
                <DishForm 
                    {dish}
                    isEditing={true}
                    on:save={handleSave}
                    on:cancel={cancelEdit}
                />
            {:else}
                <!-- Display dish -->
                <div class="dish-item">
                    <div class="dish-info">
                        <div class="dish-header">
                            <h4 class="dish-name">#{dish.nr} {dish.name}</h4>
                            <span class="dish-price">{dish.price_kr} kr</span>
                        </div>
                        <p class="dish-description">{dish.description}</p>
                        <div class="dish-meta">
                            <span class="dish-category">Category: {dish.category}</span>
                            {#if dish.dietary_restrictions.length > 0}
                                <span class="dish-restrictions">
                                    Dietary: {dish.dietary_restrictions.join(', ')}
                                </span>
                            {/if}
                        </div>
                    </div>
                    <div class="dish-actions">
                        <button
                            class="edit-button"
                            on:click={() => startEdit(dish)}
                        >
                            Edit
                        </button>
                        <button
                            class="delete-button"
                            on:click={() => confirmAndDelete(dish.id)}
                        >
                            Delete
                        </button>
                    </div>
                </div>
            {/if}
        {/each}
        
        {#if dishes.length === 0}
            <p class="no-items">No dishes found. Add your first dish above!</p>
        {/if}
    </div>
</div>

<style>
    .dish-management {
        margin: 0;
    }

    .section-header {
        margin-bottom: 16px;
        padding-bottom: 8px;
        border-bottom: 2px solid #e2e8f0;
    }

    .section-header h4 {
        margin: 0;
        color: #4a5568;
        font-size: 1.1em;
        font-weight: 600;
    }

    .dish-list {
        display: flex;
        flex-direction: column;
        gap: 16px;
    }

    .dish-item {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        gap: 16px;
        padding: 16px;
        border: 1px solid #ddd;
        border-radius: 8px;
        background-color: white;
        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    }

    .dish-info {
        flex: 1;
    }

    .dish-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 8px;
    }

    .dish-name {
        margin: 0;
        font-size: 1.1em;
        color: #333;
    }

    .dish-price {
        font-weight: bold;
        color: #007acc;
        font-size: 1.1em;
    }

    .dish-description {
        margin: 0 0 8px 0;
        color: #666;
        line-height: 1.4;
    }

    .dish-meta {
        display: flex;
        flex-direction: column;
        gap: 4px;
    }

    .dish-category,
    .dish-restrictions {
        font-size: 0.85em;
        color: #888;
    }

    .dish-actions {
        display: flex;
        flex-direction: column;
        gap: 8px;
        flex-shrink: 0;
    }

    .edit-button {
        padding: 8px 16px;
        background-color: #007acc;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        font-size: 0.9em;
        white-space: nowrap;
    }

    .edit-button:hover {
        background-color: #005a9e;
    }

    .delete-button {
        padding: 8px 16px;
        background-color: #dc3545;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        font-size: 0.9em;
        white-space: nowrap;
    }

    .delete-button:hover {
        background-color: #c82333;
    }

    .error {
        color: #dc3545;
        font-size: 0.9em;
        padding: 12px;
        background-color: #f8d7da;
        border: 1px solid #f5c6cb;
        border-radius: 4px;
        margin-bottom: 16px;
    }

    .no-items {
        color: #666;
        font-style: italic;
        text-align: center;
        padding: 32px;
        background-color: #f8f9fa;
        border-radius: 8px;
    }
</style>