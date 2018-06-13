export interface OnDropEventType {
    dataTransfer: DataTransfer,
    preventDefault: () => void,
};

export class DirData {
    readonly children: Map<string, File | DirData>;

    constructor(children: Map<string, File | DirData>) {
        this.children = children;
    }

    getMap() {
        return this.children;
    }
}

interface FileSystemEntry {
    readonly file: (resolve: ((file: File) => void)) => void,
    createReader: () => {
        readonly  readEntries: (resolve: ((list: Iterator<FileSystemEntry>) => void)) => void,
    },
    readonly isDirectory: boolean,
    readonly isFile: boolean,
    readonly name: string,
};

export const convertDropEvent = (event: OnDropEventType): Promise<DirData> => {
    const entryList: Array<FileSystemEntry> = [];

    //for (const item of event.dataTransfer.items) {
    const { items } = event.dataTransfer;
    for (let index = 0; index < items.length; index++) {
        const item = items[index];
        const entry = item.webkitGetAsEntry();
        entryList.push(entry);
    }

    return convertFileSystemEntryList(entryList);
};


function convertFileSystemEntry(item: FileSystemEntry): Promise<File | DirData> {
    return new Promise(
        (resolve: ((file: File | DirData) => void), reject: ((error: any) => void)) => {
            if (item.isFile) {
                item.file(
                    (file: File) => {
                        resolve(file);
                    }
                );
                return;
            }

            if (item.isDirectory) {
                item.createReader().readEntries((entries: Iterator<FileSystemEntry>) => {
                    const entriesArray = [];

                    //@ts-ignore TODO
                    for (const entriesItem of entries) {
                        entriesArray.push(entriesItem);
                    }

                    convertFileSystemEntryList(entriesArray).then((dir: DirData) => {
                        resolve(dir);
                    }).catch((error: any) => {
                        reject(error);
                    });
                });
                return;
            }

            reject(Error('Utils/DropFile:convertFileSystemEntry -> Problem with item processing'));
        }
    );
}

function convertFileSystemEntryList(items: Array<FileSystemEntry>): Promise<DirData> {
    const outMap = new Map();
    const outPromises = [];

    for (const item of items) {
        const name = item.name;
        outPromises.push(new Promise((resolve: (() => void), reject: ((error: any) => void)) => {
            return convertFileSystemEntry(item).then((result: File | DirData) => {
                if (outMap.has(name)) {
                    reject(Error(`Utils/DropFile:convertFileSystemEntryList -> Double name: ${name}`));
                } else {
                    outMap.set(name, result);
                    resolve();
                }
            });
        }));
    }

    return Promise.all(outPromises).then((): DirData => new DirData(outMap));
}

/*
https://github.com/facebook/flow/blob/v0.70.0/lib/dom.js#L63
https://www.meziantou.net/2017/09/04/upload-files-and-directories-using-an-input-drag-and-drop-or-copy-and-paste-with
https://stackoverflow.com/questions/3590058/does-html5-allow-drag-drop-upload-of-folders-or-a-folder-tree
*/