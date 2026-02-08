import { writable } from 'svelte/store';

type NotificationType = 'info' | 'warning' | 'error' | 'success';

interface Notification {
	id?: number;
	type: NotificationType;
	message: string;
}

let id = 0;

export const notifications = writable<Notification[]>([]);

export function addNotification({
	type = 'info',
	message = '',
	duration = 3000
}: Notification & { duration: number }) {
	const newNotification = { id: id++, type, message };
	notifications.update((n) => [...n, newNotification]);

	if (duration > 0) {
		setTimeout(() => {
			removeNotification(newNotification.id);
		}, duration);
	}
	return newNotification.id;
}

export function removeNotification(id: number) {
	notifications.update((n) => n.filter((notif) => notif.id !== id));
}

export const notify = {
	info: (msg: string, duration: number) =>
		addNotification({ type: 'info', message: msg, duration }),
	success: (msg: string, duration: number) =>
		addNotification({ type: 'success', message: msg, duration }),
	warning: (msg: string, duration: number) =>
		addNotification({ type: 'warning', message: msg, duration }),
	error: (msg: string, duration: number) =>
		addNotification({ type: 'error', message: msg, duration })
};

export type { Notification, NotificationType };
