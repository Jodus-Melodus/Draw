import { addEmptyTrack, getTrackList } from "./backend/tracks";
import { TrackInfo } from "./backend/types";

var trackList;

async function updateTrackList() {
  const channelTrackContainer = document.getElementById("mix-console");
  const trackContainer = document.getElementById("track-list");
  const channelTrackTemplate = document.getElementById("channel-track-template") as HTMLTemplateElement;
  const trackTemplate = document.getElementById("track-template") as HTMLTemplateElement;

  if (channelTrackContainer && channelTrackTemplate && trackContainer && trackTemplate) {
    channelTrackContainer.replaceChildren();
    trackContainer.replaceChildren();
    trackList = await getTrackList();

    trackList.tracks.forEach(track => {
      const newChannelTrack = channelTrackTemplate.content.cloneNode(true) as DocumentFragment;
      const newTrack = trackTemplate.content.cloneNode(true) as DocumentFragment;

      addNewChannelTrack(newChannelTrack, track, channelTrackContainer);
      addNewTrack(newTrack, track, trackContainer);
    });
  }
}

function addNewTrack(newTrack: DocumentFragment, track: TrackInfo, trackContainer: HTMLElement) {
  const trackName = newTrack.querySelector(".track-name") as HTMLElement;
  const trackMuteButton = newTrack.querySelector(".track-mute") as HTMLElement;
  const trackSoloButton = newTrack.querySelector(".track-solo") as HTMLElement;
  const trackRecordButton = newTrack.querySelector(".track-record") as HTMLElement;
  const trackMonitorButton = newTrack.querySelector(".track-monitor") as HTMLElement;

  trackName.textContent = track.name;

  trackMuteButton.addEventListener("click", () => {
    console.log("click");
    if (trackMuteButton.classList.contains("active")) {
      trackMuteButton.classList.remove("active");
    } else {
      trackMuteButton.classList.add("active");
    }
  });

  trackSoloButton.addEventListener("click", () => {
    console.log("click");
    if (trackSoloButton.classList.contains("active")) {
      trackSoloButton.classList.remove("active");
    } else {
      trackSoloButton.classList.add("active");
    }
  });

  trackRecordButton.addEventListener("click", () => {
    console.log("click");
    if (trackRecordButton.classList.contains("active")) {
      trackRecordButton.classList.remove("active");
    } else {
      trackRecordButton.classList.add("active");
    }
  });

  trackMonitorButton.addEventListener("click", () => {
    console.log("click");
    if (trackMonitorButton.classList.contains("active")) {
      trackMonitorButton.classList.remove("active");
    } else {
      trackMonitorButton.classList.add("active");
    }
  });

  trackContainer.appendChild(newTrack);
}

function addNewChannelTrack(newTrack: DocumentFragment, track: TrackInfo, trackContainer: HTMLElement) {
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