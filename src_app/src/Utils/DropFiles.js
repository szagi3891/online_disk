//@flow
/* globals Iterator */

export type OnDropEventType = {|
    dataTransfer: DataTransfer,
    preventDefault: () => void,
|};

export type DirType = Map<string, File | DirType>;

type FileSystemEntry = {
    +file: (resolve: ((file: File) => void)) => void,
    +createReader: () => {
        +readEntries: (resolve: ((list: Iterator<FileSystemEntry>) => void)) => void,
    },
    +isDirectory: bool,
    +isFile: bool,
    +name: string,
};

export const convertDropEvent = (event: OnDropEventType): Promise<DirType> => {
    const entryList: Array<FileSystemEntry> = [];

    for (const item of event.dataTransfer.items) {
        //$FlowFixMe
        const entry = item.webkitGetAsEntry();
        entryList.push(entry);
    }

    return convertFileSystemEntryList(entryList);
};


function convertFileSystemEntry(item: FileSystemEntry): Promise<File | DirType> {
    return new Promise(
        (resolve: ((file: File | DirType) => void), reject: ((error: mixed) => void)) => {
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

                    for (const entriesItem of entries) {
                        entriesArray.push(entriesItem);
                    }

                    convertFileSystemEntryList(entriesArray).then((dir: DirType) => {
                        resolve(dir);
                    }).catch((error: mixed) => {
                        reject(error);
                    });
                });
                return;
            }

            reject(Error('Utils/DropFile:convertFileSystemEntry -> Problem with item processing'));
        }
    );
}

function convertFileSystemEntryList(items: Array<FileSystemEntry>): Promise<DirType> {
    const outMap = new Map();
    const outPromises = [];

    for (const item of items) {
        const name = item.name;
        outPromises.push(new Promise((resolve: (() => void), reject: ((error: mixed) => void)) => {
            return convertFileSystemEntry(item).then((result: File | DirType) => {
                if (outMap.has(name)) {
                    reject(Error(`Utils/DropFile:convertFileSystemEntryList -> Double name: ${name}`));
                } else {
                    outMap.set(name, result);
                    resolve();
                }
            });
        }));
    }

    return Promise.all(outPromises).then((): DirType => outMap);
}

/*
https://github.com/facebook/flow/blob/v0.70.0/lib/dom.js#L63
https://www.meziantou.net/2017/09/04/upload-files-and-directories-using-an-input-drag-and-drop-or-copy-and-paste-with
https://stackoverflow.com/questions/3590058/does-html5-allow-drag-drop-upload-of-folders-or-a-folder-tree
*/