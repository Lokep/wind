import { invoke } from "@tauri-apps/api/tauri";

export const greet = async (name: string) => {
  await invoke("greet", { name });
};


export const readDir = async (path: string) => {
  const result = await invoke("read_dir", { path });
  console.log('[result]: ', result)
  return result;
};


export const readFile = async (path: string) => {
  const result = await invoke("read_file", { path });
  console.log('[result]: ', result)
  return result;
};

export const writeFile = async (path: string, content: string) => {
  const result = await invoke("write_file", { path, content });
  console.log('[result]: ', result)
  return result;
};

export const createDir = async (path: string) => {
  const result = await invoke("create_dir", { path });
  console.log('[result]: ', result)
  return result;
};

export const createFile = async (path: string) => {
  const result = await invoke("create_file", { path });
  console.log('[result]: ', result)
  return result;
};


export const updateFile = async (path: string, content: string) => {
  const result = await invoke("update_file", { path, content });
  console.log('[result]: ', result)
  return result;
};

export const removeDir = async (path: string) => {
  const result = await invoke("del_dir", { path });
  console.log('[result]: ', result)
  return result;
};


export const removeFile = async (path: string) => {
  const result = await invoke("del_file", { path });
  console.log('[result]: ', result)
  return result;
};


