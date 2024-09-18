// src/lib/api.ts
import { invoke } from "@tauri-apps/api/core";
import type { InvokeArgs } from "@tauri-apps/api/core";
import { ZodSchema } from "zod";

export async function invokeWithValidation<T>(
  command: string,
  schema: ZodSchema<T>,
  params?: InvokeArgs
): Promise<T> {
  const result = await invoke(command, params);
  return schema.parse(result);
}
