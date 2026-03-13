export type Pages =
  | { name: "RecentProjects"; meta: {} }
  | {
      name: "DefaultWorkspace";
      meta: {
        projectPath: string;
      };
    }
  | { name: "CreateProject"; meta: {} };

export * from "./resources";
