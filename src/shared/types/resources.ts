import type { HttpMethodType } from "@/shared/constants/http";

export const RESOURCE_TYPES = {
  FOLDER: "folder",
  HTTP: "http",
  SEQUENCE: "sequence",
} as const;

export type ResourceType = (typeof RESOURCE_TYPES)[keyof typeof RESOURCE_TYPES];

interface BaseResource<T extends ResourceType> {
  type: T;
  id: string;
  name: string;
  is_edited: boolean;
  folder_id: string | null;
  created_at: number;
  updated_at?: number;
}

export interface FolderResource extends BaseResource<"folder"> {
  children: Resource[];
}

export interface KeyValuePair {
  key: string;
  value: string;
  enabled?: boolean;
  description?: string;
}

export const BODY_TYPES = {
  NONE: "none",
  JSON: "json",
  TEXT: "text",
  FORM_DATA: "form-data",
  FORM_URLENCODED: "x-www-form-urlencoded",
  BINARY: "binary",
  GRAPHQL: "graphql",
} as const;

export type BodyType = (typeof BODY_TYPES)[keyof typeof BODY_TYPES];

export interface HttpBodyNone {
  type: "none";
}

export interface HttpBodyText {
  type: "text" | "json" | "graphql";
  content: string;
}

export interface HttpBodyFormData {
  type: "form-data";
  fields: Array<KeyValuePair & { isFile?: boolean }>;
}

export interface HttpBodyFormUrlencoded {
  type: "x-www-form-urlencoded";
  fields: KeyValuePair[];
}

export interface HttpBodyBinary {
  type: "binary";
  file_path?: string;
}

export type HttpBody =
  | HttpBodyNone
  | HttpBodyText
  | HttpBodyFormData
  | HttpBodyFormUrlencoded
  | HttpBodyBinary;

export const AUTH_TYPES = {
  NONE: "none",
  BASIC: "basic",
  BEARER: "bearer",
  API_KEY: "api-key",
} as const;

export type AuthType = (typeof AUTH_TYPES)[keyof typeof AUTH_TYPES];

export interface AuthNone {
  type: "none";
}

export interface AuthBasic {
  type: "basic";
  username: string;
  password: string;
}

export interface AuthBearer {
  type: "bearer";
  token: string;
  prefix?: string;
}

export interface AuthApiKey {
  type: "api-key";
  key: string;
  value: string;
  add_to: "header" | "query";
}

export type HttpAuth = AuthNone | AuthBasic | AuthBearer | AuthApiKey;

export interface HttpResource extends BaseResource<"http"> {
  $schema?: string;
  method: HttpMethodType;
  url: string;
  params: KeyValuePair[];
  headers: KeyValuePair[];
  body: HttpBody;
  auth: HttpAuth;
}

export interface SequenceNode {
  id: string;
  resource_id: string;
  resource_type: Exclude<ResourceType, "folder">;
}

export interface SequenceResource extends BaseResource<"sequence"> {
  $schema?: string;
  flow: SequenceNode[];
}

export type Resource = FolderResource | HttpResource | SequenceResource;

// File based on rust types

/** Backend JSON shape for HTTP resource (from read_resource_file or to write_resource_file). */
export type HttpResourceFileDto = Omit<HttpResource, "is_edited">;

/** Backend JSON shape for sequence resource. */
export type SequenceResourceFileDto = Omit<SequenceResource, "is_edited">;

/** Backend response from read_resource_file (tagged union). */
export type ResourceFileContentDto =
  | HttpResourceFileDto
  | SequenceResourceFileDto;
