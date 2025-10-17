import { writable } from "svelte/store";
import type { User } from "./api/models/User";
import type { Writable } from "svelte/store";
import { Component, Page } from "./types";

// Helper function to get cached user from localStorage
function getCachedUser(): User | null {
  if (typeof window === 'undefined') return null;
  
  try {
    const cached = localStorage.getItem('yayayum-selected-user');
    return cached ? JSON.parse(cached) : null;
  } catch {
    return null;
  }
}

// Helper function to cache user to localStorage
function setCachedUser(user: User | null) {
  if (typeof window === 'undefined') return;
  
  try {
    if (user) {
      localStorage.setItem('yayayum-selected-user', JSON.stringify(user));
    } else {
      localStorage.removeItem('yayayum-selected-user');
    }
  } catch {
    // Ignore localStorage errors
  }
}

// Initialize with cached user if available
const initialUser = getCachedUser();
const initialScreen = initialUser ? Component.MealActions : Component.UserSelection;

// Create the stores
function createUserStore() {
  const { subscribe, set, update } = writable<User | null>(initialUser);

  return {
    subscribe,
    set: (user: User | null) => {
      setCachedUser(user);
      set(user);
    },
    update: (updater: (value: User | null) => User | null) => {
      update((user) => {
        const newUser = updater(user);
        setCachedUser(newUser);
        return newUser;
      });
    }
  };
}

export const selectedUser = createUserStore();
export let currentScreen: Writable<Component> = writable(initialScreen);
export let currentPage: Writable<Page> = writable(Page.Main);