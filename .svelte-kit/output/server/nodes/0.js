

export const index = 0;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/fallbacks/layout.svelte.js')).default;
export const universal = {
  "ssr": false
};
export const universal_id = "src/routes/+layout.js";
export const imports = ["_app/immutable/nodes/0.BA9odH4S.js","_app/immutable/chunks/Bzak7iHL.js","_app/immutable/chunks/BitGGxQW.js","_app/immutable/chunks/D7i6tuYm.js"];
export const stylesheets = [];
export const fonts = [];
