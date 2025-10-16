<script lang="ts">
    import { onMount } from "svelte";
    import { selectedUser, currentScreen } from "./shared";
    import { Component } from "./types";
    import { DishesService, RatingsService, ApiError } from "./api";
    import type { Dish, CreateRating } from "./api";

    let dishes: Dish[] = [];
    let selectedDish: Dish | null = null;
    let rating = 0;
    let comment = "";
    let isSubmitting = false;
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

    function setRating(newRating: number) {
        rating = newRating;
    }

    function selectDish(dish: Dish) {
        selectedDish = dish;
    }

    function validateForm(): boolean {
        if (!selectedDish) {
            error = "Please select a dish to rate";
            return false;
        }
        if (rating === 0) {
            error = "Please select a rating";
            return false;
        }
        if (!$selectedUser) {
            error = "No user selected";
            return false;
        }
        error = null;
        return true;
    }

    async function submitRating() {
        if (!validateForm()) {
            return;
        }

        isSubmitting = true;
        try {
            const ratingData: CreateRating = {
                user_id: $selectedUser!.id,
                dish_id: selectedDish!.id,
                rating: rating,
                description: comment || null
            };
            
            await RatingsService.createRating(ratingData);
            
            alert(`Tack ${$selectedUser?.username}! ${rating}/5 stjärnor`);
            
            // Reset form
            selectedDish = null;
            rating = 0;
            comment = "";
            error = null;
            
            // Navigate back to meal actions
            currentScreen.set(Component.MealActions);
        } catch (err) {
            if (err instanceof ApiError && err.status === 409) {
                error = `Du har redan lämnat en recension idag, ${$selectedUser?.username}...`;
            } else if (err instanceof ApiError && err.status === 400) {
                error = "Ogiltigt betyg. Vänligen välj ett betyg mellan 1-5 stjärnor.";
            } else {
                error = (err as Error).message || "Ett fel uppstod när betyget skulle sparas. Försök igen.";
            }
        } finally {
            isSubmitting = false;
        }
    }

    function goBack() {
        currentScreen.set(Component.MealActions);
    }
</script>

