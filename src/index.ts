import { getTrackList } from "./rust.js";

getTrackList().then(list => console.log(list.tracks));
