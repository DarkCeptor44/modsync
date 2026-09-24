export interface Profile {
  id: string;
  name: string;
  source: string;
  destination: string;
  exclusions: string[];
}

export type ProfileInput = Omit<Profile, "id">;
