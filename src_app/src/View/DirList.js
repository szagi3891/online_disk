//@flow

import * as React from 'react';
import { observer } from 'mobx-react';
import { Store } from '../Store';
import { DirItem } from '../Store/Root/DirItem';
import { DirListItem } from './DirListItem';

type PropsType = {|
    className: string,
    store: Store,
    dirItem: DirItem
|};

@observer
export class DirList extends React.Component<PropsType> {
    render(): React.Node {
        const { className, store, dirItem } = this.props;

        const childList = dirItem.childList;

        if (!childList) {
            return (
                <div className={className}>
                    <i>Loading...</i>
                </div>
            );
        }

        const out = [];

        const parent = dirItem.parent;

        if (parent) {
            out.push((
                <DirListItem
                    store={store}
                    name={null}
                    node={parent}
                />
            ));
        }

        for (const [name, node] of childList.entries()) {
            out.push((
                <DirListItem
                    store={store}
                    name={name}
                    node={node}
                />
            ));
        }
        
        return (
            <div className={className}>
                { out }
            </div>
        );
    }
}
