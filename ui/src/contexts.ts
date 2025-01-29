import { type AppClient } from "@holochain/client";

export const clientContext = "AppClient";

export type ClientContext = {
  getClient: () => Promise<AppClient>;
};
