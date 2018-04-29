//@flow

import * as React from 'react';
import { Map as IMap } from 'immutable';
import { observable } from 'mobx';
import { observer } from 'mobx-react';
import glamorous from 'glamorous';
import { Store } from '../Store';
import type { NodeItemType } from '../Store/Type';
import { DirItem } from '../Store/Root/DirItem';

const Main = glamorous.div({
    cursor: 'pointer'
});

type DirListItemPropsType = {|
    store: Store,
    name: string,
    is_dir: bool,
    hash: string
|};

@observer
export class DirListItem extends React.Component<DirListItemPropsType> {
    render(): React.Node {
        const { name, is_dir } = this.props;
        return (
            <Main onClick={this._onClick}>
                <div>name: {name}</div>
            </Main>
        )
    }

    _onClick = () => {
        const { store, name } = this.props;
        const current = store.path.value;
        store.path.goTo(current.push(name));
    }
}
