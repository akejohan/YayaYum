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

    interface UserStats {
        user: User;
        totalReviews: number;
        uniqueDishes: number;
        averageRating: number;
        topDishes: { dish: Dish; count: number }[];
    }

    let userStats: UserStats[] = [];

    // Create a map of dish_id to dish for quick lookup
    $: dishMap = dishes.reduce((map, dish) => {
        map[dish.id] = dish;
        return map;
    }, {} as Record<number, Dish>);

    onMount(async () => {
        try {
            // Load ratings, dishes, and users
            const [ratingsResponse, dishesResponse, usersResponse] = await Promise.all([
                RatingsService.getRatings(),
                DishesService.getDishes(),
                UsersService.getUsers()
            ]);
            
            allRatings = ratingsResponse;
            dishes = dishesResponse;
            users = usersResponse;
            
            calculateUserStats();
        } catch (err) {
            error = "Kunde inte ladda leaderboard data";
            console.error("Error loading leaderboard data:", err);
        } finally {
            loading = false;
        }
    });

    function calculateUserStats() {
        userStats = users.map(user => {
            const userRatings = allRatings.filter(rating => rating.user_id === user.id);
            const uniqueDishIds = [...new Set(userRatings.map(rating => rating.dish_id))];
            
            // Calculate average rating
            const averageRating = userRatings.length > 0 
                ? userRatings.reduce((sum, rating) => sum + rating.rating, 0) / userRatings.length 
                : 0;

            // Find top dishes (most reviewed dishes by this user)
            const dishCounts = userRatings.reduce((counts, rating) => {
                counts[rating.dish_id] = (counts[rating.dish_id] || 0) + 1;
                return counts;
            }, {} as Record<number, number>);

            const topDishes = Object.entries(dishCounts)
                .map(([dishId, count]) => ({
                    dish: dishMap[parseInt(dishId)],
                    count: count
                }))
                .filter(item => item.dish) // Only include dishes we have data for
                .sort((a, b) => b.count - a.count)
                .slice(0, 3); // Top 3 dishes

            return {
                user,
                totalReviews: userRatings.length,
                uniqueDishes: uniqueDishIds.length,
                averageRating,
                topDishes
            };
        });

        // Sort by total reviews (descending), then by unique dishes (descending)
        userStats.sort((a, b) => {
            if (b.totalReviews !== a.totalReviews) {
                return b.totalReviews - a.totalReviews;
            }
            return b.uniqueDishes - a.uniqueDishes;
        });
    }

    function goBack() {
        currentScreen.set(Component.MealActions);
    }

    function getRankEmoji(index: number): string {
        switch (index) {
            case 0: return "🥇";
            case 1: return "🥈";
            case 2: return "🥉";
            default: return "🏅";
        }
    }

    function getStars(rating: number): string {
        return '★'.repeat(Math.round(rating)) + '☆'.repeat(5 - Math.round(rating));
    }
</script>

