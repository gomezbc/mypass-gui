

export const index = 1;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/fallbacks/error.svelte.js')).default;
export const imports = ["_app/immutable/nodes/1.0uPQnUol.js","_app/immutable/chunks/scheduler.UPLnIPFV.js","_app/immutable/chunks/index.ImbyZqG5.js","_app/immutable/chunks/entry.b56YFRjc.js"];
export const stylesheets = [];
export const fonts = [];
