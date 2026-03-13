import { describe, it, expect } from "vitest";
import { sortedResources } from "./resources";
import type { Resource } from "./types";

const mockHttp = (id: string, name: string, created_at: number, updated_at?: number): Resource => ({
  id,
  name,
  type: "http",
  is_edited: false,
  folderId: null,
  created_at,
  updated_at,
  method: "GET",
  url: "https://example.com",
  params: [],
  headers: [],
  body: { type: "none" },
  auth: { type: "none" },
});

const mockFolder = (id: string, name: string, created_at: number, updated_at?: number): Resource => ({
  id,
  name,
  type: "folder",
  is_edited: false,
  folderId: null,
  created_at,
  updated_at,
  children: [],
});

const mockSequence = (id: string, name: string, created_at: number, updated_at?: number): Resource => ({
  id,
  name,
  type: "sequence",
  is_edited: false,
  folderId: null,
  created_at,
  updated_at,
  flow: [],
});

describe("sortedResources", () => {
  it("should always put folders at the top even with grouping: NONE", () => {
    const resources: Resource[] = [
      mockHttp("1", "Request A", 100),
      mockFolder("2", "Folder Z", 200),
      mockHttp("3", "Request B", 300),
    ];

    const sorted = sortedResources(resources, { sorting: "NAME", grouping: "NONE" });
    expect(sorted[0].type).toBe("folder");
    expect(sorted[0].name).toBe("Folder Z");
  });

  it("should group by TYPE: folders -> http -> sequence", () => {
    const resources: Resource[] = [
      mockSequence("1", "Seq A", 100),
      mockHttp("2", "Req A", 100),
      mockFolder("3", "Fold A", 100),
    ];

    const sorted = sortedResources(resources, { sorting: "NAME", grouping: "TYPE" });
    expect(sorted[0].type).toBe("folder");
    expect(sorted[1].type).toBe("http");
    expect(sorted[2].type).toBe("sequence");
  });

  it("should sort by NAME alphabetically (natural sort)", () => {
    const resources: Resource[] = [
      mockHttp("1", "Request 10", 100),
      mockHttp("2", "Request 2", 100),
      mockHttp("3", "Request 1", 100),
    ];

    const sorted = sortedResources(resources, { sorting: "NAME", grouping: "NONE" });
    expect(sorted[0].name).toBe("Request 1");
    expect(sorted[1].name).toBe("Request 2");
    expect(sorted[2].name).toBe("Request 10");
  });

  it("should sort by DATE_CREATED (newest first)", () => {
    const resources: Resource[] = [
      mockHttp("1", "Old", 100),
      mockHttp("2", "New", 300),
      mockHttp("3", "Mid", 200),
    ];

    const sorted = sortedResources(resources, { sorting: "DATE_CREATED", grouping: "NONE" });
    expect(sorted[0].name).toBe("New");
    expect(sorted[1].name).toBe("Mid");
    expect(sorted[2].name).toBe("Old");
  });

  it("should sort by DATE_UPDATED (newest first, fallback to created_at)", () => {
    const resources: Resource[] = [
      mockHttp("1", "Updated Long Ago", 100, 150),
      mockHttp("2", "Just Created", 400),
      mockHttp("3", "Updated Recently", 200, 500),
    ];

    const sorted = sortedResources(resources, { sorting: "DATE_UPDATED", grouping: "NONE" });
    expect(sorted[0].name).toBe("Updated Recently");
    expect(sorted[1].name).toBe("Just Created");
    expect(sorted[2].name).toBe("Updated Long Ago");
  });

  it("should maintain grouping while sorting by criteria within groups", () => {
    const resources: Resource[] = [
      mockHttp("1", "B Request", 100),
      mockFolder("2", "B Folder", 100),
      mockHttp("3", "A Request", 100),
      mockFolder("4", "A Folder", 100),
    ];

    const sorted = sortedResources(resources, { sorting: "NAME", grouping: "TYPE" });
    expect(sorted[0].name).toBe("A Folder");
    expect(sorted[1].name).toBe("B Folder");
    expect(sorted[2].name).toBe("A Request");
    expect(sorted[3].name).toBe("B Request");
  });
});
