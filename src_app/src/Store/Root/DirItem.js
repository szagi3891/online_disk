//@flow
import { action, computed } from 'mobx';
import { List as IList, OrderedMap } from 'immutable';
import { BlobStore } from '../Blob/BlobStore';
import type { CurrentHead, NodeItemType } from '../Type';
import { FileItem } from './FileItem';
import { HeadStore } from '../HeadStore';

const addDir = (node_hash: string, path: IList<string>, dir: string): Promise<CurrentHead> => {
    const param = {
        node_hash,
        path: path.toArray(),
        dir
    };

    const fetchParam = {
        method: 'POST',
        body: JSON.stringify(param)
    };

    return fetch('/api/add_dir', fetchParam).then(response => response.json());
};

const addEmptyFile = (node_hash: string, path: IList<string>, file_name: string): Promise<CurrentHead> => {
    const param = {
        node_hash,
        path: path.toArray(),
        file_name
    };
    const fetchParam = {
        method: 'POST',
        body: JSON.stringify(param)
    };

    return fetch('/api/add_empty_file', fetchParam)
        .then(response => response.json());
};

/*
const appendBuffer = (buffer1: ArrayBuffer, buffer2: ArrayBuffer): ArrayBuffer => {
    const tmp = new Uint8Array(buffer1.byteLength + buffer2.byteLength);
    tmp.set(new Uint8Array(buffer1), 0);
    tmp.set(new Uint8Array(buffer2), buffer1.byteLength);
    return tmp.buffer;
};

const convertFile = (file: File): Promise<ArrayBuffer> => new Promise((resolve) => {
    const reader = new FileReader();

    reader.addEventListener('load', function (e) {
    resolve(this.result);
    //e.target.result
    }, false);

    reader.readAsArrayBuffer(file);
});
*/

/*
var b1 = new Uint8Array([0x01, 0x02, 0x03]);
var b2 = new Uint8Array([0x04, 0x05, 0x06]);
var b3 = new Uint8Array([0x07, 0x08, 0x09]);

// combine all three arrays into a new array buffer
// if you need the ArrayBuffer instead of a TypedArray, it's at `combined.buffer
// NOTE: square brackets in the Uint8Array constructor -- Uint8Array([...])
var combined = new Uint8Array([
    ...b1,
    ...b2,
    ...b3
]);
*/

/*
type BodyInit = string | URLSearchParams | FormData | Blob | ArrayBuffer | $ArrayBufferView;
declare function fetch(input: RequestInfo, init?: RequestOptions): Promise<Response>;
*/

/*
    //https://stackoverflow.com/questions/6965107/converting-between-strings-and-arraybuffers
const stringToArrayBuffer = (text: string): ArrayBuffer => {
    const enc = new TextEncoder(); // always utf-8
    return enc.encode(text).buffer;
};
*/

//https://github.com/react-dropzone/react-dropzone/blob/master/src/index.js

//TODO - fetch przyjmuje arrayBuffer

export class DirItem {
    +_parent: DirItem | null;
    +_head: HeadStore;
    +_blob: BlobStore;
    +_hash: string;
    +_path: IList<string>;
    +_name: string;

    constructor(parent: DirItem | null, head: HeadStore, blob: BlobStore, hash: string, path: IList<string>, name: string) {
        this._parent = parent;
        this._head = head;
        this._blob = blob;
        this._hash = hash;
        this._path = path;
        this._name = name;
    }

    get path(): IList<string> {
        return this._path;
    }

    get name(): string {
        return this._name;
    }

    @computed get _listNodes(): OrderedMap<string, NodeItemType> | null {
        return this._blob.getDir(this._hash, this._path);
    }

    @action addDir(dir: string): Promise<void> {
        return addDir(this._hash, this._path, dir).then((response: CurrentHead) => {
            this._head.saveHead(response);
        });
    }

    @action addEmptyFile(fileName: string): Promise<void> {
        return addEmptyFile(this._hash, this._path, fileName).then((response: CurrentHead) => {
            this._head.saveHead(response);
        });
    }

    @computed get childList(): OrderedMap<string, DirItem | FileItem> | null {
        const listNodes = this._listNodes;

        if (listNodes !== null) {
            return listNodes.map(
                (item: NodeItemType, name: string): DirItem | FileItem => this._mapNode(name, item)
            );
        }

        return null;
    }

    _mapNode(name: string, item: NodeItemType): DirItem | FileItem {
        if (item.is_dir) {
            return new DirItem(
                this,
                this._head,
                this._blob,
                item.hash,
                this._path.push(name),
                name
            );
        }
        return new FileItem(
            this._head,
            this._blob,
            item.hash,
            this._path.push(name),
            name
        );
    }

    child(name: string): DirItem | FileItem | null {
        const listNodes = this._listNodes;

        if (listNodes !== null) {
            const hashChild = listNodes.get(name);
            if (hashChild) {
                return this._mapNode(name, hashChild);
            }
        }

        return null;
    }

    get parent(): DirItem | null {
        return this._parent;
    }
}
