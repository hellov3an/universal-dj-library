export interface Playlist {
  id: string;
  name: string;
  parent_id?: string;
  is_folder: boolean;
  is_smart: boolean;
  smart_rules?: string; // JSON string of SmartRules
  sort_order: number;
  track_count: number;
  date_created: string;
  date_modified: string;
  
  // For building trees on the frontend
  children?: Playlist[];
}

export interface SmartCondition {
  field: string;
  operator: 'equals' | 'contains' | 'starts_with' | 'ends_with' | 'greater_than' | 'less_than';
  value: string | number;
}

export interface SmartRules {
  match_all: boolean;
  conditions: SmartCondition[];
}