{#if loading}
    <div class="loading">
        <p>Loading dishes...</p>
    </div>
{:else}
    <div class="rate-meal-container">
        <div class="header">
            <h2>Fram med kritiken, {$selectedUser?.username}!</h2>
        </div>

        {#if error}
            <div class="error-message">
                <p>{error}</p>
            </div>
        {/if}

        <div class="dish-selection-section">
            <h3>Vad tryckte du i dig?</h3>
            <div class="dishes-grid">
                {#each dishes as dish (dish.id)}
                    <button 
                        class="dish-card"
                        class:selected={selectedDish?.id === dish.id}
                        on:click={() => selectDish(dish)}
                    >
                        <div class="dish-number">#{dish.nr}</div>
                        <div class="dish-name">{dish.name}</div>
                        {#if selectedDish?.id === dish.id}
                            <div class="selected-indicator">✓</div>
                        {/if}
                    </button>
                {/each}
            </div>
            <!-- {#if selectedDish}
                <div class="selected-dish-info">
                    <h4>Selected: #{selectedDish.nr} {selectedDish.name}</h4>
                    <p>{selectedDish.description}</p>
                </div>
            {/if} -->
        </div>

        <div class="rating-section">
            <h3>Ge några stjärnor:</h3>
            <div class="stars">
                {#each Array(5) as _, i}
                    <button 
                        class="star" 
                        class:filled={i < rating}
                        class:hover={i < rating}
                        on:click={() => setRating(i + 1)}
                        disabled={!selectedDish}
                    >
                        ★
                    </button>
                {/each}
            </div>
            <p class="rating-text">
                {#if !selectedDish}
                    
                {:else if rating === 0}
                    Hur många stjärnor?
                {:else if rating === 1}
                    Råttpiss ⭐
                {:else if rating === 2}
                    Gick väl ner ⭐⭐
                {:else if rating === 3}
                    Inte så pjåkig! ⭐⭐⭐
                {:else if rating === 4}
                    Smaka fasen fint! ⭐⭐⭐⭐
                {:else}
                    Förbannat god! ⭐⭐⭐⭐⭐
                {/if}
            </p>
        </div>

        <div class="comment-section">
            <textarea 
            style="width: 75%;"
                bind:value={comment}
                placeholder="Skriv en bok..."
                rows="3"
                disabled={!selectedDish}
            ></textarea>
        </div>

        <div class="actions">
            <button 
                class="submit-button" 
                on:click={submitRating}
                disabled={isSubmitting || !selectedDish || rating === 0}
            >
                {#if isSubmitting}
                    Faxar...
                {:else}
                    Upp i molnet!
                {/if}
            </button>
        </div>
    </div>
{/if}

<style>
    .loading {
        display: flex;
        justify-content: center;
        align-items: center;
        min-height: 300px;
        font-size: 1.1rem;
        color: #666;
    }

    .rate-meal-container {
        max-width: 700px;
        margin: 0 auto;
        padding: 2rem;
        background: white;
        border-radius: 16px;
        box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
    }

    .header {
        text-align: center;
        margin-bottom: 2rem;
        position: relative;
    }

    .back-button {
        position: absolute;
        left: 0;
        top: 0;
        background: #f0f0f0;
        border: 1px solid #ccc;
        border-radius: 8px;
        padding: 0.5rem 1rem;
        cursor: pointer;
        transition: background 0.2s;
    }

    .back-button:hover {
        background: #e0e0e0;
    }

    .header h2 {
        color: #ff69b4;
        margin: 0 0 0.5rem 0;
        text-shadow: 1px 1px 2px rgba(255, 255, 255, 0.8);
    }

    .user-info {
        color: #666;
        margin: 0;
        font-style: italic;
    }

    .error-message {
        background: #f8d7da;
        color: #721c24;
        padding: 1rem;
        border-radius: 8px;
        margin-bottom: 1.5rem;
        border: 1px solid #f5c6cb;
    }

    .error-message p {
        margin: 0;
        font-weight: 500;
    }

    .dish-selection-section {
        margin-bottom: 2rem;
    }

    .dish-selection-section h3 {
        color: #333;
        margin-bottom: 1rem;
        font-size: 1.2rem;
    }

    .dishes-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
        gap: 1rem;
        margin-bottom: 1.5rem;
    }

    .dish-card {
        background: white;
        border: 2px solid #e0e0e0;
        border-radius: 12px;
        padding: 0.5rem;
        cursor: pointer;
        transition: all 0.2s ease;
        position: relative;
        text-align: center;
        display: flex;
        flex-direction: column;
        justify-content: center;
    }

    .dish-card:hover {
        border-color: #ff69b4;
        transform: translateY(-2px);
        box-shadow: 0 4px 12px rgba(255, 105, 180, 0.2);
    }

    .dish-card.selected {
        border-color: #ff69b4;
        background: linear-gradient(135deg, rgba(255, 105, 180, 0.1), rgba(255, 140, 200, 0.1));
        box-shadow: 0 4px 12px rgba(255, 105, 180, 0.3);
    }

    .dish-number {
        font-size: 1.2rem;
        font-weight: bold;
        color: #ff69b4;
        margin-bottom: 0.5rem;
    }

    .dish-name {
        font-weight: 600;
        color: #333;
        margin-bottom: 0.3rem;
        font-size: 0.95rem;
    }

    .dish-price {
        color: #2d7d32;
        font-weight: 500;
        font-size: 0.9rem;
    }

    .selected-indicator {
        position: absolute;
        top: 0.5rem;
        right: 0.5rem;
        background: #ff69b4;
        color: white;
        border-radius: 50%;
        width: 24px;
        height: 24px;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 0.8rem;
        font-weight: bold;
    }

    .selected-dish-info {
        background: linear-gradient(135deg, rgba(255, 105, 180, 0.05), rgba(255, 140, 200, 0.05));
        border: 1px solid rgba(255, 105, 180, 0.2);
        border-radius: 8px;
        padding: 1rem;
        margin-top: 1rem;
    }

    .selected-dish-info h4 {
        margin: 0 0 0.5rem 0;
        color: #ff69b4;
        font-size: 1.1rem;
    }

    .selected-dish-info p {
        margin: 0;
        color: #666;
        font-size: 0.9rem;
        line-height: 1.4;
    }

    .rating-section, .comment-section {
        margin-bottom: 2rem;
    }

    .rating-section h3, .comment-section h3 {
        color: #333;
        margin-bottom: 1rem;
        font-size: 1.2rem;
    }

    .stars {
        display: flex;
        justify-content: center;
        gap: 0.5rem;
        margin-bottom: 1rem;
    }

    .star {
        background: none;
        border: none;
        font-size: 2.5rem;
        color: #ddd;
        cursor: pointer;
        transition: all 0.2s ease;
        padding: 0.2rem;
        border-radius: 4px;
    }

    .star:hover:not(:disabled) {
        color: #ffd700;
        transform: scale(1.1);
    }

    .star.filled {
        color: #ffd700;
    }

    .star:disabled {
        opacity: 0.3;
        cursor: not-allowed;
    }

    .rating-text {
        text-align: center;
        font-size: 1.1rem;
        font-weight: 500;
        color: #555;
        margin: 0;
    }

    .comment-section textarea {
        width: 100%;
        padding: 1rem;
        border: 2px solid #e0e0e0;
        border-radius: 8px;
        font-family: inherit;
        font-size: 1rem;
        resize: vertical;
        min-height: 100px;
        transition: border-color 0.2s;
    }

    .comment-section textarea:focus {
        outline: none;
        border-color: #ff69b4;
        box-shadow: 0 0 0 2px rgba(255, 105, 180, 0.1);
    }

    .comment-section textarea:disabled {
        background-color: #f5f5f5;
        cursor: not-allowed;
        opacity: 0.6;
    }

    .actions {
        text-align: center;
    }

    .submit-button {
        background: linear-gradient(135deg, #ff69b4, #ff8cc8);
        color: white;
        border: none;
        border-radius: 12px;
        padding: 1rem 2rem;
        font-size: 1.1rem;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.2s ease;
        box-shadow: 0 4px 12px rgba(255, 105, 180, 0.3);
    }

    .submit-button:hover:not(:disabled) {
        transform: translateY(-2px);
        box-shadow: 0 6px 16px rgba(255, 105, 180, 0.4);
    }

    .submit-button:disabled {
        opacity: 0.6;
        cursor: not-allowed;
        transform: none;
        background: #ccc;
        box-shadow: none;
    }

    /* Mobile responsive */
    @media (max-width: 500px) {
        .rate-meal-container {
            margin: 0rem;
            padding: 0.5rem;
            min-width: 17rem;
        }

        .dishes-grid {
            grid-template-columns: 1fr;
            gap: 0.8rem;
        }

        .dish-card {
            min-height: 65px;
        }

        .star {
            font-size: 2rem;
        }

        .back-button {
            position: static;
            margin-bottom: 1rem;
        }

        .header {
            text-align: left;
        }
    }
</style>