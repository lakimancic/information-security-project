function getPath() {
	return location.hash.replace(/^#/, '') || '/';
}

export const route = $state({
	path: getPath()
});

export function navigate(path: string) {
	location.hash = path;
	route.path = path;
}

window.addEventListener('hashchange', () => {
	route.path = getPath();
});
