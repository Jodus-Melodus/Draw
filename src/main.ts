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

  await listen("audio-samples", (_) => {
    // handle audio samples
    // display wave form or gain
    console.log("Received audio");
  });
}

window.addEventListener("DOMContentLoaded", async () => {
  loadTheme("dark");
})

init();