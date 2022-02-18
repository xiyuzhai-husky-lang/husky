import type FigureProps from "./FigureProps";
import type { Writable } from "svelte/store";
import { writable } from "svelte/store";

export let figures: { [id: number]: FigureProps | null } = {};
export const current_figure: Writable<FigureProps | null> = writable(null);
