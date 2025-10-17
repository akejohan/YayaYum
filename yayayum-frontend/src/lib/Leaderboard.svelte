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

    interface Achievement {
        id: string;
        name: string;
        description: string;
        emoji: string;
        unlocked: boolean;
    }

    interface UserStats {
        user: User;
        totalReviews: number;
        uniqueDishes: number;
        averageRating: number;
        topDishes: { dish: Dish; count: number }[];
        achievements: Achievement[];
        consecutiveDays: number;
    }

    let userStats: UserStats[] = [];
    let achievementsExpanded = false;
    let selectedAchievement: Achievement | null = null;
    let modalUserName: string = '';

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

            // Calculate consecutive days
            const consecutiveDays = calculateConsecutiveDays(userRatings);

            // Calculate achievements
            const achievements = calculateAchievements(userRatings, uniqueDishIds.length, consecutiveDays);

            return {
                user,
                totalReviews: userRatings.length,
                uniqueDishes: uniqueDishIds.length,
                averageRating,
                topDishes,
                achievements,
                consecutiveDays
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

    function calculateConsecutiveDays(userRatings: Rating[]): number {
        if (userRatings.length === 0) return 0;

        // Get unique dates sorted in descending order
        const uniqueDates = [...new Set(userRatings.map(rating => 
            new Date(rating.date).toDateString()
        ))].sort((a, b) => new Date(b).getTime() - new Date(a).getTime());

        if (uniqueDates.length === 0) return 0;

        let consecutive = 1;
        let maxConsecutive = 1;

        for (let i = 1; i < uniqueDates.length; i++) {
            const currentDate = new Date(uniqueDates[i]);
            const previousDate = new Date(uniqueDates[i - 1]);
            const dayDifference = Math.abs(previousDate.getTime() - currentDate.getTime()) / (1000 * 60 * 60 * 24);

            if (dayDifference === 1) {
                consecutive++;
                maxConsecutive = Math.max(maxConsecutive, consecutive);
            } else {
                consecutive = 1;
            }
        }

        return maxConsecutive;
    }

    function calculateAchievements(userRatings: Rating[], uniqueDishes: number, consecutiveDays: number): Achievement[] {
        const totalDishes = dishes.length;
        
        // Count specific dish ratings
        const baliGorenRatings = userRatings.filter(rating => {
            const dish = dishMap[rating.dish_id];
            return dish?.nr === 43; // Bali Goreng
        }).length;
        
        const spicyChiliNoodleRatings = userRatings.filter(rating => {
            const dish = dishMap[rating.dish_id];
            return dish?.nr === 7; // Spicy Chili Noodles
        }).length;
        
        return [
            {
                id: 'first_steps',
                name: 'Gröngöling',
                description: 'Skriv 3 recensioner',
                emoji: '🌱',
                unlocked: userRatings.length >= 3
            },
            {
                id: 'food_critic',
                name: 'Kritiker',
                description: 'Skriv 10 recensioner',
                emoji: '📝',
                unlocked: userRatings.length >= 10
            },
            {
                id: 'completionist',
                name: 'Stormästare',
                description: 'Recensera alla rätter',
                emoji: '🎯',
                unlocked: uniqueDishes >= totalDishes && totalDishes > 0
            },
            {
                id: 'consistency',
                name: 'Fett hungrig',
                description: 'Recensera 3 dagar i rad',
                emoji: '🔥',
                unlocked: consecutiveDays >= 3
            },
            {
                id: 'dedication',
                name: 'Kung av Yaya',
                description: 'Recensera 5 dagar i rad',
                emoji: '⚡',
                unlocked: consecutiveDays >= 5
            },
            {
                id: 'bali_goreng_king',
                name: 'Kung av Bali Goreng',
                description: 'Ät Bali Goreng 5 gånger',
                emoji: '👑',
                unlocked: baliGorenRatings >= 5
            },
            {
                id: 'spicy_chili_king',
                name: 'Kung av Spicy Chili Noodles',
                description: 'Ät Spicy Chili Noodles 5 gånger',
                emoji: '🌶️',
                unlocked: spicyChiliNoodleRatings >= 5
            }
        ];
    }

    function goBack() {
        currentScreen.set(Component.MealActions);
    }

    function toggleAchievements() {
        achievementsExpanded = !achievementsExpanded;
    }

    function showAchievementModal(achievement: Achievement, userName: string) {
        selectedAchievement = achievement;
        modalUserName = userName;
    }

    function hideAchievementModal() {
        selectedAchievement = null;
        modalUserName = '';
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

    // Static list of all possible achievements for the overview
    function getAllAchievementTypes(): Achievement[] {
        return [
            {
                id: 'first_steps',
                name: 'Gröngöling',
                description: 'Skriv 3 recensioner',
                emoji: '🌱',
                unlocked: false
            },
            {
                id: 'food_critic',
                name: 'Kritiker',
                description: 'Skriv 10 recensioner',
                emoji: '📝',
                unlocked: false
            },
            {
                id: 'completionist',
                name: 'Stormästare',
                description: 'Recensera alla rätter',
                emoji: '🎯',
                unlocked: false
            },
            {
                id: 'consistency',
                name: 'Fett hungrig',
                description: 'Recensera 3 dagar i rad',
                emoji: '🔥',
                unlocked: false
            },
            {
                id: 'dedication',
                name: 'Kung av Yaya',
                description: 'Recensera 5 dagar i rad',
                emoji: '⚡',
                unlocked: false
            },
            {
                id: 'bali_goreng_king',
                name: 'Kung av Bali Goreng',
                description: 'Ät Bali Goreng 5 gånger',
                emoji: '👑',
                unlocked: false
            },
            {
                id: 'spicy_chili_king',
                name: 'Kung av Spicy Chili Noodles',
                description: 'Ät Spicy Chili Noodles 5 gånger',
                emoji: '🌶️',
                unlocked: false
            }
        ];
    }
</script>

<div class="leaderboard-container">
    <div class="header">
        <h2>🏆 Leaderboard</h2>
        <p class="page-info">Rankning och achivements</p>
    </div>

    {#if !loading && !error && userStats.length > 0}
        <!-- Global Achievements Overview -->
        <div class="global-achievements">
            <div 
                class="achievements-header" 
                role="button"
                tabindex="0"
                onclick={toggleAchievements}
                onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && toggleAchievements()}
            >
                <h3>🎯 Utmärkelser</h3>
                <button class="toggle-btn" type="button">
                    {achievementsExpanded ? '▼' : '▶'}
                </button>
            </div>
            {#if achievementsExpanded}
                <div class="achievements-overview">
                {#each getAllAchievementTypes() as achievementType}
                    {@const unlockedBy = userStats.filter(stats => 
                        stats.achievements.find(a => a.id === achievementType.id)?.unlocked
                    )}
                    <div class="achievement-info-card">
                        <div class="achievement-header">
                            <span class="achievement-emoji">{achievementType.emoji}</span>
                            <div class="achievement-details">
                                <span class="achievement-name">{achievementType.name}</span>
                                <span class="achievement-description">{achievementType.description}</span>
                            </div>
                        </div>
                        {#if unlockedBy.length > 0}
                            <div class="unlocked-users">
                                <span class="unlocked-label">Upplåst av:</span>
                                {#each unlockedBy as stats}
                                    <span class="unlocked-user">
                                        {stats.user.username}
                                        {#if stats.user.id === $selectedUser?.id}
                                            <span class="you-badge">Du</span>
                                        {/if}
                                    </span>
                                {/each}
                            </div>
                        {:else}
                            <div class="locked-info">
                                <span class="locked-label">Ingen har låst upp denna än</span>
                            </div>
                        {/if}
                    </div>
                {/each}
                </div>
            {/if}
        </div>
    {/if}

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
            <button class="primary-button" onclick={goBack}>
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
                                <span class="stat-label">rätter</span>
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

                        <div class="user-achievements">
                            <div class="achievement-icons">
                                {#each stats.achievements as achievement (achievement.id)}
                                    <button 
                                        class="achievement-icon" 
                                        class:unlocked={achievement.unlocked}
                                        class:locked={!achievement.unlocked}
                                        title="{achievement.name}: {achievement.description}"
                                        onclick={(e) => showAchievementModal(achievement, stats.user.username)}
                                        aria-label="View {achievement.name} achievement details"
                                    >
                                        {achievement.emoji}
                                    </button>
                                {/each}
                            </div>
                        </div>
                    </div>
                </div>
            {/each}
        </div>
    {/if}
</div>

<!-- Achievement Modal -->
{#if selectedAchievement}
    <div 
        class="modal-overlay" 
        onclick={hideAchievementModal} 
        onkeydown={(e) => e.key === 'Escape' && hideAchievementModal()}
        role="dialog" 
        aria-modal="true"
        tabindex="-1"
    >
        <div 
            class="modal-content achievement-modal" 
            role="document"
        >
            <div class="modal-header">
                <h3>{selectedAchievement.emoji} {selectedAchievement.name}</h3>
                <button class="close-button" onclick={hideAchievementModal} aria-label="Stäng modal">
                    ✕
                </button>
            </div>
            
            <div class="modal-body">
                <div class="achievement-status">
                    {#if selectedAchievement.unlocked}
                        <span class="status-badge unlocked">🎉 Upplåst!</span>
                    {:else}
                        <span class="status-badge locked">🔒 Låst</span>
                    {/if}
                </div>
                
                <p class="achievement-description">
                    {selectedAchievement.description}
                </p>
                
                {#if modalUserName}
                    <div class="user-context">
                        <p><strong>Användare:</strong> {modalUserName}</p>
                    </div>
                {/if}
            </div>
        </div>
    </div>
{/if}

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

    .global-achievements {
        background: linear-gradient(135deg, rgba(255, 105, 180, 0.05), rgba(255, 140, 200, 0.05));
        border: 1px solid rgba(255, 105, 180, 0.2);
        border-radius: 12px;
        padding: 1.5rem;
        margin-bottom: 2rem;
    }

    .achievements-header {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 1rem;
        cursor: pointer;
        padding: 0.5rem;
        border-radius: 8px;
        transition: background-color 0.2s ease;
        margin-bottom: 1rem;
    }

    .achievements-header:hover {
        background: rgba(255, 105, 180, 0.1);
    }

    .achievements-header h3 {
        margin: 0;
        color: #ff69b4;
        font-size: 1.2rem;
        flex: 1;
        text-align: center;
    }

    .toggle-btn {
        background: none;
        border: none;
        color: #ff69b4;
        font-size: 1rem;
        cursor: pointer;
        padding: 0.2rem;
        min-width: 24px;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .achievements-overview {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
        gap: 1rem;
    }

    .achievement-info-card {
        background: white;
        border: 1px solid rgba(255, 105, 180, 0.3);
        border-radius: 12px;
        padding: 1rem;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        transition: all 0.2s ease;
    }

    .achievement-info-card:hover {
        transform: translateY(-2px);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    }

    .achievement-header {
        display: flex;
        align-items: flex-start;
        gap: 0.8rem;
        margin-bottom: 0.8rem;
    }

    .achievement-emoji {
        font-size: 2rem;
        flex-shrink: 0;
    }

    .achievement-details {
        flex: 1;
    }

    .achievement-name {
        display: block;
        font-weight: 600;
        font-size: 1rem;
        color: #333;
        margin-bottom: 0.2rem;
    }

    .achievement-description {
        display: block;
        font-size: 0.85rem;
        color: #666;
        line-height: 1.3;
    }

    .unlocked-users {
        background: rgba(76, 175, 80, 0.1);
        border: 1px solid rgba(76, 175, 80, 0.3);
        border-radius: 8px;
        padding: 0.5rem;
    }

    .unlocked-label {
        font-size: 0.8rem;
        color: #2e7d32;
        font-weight: 500;
        margin-bottom: 0.3rem;
        display: block;
    }

    .unlocked-user {
        display: inline-block;
        background: rgba(76, 175, 80, 0.2);
        color: #2e7d32;
        padding: 0.2rem 0.5rem;
        border-radius: 12px;
        font-size: 0.8rem;
        margin: 0.1rem 0.2rem 0.1rem 0;
        font-weight: 500;
    }

    .locked-info {
        background: rgba(0, 0, 0, 0.05);
        border: 1px solid rgba(0, 0, 0, 0.1);
        border-radius: 8px;
        padding: 0.5rem;
    }

    .locked-label {
        font-size: 0.8rem;
        color: #666;
        font-style: italic;
    }

    .you-badge {
        background: #ff69b4;
        color: white;
        padding: 0.1rem 0.3rem;
        border-radius: 8px;
        font-size: 0.7rem;
        font-weight: 600;
        margin-left: 0.3rem;
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
        gap: 1rem;
    }

    .user-card {
        display: flex;
        align-items: flex-start;
        background: #fffdfb;
        border-radius: 12px;
        padding: 1rem;
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
        margin-right: 1rem;
        flex-shrink: 0;
    }

    .rank-badge {
        display: flex;
        flex-direction: column;
        align-items: center;
        text-align: center;
    }

    .rank-emoji {
        font-size: 1.5rem;
        margin-bottom: 0.1rem;
    }

    .rank-number {
        font-size: 0.8rem;
        font-weight: 600;
        color: #666;
    }

    .user-info {
        flex: 1;
    }

    .user-name {
        font-size: 1.1rem;
        font-weight: 600;
        color: #333;
        margin-bottom: 0.6rem;
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
        gap: 0.6rem;
        margin-bottom: 0.8rem;
    }

    .stat {
        text-align: center;
        background: rgba(255, 105, 180, 0.1);
        border-radius: 6px;
        padding: 0.5rem 0.3rem;
        border: 1px solid rgba(255, 105, 180, 0.2);
    }

    .stat-value {
        display: block;
        font-size: 1.2rem;
        font-weight: 700;
        color: #ff69b4;
    }

    .stat-label {
        display: block;
        font-size: 0.7rem;
        color: #666;
        text-transform: uppercase;
        letter-spacing: 0.5px;
        margin-top: 0.1rem;
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

    .user-achievements {
        margin-top: 0.6rem;
        display: flex;
        justify-content: center;
    }

    .achievement-icons {
        display: flex;
        gap: 0.3rem;
        flex-wrap: wrap;
        justify-content: center;
    }

    .achievement-icon {
        font-size: 1rem;
        padding: 0.2rem;
        border-radius: 50%;
        background: rgba(255, 215, 0, 0.1);
        border: 1px solid rgba(255, 215, 0, 0.3);
        transition: all 0.2s ease;
        cursor: pointer;
        outline: none;
        position: relative;
    }

    .achievement-icon:focus {
        box-shadow: 0 0 0 2px #ff69b4;
    }

    .achievement-icon.unlocked {
        background: linear-gradient(135deg, rgba(255, 215, 0, 0.2), rgba(255, 193, 7, 0.2));
        border-color: rgba(255, 193, 7, 0.5);
        animation: glow 2s ease-in-out infinite alternate;
    }

    .achievement-icon.locked {
        background: rgba(0, 0, 0, 0.05);
        border-color: rgba(0, 0, 0, 0.2);
        opacity: 0.4;
        filter: grayscale(1);
        animation: none;
    }

    @keyframes glow {
        from {
            box-shadow: 0 0 5px rgba(255, 193, 7, 0.3);
        }
        to {
            box-shadow: 0 0 10px rgba(255, 193, 7, 0.6);
        }
    }

    .achievement-icon.unlocked:hover {
        transform: scale(1.15);
        filter: brightness(1.2);
        box-shadow: 0 2px 8px rgba(255, 193, 7, 0.4);
    }

    .achievement-icon.locked:hover {
        transform: scale(1.05);
        filter: grayscale(0.8);
        background: rgba(0, 0, 0, 0.08);
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
            grid-template-columns: repeat(3, 1fr);
            gap: 0.4rem;
        }

        .stat {
            padding: 0.4rem 0.2rem;
        }

        .stat-value {
            font-size: 1rem;
        }

        .stat-label {
            font-size: 0.65rem;
        }

        .dishes-list {
            justify-content: center;
        }

        .achievements-overview {
            grid-template-columns: 1fr;
            gap: 0.8rem;
        }

        .achievement-info-card {
            padding: 0.8rem;
        }

        .achievement-emoji {
            font-size: 1.5rem;
        }

        .achievement-name {
            font-size: 0.9rem;
        }

        .achievement-description {
            font-size: 0.8rem;
        }

        .achievement-icons {
            gap: 0.3rem;
        }

        .achievement-icon {
            font-size: 1rem;
            padding: 0.25rem;
        }

        .header h2 {
            font-size: 1.5rem;
        }
    }

    /* Modal Styles */
    .modal-overlay {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background: rgba(0, 0, 0, 0.7);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
        padding: 1rem;
    }

    .modal-content {
        background: white;
        border-radius: 16px;
        max-width: 500px;
        width: 100%;
        max-height: 80vh;
        overflow-y: auto;
        box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
        animation: modal-appear 0.2s ease-out;
    }

    .modal-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1.5rem 1.5rem 1rem;
        border-bottom: 1px solid #f0f0f0;
    }

    .modal-header h3 {
        margin: 0;
        color: #ff69b4;
        font-size: 1.3rem;
    }

    .close-button {
        background: none;
        border: none;
        font-size: 1.5rem;
        cursor: pointer;
        color: #666;
        padding: 0.25rem;
        border-radius: 4px;
        transition: all 0.2s;
    }

    .close-button:hover {
        background: #f0f0f0;
        color: #333;
    }

    .modal-body {
        padding: 1.5rem;
    }

    .achievement-status {
        margin-bottom: 1rem;
    }

    .status-badge {
        display: inline-block;
        padding: 0.5rem 1rem;
        border-radius: 20px;
        font-weight: 600;
        font-size: 0.9rem;
    }

    .status-badge.unlocked {
        background: #e8f5e8;
        color: #2d6e2d;
        border: 1px solid #81c784;
    }

    .status-badge.locked {
        background: #f5f5f5;
        color: #666;
        border: 1px solid #ccc;
    }

    .achievement-description {
        font-size: 1rem;
        line-height: 1.5;
        color: #333;
        margin-bottom: 1rem;
    }

    .user-context {
        background: #f8f9fa;
        padding: 1rem;
        border-radius: 8px;
        margin-top: 1rem;
        border-left: 4px solid #ff69b4;
    }

    .user-context p {
        margin: 0;
        color: #555;
    }

    @keyframes modal-appear {
        from {
            opacity: 0;
            transform: scale(0.9);
        }
        to {
            opacity: 1;
            transform: scale(1);
        }
    }

    @media (max-width: 768px) {
        .modal-content {
            margin: 1rem;
            max-height: 90vh;
        }

        .modal-header h3 {
            font-size: 1.1rem;
        }

        .close-button {
            font-size: 1.3rem;
        }
    }
</style>