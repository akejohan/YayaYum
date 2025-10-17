<script lang="ts">
    import type { Dish } from './api/models/Dish';
    import type { Rating } from './api/models/Rating';
    import { selectedUser } from './shared';
  
    export let dish: Dish;
    export let allRatings: Rating[] = []; // All ratings for this dish from all users
    export let currentUserEatenCount: number = 0; // How many times current user has eaten this dish
    
    // Calculate overall statistics from all ratings
    $: dishRatings = allRatings.filter(rating => rating.dish_id === dish.id);
    $: averageRating = dishRatings.length > 0 
        ? dishRatings.reduce((sum, rating) => sum + rating.rating, 0) / dishRatings.length 
        : 0;
    $: totalReviews = dishRatings.length;
    
    // Get star display for average rating
    function getStarDisplay(rating: number): {filled: number, half: boolean} {
        const filled = Math.floor(rating);
        const half = rating - filled >= 0.5;
        return { filled, half };
    }
    
    $: starDisplay = getStarDisplay(averageRating);
</script>
  
<div class="dish-item">
    <div class="dish-info">
        <div class="dish-header">
            <h3 class="dish-name">#{dish.nr} {dish.name}</h3>
            <span class="dish-price">{dish.price_kr} kr</span>
        </div>
        <p class="dish-description">{dish.description}</p>
        {#if dish.dietary_restrictions.length > 0}
            <div class="dietary-info">
                {#each dish.dietary_restrictions as restriction}
                    <span class="dietary-tag">{restriction}</span>
                {/each}
            </div>
        {/if}
    </div>
    <div class="dish-stats">
        <div class="overall-rating">
            <div class="stars">
                {#each Array(5) as _, i}
                    <span class="star" class:filled={i < starDisplay.filled} class:half={i === starDisplay.filled && starDisplay.half}>
                        {#if i === starDisplay.filled && starDisplay.half}
                            ⯨
                        {:else}
                            ★
                        {/if}
                    </span>
                {/each}
            </div>
            <div class="rating-summary">
                {#if totalReviews > 0}
                    <span class="rating-number">{averageRating.toFixed(1)}</span>
                    <span class="review-count">({totalReviews} {totalReviews === 1 ? 'recension' : 'recensioner'})</span>
                {:else}
                    <span class="no-reviews">Ingen recension än</span>
                {/if}
            </div>
        </div>
        
        <div class="personal-stats">
            <div class="eaten-count">
                <span class="eaten-icon">🍽️</span>
                <span class="count-number">{currentUserEatenCount}</span>
                {#if currentUserEatenCount > 0}
                    <span class="count-label">gånger</span>
                {:else}
                    <span class="count-label not-eaten">ej prövad</span>
                {/if}
            </div>
            <!-- {#if $selectedUser}
                <div class="personal-indicator">
                    <span class="user-icon">👤</span>
                    <span class="user-label">Du</span>
                </div>
            {/if} -->
        </div>
    </div>
</div>
  
<style>
    .dish-item {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        background: #fffdfb;
        border-radius: 12px;
        padding: 1rem;
        margin: 0.5rem 0;
        box-shadow: 0 2px 8px rgba(0,0,0,0.08);
        transition: all 0.2s ease;
        border: 1px solid rgba(0,0,0,0.05);
    }
  
    .dish-item:hover {
        background: #fff7f2;
        transform: translateY(-2px);
        box-shadow: 0 4px 12px rgba(0,0,0,0.12);
    }

    .dish-info {
        flex: 1;
        margin-right: 1rem;
    }

    .dish-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 0.5rem;
    }
  
    .dish-name {
        margin: 0;
        font-size: 1.1rem;
        font-weight: 600;
        color: #ff7e6b;
    }

    .dish-price {
        font-size: 1rem;
        font-weight: bold;
        color: #2d7d32;
        background: rgba(45, 125, 50, 0.1);
        padding: 0.2rem 0.6rem;
        border-radius: 20px;
        white-space: nowrap;
    }

    .dish-description {
        margin: 0 0 0.5rem 0;
        font-size: 0.9rem;
        color: #666;
        line-height: 1.4;
    }

    .dietary-info {
        display: flex;
        flex-wrap: wrap;
        gap: 0.3rem;
    }

    .dietary-tag {
        font-size: 0.75rem;
        background: #e3f2fd;
        color: #1976d2;
        padding: 0.2rem 0.5rem;
        border-radius: 12px;
        border: 1px solid #bbdefb;
    }
  
    .dish-stats {
        flex-shrink: 0;
        display: flex;
        flex-direction: column;
        gap: 0.8rem;
        margin-top: 0.2rem;
        min-width: 140px;
    }

    .overall-rating {
        text-align: center;
        padding: 0.5rem;
        background: rgba(255, 215, 0, 0.05);
        border-radius: 8px;
        border: 1px solid rgba(255, 215, 0, 0.2);
    }

    .stars {
        display: flex;
        justify-content: center;
        gap: 1px;
        margin-bottom: 0.3rem;
    }

    .star {
        font-size: 1.1rem;
        color: #ddd;
        transition: color 0.2s ease;
    }

    .star.filled {
        color: #ffd700;
    }

    .star.half {
        color: #ffd700;
    }

    .rating-summary {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.1rem;
    }

    .rating-number {
        font-size: 0.9rem;
        font-weight: 600;
        color: #333;
    }

    .review-count {
        font-size: 0.75rem;
        color: #666;
    }

    .no-reviews {
        font-size: 0.75rem;
        color: #999;
        font-style: italic;
    }

    .personal-stats {
        display: flex;
        flex-direction: column;
        gap: 0.3rem;
    }

    .eaten-count {
        display: flex;
        align-items: center;
        gap: 0.3rem;
        font-size: 0.85rem;
        background: rgba(76, 175, 80, 0.1);
        padding: 0.3rem 0.6rem;
        border-radius: 16px;
        border: 1px solid rgba(76, 175, 80, 0.2);
    }

    .eaten-icon {
        font-size: 1rem;
    }

    .count-number {
        font-weight: 600;
        color: #2e7d32;
    }

    .count-label {
        color: #666;
        font-size: 0.75rem;
    }

    .count-label.not-eaten {
        color: #999;
        font-style: italic;
    }

    .personal-indicator {
        display: flex;
        align-items: center;
        gap: 0.3rem;
        justify-content: center;
        font-size: 0.75rem;
        color: #ff7e6b;
        background: rgba(255, 126, 107, 0.1);
        padding: 0.2rem 0.5rem;
        border-radius: 12px;
        border: 1px solid rgba(255, 126, 107, 0.2);
    }

    .user-icon {
        font-size: 0.8rem;
    }

    .user-label {
        font-weight: 500;
    }
  
    /* 📱 Mobilanpassning */
    @media (max-width: 500px) {
        .dish-item {
            padding: 0.8rem;
            flex-direction: column;
            align-items: stretch;
        }

        .dish-info {
            margin-right: 0;
            margin-bottom: 0.8rem;
        }

        .dish-header {
            flex-direction: column;
            align-items: flex-start;
            gap: 0.3rem;
        }
  
        .dish-name {
            font-size: 1rem;
        }

        .dish-price {
            font-size: 0.9rem;
        }

        .dish-stats {
            flex-direction: row;
            align-items: center;
            justify-content: space-between;
            min-width: auto;
            margin-top: 0;
            gap: 0.5rem;
        }

        .overall-rating {
            flex: 1;
            padding: 0.4rem;
        }

        .personal-stats {
            flex-direction: column;
            gap: 0.3rem;
            min-width: 100px;
        }

        .eaten-count {
            font-size: 0.8rem;
            padding: 0.2rem 0.5rem;
        }

        .personal-indicator {
            font-size: 0.7rem;
            padding: 0.15rem 0.4rem;
        }
    }
</style>
  