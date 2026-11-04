import { useQuery } from "@tanstack/react-query";
import { commands } from "../bindings";
import { queryKeys } from "./queryKeys";

export function useAppInfo() {
  return useQuery({
    queryKey: queryKeys.appInfo,
    queryFn: commands.appInfo,
  });
}
