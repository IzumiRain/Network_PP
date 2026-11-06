// Thin typed wrapper around Tauri invoke. In a plain browser (npm run dev
// without the Tauri shell) commands fail gracefully with a clear message
// instead of throwing opaque errors.

type TauriGlobal = {
  invoke: (cmd: string, args?: Record<string, unknown>) => Promise<unknown>;
};

function bridge(): TauriGlobal | null {
  const w = window as unknown as { __TAURI__?: TauriGlobal };
  return w.__TAURI__ ?? null;
}

export const inTauri = (): boolean => bridge() !== null;

export async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  const t = bridge();
  if (!t) {
    throw new Error(
      "Tauri backend is not available. Run the app with `npm run tauri dev` to use live tools."
    );
  }
  return t.invoke(cmd, args) as Promise<T>;
}

/** Native file picker for pack imports (desktop only, null in browser). */
export async function pickZipFile(): Promise<string | null> {
  if (!inTauri()) return null;
  const { open } = await import("@tauri-apps/plugin-dialog");
  const picked = await open({
    multiple: false,
    filters: [{ name: "Network++ pack", extensions: ["zip"] }],
  });
  return typeof picked === "string" ? picked : null;
}
