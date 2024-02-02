

export const index = 2;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_page.svelte.js')).default;
export const imports = ["_app/immutable/nodes/2.8BVlLC87.js","_app/immutable/chunks/scheduler.UPLnIPFV.js","_app/immutable/chunks/index.ImbyZqG5.js"];
export const stylesheets = ["_app/immutable/assets/2.lAW3ojlO.css"];
export const fonts = ["_app/immutable/assets/onest-cyrillic-wght-normal.okE7jKFK.woff2","_app/immutable/assets/onest-latin-ext-wght-normal.NATBPiDw.woff2","_app/immutable/assets/onest-latin-wght-normal.ycwkluYs.woff2"];
