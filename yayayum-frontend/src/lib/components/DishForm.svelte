<script lang="ts">
    import { DishCategory, DietaryRestriction } from "../api";
    import type { Dish, CreateDish } from "../api";
    import { createEventDispatcher } from 'svelte';
    import DietaryRestrictionSelector from './DietaryRestrictionSelector.svelte';

    export let dish: Dish | null = null; // If provided, we're editing
    export let isEditing: boolean = false;
    
    const dispatch = createEventDispatcher();
    
    let dishName: string = "";
    let dishDescription: string = "";
    let dishPrice: number = 0;
    let dishNr: number = 0;
    let dishCategory: DishCategory = DishCategory.WOK_WITH_NOODLES;
    let selectedDietaryRestrictions: DietaryRestriction[] = [];
    let error: string | null = null;

    // Initialize form if editing
    $: if (dish && isEditing) {
        dishName = dish.name;
        dishDescription = dish.description;
        dishPrice = dish.price_kr;
        dishNr = dish.nr;
        dishCategory = dish.category;
        selectedDietaryRestrictions = [...dish.dietary_restrictions];
    }

    // Reset form if not editing
    $: if (!isEditing) {
        resetForm();
    }

    function resetForm() {
        dishName = "";
        dishDescription = "";
        dishPrice = 0;
        dishNr = 0;
        dishCategory = DishCategory.WOK_WITH_NOODLES;
        selectedDietaryRestrictions = [];
        error = null;
    }

    function validateForm(): boolean {
        if (!dishName.trim()) {
            error = "Dish name is required";
            return false;
        }
        if (!dishDescription.trim()) {
            error = "Description is required";
            return false;
        }
        if (dishPrice <= 0) {
            error = "Price must be greater than 0";
            return false;
        }
        if (dishNr <= 0) {
            error = "Dish number must be greater than 0";
            return false;
        }
        error = null;
        return true;
    }

    function handleSubmit() {
        if (!validateForm()) {
            return;
        }

        const dishData: CreateDish = {
            name: dishName,
            description: dishDescription,
            price_kr: dishPrice,
            nr: dishNr,
            category: dishCategory,
            dietary_restrictions: selectedDietaryRestrictions
        };

        if (isEditing) {
            dispatch('save', { id: dish?.id, data: dishData });
        } else {
            dispatch('create', dishData);
        }
        
        if (!isEditing) {
            resetForm();
        }
    }

    function handleCancel() {
        resetForm();
        dispatch('cancel');
    }

    function handleDietaryChange(event: CustomEvent<DietaryRestriction[]>) {
        selectedDietaryRestrictions = event.detail;
    }
</script>

<div class="dish-form" class:editing={isEditing}>
    {#if isEditing}
        <h4>Edit Dish: {dish?.name || 'Unknown'}</h4>
    {:else}
        <h3>Add New Dish</h3>
    {/if}
    
    {#if error}
        <p class="error">{error}</p>
    {/if}

    <div class="form-group">
        <label for="dish-name">Dish Name:</label>
        <input 
            id="dish-name"
            type="text" 
            placeholder="Enter dish name" 
            bind:value={dishName} 
        />
    </div>

    <div class="form-group">
        <label for="dish-description">Description:</label>
        <textarea 
            id="dish-description"
            placeholder="Enter dish description" 
            bind:value={dishDescription}
        ></textarea>
    </div>

    <div class="form-group">
        <label for="dish-price">Price (kr):</label>
        <input 
            id="dish-price"
            type="number" 
            placeholder="0.00" 
            bind:value={dishPrice} 
            min="0" 
            step="0.01" 
        />
    </div>

    <div class="form-group">
        <label for="dish-nr">Dish Number:</label>
        <input 
            id="dish-nr"
            type="number" 
            placeholder="Enter dish number" 
            bind:value={dishNr} 
            min="1" 
            step="1" 
        />
    </div>
    
    <div class="form-group">
        <label for="dish-category">Category:</label>
        <select id="dish-category" bind:value={dishCategory}>
            {#each Object.values(DishCategory) as category}
                <option value={category}>{category}</option>
            {/each}
        </select>
    </div>

    <DietaryRestrictionSelector 
        selectedRestrictions={selectedDietaryRestrictions}
        on:change={handleDietaryChange}
    />

    <div class="form-actions">
        <button class="submit-button" on:click={handleSubmit} type="button">
            {isEditing ? 'Save Changes' : 'Add Dish'}
        </button>
        {#if isEditing}
            <button class="cancel-button" on:click={handleCancel} type="button">
                Cancel
            </button>
        {/if}
    </div>
</div>

<style>
    .dish-form {
        display: flex;
        flex-direction: column;
        gap: 16px;
        max-width: 500px;
        margin-bottom: 24px;
        padding: 20px;
        border: 1px solid #ddd;
        border-radius: 8px;
        background-color: white;
    }

    .dish-form.editing {
        border-color: #007acc;
        background-color: #f8f9fa;
    }

    .dish-form h3,
    .dish-form h4 {
        margin: 0 0 8px 0;
        color: #333;
    }

    .dish-form.editing h4 {
        color: #007acc;
    }

    .form-group {
        display: flex;
        flex-direction: column;
        gap: 4px;
    }

    .form-group label {
        font-weight: 500;
        font-size: 0.9em;
        color: #555;
    }

    .form-group input,
    .form-group textarea,
    .form-group select {
        padding: 10px;
        border: 1px solid #ccc;
        border-radius: 4px;
        font-size: 14px;
    }

    .form-group input:focus,
    .form-group textarea:focus,
    .form-group select:focus {
        outline: none;
        border-color: #007acc;
        box-shadow: 0 0 0 2px rgba(0, 122, 204, 0.1);
    }

    .form-group textarea {
        min-height: 80px;
        resize: vertical;
        font-family: inherit;
    }

    .form-actions {
        display: flex;
        gap: 12px;
        margin-top: 8px;
    }

    .submit-button {
        flex: 1;
        padding: 12px 20px;
        background-color: #28a745;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        font-weight: 500;
    }

    .submit-button:hover {
        background-color: #218838;
    }

    .cancel-button {
        flex: 1;
        padding: 12px 20px;
        background-color: #6c757d;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        font-weight: 500;
    }

    .cancel-button:hover {
        background-color: #5a6268;
    }

    .error {
        color: #dc3545;
        font-size: 0.9em;
        padding: 8px;
        background-color: #f8d7da;
        border: 1px solid #f5c6cb;
        border-radius: 4px;
    }
</style>