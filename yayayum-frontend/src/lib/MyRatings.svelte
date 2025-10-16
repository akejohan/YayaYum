<script lang="ts">
    import { onMount } from "svelte";
    import { selectedUser, currentScreen } from "./shared";
    import { Component } from "./types";
    import { RatingsService, DishesService } from "./api";
    import type { Rating, Dish } from "./api";

    let userRatings: Rating[] = [];
    let dishes: Dish[] = [];
    let loading = true;
    let error: string | null = null;

    // Create a map of dish_id to dish for quick lookup
    $: dishMap = dishes.reduce((map, dish) => {
        map[dish.id] = dish;
        return map;
    }, {} as Record<number, Dish>);

    // Sort ratings by date (newest first)
    $: sortedRatings = userRatings.sort((a, b) => 
        new Date(b.date).getTime() - new Date(a.date).getTime()
    );

    onMount(async () => {
        if (!$selectedUser) {
            error = "Ingen användare vald";
            loading = false;
            return;
        }

        try {
            // Load both ratings and dishes
            const [ratingsResponse, dishesResponse] = await Promise.all([
                RatingsService.getRatingsByUser($selectedUser.id),
                DishesService.getDishes()
            ]);
            
            userRatings = ratingsResponse;
            dishes = dishesResponse;
        } catch (err) {
            error = "Kunde inte ladda dina recensioner";
            console.error("Error loading user ratings:", err);
        } finally {
            loading = false;
        }
    });

    function goBack() {
        currentScreen.set(Component.MealActions);
    }

    function formatDate(dateString: string): string {
        const date = new Date(dateString);
        return date.toLocaleDateString('sv-SE', {
            year: 'numeric',
            month: 'long',
            day: 'numeric'
        });
    }

    function getStars(rating: number): string {
        return '★'.repeat(rating) + '☆'.repeat(5 - rating);
    }
</script>

<div class="my-ratings-container">
    <div class="header">
        <button class="back-button" on:click={goBack}>← Tillbaka</button>
        <h2>Mina recensioner</h2>
        <p class="user-info">Dina tidigare omdömen, {$selectedUser?.username}</p>
    </div>

    {#if loading}
        <div class="loading">
            <p>Laddar dina recensioner...</p>
        </div>
    {:else if error}
        <div class="error-message">
            <p>{error}</p>
        </div>
    {:else if sortedRatings.length === 0}
        <div class="empty-state">
            <h3>Inga recensioner ännu</h3>
            <p>Du har inte lämnat några recensioner än. Gå och ät något gott och kom sedan tillbaka för att berätta vad du tyckte!</p>
            <button class="primary-button" on:click={goBack}>
                Tillbaka till huvudmenyn
            </button>
        </div>
    {:else}
        <div class="ratings-list">
            <div class="stats">
                <p class="total-count">Du har lämnat <strong>{sortedRatings.length}</strong> recensioner</p>
            </div>

            {#each sortedRatings as rating (rating.id)}
                {@const dish = dishMap[rating.dish_id]}
                <div class="rating-card">
                    <div class="rating-header">
                        <div class="dish-info">
                            {#if dish}
                                <h3 class="dish-name">#{dish.nr} {dish.name}</h3>
                                <p class="dish-category">{dish.category}</p>
                            {:else}
                                <h3 class="dish-name">Okänd rätt (ID: {rating.dish_id})</h3>
                            {/if}
                        </div>
                        <div class="rating-info">
                            <div class="stars">{getStars(rating.rating)}</div>
                            <div class="date">{formatDate(rating.date)}</div>
                        </div>
                    </div>
                    
                    {#if dish}
                        <p class="dish-description">{dish.description}</p>
                        <p class="dish-price">{dish.price_kr} kr</p>
                    {/if}

                    {#if rating.description}
                        <div class="user-comment">
                            <h4>Din kommentar:</h4>
                            <p>"{rating.description}"</p>
                        </div>
                    {/if}
                </div>
            {/each}
        </div>
    {/if}
</div>

<style>
    .my-ratings-container {
        max-width: 800px;
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

    .loading {
        display: flex;
        justify-content: center;
        align-items: center;
        min-height: 200px;
        font-size: 1.1rem;
        color: #666;
    }

    .error-message {
        background: #f8d7da;
        color: #721c24;
        padding: 1rem;
        border-radius: 8px;
        margin-bottom: 1.5rem;
        border: 1px solid #f5c6cb;
        text-align: center;
    }

    .empty-state {
        text-align: center;
        padding: 3rem 1rem;
        color: #666;
    }

    .empty-state h3 {
        color: #333;
        margin-bottom: 1rem;
    }

    .empty-state p {
        margin-bottom: 2rem;
        line-height: 1.6;
    }

    .primary-button {
        background: linear-gradient(135deg, #ff69b4, #ff8cc8);
        color: white;
        border: none;
        border-radius: 12px;
        padding: 1rem 2rem;
        font-size: 1rem;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.2s ease;
        box-shadow: 0 4px 12px rgba(255, 105, 180, 0.3);
    }

    .primary-button:hover {
        transform: translateY(-2px);
        box-shadow: 0 6px 16px rgba(255, 105, 180, 0.4);
    }

    .stats {
        background: linear-gradient(135deg, rgba(255, 105, 180, 0.1), rgba(255, 140, 200, 0.1));
        border: 1px solid rgba(255, 105, 180, 0.2);
        border-radius: 8px;
        padding: 1rem;
        margin-bottom: 2rem;
        text-align: center;
    }

    .total-count {
        margin: 0;
        color: #333;
        font-size: 1.1rem;
    }

    .ratings-list {
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    .rating-card {
        background: white;
        border: 1px solid #e0e0e0;
        border-radius: 12px;
        padding: 1.5rem;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
        transition: all 0.2s ease;
    }

    .rating-card:hover {
        transform: translateY(-2px);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.12);
    }

    .rating-header {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        margin-bottom: 1rem;
    }

    .dish-info {
        flex: 1;
    }

    .dish-name {
        margin: 0 0 0.3rem 0;
        color: #ff7e6b;
        font-size: 1.2rem;
        font-weight: 600;
    }

    .dish-category {
        margin: 0;
        color: #888;
        font-size: 0.9rem;
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }

    .rating-info {
        text-align: right;
        flex-shrink: 0;
        margin-left: 1rem;
    }

    .stars {
        font-size: 1.2rem;
        color: #ffd700;
        margin-bottom: 0.3rem;
    }

    .date {
        font-size: 0.85rem;
        color: #888;
    }

    .dish-description {
        color: #666;
        margin: 0.5rem 0;
        line-height: 1.4;
        font-size: 0.95rem;
    }

    .dish-price {
        color: #2d7d32;
        font-weight: 600;
        margin: 0.5rem 0;
    }

    .user-comment {
        background: #f8f9fa;
        border-left: 4px solid #ff69b4;
        padding: 1rem;
        margin-top: 1rem;
        border-radius: 0 8px 8px 0;
    }

    .user-comment h4 {
        margin: 0 0 0.5rem 0;
        color: #333;
        font-size: 0.9rem;
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }

    .user-comment p {
        margin: 0;
        color: #555;
        font-style: italic;
        line-height: 1.4;
    }

    /* Mobile responsive */
    @media (max-width: 600px) {
        .my-ratings-container {
            margin: 1rem;
            padding: 1.5rem;
        }

        .rating-header {
            flex-direction: column;
            align-items: flex-start;
        }

        .rating-info {
            margin-left: 0;
            margin-top: 0.5rem;
            text-align: left;
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