<div class="leaderboard-container">
    <div class="header">
        <button class="back-button" on:click={goBack}>← Tillbaka</button>
        <h2>🏆 Leaderboard</h2>
        <p class="page-info">Rankning baserad på antal recensioner och unika rätter</p>
    </div>

    {#if loading}
        <div class="loading">
            <p>Laddar leaderboard...</p>
        </div>
    {:else if error}
        <div class="error-message">
            <p>{error}</p>
        </div>
    {:else if userStats.length === 0}
        <div class="empty-state">
            <h3>Inga användare att visa</h3>
            <p>Det finns inga användare med recensioner än.</p>
            <button class="primary-button" on:click={goBack}>
                Tillbaka till huvudmenyn
            </button>
        </div>
    {:else}
        <div class="leaderboard-list">
            {#each userStats as stats, index (stats.user.id)}
                <div class="user-card" class:current-user={stats.user.id === $selectedUser?.id}>
                    <div class="rank-section">
                        <div class="rank-badge">
                            <span class="rank-emoji">{getRankEmoji(index)}</span>
                            <span class="rank-number">#{index + 1}</span>
                        </div>
                    </div>
                    
                    <div class="user-info">
                        <div class="user-name">
                            {stats.user.username}
                            {#if stats.user.id === $selectedUser?.id}
                                <span class="you-indicator">(Du)</span>
                            {/if}
                        </div>
                        
                        <div class="stats-grid">
                            <div class="stat">
                                <span class="stat-value">{stats.totalReviews}</span>
                                <span class="stat-label">recensioner</span>
                            </div>
                            <div class="stat">
                                <span class="stat-value">{stats.uniqueDishes}</span>
                                <span class="stat-label">unika rätter</span>
                            </div>
                            <div class="stat">
                                <span class="stat-value">{stats.averageRating.toFixed(1)}</span>
                                <span class="stat-label">snittbetyg</span>
                            </div>
                        </div>

                        {#if stats.averageRating > 0}
                            <div class="average-stars">
                                {getStars(stats.averageRating)}
                            </div>
                        {/if}

                        {#if stats.topDishes.length > 0}
                            <div class="top-dishes">
                                <h4>Favoriter:</h4>
                                <div class="dishes-list">
                                    {#each stats.topDishes as dishStat (dishStat.dish.id)}
                                        <span class="dish-tag">
                                            #{dishStat.dish.nr} {dishStat.dish.name}
                                            {#if dishStat.count > 1}
                                                <span class="count-badge">×{dishStat.count}</span>
                                            {/if}
                                        </span>
                                    {/each}
                                </div>
                            </div>
                        {/if}
                    </div>
                </div>
            {/each}
        </div>
    {/if}
</div>

<style>
    .leaderboard-container {
        max-width: 900px;
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
        font-size: 2rem;
    }

    .page-info {
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

    .leaderboard-list {
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    .user-card {
        display: flex;
        align-items: flex-start;
        background: #fffdfb;
        border-radius: 16px;
        padding: 1.5rem;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
        transition: all 0.2s ease;
        border: 1px solid rgba(0, 0, 0, 0.05);
    }

    .user-card:hover {
        background: #fff7f2;
        transform: translateY(-2px);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.12);
    }

    .user-card.current-user {
        border: 2px solid #ff69b4;
        background: linear-gradient(135deg, rgba(255, 105, 180, 0.05), rgba(255, 140, 200, 0.05));
    }

    .rank-section {
        margin-right: 1.5rem;
        flex-shrink: 0;
    }

    .rank-badge {
        display: flex;
        flex-direction: column;
        align-items: center;
        text-align: center;
    }

    .rank-emoji {
        font-size: 2rem;
        margin-bottom: 0.2rem;
    }

    .rank-number {
        font-size: 0.9rem;
        font-weight: 600;
        color: #666;
    }

    .user-info {
        flex: 1;
    }

    .user-name {
        font-size: 1.3rem;
        font-weight: 600;
        color: #333;
        margin-bottom: 1rem;
    }

    .you-indicator {
        color: #ff69b4;
        font-weight: 600;
        font-size: 1rem;
        margin-left: 0.5rem;
    }

    .stats-grid {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        gap: 1rem;
        margin-bottom: 1rem;
    }

    .stat {
        text-align: center;
        background: rgba(255, 105, 180, 0.1);
        border-radius: 8px;
        padding: 0.8rem 0.5rem;
        border: 1px solid rgba(255, 105, 180, 0.2);
    }

    .stat-value {
        display: block;
        font-size: 1.5rem;
        font-weight: 700;
        color: #ff69b4;
    }

    .stat-label {
        display: block;
        font-size: 0.8rem;
        color: #666;
        text-transform: uppercase;
        letter-spacing: 0.5px;
        margin-top: 0.2rem;
    }

    .average-stars {
        text-align: center;
        font-size: 1.2rem;
        color: #ffd700;
        margin-bottom: 1rem;
    }

    .top-dishes {
        margin-top: 1rem;
    }

    .top-dishes h4 {
        margin: 0 0 0.5rem 0;
        font-size: 0.9rem;
        color: #666;
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }

    .dishes-list {
        display: flex;
        flex-wrap: wrap;
        gap: 0.5rem;
    }

    .dish-tag {
        background: #e3f2fd;
        color: #1976d2;
        padding: 0.3rem 0.6rem;
        border-radius: 12px;
        font-size: 0.8rem;
        border: 1px solid #bbdefb;
        display: flex;
        align-items: center;
        gap: 0.3rem;
    }

    .count-badge {
        background: #1976d2;
        color: white;
        border-radius: 50%;
        width: 18px;
        height: 18px;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 0.7rem;
        font-weight: 600;
    }

    /* 📱 Mobilanpassning */
    @media (max-width: 600px) {
        .leaderboard-container {
            padding: 1rem;
        }

        .user-card {
            flex-direction: column;
            text-align: center;
        }

        .rank-section {
            margin-right: 0;
            margin-bottom: 1rem;
        }

        .stats-grid {
            grid-template-columns: 1fr;
            gap: 0.5rem;
        }

        .stat {
            padding: 0.6rem 0.4rem;
        }

        .stat-value {
            font-size: 1.2rem;
        }

        .dishes-list {
            justify-content: center;
        }

        .header h2 {
            font-size: 1.5rem;
        }
    }
</style>