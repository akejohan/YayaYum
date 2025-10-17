<script lang="ts">
    import { onMount } from "svelte";
    import { slide } from "svelte/transition";
    import { DishesService, type Dish, DishCategory, RatingsService, type Rating, UsersService, type User } from "./api";
    import DishListItem from "./DishListItem.svelte";
    import { selectedUser } from "./shared";

    let dishes: Dish[] = [];
    let allRatings: Rating[] = []; // All ratings from all users
    let userRatings: Rating[] = []; // Only current user's ratings
    let loading = true;
    let error: string | null = null;
    
    // Filter and sort state
    let selectedCategory: string = "all";
    let triedFilter: "all" | "tried" | "untried" = "all";
    let sortBy: "name" | "rating" | "category" | "times-eaten" = "category";
    let filtersExpanded: boolean = false;
    
    // Detailed view state
    let selectedDishForDetails: Dish | null = null;

    // Filter and sort dishes
    $: filteredDishes = dishes.filter(dish => {
        // Category filter
        if (selectedCategory !== "all" && dish.category !== selectedCategory) {
            return false;
        }
        
        // Tried/untried filter
        const eatenCount = getDishEatenCount(dish.id);
        if (triedFilter === "tried" && eatenCount === 0) {
            return false;
        }
        if (triedFilter === "untried" && eatenCount > 0) {
            return false;
        }
        
        return true;
    }).sort((a, b) => {
        switch (sortBy) {
            case "name":
                return a.name.localeCompare(b.name);
            case "rating":
                const ratingA = getDishRating(a.id);
                const ratingB = getDishRating(b.id);
                return ratingB - ratingA; // Highest first
            case "times-eaten":
                const eatenA = getDishEatenCount(a.id);
                const eatenB = getDishEatenCount(b.id);
                return eatenB - eatenA; // Most eaten first
            case "category":
            default:
                return a.category.localeCompare(b.category);
        }
    });

    // Group filtered dishes by category
    $: dishesByCategory = filteredDishes.reduce((groups, dish) => {
        const category = dish.category;
        if (!groups[category]) {
            groups[category] = [];
        }
        groups[category].push(dish);
        return groups;
    }, {} as Record<string, Dish[]>);

    // Get category display names
    function getCategoryDisplayName(category: string): string {
        switch (category) {
            case DishCategory.WOK_WITH_NOODLES: return "Wok med nudlar";
            case DishCategory.WOK_WITH_RICE: return "Wok med ris";
            case DishCategory.RAMEN: return "Ramen";
            case DishCategory.SPECIAL_DISH: return "Specialrätter";
            case DishCategory.STEW: return "Grytor";
            case DishCategory.KIDS_MENU: return "Barnmeny";
            case DishCategory.SIDE_ORDER: return "Tillbehör";
            default: return category;
        }
    }

    // Get user rating for a specific dish
    function getDishRating(dishId: number): number {
        const rating = userRatings.find(r => r.dish_id === dishId);
        return rating ? rating.rating : 0;
    }

    // Get how many times user has eaten a specific dish
    function getDishEatenCount(dishId: number): number {
        return userRatings.filter(r => r.dish_id === dishId).length;
    }

    // Get unique categories from dishes
    $: availableCategories = [...new Set(dishes.map(dish => dish.category))].sort();

    // Show dish details
    async function showDishDetails(dish: Dish) {
        selectedDishForDetails = dish;
    }

    // Close dish details
    function closeDishDetails() {
        selectedDishForDetails = null;
    }

    // Get all ratings for a specific dish with user details
    function getDishRatings(dishId: number) {
        return allRatings.filter(r => r.dish_id === dishId);
    }

    async function fetchData() {
        try {
            loading = true;
            
            // Always fetch dishes and all ratings
            const [dishesResponse, allRatingsResponse] = await Promise.all([
                DishesService.getDishes(),
                RatingsService.getRatings()
            ]);
            
            dishes = dishesResponse;
            allRatings = allRatingsResponse;
            
            // Fetch user-specific ratings if user is selected
            if ($selectedUser) {
                try {
                    userRatings = await RatingsService.getRatingsByUser($selectedUser.id);
                } catch (ratingsErr) {
                    // If user ratings fail, just continue without them
                    console.warn('Could not load user ratings:', ratingsErr);
                    userRatings = [];
                }
            } else {
                userRatings = [];
            }
        } catch (err) {
            error = (err as Error).message;
        } finally {
            loading = false;
        }
    }

    onMount(fetchData);

    // Refetch ratings when user changes
    $: if ($selectedUser) {
        fetchData();
    }
