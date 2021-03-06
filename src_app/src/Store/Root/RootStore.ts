import { computed } from 'mobx';
import { List as IList } from 'immutable';
import { HeadStore } from 'Store/HeadStore';
import { BlobStore } from 'Store/Blob/BlobStore';
import { DirItem } from 'Store/Root/DirItem';
import { FileItem } from 'Store/Root/FileItem';
import { PathStore } from 'Store/PathStore';

const findItemDir = (parent: DirItem, currentPath: IList<string>): [IList<DirItem>, null | FileItem] => {
    const first = currentPath.first();

    if (typeof first === 'string') {
        const nextChild = parent.child(first);

        if (nextChild === null) {
            return [IList(), null];
        }

        if (nextChild instanceof FileItem) {
            return [IList(), nextChild];
        }
        
        if (nextChild instanceof DirItem) {
            const [dirList, lastElement] = findItemDir(nextChild, currentPath.shift());
            return [dirList.unshift(nextChild), lastElement];
        }

        throw Error('Store.Root.findItemDir: Nieobsłużone odgałęzienie');
    }

    return [IList(), null];
};

interface CurrentPathNodesType {
    path: IList<DirItem>,
    last: null | FileItem
}

export class RootStore {
    private readonly _head: HeadStore;
    private readonly _path: PathStore;
    private readonly _blob: BlobStore;

    constructor(head: HeadStore, path: PathStore) {
        this._head = head;
        this._path = path;
        this._blob = new BlobStore();
    }

    @computed get root(): DirItem | null {
        const head = this._head.head;

        if (head === null) {
            return null;
        }

        return new DirItem(null, this._head, this._blob, head, IList(), 'ROOT');
    }

    @computed get currentPathNodes(): CurrentPathNodesType {
        const root = this.root;
        if (!root) {
            return {
                path: IList(),
                last: null
            };
        }

        const [path, last] = findItemDir(root, this._path.value);
        return {
            path: path.unshift(root),
            last
        };
    }
}
