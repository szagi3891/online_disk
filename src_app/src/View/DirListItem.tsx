import * as React from 'react';
import { observer } from 'mobx-react';
import styled from 'react-emotion';
//@ts-ignore TODO
import rgba from 'hex-rgba';
import { Store } from 'Store';
import { DirItem } from 'Store/Root/DirItem';
import { FileItem } from 'Store/Root/FileItem';
import { DirListItemName } from 'View/DirListItemName';

const backgroundColor = '#e0e0e0';

const Main = styled('div')`
    display: flex;
    justify-content: space-between;
    padding-left: 5px;
    padding-right: 5px;
    cursor: pointer;
    background-color: ${backgroundColor};
    margin-bottom: 3px;
    border: 1px solid transparent;
    &:hover {
        border: 1px solid blue;
        background-color: rgba(${backgroundColor}, 50);
    }
`;

const OptionDiv = styled('div')`
    &:hover {
        color: blue;
    }
`;

interface PropsType {
    store: Store,
    name: string | null,
    node: DirItem | FileItem
}

@observer
export class DirListItem extends React.Component<PropsType> {
    render() {
        const { name, node } = this.props;
        const isDir = node instanceof DirItem;

        if (name === null) {
            return (
                <Main onClick={this._onClick}>
                    <DirListItemName name=".." isDir={isDir} />
                </Main>
            );
        }
        return (
            <Main onClick={this._onClick}>
                <DirListItemName name={name} isDir={isDir} />
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

    _onDelete = (event: React.SyntheticEvent) => {
        event.stopPropagation();
        console.info('Kasuję element', this.props.name);
    }
}