</script>

{#if loading}
    <p>Ringer yayamee...</p>
{:else if error}
    <p style="color: red;">Error: {error}</p>
{:else if dishes.length === 0}
    <p>No meals found.</p>
{:else}
    <!-- Filter Controls -->
    <div class="filter-container">
        <div class="filter-header">
            <button 
                class="filter-toggle"
                on:click={() => filtersExpanded = !filtersExpanded}
                aria-expanded={filtersExpanded}
            >
                <span class="filter-icon" class:expanded={filtersExpanded}>🔽</span>
                <span>Filters</span>
                <div class="results-info-compact">
                    {filteredDishes.length} av {dishes.length} rätter
                </div>
            </button>
        </div>
        
        {#if filtersExpanded}
            <div class="filters" transition:slide>
                <div class="filter-group">
                    <label for="category-filter">Kategori:</label>
                    <select id="category-filter" bind:value={selectedCategory}>
                        <option value="all">Alla kategorier</option>
                        {#each availableCategories as category}
                            <option value={category}>{getCategoryDisplayName(category)}</option>
                        {/each}
                    </select>
                </div>

                <div class="filter-group">
                    <label for="tried-filter">Status:</label>
                    <select id="tried-filter" bind:value={triedFilter}>
                        <option value="all">Alla rätter</option>
                        <option value="tried">Prövade</option>
                        <option value="untried">Oprövade</option>
                    </select>
                </div>

                <div class="filter-group">
                    <label for="sort-filter">Sortering:</label>
                    <select id="sort-filter" bind:value={sortBy}>
                        <option value="category">Kategori</option>
                        <option value="name">Namn</option>
                        <option value="rating">Bäst betyg</option>
                        <option value="times-eaten">Mest ätna</option>
                    </select>
                </div>
            </div>
        {/if}
    </div>
    <div class="dish-list">
        {#each Object.entries(dishesByCategory) as [category, categoryDishes] (category)}
            <div class="category-section">
                <h2 class="category-heading">{getCategoryDisplayName(category)}</h2>
                <div class="category-dishes">
                    {#each categoryDishes as dish (dish.id)}
                        <div 
                            class="dish-clickable"
                            on:click={() => showDishDetails(dish)}
                            on:keydown={(e) => e.key === 'Enter' && showDishDetails(dish)}
                            role="button"
                            tabindex="0"
                        >
                            <DishListItem 
                                {dish} 
                                allRatings={allRatings}
                                currentUserEatenCount={getDishEatenCount(dish.id)}
                            />
                        </div>
                    {/each}
                </div>
            </div>
        {/each}
        
        {#if Object.keys(dishesByCategory).length === 0}
            <div class="no-results">
                <p>🔍 Inga rätter matchar dina filter</p>
                <p class="suggestion">Prova att ändra kategori eller status för att hitta fler rätter</p>
            </div>
        {/if}
    </div>
{/if}

<!-- Detailed Dish View Modal -->
{#if selectedDishForDetails}
    <div 
        class="modal-overlay" 
        on:click={closeDishDetails}
        on:keydown={(e) => e.key === 'Escape' && closeDishDetails()}
        role="dialog"
        aria-modal="true"
        tabindex="-1"
    >
        <div class="modal-content">
            <div class="modal-header">
                <h2>{selectedDishForDetails.nr} {selectedDishForDetails.name}</h2>
                <button class="close-button" on:click={closeDishDetails} aria-label="Stäng">✕</button>
            </div>
            
            <div class="modal-body">
                <div class="dish-overview">
                    <p class="dish-price-large">{selectedDishForDetails.price_kr} kr</p>
                    <p class="dish-description-large">{selectedDishForDetails.description}</p>
                    
                    {#if selectedDishForDetails.dietary_restrictions.length > 0}
                        <div class="dietary-info-large">
                            {#each selectedDishForDetails.dietary_restrictions as restriction}
                                <span class="dietary-tag-large">{restriction}</span>
                            {/each}
                        </div>
                    {/if}
                </div>

                <div class="ratings-history">
                    <h3>Alla recensioner ({getDishRatings(selectedDishForDetails.id).length})</h3>
                    
                    {#if getDishRatings(selectedDishForDetails.id).length > 0}
                        <div class="ratings-list">
                            {#each getDishRatings(selectedDishForDetails.id) as rating}
                                <div class="rating-card">
                                    <div class="rating-header">
                                        <div class="rating-stars">
                                            {#each Array(5) as _, i}
                                                <span class="star" class:filled={i < rating.rating}>★</span>
                                            {/each}
                                        </div>
                                        <div class="rating-date">
                                            {new Date(rating.date).toLocaleDateString('sv-SE', {
                                                year: 'numeric',
                                                month: 'short',
                                                day: 'numeric',
                                                hour: '2-digit',
                                                minute: '2-digit'
                                            })}
                                        </div>
                                    </div>
                                    {#if rating.description}
                                        <p class="rating-comment">"{rating.description}"</p>
                                    {/if}
                                    <!-- Add user indicator if this is the current user's rating -->
                                    {#if $selectedUser && rating.user_id === $selectedUser.id}
                                        <div class="current-user-indicator">
                                            <span class="user-badge">Din recension</span>
                                        </div>
                                    {/if}
                                </div>
                            {/each}
                        </div>
                    {:else}
                        <p class="no-ratings">Ingen har recenserat den här rätten än. Bli den första!</p>
                    {/if}
                </div>
            </div>
        </div>
    </div>
{/if}

<style>
    .filter-container {
        max-width: 700px;
        margin: 0 auto 2rem auto;
        padding: 0 1rem;
    }

    .filter-header {
        margin-bottom: 0;
    }

    .filter-toggle {
        width: 100%;
        padding: 1rem;
        background: rgba(255, 255, 255, 0.9);
        border: none;
        border-radius: 12px;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        cursor: pointer;
        display: flex;
        align-items: center;
        gap: 0.8rem;
        font-size: 1rem;
        font-weight: 600;
        color: #333;
        transition: all 0.2s ease;
    }

    .filter-toggle:hover {
        background: rgba(255, 255, 255, 1);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    }

    .filter-icon {
        font-size: 0.8rem;
        transition: transform 0.2s ease;
    }

    .filter-icon.expanded {
        transform: rotate(180deg);
    }

    .results-info-compact {
        margin-left: auto;
        font-size: 0.85rem;
        color: #666;
        background: rgba(255, 126, 107, 0.1);
        padding: 0.3rem 0.8rem;
        border-radius: 20px;
        font-weight: 500;
    }

    .filters {
        padding: 1rem;
        background: rgba(255, 255, 255, 0.95);
        border-radius: 0 0 12px 12px;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        display: flex;
        flex-wrap: wrap;
        gap: 1rem;
        align-items: center;
        justify-content: space-between;
        margin-top: -2px;
    }

    .filter-group {
        display: flex;
        flex-direction: column;
        gap: 0.3rem;
        min-width: 140px;
    }

    .filter-group label {
        font-size: 0.85rem;
        font-weight: 600;
        color: #ff7e6b;
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }

    .filter-group select {
        padding: 0.5rem 0.8rem;
        border: 2px solid #e0e0e0;
        border-radius: 8px;
        background: white;
        font-size: 0.9rem;
        color: #333;
        cursor: pointer;
        transition: all 0.2s ease;
    }

    .filter-group select:focus {
        outline: none;
        border-color: #ff7e6b;
        box-shadow: 0 0 0 3px rgba(255, 126, 107, 0.1);
    }

    .filter-group select:hover {
        border-color: #ff9985;
    }

    .dish-clickable {
        cursor: pointer;
        border-radius: 12px;
        transition: all 0.2s ease;
    }

    .dish-clickable:hover {
        transform: translateY(-2px);
        box-shadow: 0 6px 20px rgba(0, 0, 0, 0.15);
    }

    .dish-clickable:focus {
        outline: 3px solid rgba(255, 126, 107, 0.3);
        outline-offset: 2px;
    }

    .dish-list {
        max-width: 700px;
        margin: 0 auto;
        padding: 0 1rem 2rem 1rem;
    }

    .category-section {
        margin-bottom: 2.5rem;
    }

    .category-heading {
        font-size: 1.5rem;
        font-weight: 700;
        color: #ff7e6b;
        margin: 0 0 1rem 0;
        padding: 0.5rem 0;
        border-bottom: 3px solid #ff7e6b;
        text-align: center;
        position: relative;
        background: linear-gradient(135deg, #ff7e6b, #ff9985);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
    }

    .category-heading:after {
        content: '';
        position: absolute;
        bottom: -3px;
        left: 50%;
        transform: translateX(-50%);
        width: 60px;
        height: 3px;
        background: linear-gradient(135deg, #ff7e6b, #ff9985);
        border-radius: 2px;
    }

    .category-dishes {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .no-results {
        text-align: center;
        padding: 3rem 1rem;
        color: #666;
    }

    .no-results p {
        margin: 0 0 0.5rem 0;
        font-size: 1.1rem;
    }

    .no-results .suggestion {
        font-size: 0.9rem;
        color: #999;
        font-style: italic;
    }

    /* Modal styles */
    .modal-overlay {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.7);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
    }

    .modal-content {
        background: white;
        border-radius: 16px;
        max-width: 600px;
        width: 100%;
        max-height: 90vh;
        overflow-y: auto;
        box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
        animation: modalAppear 0.2s ease-out;
    }

    @keyframes modalAppear {
        from {
            opacity: 0;
            transform: scale(0.9) translateY(20px);
        }
        to {
            opacity: 1;
            transform: scale(1) translateY(0);
        }
    }

    .modal-header {
        padding: 1.5rem;
        border-bottom: 1px solid #eee;
        display: flex;
        justify-content: space-between;
        align-items: center;
        background: linear-gradient(135deg, #ff7e6b, #ff9985);
        border-radius: 16px 16px 0 0;
        color: white;
    }

    .modal-header h2 {
        margin: 0;
        font-size: 1.4rem;
        font-weight: 600;
    }

    .close-button {
        background: none;
        border: none;
        font-size: 1.5rem;
        color: white;
        cursor: pointer;
        padding: 0.5rem;
        border-radius: 50%;
        transition: background 0.2s ease;
    }

    .close-button:hover {
        background: rgba(255, 255, 255, 0.2);
    }

    .modal-body {
        padding: 1.5rem;
    }

    .dish-overview {
        margin-bottom: 2rem;
    }

    .dish-price-large {
        font-size: 1.5rem;
        font-weight: bold;
        color: #2d7d32;
        margin: 0 0 1rem 0;
    }

    .dish-description-large {
        font-size: 1.1rem;
        color: #666;
        line-height: 1.5;
        margin: 0 0 1rem 0;
    }

    .dietary-info-large {
        display: flex;
        flex-wrap: wrap;
        gap: 0.5rem;
    }

    .dietary-tag-large {
        font-size: 0.85rem;
        background: #e3f2fd;
        color: #1976d2;
        padding: 0.3rem 0.8rem;
        border-radius: 16px;
        border: 1px solid #bbdefb;
    }

    .ratings-history h3 {
        color: #ff7e6b;
        margin: 0 0 1rem 0;
        font-size: 1.2rem;
    }

    .ratings-list {
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .rating-card {
        background: #f8f9fa;
        padding: 1rem;
        border-radius: 12px;
        border-left: 4px solid #ffd700;
    }

    .rating-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 0.5rem;
    }

    .rating-stars .star {
        font-size: 1.1rem;
        color: #ddd;
    }

    .rating-stars .star.filled {
        color: #ffd700;
    }

    .rating-date {
        font-size: 0.85rem;
        color: #666;
    }

    .rating-comment {
        margin: 0;
        font-style: italic;
        color: #555;
        line-height: 1.4;
    }

    .current-user-indicator {
        margin-top: 0.5rem;
        display: flex;
        justify-content: flex-end;
    }

    .user-badge {
        background: linear-gradient(135deg, #ff7e6b, #ff9985);
        color: white;
        padding: 0.2rem 0.6rem;
        border-radius: 12px;
        font-size: 0.75rem;
        font-weight: 500;
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }

    .no-ratings {
        text-align: center;
        color: #999;
        font-style: italic;
        padding: 2rem;
    }

    /* Mobile responsive */
    @media (max-width: 768px) {
        .filters {
            flex-direction: column;
            align-items: stretch;
            gap: 1rem;
        }

        .filter-group {
            min-width: auto;
        }

        .modal-content {
            margin: 0.5rem;
            max-height: 95vh;
        }

        .modal-header {
            padding: 1rem;
        }

        .modal-header h2 {
            font-size: 1.2rem;
        }

        .modal-body {
            padding: 1rem;
        }
    }

    @media (max-width: 500px) {
        .filter-container {
            padding: 0 0.5rem;
        }

        .filters {
            padding: 1rem 0.8rem;
        }

        .category-heading {
            font-size: 1.3rem;
        }
        
        .dish-list {
            padding: 0 0.5rem 2rem 0.5rem;
            min-width: 17em;
        }

        .filter-group select {
            padding: 0.6rem;
            font-size: 0.9rem;
        }

        .results-info-compact {
            display: none;
        }

        .dish-price-large {
            font-size: 1.3rem;
        }

        .ratings-list {
            gap: 0.8rem;
        }

        .rating-card {
            padding: 0.8rem;
        }

        .rating-header {
            flex-direction: column;
            align-items: flex-start;
            gap: 0.3rem;
        }
    }
</style>
