import { addEmptyTrack, getTrackList } from "./backend/tracks";
import { TrackInfo } from "./backend/types";

var trackList;

async function updateTrackList() {
  const trackContainer = document.getElementById("mix-console");
  const trackTemplate = document.getElementById("track-template") as HTMLTemplateElement;

  if (trackContainer && trackTemplate) {
    trackContainer.replaceChildren();
    trackList = await getTrackList();
    
    trackList.tracks.forEach(track => {
      const newTrack = trackTemplate.content.cloneNode(true) as DocumentFragment;
      addNewTrack(newTrack, track, trackContainer);
    });
  }
}

function addNewTrack(newTrack: DocumentFragment, track: TrackInfo, trackContainer: HTMLElement) {
  const channelMuteButton = newTrack.querySelector(".channel-mute") as HTMLElement;
  const channelSoloButton = newTrack.querySelector(".channel-solo") as HTMLElement;
  const channelRecordButton = newTrack.querySelector(".channel-record") as HTMLElement;
  const channelMonitorButton = newTrack.querySelector(".channel-monitor") as HTMLElement;
  const channelName = newTrack.querySelector(".channel-name") as HTMLElement;

  channelName.textContent = track.name;

  channelMuteButton.addEventListener("click", () => {
    if (channelMuteButton.classList.contains("active")) {
      channelMuteButton.classList.remove("active");
    } else {
      channelMuteButton.classList.add("active");
    }
  });

  channelSoloButton.addEventListener("click", () => {
    if (channelSoloButton.classList.contains("active")) {
      channelSoloButton.classList.remove("active");
    } else {
      channelSoloButton.classList.add("active");
    }
  });

  channelRecordButton.addEventListener("click", () => {
    if (channelRecordButton.classList.contains("active")) {
      channelRecordButton.classList.remove("active");
    } else {
      channelRecordButton.classList.add("active");
    }
  });

  channelMonitorButton.addEventListener("click", () => {
    if (channelMonitorButton.classList.contains("active")) {
      channelMonitorButton.classList.remove("active");
    } else {
      channelMonitorButton.classList.add("active");
    }
  });
  
  trackContainer.appendChild(newTrack);
}

async function init() {
  updateTrackList();

  const addTrackButton = document.querySelector(".add-track") as HTMLElement;
  addTrackButton.addEventListener("click", () => {
    addEmptyTrack();
    updateTrackList();
  });
}

init();