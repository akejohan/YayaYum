<script lang="ts">
    import yayaLogo from "../../assets/yayalogo.png";
    import Users from "../../lib/Users.svelte";
    import { Component } from "../../lib/types";
    import MealActions from "../../lib/MealActions.svelte";
    import { selectedUser } from "../../lib/shared";
    import { currentScreen } from "../../lib/shared";
    import MealInspiration from "../../lib/MealInspiration.svelte";
    import RateMeal from "../../lib/RateMeal.svelte";
    import ManagePage from "../../lib/ManagePage.svelte";
</script>

<main>
    <div>
        <span>
            <img src={yayaLogo} class="logo" alt="Yayalogo" />
        </span>
    </div>
    <h1>YayaYum</h1>

    <div class="card">
        {#if $currentScreen === Component.MealActions}
            <MealActions />
        {:else if $currentScreen === Component.MealInspiration}
            <MealInspiration />
        {:else if $currentScreen === Component.RateMeal}
            <RateMeal />
        {:else if $currentScreen === Component.UserSelection}
            <Users />
        {/if}
    </div>

    {#if $selectedUser}
        <p>Idag är du: {$selectedUser?.username}</p>
    {:else}
        <p>Vem vill du vara idag?</p>

        <p class="click-your-name">Klicka på ditt namn</p>
    {/if}

    <div class="background">
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 800 600"
            width="100%"
            height="100%"
            preserveAspectRatio="xMidYMid slice"
        >
            <style>
                .bg {
                    fill: url(#grad);
                }
                @keyframes float {
                    0% {
                        transform: translateY(0px) rotate(0deg);
                    }
                    50% {
                        transform: translateY(-20px) rotate(3deg);
                    }
                    100% {
                        transform: translateY(0px) rotate(0deg);
                    }
                }
                @keyframes drift {
                    0% {
                        transform: translateY(0) translateX(0);
                        opacity: 0.8;
                    }
                    50% {
                        transform: translateY(-40px) translateX(10px);
                        opacity: 1;
                    }
                    100% {
                        transform: translateY(-80px) translateX(-10px);
                        opacity: 0.8;
                    }
                }
                .floaty {
                    animation: float 4s ease-in-out infinite;
                    transform-origin: center;
                }
                .bubble {
                    animation: drift 8s ease-in-out infinite;
                    opacity: 0.5;
                }
                .bubble:nth-child(1) {
                    animation-delay: 0s;
                }
                .bubble:nth-child(2) {
                    animation-delay: 2s;
                }
                .bubble:nth-child(3) {
                    animation-delay: 4s;
                }
                .bubble:nth-child(4) {
                    animation-delay: 6s;
                }
            </style>

            <!-- Bakgrundsgradient -->
            <defs>
                <linearGradient id="grad" x1="0" x2="0" y1="0" y2="1">
                    <stop offset="0%" stop-color="#ffff87" />
                    <stop offset="100%" stop-color="#ffd587" />
                </linearGradient>
            </defs>
            <rect width="100%" height="100%" class="bg" />

            <!-- Svävande figurer -->
            <g class="floaty" style="animation-duration: 6s;">
                <ellipse
                    cx="200"
                    cy="250"
                    rx="40"
                    ry="30"
                    fill="#fff4d9"
                    stroke="#f7c85d"
                    stroke-width="3"
                />
                <circle cx="190" cy="245" r="10" fill="#fcd86d" />
            </g>

            <g
                class="floaty"
                style="animation-duration: 7s; animation-delay: 1s;"
            >
                <polygon
                    points="600,220 620,270 580,270"
                    fill="#ffb3b3"
                    stroke="#e88b8b"
                    stroke-width="3"
                />
                <circle cx="600" cy="255" r="4" fill="#fff" />
            </g>

            <g
                class="floaty"
                style="animation-duration: 5.5s; animation-delay: 0.5s;"
            >
                <path
                    d="M400 350 Q420 330 440 350 Q420 370 400 350 Z"
                    fill="#ffdf80"
                    stroke="#f7c85d"
                    stroke-width="3"
                />
                <circle cx="410" cy="345" r="3" fill="#fff" />
            </g>

            <g
                class="floaty"
                style="animation-duration: 8s; animation-delay: 2s;"
            >
                <path
                    d="M300 180 Q310 150 330 180 Q310 210 300 180 Z"
                    fill="#ffb6c1"
                    stroke="#e69aa7"
                    stroke-width="3"
                />
                <circle cx="312" cy="175" r="2" fill="#fff" />
            </g>

            <g
                class="floaty"
                style="animation-duration: 6.5s; animation-delay: 3s;"
            >
                <polygon
                    points="480,120 490,140 470,140"
                    fill="#ffec8b"
                    stroke="#f2cf56"
                    stroke-width="3"
                />
            </g>

            <!-- Bubblor -->
            <circle class="bubble" cx="150" cy="500" r="12" fill="#ffd6e8" />
            <circle class="bubble" cx="400" cy="550" r="18" fill="#ffe4c2" />
            <circle class="bubble" cx="650" cy="520" r="10" fill="#ffdfef" />
            <circle class="bubble" cx="550" cy="580" r="14" fill="#fff2d1" />
        </svg>
    </div>
</main>

<style>
    .background {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        z-index: -1; /* bakom allt annat */
        overflow: hidden;
    }

    main {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: flex-start;
        padding: 1em;
    }

    .card {
        background-color: rgba(255, 255, 255, 0.8); /* semi-transparent card */
        border-radius: 16px;
        padding: 1em;
        margin-top: 1em;
        box-shadow: 0 8px 16px rgba(0, 0, 0, 0.1);
    }

    h1 {
        color: #ff69b4;
        text-shadow: 1px 1px 2px #fff;
    }
    .logo {
        height: 6em;
        padding: 1.5em;
        will-change: filter;
        transition: filter 300ms;
    }
    .logo:hover {
        filter: drop-shadow(0 0 2em #646cffaa);
    }
    .click-your-name {
        color: #888;
    }
</style>
