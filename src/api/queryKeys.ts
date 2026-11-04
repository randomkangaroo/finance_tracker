// Every query key in the app, in one place. Mutations invalidate by these keys,
// so keep related data under a shared prefix (e.g. `["accounts", ...]`).

export const queryKeys = {
  appInfo: ["appInfo"] as const,
};
