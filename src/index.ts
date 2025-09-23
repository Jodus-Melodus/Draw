import { getTrackList } from "./backend.js";

getTrackList().then(list => console.log(list.tracks));
