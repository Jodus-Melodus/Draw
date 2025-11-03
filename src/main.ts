import { listen } from "@tauri-apps/api/event";
import { addEmptyTrack, updateTrackList } from "./backend/tracks";
import { loadTheme } from "./backend/theme";

async function init() {
  updateTrackList();

  const addTrackButton = document.querySelector(".add-track") as HTMLElement;
  addTrackButton.addEventListener("click", () => {
    addEmptyTrack();
    updateTrackList();
  });

  await listen("updated-track-list", (_) => {
    updateTrackList();
  });
}

window.addEventListener("DOMContentLoaded", async () => {
  const savedTheme = localStorage.getItem("theme") || "dark";
  loadTheme(savedTheme);
})

window.addEventListener("storage", event => {
  if (event.key === "theme" && event.newValue) {
    loadTheme(event.newValue);
  }
});


init();

// TODO fix button onclick