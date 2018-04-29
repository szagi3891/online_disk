//@flow

import * as React from 'react';
import { Map as IMap } from 'immutable';
import { observable } from 'mobx';
import { observer } from 'mobx-react';
import { Store } from '../Store';
import type { NodeItemType } from '../Store/Type';
import { DirItem } from '../Store/Root/DirItem';
import { DirListItem } from './DirListItem';

type PropsType = {|
    store: Store,
    dirItem: DirItem
|};

@observer
export class DirList extends React.Component<PropsType> {
    render(): React.Node {
        const { store, dirItem } = this.props;

        const list = dirItem.value;

        if (!list) {
            return (
                <div>
                    <i>Loading...</i>
                </div>
            );
        }

        const out = [];

        for (const [name, node] of list.entries()) {
            out.push((
                <DirListItem
                    store={store}
                    name={name}
                    is_dir={node.is_dir}
                    hash={node.hash}
                />
            ));
        }
        
        return (
            <div>
                { out }
            </div>
        );
    }
}
