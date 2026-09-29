import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { RebootClientProvider } from "@reboot-dev/reboot-react";
import App from "./App";
import "./styles.css";

const url = (import.meta.env.VITE_REBOOT_URL as string | undefined) ?? window.location.origin;
createRoot(document.getElementById("root")!).render(<StrictMode><RebootClientProvider url={url}><App /></RebootClientProvider></StrictMode>);
