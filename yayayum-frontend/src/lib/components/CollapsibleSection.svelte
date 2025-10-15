<script lang="ts">
    import { slide } from 'svelte/transition';
    
    export let title: string;
    export let isOpen: boolean = true;
    export let icon: string = "👥"; // Default icon

    function toggle() {
        isOpen = !isOpen;
    }
</script>

<div class="collapsible-section">
    <button class="section-header" class:open={isOpen} on:click={toggle}>
        <div class="header-content">
            <span class="icon">{icon}</span>
            <h3 class="title">{title}</h3>
            <span class="count-badge">
                <slot name="count"></slot>
            </span>
        </div>
        <svg 
            class="chevron" 
            class:rotated={isOpen} 
            width="20" 
            height="20" 
            viewBox="0 0 20 20" 
            fill="none" 
            xmlns="http://www.w3.org/2000/svg"
        >
            <path 
                d="M6 8L10 12L14 8" 
                stroke="currentColor" 
                stroke-width="2" 
                stroke-linecap="round" 
                stroke-linejoin="round"
            />
        </svg>
    </button>

    {#if isOpen}
        <div class="section-content" transition:slide={{ duration: 300 }}>
            <slot></slot>
        </div>
    {/if}
</div>

<style>
    .collapsible-section {
        margin-bottom: 24px;
        border: 1px solid #e2e8f0;
        border-radius: 12px;
        background: white;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.06);
        overflow: hidden;
    }

    .section-header {
        width: 100%;
        padding: 20px 24px;
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        border: none;
        cursor: pointer;
        display: flex;
        justify-content: space-between;
        align-items: center;
        transition: all 0.2s ease;
        color: white;
    }

    .section-header:hover {
        background: linear-gradient(135deg, #5a6fd8 0%, #6a4190 100%);
        transform: translateY(-1px);
        box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
    }

    .section-header.open {
        background: linear-gradient(135deg, #4c63d2 0%, #5a3c7e 100%);
    }

    .header-content {
        display: flex;
        align-items: center;
        gap: 16px;
        flex: 1;
    }

    .icon {
        font-size: 1.5em;
        filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.2));
    }

    .title {
        margin: 0;
        font-size: 1.25em;
        font-weight: 600;
        text-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
    }

    .count-badge {
        background: rgba(255, 255, 255, 0.2);
        color: white;
        padding: 4px 12px;
        border-radius: 12px;
        font-size: 0.9em;
        font-weight: 500;
        backdrop-filter: blur(10px);
        border: 1px solid rgba(255, 255, 255, 0.1);
    }

    .chevron {
        transition: transform 0.3s ease;
        opacity: 0.8;
    }

    .chevron.rotated {
        transform: rotate(180deg);
    }

    .section-content {
        padding: 24px;
        background: #fafbfc;
        border-top: 1px solid rgba(0, 0, 0, 0.06);
    }

    /* Animation imports */
    @import url('https://cdnjs.cloudflare.com/ajax/libs/animate.css/4.1.1/animate.min.css');
</style>