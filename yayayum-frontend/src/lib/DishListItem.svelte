<script lang="ts">
    import type { Dish } from './api/models/Dish';
  
    export let dish: Dish;
    export let rating: number = 0; // 0–5
  </script>
  
  <div class="dish-item">
    <h3>{dish.id} {dish.name}</h3>
    <div class="rating">
      <!-- {#each Array(5) as _, i}
        <span class="star" class:filled={i < rating}>★</span>
      {/each} -->
    </div>
  </div>
  
  <style>
    .dish-item {
      display: flex;
      justify-content: space-between;
      align-items: center;
      background: #fffdfb;
      border-radius: 10px;
      padding: 0.6rem 1rem;
      margin: 0.4rem 0;
      box-shadow: 0 1px 3px rgba(0,0,0,0.08);
      transition: background 0.2s ease;
    }
  
    .dish-item:hover {
      background: #fff7f2;
    }
  
    h3 {
      margin: 0;
      font-size: 1rem;
      color: #ff7e6b;
      flex: 1;
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
    }
  
    .rating {
      flex-shrink: 0;
      display: flex;
      gap: 2px;
    }
  
    .star {
      color: #e0e0e0;
      font-size: 1rem;
    }
  
    .filled {
      color: #ffd36b;
    }
  
    /* 📱 Mobilanpassning */
    @media (max-width: 500px) {
      .dish-item {
        padding: 0.5rem 0.75rem;
      }
  
      h3 {
        font-size: 0.95rem;
      }
  
      .star {
        font-size: 0.9rem;
      }
    }
  </style>
  