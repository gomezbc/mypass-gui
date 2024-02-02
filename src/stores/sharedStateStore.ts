import { writable } from 'svelte/store';
import { State } from '../enums/State';

export const sharedStateStore = writable(State.LOCK);
