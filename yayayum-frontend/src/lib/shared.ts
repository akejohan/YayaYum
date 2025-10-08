import { writable } from "svelte/store";
import type { User } from "./api/models/User";
import type { Writable } from "svelte/store";
import { AppScreen } from "./types";

export let selectedUser: Writable<User | null> = writable(null);
export let currentScreen: Writable<AppScreen> = writable(AppScreen.UserSelection);