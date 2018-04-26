//@flow
import { action, computed, observable } from "mobx";
import { List as IList } from 'immutable';

export class PathStore {
    @observable _path: IList<string>;

    constructor() {
        this._path = IList();
    }

    get value(): IList<string> {
        return this._path;
    }

    @computed get serialized(): string {
        if (this._path.size === 0) {
            return '';
        }

        return `/${this._path.join('/')}`;
    }

    @action goTo(newPath: IList<string>) {
        this._path = newPath;
    }
}
