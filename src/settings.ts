import { loadTheme, THEMES } from "./backend/theme";

// Read persisted theme index, fall back to 0 when missing or invalid.
let currentTheme = Number(localStorage.getItem("theme") ?? "0");
if (!Number.isInteger(currentTheme) || THEMES.length === 0) currentTheme = 0;

// Ensure initial theme is in range and applied.
currentTheme = ((currentTheme % THEMES.length) + THEMES.length) % THEMES.length;
loadTheme(currentTheme);

document.getElementById("toggle-theme")?.addEventListener("click", () => {
    console.log("Cycling")
    // Cycle forward and wrap using the number of available themes.
    currentTheme = (currentTheme + 1) % THEMES.length;
    localStorage.setItem("theme", currentTheme.toString());
    loadTheme(currentTheme);
});
