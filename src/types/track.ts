export type AudioFormat = 
  | "Mp3" 
  | "Flac" 
  | "Wav" 
  | "Aac" 
  | "Ogg" 
  | "Aiff" 
  | "Alac" 
  | "Wma" 
  | "Unknown";

export interface FileIdentity {
  hash: string | null;
  fingerprint: string | null;
  duration_ms: number;
  file_size: number;
  metadata_signature: string | null;
}

export interface TrackMetadata {
  title: string | null;
  artist: string | null;
  album: string | null;
  album_artist: string | null;
  genre: string | null;
  year: number | null;
  label: string | null;
  composer: string | null;
  comment: string | null;
  grouping: string | null;
  track_number: number | null;
  disc_number: number | null;
  duration_ms: number;
  bitrate: number | null;
  sample_rate: number | null;
  format: AudioFormat;
  file_size: number;
}

export interface CuePoint {
  id: string;
  position_ms: number;
  cue_type: string;
  name?: string;
  color?: string;
  hot_cue_index?: number;
}

export interface Loop {
  id: string;
  start_ms: number;
  length_ms: number;
  name?: string;
  color?: string;
}

export interface BeatgridMarker {
  position_ms: number;
  bpm: number;
}

export interface Beatgrid {
  markers: BeatgridMarker[];
}

export interface Track {
  id: string;
  file_identity: FileIdentity;
  file_path: string;
  relative_path?: string;
  filename: string;
  metadata: TrackMetadata;
  cue_points: CuePoint[];
  loops: Loop[];
  beatgrid?: Beatgrid;
  tags: string[];
  color?: string;
  rating: number;
  bpm?: number;
  musical_key?: string;
  energy?: number;
  date_added: string;
  date_modified: string;
  date_analyzed?: string;
}
