import type { Profile, ProfileInput } from "./types/profile";
import { invoke } from "@tauri-apps/api/core";
import { DEV } from "./utils";

class AppState {
  profiles = $state<Profile[]>([]);
  loading = $state(false);

  async addProfile(profile: ProfileInput) {
    try {
      await invoke("add_profile", { profile });
      if (DEV) {
        console.log("added profile", profile);
      }

      await this.fetchProfiles();
    } catch (err) {
      console.error("Failed to save profile:", err);
      throw err;
    }
  }

  async fetchProfiles() {
    this.loading = true;
    try {
      this.profiles = await invoke("get_profiles");
    } catch (err) {
      console.error("Failed to fetch profiles:", err);
      throw err;
    } finally {
      this.loading = false;
    }
  }
}

export const appState = new AppState();
