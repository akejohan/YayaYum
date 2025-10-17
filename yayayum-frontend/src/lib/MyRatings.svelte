<script lang="ts">
    import { onMount } from "svelte";
    import { selectedUser, currentScreen } from "./shared";
    import { Component } from "./types";
    import { RatingsService, DishesService, UsersService } from "./api";
    import type { Rating, Dish, User } from "./api";

    let allRatings: Rating[] = [];
    let dishes: Dish[] = [];
    let users: User[] = [];
    let loading = true;
    let error: string | null = null;
    let selectedFilterUser: User | null = null; // null means show all users

    // Create a map of dish_id to dish for quick lookup
    $: dishMap = dishes.reduce((map, dish) => {
        map[dish.id] = dish;
        return map;
    }, {} as Record<number, Dish>);

    // Create a map of user_id to user for quick lookup
    $: userMap = users.reduce((map, user) => {
        map[user.id] = user;
        return map;
    }, {} as Record<number, User>);

    // Filter ratings based on selected user
    $: filteredRatings = selectedFilterUser 
        ? allRatings.filter(rating => rating.user_id === selectedFilterUser.id)
        : allRatings;

    // Sort ratings by date (newest first)
    $: sortedRatings = filteredRatings.sort((a, b) => 
        new Date(b.date).getTime() - new Date(a.date).getTime()
    );

    onMount(async () => {
        try {
            // Load ratings, dishes, and users
            const [ratingsResponse, dishesResponse, usersResponse] = await Promise.all([
                RatingsService.getRatings(), // Get all ratings instead of just current user's
                DishesService.getDishes(),
                UsersService.getUsers()
            ]);
            
            allRatings = ratingsResponse;
            dishes = dishesResponse;
            users = usersResponse;
            
            // Set default filter to current user if they exist
            if ($selectedUser) {
                selectedFilterUser = $selectedUser;
            }
        } catch (err) {
            error = "Kunde inte ladda recensioner";
            console.error("Error loading ratings:", err);
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

    function handleUserFilter(event: Event) {
        const select = event.target as HTMLSelectElement;
        const userId = select.value;
        
        if (userId === 'all') {
            selectedFilterUser = null;
        } else {
            selectedFilterUser = users.find(user => user.id === parseInt(userId)) || null;
        }
    }
</script>

<div class="my-ratings-container">
    <div class="header">
        <h2>Recensioner</h2>
        <p class="page-info">Alla recensioner från användare</p>
        
        <div class="filter-section">
            <label for="user-filter">Filtrera efter användare:</label>
            <select id="user-filter" on:change={handleUserFilter} value={selectedFilterUser?.id || 'all'}>
                <option value="all">Alla användare</option>
                {#each users as user (user.id)}
                    <option value={user.id}>{user.username}</option>
                {/each}
            </select>
        </div>
    </div>

    {#if loading}
        <div class="loading">
            <p>Laddar recensioner...</p>
        </div>
    {:else if error}
        <div class="error-message">
            <p>{error}</p>
        </div>
    {:else if sortedRatings.length === 0}
        <div class="empty-state">
            {#if selectedFilterUser}
                <h3>Inga recensioner från {selectedFilterUser.username}</h3>
                <p>{selectedFilterUser.username} har inte lämnat några recensioner än.</p>
            {:else}
                <h3>Inga recensioner ännu</h3>
                <p>Ingen har lämnat några recensioner än. Var den första att recensera en maträtt!</p>
            {/if}
            <button class="primary-button" on:click={goBack}>
                Tillbaka till huvudmenyn
            </button>
        </div>
    {:else}
        <div class="ratings-list">
            <div class="stats">
                {#if selectedFilterUser}
                    <p class="total-count">{selectedFilterUser.username} har lämnat <strong>{sortedRatings.length}</strong> recensioner</p>
                {:else}
                    <p class="total-count">Totalt <strong>{sortedRatings.length}</strong> recensioner från alla användare</p>
                {/if}
            </div>

            {#each sortedRatings as rating (rating.id)}
                {@const dish = dishMap[rating.dish_id]}
                {@const reviewUser = userMap[rating.user_id]}
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
                            {#if reviewUser}
                                <div class="review-user" class:current-user={reviewUser.id === $selectedUser?.id}>
                                    av {reviewUser.username}
                                    {#if reviewUser.id === $selectedUser?.id}
                                        <span class="you-indicator">(Du)</span>
                                    {/if}
                                </div>
                            {/if}
                        </div>
                    </div>
                    
                    {#if dish}
                        <p class="dish-description">{dish.description}</p>
                        <p class="dish-price">{dish.price_kr} kr</p>
                    {/if}

                    {#if rating.description}
                        <div class="user-comment">
                            <h4>Kommentar:</h4>
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

    .page-info {
        color: #666;
        margin: 0 0 1rem 0;
        font-style: italic;
    }

    .filter-section {
        margin-top: 1.5rem;
        padding: 1rem;
        background: rgba(255, 105, 180, 0.05);
        border-radius: 8px;
        border: 1px solid rgba(255, 105, 180, 0.1);
    }

    .filter-section label {
        display: block;
        margin-bottom: 0.5rem;
        font-weight: 500;
        color: #333;
    }

    .filter-section select {
        width: 100%;
        max-width: 300px;
        padding: 0.5rem;
        border: 1px solid #ccc;
        border-radius: 6px;
        background: white;
        font-size: 1rem;
        color: #333;
    }

    .filter-section select:focus {
        outline: none;
        border-color: #ff69b4;
        box-shadow: 0 0 0 2px rgba(255, 105, 180, 0.2);
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

    .review-user {
        font-size: 0.8rem;
        color: #666;
        margin-top: 0.3rem;
        font-style: italic;
    }

    .review-user.current-user {
        color: #ff69b4;
        font-weight: 500;
    }

    .you-indicator {
        color: #ff69b4;
        font-weight: 600;
        font-style: normal;
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