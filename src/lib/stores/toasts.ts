import { writable } from "svelte/store";

export type ToastType = "success" | "info" | "warning" | "error";

export type Toast = {
  id: number;
  message: string;
  type: ToastType;
  unique?: boolean;
};

export type ToastInput = Omit<Toast, "id">;

type toastStore = {
  toastLimit: number;
  duration: {
    success?: number;
    info?: number;
    warning?: number;
    error?: number;
  };
};

function toastStore({ toastLimit, duration }: toastStore) {
  const { subscribe, update } = writable<Toast[]>([]);

  return {
    subscribe,
    removeAtIndex: (index: number) =>
      update((toasts) =>
        ~index ? [...toasts.slice(0, index), ...toasts.slice(index + 1)] : toasts,
      ),
    push: (toast: ToastInput) => {
      const id = Math.random();

      let shouldExit = false;

      update((toasts) => {
        if (toast.unique && !!toasts.find(({ message }) => message === toast.message)) {
          shouldExit = true;
          return toasts;
        }

        return [
          {
            id,
            ...toast,
          },
          ...toasts,
        ].slice(0, toastLimit);
      });

      if (shouldExit) {
        return;
      }

      const remove = (toasts: Toast[]) => {
        const index = toasts.findIndex(({ id: toastId }) => toastId === id);

        if (!~index) {
          return toasts;
        }

        return [...toasts.slice(0, index), ...toasts.slice(index + 1)];
      };

      let removalTime;

      switch (toast.type) {
        case "success": {
          removalTime = duration.success;
          break;
        }

        case "info": {
          removalTime = duration.info;
          break;
        }

        case "warning": {
          removalTime = duration.warning;
          break;
        }

        case "error": {
          removalTime = duration.error;
          break;
        }
      }

      if (removalTime) {
        setTimeout(() => update(remove), removalTime);
      }
    },
  };
}

export const toasts = toastStore({
  duration: {
    info: 3000,
    success: 5000,
    warning: 10000,
    error: 10000,
  },
  toastLimit: 3,
});

export function pushError(error: unknown) {
  toasts.push({
    message: typeof error === "string" ? error : JSON.stringify(error),
    type: "error",
    unique: true,
  });
}
