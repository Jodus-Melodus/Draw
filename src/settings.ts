import { loadTheme } from "./backend/theme";

let currentTheme = localStorage.getItem("theme") || "dark";
loadTheme(currentTheme);

document.getElementById("toggle-theme")?.addEventListener("click", () => {
    currentTheme = currentTheme === "dark" ? "light" : "dark";
    localStorage.setItem("theme", currentTheme);
    loadTheme(currentTheme);
});
