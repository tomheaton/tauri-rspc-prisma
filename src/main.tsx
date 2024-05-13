import Index from "@/routes";
import "@/styles/globals.css";
import type { Procedures } from "@/types/bindings";
import { createClient } from "@rspc/client";
import { createReactQueryHooks } from "@rspc/react";
import { TauriTransport } from "@rspc/tauri";
import { QueryClient } from "@tanstack/react-query";
import ReactDOM from "react-dom/client";
import {
  Route,
  RouterProvider,
  createBrowserRouter,
  createRoutesFromElements,
} from "react-router-dom";

const router = createBrowserRouter(
  createRoutesFromElements(
    <Route path="/" element={<Index />}>
      {/* Add more routes here. */}
    </Route>,
  ),
);

const client = createClient<Procedures>({
  transport: new TauriTransport(),
});
const queryClient = new QueryClient();

export const api = createReactQueryHooks<Procedures>();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <api.Provider client={client} queryClient={queryClient}>
    <RouterProvider router={router} />
  </api.Provider>,
);
