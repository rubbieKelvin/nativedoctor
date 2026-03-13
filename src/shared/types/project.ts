/** Configuration schema for a nativedoctor project (nativedoctor.json). */
export interface NativedoctorJson {
  name: string;
  description?: string;
  metadata?: Record<string, unknown>;
  envSources?: Array<{ name: string; path: string }>;
}
