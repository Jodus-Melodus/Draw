import { getTrackList } from "./backend/tracks";

var trackList;

async function updateTrackList() {
  const trackContainer = document.getElementById("mix-console");
  const trackTemplate = document.getElementById("track-template") as HTMLTemplateElement;

  if (trackContainer && trackTemplate) {
    // clear track containers children
    trackContainer.replaceChildren();
    const clone = trackTemplate.content.cloneNode(true) as DocumentFragment;
    trackList = await getTrackList();

    trackList.tracks.forEach(track => {
      // Populate template
      (clone.querySelector(".meterR") as HTMLElement).textContent = ""; // FIXME null when add track
      (clone.querySelector(".meterL") as HTMLElement).textContent = "";
      (clone.querySelector(".metergain") as HTMLElement).textContent = "";
      (clone.querySelector(".channel-mute") as HTMLElement).textContent = "";
      (clone.querySelector(".channel-solo") as HTMLElement).textContent = track.solo ? "true" : "false";
      (clone.querySelector(".channel-pan") as HTMLElement).textContent = track.pan.toPrecision(2);
      (clone.querySelector(".fadergain") as HTMLElement).textContent = track.gain.toPrecision(2);
      (clone.querySelector(".channel-name") as HTMLElement).textContent = track.name;

      // Add behavior
      // TODO get buttons with query selectors
      // TODO update other buttons

      // Example
      // (clone.querySelector(".channel-mute") as HTMLElement).addEventListener("click", () => {});

      trackContainer.appendChild(clone);
    });
  }
}


async function init() { }

setInterval(() => {
  updateTrackList();
}, 1000);

init();