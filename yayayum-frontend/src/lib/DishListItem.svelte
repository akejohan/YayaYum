<script lang="ts">
    import type { Dish } from './api/models/Dish';
  
    export let dish: Dish;
    export let rating: number = 0; // 0–5
  </script>
  
  <div class="dish-item">
    <div class="dish-info">
      <div class="dish-header">
        <h3 class="dish-name">{dish.nr} {dish.name}</h3>
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
    <div class="rating">
      {#each Array(5) as _, i}
        <span class="star" class:filled={i < rating}>★</span>
      {/each}
      {#if rating > 0}
        <span class="rating-number">({rating})</span>
      {/if}
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
  
    .rating {
      flex-shrink: 0;
      display: flex;
      align-items: center;
      gap: 2px;
      margin-top: 0.2rem;
    }

    .star {
      font-size: 1.2rem;
      color: #ddd;
      transition: color 0.2s ease;
    }

    .star.filled {
      color: #ffd700;
    }

    .rating-number {
      font-size: 0.85rem;
      color: #666;
      margin-left: 0.3rem;
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
        margin-bottom: 0.5rem;
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
    }
  </style>
  