import React from "react";
import { createRoot } from "react-dom/client";
import { MainLayout } from "./layouts/MainLayout";
import Dashboard from "./pages/Dashboard";

createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <MainLayout><Dashboard /></MainLayout>
  </React.StrictMode>,
);
