//@flow

import * as React from 'react';
import { Map as IMap } from 'immutable';
import { observable } from 'mobx';
import { observer } from 'mobx-react';
import glamorous from 'glamorous';
import { Store } from '../Store';
import type { NodeItemType } from '../Store/Type';
import { DirItem } from '../Store/Root/DirItem';
import { FileItem } from '../Store/Root/FileItem';

const Main = glamorous.div({
    cursor: 'pointer'
});

type DirListItemPropsType = {|
    store: Store,
    name: string | null,
    node: DirItem | FileItem
|};

@observer
export class DirListItem extends React.Component<DirListItemPropsType> {
    render(): React.Node {
        const { name } = this.props;
        return (
            <Main onClick={this._onClick}>
                <div>name: {name !== null ? name : '..'}</div>
            </Main>
        )
    }

    _onClick = () => {
        const { store, node } = this.props;
        store.path.goTo(node.path);
    }
}
