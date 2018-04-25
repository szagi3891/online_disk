//@flow

import * as React from 'react';
import { Map as IMap } from 'immutable';
import { observable } from 'mobx';
import { observer } from 'mobx-react';
import { Store } from '../Store';
import type { NodeItemType } from '../Store/Type';

type DirListItemPropsType = {|
    name: string,
    is_dir: bool,
    hash: string
|};

@observer
class DirListItem extends React.Component<DirListItemPropsType> {
    render(): React.Node {
        const { name, is_dir } = this.props;
        return (
            <div>
                <div>name: {name}</div>
            </div>
        )
    }
}

type PropsType = {|
    store: Store,
    hash: string
|};

@observer
export class DirList extends React.Component<PropsType> {
    render(): React.Node {
        const { store, hash } = this.props;

        const list = store.dir.getDir(hash);

        if (!list) {
            return (
                <div>
                    <i>Loading...</i>
                </div>
            );
        }

        //export type NodeItemType = {|
        /*
            +is_dir: bool,
            +hash: string,
        |};
        */

        return (
            <div>
                { this._renderList(list) }
            </div>
        );
    }

    _renderList(list: IMap<string, NodeItemType>): React.Node {
        const out = [];
        console.info('AAAA', list.toJS());
        for (const [name, node] of list.entries()) {
            out.push(
                this._renderListItem(name, node)
            );
        }
        return out;
    }

    _renderListItem = (name: string, node: NodeItemType): React.Node => {
        return (
            <div key={node.hash}>
                {name} -- { node.is_dir ? 'dir' : 'file'}
            </div>
        );
    }
}
