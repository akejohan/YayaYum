<script lang="ts">
  import type { User } from "./api/models/User";
  import { currentScreen, selectedUser } from "./shared";
    import { Component } from "./types";

  function handleEat() {
    currentScreen.update(() => Component.MealInspiration);
  }

  function handleRate() {
    currentScreen.update(() => Component.RateMeal);
  }

  function handleViewRatings() {
    currentScreen.update(() => Component.MyRatings);
  }
</script>

<h2>Välkommen, {$selectedUser?.username}!</h2>

<div class="card-container">
  <div 
    class="card eat" 
    on:click={handleEat}
    on:keydown={(e) => e.key === 'Enter' && handleEat()}
    role="button"
    tabindex="0"
  >
    Jag ska äta
  </div>
  <div 
    class="card rate" 
    on:click={handleRate}
    on:keydown={(e) => e.key === 'Enter' && handleRate()}
    role="button"
    tabindex="0"
  >
    Jag har ätit
  </div>
  <div 
    class="card history" 
    on:click={handleViewRatings}
    on:keydown={(e) => e.key === 'Enter' && handleViewRatings()}
    role="button"
    tabindex="0"
  >
    Recensioner
  </div>
</div>

<style>
  h2 {
    text-align: center;
    color: #ff69b4;
    margin-bottom: 2rem;
    text-shadow: 1px 1px 2px #fff;
  }

  .card-container {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: 2rem;
  }

  .card {
    justify-content: center;
    border-radius: 24px;
    cursor: pointer;
    transition: transform 0.2s, box-shadow 0.2s;
    color: white;
    text-align: center;
    box-shadow: 0 4px 10px rgba(0,0,0,0.15);
    padding: 2rem 1.5rem;
    font-size: 1.1rem;
    font-weight: 600;
    min-height: 100px;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    overflow: hidden;
  }

  .card:focus {
    outline: 3px solid #007acc;
    outline-offset: 2px;
  }

  .card.eat {
    background: linear-gradient(135deg, #ffb3b3, #e88b8b);
  }

  .card.rate {
    background: linear-gradient(135deg, #fcd86d, #e7dd98);
  }

  .card.history {
    background: linear-gradient(135deg, #87ceeb, #5f9ea0);
  }

  .card:hover {
    transform: translateY(-5px);
    box-shadow: 0 8px 20px rgba(0,0,0,0.2);
  }

  @media (max-width: 500px) {
    .card-container {
      flex-direction: column;
      gap: 1rem;
    }

    .card {
      max-width: none;
    }
  }
</style>
