import { List as IList } from 'immutable';
import { BlobStore } from '../Blob/BlobStore';
import { HeadStore } from '../HeadStore';

export class FileItem {
    readonly _head: HeadStore;
    readonly _blob: BlobStore
    readonly _hash: string;
    readonly _path: IList<string>;
    readonly _name: string;         //Nazwa pliku. Jego rozszerzenie określa zawartość

    constructor(head: HeadStore, blob: BlobStore, hash: string, path: IList<string>, name: string) {
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
}
