//@flow

import * as React from 'react';
import { observer } from 'mobx-react';
import glamorous from 'glamorous';
import rgba from 'hex-rgba';
import { Store } from '../Store';
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

const renderName = (name: string) => {
    return <div>{name}</div>;
};

type PropsType = {|
    store: Store,
    name: string | null,
    node: DirItem | FileItem
|};

@observer
export class DirListItem extends React.Component<PropsType> {
    render(): React.Node {
        const { name } = this.props;
        if (name === null) {
            return (
                <Main onClick={this._onClick}>
                    { renderName('..') }
                </Main>
            );
        }
        return (
            <Main onClick={this._onClick}>
                { renderName(name) }
                { this._renderDeleteOption() }
            </Main>
        );
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
