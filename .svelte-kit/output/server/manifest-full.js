export const manifest = (() => {
function __memo(fn) {
	let value;
	return () => value ??= (value = fn());
}

return {
	appDir: "_app",
	appPath: "_app",
	assets: new Set(["favicon.png","svelte.svg","tauri.svg","vite.svg"]),
	mimeTypes: {".png":"image/png",".svg":"image/svg+xml"},
	_: {
		client: {start:"_app/immutable/entry/start.DuNRQcqr.js",app:"_app/immutable/entry/app.M4UtiOG-.js",imports:["_app/immutable/entry/start.DuNRQcqr.js","_app/immutable/chunks/DYjwGeWp.js","_app/immutable/chunks/jVRCAlOD.js","_app/immutable/chunks/BitGGxQW.js","_app/immutable/chunks/CKAymY6x.js","_app/immutable/chunks/D7i6tuYm.js","_app/immutable/entry/app.M4UtiOG-.js","_app/immutable/chunks/BitGGxQW.js","_app/immutable/chunks/CKAymY6x.js","_app/immutable/chunks/Bzak7iHL.js","_app/immutable/chunks/jVRCAlOD.js","_app/immutable/chunks/D7i6tuYm.js","_app/immutable/chunks/Cb4_0wpw.js","_app/immutable/chunks/CqPrI6_m.js"],stylesheets:[],fonts:[],uses_env_dynamic_public:false},
		nodes: [
			__memo(() => import('./nodes/0.js')),
			__memo(() => import('./nodes/1.js')),
			__memo(() => import('./nodes/2.js')),
			__memo(() => import('./nodes/3.js')),
			__memo(() => import('./nodes/4.js'))
		],
		remotes: {
			
		},
		routes: [
			{
				id: "/",
				pattern: /^\/$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 2 },
				endpoint: null
			},
			{
				id: "/history",
				pattern: /^\/history\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 3 },
				endpoint: null
			},
			{
				id: "/settings",
				pattern: /^\/settings\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 4 },
				endpoint: null
			}
		],
		prerendered_routes: new Set([]),
		matchers: async () => {
			
			return {  };
		},
		server_assets: {}
	}
}
})();
