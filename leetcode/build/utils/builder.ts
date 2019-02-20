import chokidar from 'chokidar';
import { writeFileSync } from 'fs';
import { ensureDirSync } from 'fs-extra';
import { parse, resolve } from 'path';
import includeProcessor from '../processor/include-processor';
import typescriptProcessor from '../processor/typescript-processor';

import FilePack from './file-pack';
import getEntry from './get-entry';

export default class Builder {
  readonly outdir: string;
  constructor(readonly root: string) {
    this.outdir = resolve(root, '../.leetcode');
  }
  build() {
    const entry = getEntry(this.root);
    for (const [, path] of entry.entries()) {
      this.transform(path);
    }
  }
  watch() {
    console.log(`builder was watch: ${this.root}`);

    const handleFileChange = (path: string) => {
      if (parse(path).ext === '.ts') {
        console.log(`find file change: ${path}`);
        this.transform(path);
      }
    };
    const watcher = chokidar.watch(this.root, {
      ignoreInitial: true
    });

    watcher.on('add', handleFileChange);
    watcher.on('change', handleFileChange);
    return watcher;
  }
  transform(path: string) {
    const pack = FilePack.read(path);
    includeProcessor(pack);
    typescriptProcessor(pack, this);
    ensureDirSync(this.outdir);
    writeFileSync(
      resolve(this.outdir, parse(pack.path).name + '.js'),
      pack.content
    );
  }
}
