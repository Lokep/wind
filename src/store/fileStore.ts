import { readDir, readFile } from '@/api/command';
import { defineStore } from 'pinia';

interface IDir {
  name: string;
  path: string;
  is_dir: boolean;
  is_file: boolean;
}

export const useFileStore = defineStore('fileStore', {
  state: () => ({
    directories: [] as IDir[],
    activePath: '',
    contentCache: {

    } as { [key: string]: any }
  }),

  actions: {
    async getDirectories() {
      this.directories = await readDir(
        "/Users/lokep/Desktop/project/github/leetcode/"
      ) as any
    },

    async getFileContent(filePath: string, isFile: boolean) {

      if (isFile) {
        this.contentCache[filePath] = await readFile(filePath) as any
        this.activePath = filePath
        
      }
    },
  }
})