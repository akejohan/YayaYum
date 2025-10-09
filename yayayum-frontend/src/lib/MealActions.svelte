<script lang="ts">
  import type { User } from "./api/models/User";
  import { currentScreen, selectedUser } from "./shared";
    import { Component } from "./types";

  function handleEat() {
    currentScreen.update(() => Component.MealInspiration);
  }

  function handleRate() {
    alert(`${$selectedUser?.username} wants to RATE their meal ⭐`);
  }
</script>

<h2>Välkommen, {$selectedUser?.username}!</h2>

<div class="card-container">
  <div class="card eat" on:click={handleEat}>
    Jag ska äta
  </div>
  <div class="card rate" on:click={handleRate}>
    Jag har ätit
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
  }

  .card.eat {
    background: linear-gradient(135deg, #ffb3b3, #e88b8b);
  }

  .card.rate {
    background: linear-gradient(135deg, #fcd86d, #e7dd98);
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
