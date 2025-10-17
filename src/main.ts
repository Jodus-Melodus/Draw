import { listen } from "@tauri-apps/api/event";
import { addEmptyTrack, addNewTrack, getTrackList } from "./backend/tracks";

var trackList;

export async function updateTrackList() {
  const channelTrackContainer = document.getElementById("mix-console");
  const trackContainer = document.getElementById("track-list");
  const channelTrackTemplate = document.getElementById("channel-track-template") as HTMLTemplateElement;
  const trackTemplate = document.getElementById("track-template") as HTMLTemplateElement;

  if (channelTrackContainer && channelTrackTemplate && trackContainer && trackTemplate) {
    channelTrackContainer.replaceChildren();
    trackContainer.replaceChildren();
    trackList = await getTrackList();

    trackList.tracks.forEach(track => {
      addNewTrack(trackTemplate, channelTrackTemplate, track, trackContainer, channelTrackContainer);
    });
  }
}

async function init() {
  updateTrackList();

  const addTrackButton = document.querySelector(".add-track") as HTMLElement;
  addTrackButton.addEventListener("click", () => {
    addEmptyTrack();
    updateTrackList();
  });

  await listen("updated-track-list", (_) => {
    updateTrackList();
  })

  await listen("audio-samples", (_) => {
    // handle audio samples
    // display wave form or gain
    console.log("Received audio");
  })
}
init();