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

  function handleViewLeaderboard() {
    currentScreen.update(() => Component.Leaderboard);
  }

  function handleChangeUser() {
    selectedUser.set(null);
    currentScreen.set(Component.UserSelection);
  }
</script>

<h2>
  Välkommen, 
  <button 
    class="username-button"
    on:click={handleChangeUser}
    title="Klicka för att byta användare"
  >
    {$selectedUser?.username}!
  </button>
</h2>

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
  <div 
    class="card leaderboard" 
    on:click={handleViewLeaderboard}
    on:keydown={(e) => e.key === 'Enter' && handleViewLeaderboard()}
    role="button"
    tabindex="0"
  >
    🏆 Leaderboard
  </div>
</div>

<style>
  h2 {
    text-align: center;
    color: #ff69b4;
    margin-bottom: 2rem;
    text-shadow: 1px 1px 2px #fff;
  }

  .username-button {
    background: none;
    border: none;
    color: inherit;
    font-size: inherit;
    font-weight: inherit;
    text-shadow: inherit;
    cursor: pointer;
    padding: 0.2rem 0.5rem;
    border-radius: 8px;
    transition: all 0.2s ease;
    text-decoration: underline;
    text-decoration-style: dotted;
    text-underline-offset: 3px;
  }

  .username-button:hover {
    background: rgba(255, 105, 180, 0.1);
    transform: scale(1.05);
    text-decoration-style: solid;
  }

  .username-button:active {
    transform: scale(0.98);
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

  .card.leaderboard {
    background: linear-gradient(135deg, #ffd700, #ffb347);
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
