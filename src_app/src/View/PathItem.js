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

type PropsType = {|
    store: Store,
    fullPath: IList<string>,
    amountItem: number,
|};

@observer
export class PathItem extends React.Component<PropsType> {
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
