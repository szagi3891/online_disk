import * as React from 'react';
import { observer } from 'mobx-react';
import styled from 'react-emotion';
import { Store } from '../Store';
import { PathItem } from './PathItem';

const Main = styled('div')`
    display: flex;
`;

interface PropsType {
    store: Store,
    className: string
}

@observer
export class Path extends React.Component<PropsType> {

    render() {
        const { store, className } = this.props;
        const currentPathNodes = store.root.currentPathNodes;

        const out = [];

        for (const item of currentPathNodes.path) {
            out.push(
                <PathItem
                    key={item.path.join('/')}
                    caption={item.name}
                    store={store}
                    path={item.path}
                    idDir={true}
                />
            );
        }

        const fileItem = currentPathNodes.last;

        if (fileItem) {
            out.push(
                <PathItem
                    key={fileItem.path.join('/')}
                    caption={fileItem.name}
                    store={store}
                    path={fileItem.path}
                    idDir={false}
                />
            );
        }

        return (
            <Main className={className}>
                { out }
            </Main>
        );
    }
}
