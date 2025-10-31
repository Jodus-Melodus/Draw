import { Theme } from "./types";

export async function loadTheme(mode: "light" | "dark") {
    const response = await fetch(`themes/${mode}.json`);
    const theme = await response.json();
    applyTheme(theme);
}

export function applyTheme(theme: Theme) {
    document.body.style.backgroundColor = theme.background;
    document.body.style.color = theme.text;

    const buttons = document.querySelectorAll<HTMLButtonElement>("button");
    buttons.forEach(button => {
        button.style.backgroundColor = theme.primary;
        button.style.color = theme.text;
    })
}