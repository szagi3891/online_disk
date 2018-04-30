//@flow

import * as React from 'react';
import { Map as IMap } from 'immutable';
import { observable } from 'mobx';
import { observer } from 'mobx-react';
import glamorous from 'glamorous';
import rgba from 'hex-rgba';
import { Store } from '../Store';
import type { NodeItemType } from '../Store/Type';
import { DirItem } from '../Store/Root/DirItem';
import { FileItem } from '../Store/Root/FileItem';

const backgroundColor = '#e0e0e0';

const Main = glamorous.div({
    display: 'flex',
    justifyContent: 'space-between',
    paddingLeft: '5px',
    paddingRight: '5px',
    cursor: 'pointer',
    backgroundColor: backgroundColor,
    marginBottom: '3px',
    border: '1px solid transparent',
    ':hover': {
        border: '1px solid blue',
        backgroundColor: rgba(backgroundColor, 50)
    }
});

const OptionDiv = glamorous.div({
    ':hover': {
        color: 'blue'
    }
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
        if (name === null) {
            return (
                <Main onClick={this._onClick}>
                    { this._renderName('..') }
                </Main>
            );
        } else {
            return (
                <Main onClick={this._onClick}>
                    { this._renderName(name) }
                    { this._renderDeleteOption() }
                </Main>
            );
        }
    }

    _renderName(name: string) {
        return <div>{name}</div>;
    }

    _renderDeleteOption() {
        return <OptionDiv onClick={this._onDelete}>Delete</OptionDiv>;
    }

    _onClick = () => {
        const { store, node } = this.props;
        store.path.goTo(node.path);
    }

    _onDelete = (event: SyntheticEvent<>) => {
        event.stopPropagation();
        console.info('Kasuję element', this.props.name);
    }
}
