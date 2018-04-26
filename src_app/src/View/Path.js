//@flow

import * as React from 'react';
import { List as IList } from 'immutable';
import { observer } from 'mobx-react';
import glamorous from 'glamorous';
import { Store } from '../Store';

const PathItemSpan = glamorous.span({
    paddingRight: '5px',
    cursor: 'pointer',
    ':hover': {
        textDecoration: 'underline',
        opacity: '0.5'
    }
});

type PathItemPropsType = {|
    store: Store,
    fullPath: IList<string>,
    amountItem: number,
|};

class PathItem extends React.Component<PathItemPropsType> {
    render(): React.Node {
        const currentPath = this._currentPath();
        const caption = currentPath.last();
        return (
            <PathItemSpan onClick={this._onClick}>
                { caption }
            </PathItemSpan>
        );
    }

    _currentPath(): IList<string> {
        const { fullPath, amountItem } = this.props;
        return fullPath.slice(0, amountItem);
    }

    _onClick = () => {
        const { store } = this.props;
        store.path.goTo(this._currentPath());
    }
}

type PropsType = {|
    store: Store
|};

@observer
export class Path extends React.Component<PropsType> {
    render(): React.Node {
        const { store } = this.props;
        const fullPath = store.path.value;

        const out = [];

        for (let amountItem = 1; amountItem <= fullPath.size; amountItem++) {
            const key = `${fullPath.join('/')}--${amountItem}`;
            out.push(
                <PathItem
                    key={key}
                    store={store}
                    fullPath={fullPath}
                    amountItem={amountItem}
                />
            );
        }

        return (
            <div>
                path: { out }
            </div>
        );
    }
}
