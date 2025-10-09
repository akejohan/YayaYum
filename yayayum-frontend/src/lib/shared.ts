import { writable } from "svelte/store";
import type { User } from "./api/models/User";
import type { Writable } from "svelte/store";
import { Component, Page } from "./types";

export let selectedUser: Writable<User | null> = writable(null);
export let currentScreen: Writable<Component> = writable(Component.UserSelection);
export let currentPage: Writable<Page> = writable(Page.Main);