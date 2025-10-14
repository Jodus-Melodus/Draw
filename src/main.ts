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


  // timeline
  const template = document.getElementById('timeline-template') as HTMLTemplateElement;
  const projectWindow = document.getElementById('project-window') as HTMLElement;

  const secondsPerGrid = 2;    // grid every 2 seconds
  const pixelsPerSecond = 50;  // 1 second = 50px

  function createGrid() {
    if (!template || !projectWindow) return;

    // clear old grids
    projectWindow.innerHTML = '';

    const totalWidth = projectWindow.offsetWidth;
    const interval = secondsPerGrid * pixelsPerSecond;
    const totalGrids = Math.floor(totalWidth / interval);

    for (let i = 0; i <= totalGrids; i++) {
      const clone = template.content.cloneNode(true);
      projectWindow.appendChild(clone);
    }
  }
  createGrid();
}
init();