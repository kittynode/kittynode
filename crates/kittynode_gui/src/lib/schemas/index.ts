import { z } from "zod";

const packageInfoSchema = z.object({
  name: z.string(),
  version: z.string(),
});

const containerSchema = z.object({
  image: z.string(),
});

export const packageSchema = z.object({
  package: packageInfoSchema,
  containers: z.array(containerSchema),
});

export const getPackagesSchema = z.array(packageSchema);

export type PackageInfo = z.infer<typeof packageInfoSchema>;
export type Container = z.infer<typeof containerSchema>;
export type Package = z.infer<typeof packageSchema>;
export type GetPackages = z.infer<typeof getPackagesSchema>;
